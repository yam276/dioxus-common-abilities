(() => {
    const scopeId = __DIOXUS_FOCUS_SCOPE_ID__;
    const registryKey = "__dioxusFocusScopeV1";
    const state = window[registryKey];
    if (!state) return;

    const index = state.stack.indexOf(scopeId);
    const scope = state.scopes.get(scopeId);
    const wasActive = index === state.stack.length - 1;
    if (index >= 0) state.stack.splice(index, 1);
    state.scopes.delete(scopeId);

    if (wasActive && scope) {
        queueMicrotask(() => {
            const restore = scope.restoreCandidates.find(state.canRestore);
            if (restore) {
                restore.focus();
                return;
            }
            const parentId = state.stack[state.stack.length - 1];
            state.focusInside(state.scopes.get(parentId));
        });
    }

    if (state.stack.length === 0) {
        document.removeEventListener("keydown", state.handleKeydown, true);
        delete window[registryKey];
    }
})();
