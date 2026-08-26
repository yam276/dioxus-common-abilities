(() => {
    const scopeId = __DIOXUS_FOCUS_SCOPE_ID__;
    const initialId = __DIOXUS_FOCUS_INITIAL_ID__;
    const registryKey = "__dioxusFocusScopeV1";
    const root = document.getElementById(scopeId);
    if (!(root instanceof HTMLElement)) return;

    let state = window[registryKey];
    if (!state) {
        const selector = [
            "a[href]",
            "area[href]",
            "button",
            "input:not([type='hidden'])",
            "select",
            "textarea",
            "iframe",
            "object",
            "embed",
            "audio[controls]",
            "video[controls]",
            "[contenteditable='true']",
            "[tabindex]",
        ].join(",");

        const isVisible = (element) => {
            const style = window.getComputedStyle(element);
            return style.visibility !== "hidden"
                && style.display !== "none"
                && element.getClientRects().length > 0;
        };
        const isDisabled = (element) => element.matches(":disabled")
            || element.closest("[inert]") !== null
            || element.closest("[aria-hidden='true']") !== null;
        const isTabbable = (element) => element instanceof HTMLElement
            && element.tabIndex >= 0
            && !isDisabled(element)
            && isVisible(element);
        const tabbables = (scopeRoot) => Array.from(scopeRoot.querySelectorAll(selector))
            .filter(isTabbable);
        const canRestore = (element) => element instanceof HTMLElement
            && element.isConnected
            && !isDisabled(element)
            && isVisible(element);
        const focusInside = (scope) => {
            if (!scope || !scope.root.isConnected) return;
            const preferred = scope.initialId
                ? document.getElementById(scope.initialId)
                : null;
            if (preferred instanceof HTMLElement
                && scope.root.contains(preferred)
                && !isDisabled(preferred)
                && isVisible(preferred)) {
                preferred.focus();
                return;
            }
            const first = tabbables(scope.root)[0];
            (first || scope.root).focus();
        };

        state = {
            stack: [],
            scopes: new Map(),
            tabbables,
            canRestore,
            focusInside,
            handleKeydown: null,
        };
        state.handleKeydown = (event) => {
            if (event.key !== "Tab" || event.defaultPrevented) return;
            const activeId = state.stack[state.stack.length - 1];
            const scope = state.scopes.get(activeId);
            if (!scope || !scope.root.isConnected) return;

            const candidates = state.tabbables(scope.root);
            if (candidates.length === 0) {
                event.preventDefault();
                scope.root.focus();
                return;
            }

            const active = document.activeElement;
            const first = candidates[0];
            const last = candidates[candidates.length - 1];
            if (!scope.root.contains(active)) {
                event.preventDefault();
                (event.shiftKey ? last : first).focus();
            } else if (event.shiftKey && (active === first || !candidates.includes(active))) {
                event.preventDefault();
                last.focus();
            } else if (!event.shiftKey && (active === last || !candidates.includes(active))) {
                event.preventDefault();
                first.focus();
            }
        };
        document.addEventListener("keydown", state.handleKeydown, true);
        window[registryKey] = state;
    }

    const priorActiveId = state.stack[state.stack.length - 1];
    const parent = state.scopes.get(priorActiveId);
    const opener = document.activeElement instanceof HTMLElement
        ? document.activeElement
        : null;
    const restoreCandidates = [
        opener,
        ...(parent ? parent.restoreCandidates : []),
    ].filter((element, index, all) => element && all.indexOf(element) === index);

    state.stack = state.stack.filter((id) => id !== scopeId);
    state.scopes.set(scopeId, {
        root,
        initialId,
        restoreCandidates,
    });
    state.stack.push(scopeId);
    queueMicrotask(() => {
        if (state.stack[state.stack.length - 1] === scopeId) {
            state.focusInside(state.scopes.get(scopeId));
        }
    });
})();
