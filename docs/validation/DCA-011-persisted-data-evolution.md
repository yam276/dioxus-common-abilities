# DCA-011 Persisted-data Evolution Validation

Status：in progress

Catalog entry：`DCA-011`

## Objective

驗證 `templates/PERSISTED_DATA_CHANGE_CHECKLIST.md` 是否能同時服務：

1. Deductree 的 public mystery file contract；
2. Gentle 的 database migration + backup compatibility contract；
3. 若前兩者通過，再以 OxDM config + keyring split 作 opposing third case。

Validation 的目標是證明 checklist 的問題與順序可共通，不是統一 schema、codec、版本值
或 compatibility promise。

## Hypothesis

三類產品都需要先回答相同的高階問題：

- 哪一個 persisted identity 正在改變；
- 誰擁有該 identity，source of truth 在哪裡；
- old-to-new 與 new-to-old 各自承諾什麼；
- migration、default、unknown data、resource bounds 與 failure UX 如何處理；
- 哪個 fixture／negative test 證明合約；
- package version 是否與 format/schema identity 被錯誤綁在一起。

若 checklist 必須知道 mystery、album、camera 或 Visual Novel domain type，hypothesis 失敗。

## Authoritative inputs

### Deductree case

- `Deductree/docs/FileContract_V1.md`
- `Deductree/AGENTS.md` 的 Diolama version identity 與 file-contract sections
- 實際 parser/checker/format code，只在填寫時依 checklist 要求讀取

### Gentle case

- `gentle/gentle-core/CLAUDE.md` 的 Data contract 與 Schema change checklist
- `gentle/gentle-core/migrations/`
- `gentle/gentle-core/migrations/sqlite/`
- backup model、restore code 與 current fixtures，只在填寫時讀取

### Opposing case

- `oxdm/CLAUDE.md` 的 Credentials + persistence
- config、device metadata、keyring persistence code

## Validation scenarios

### Scenario A：additive field

以不破壞舊 reader 的新增欄位為假設，分別填寫 Deductree 與 Gentle：

- 是否需要 bump format/schema/package version；
- default 在 parser、Serde 或 database migration 哪一層生效；
- 舊資料進新程式與新資料進舊程式的結果；
- fixture 與 negative case。

Checklist 若暗示所有 additive field 都使用同一個 bump policy，則不通過。

### Scenario B：rename or structural change

以 released field rename 或 nesting/type change 為假設：

- source version identity；
- explicit migration；
- compatibility refusal/warning；
- rollback/downgrade；
- old fixture replay。

Checklist 必須讓兩個 consumer 表達不同 promise，不可替它們決定策略。

### Scenario C：secret split

以 OxDM ordinary config 與 keyring secret 的變更為例：

- ordinary persisted data 與 secret identity 分開；
- plaintext prohibition；
- legacy config migration；
- missing/locked keyring failure UX；
- logs、backup 與export是否排除 secrets。

若 checklist 把 credentials 當作普通 settings field，則不通過。

## Acceptance criteria

- Deductree 與 Gentle 都能完整填寫，而不刪除產品自己的 compatibility rule。
- 同一問題可得到不同答案；template 不把答案硬編碼成共同 policy。
- Package、database、document/wire、backup/save 與 catalog identities 可以分列。
- Old-to-new 和 new-to-old 不被合併成模糊的「backward compatible」。
- Additive、breaking、unknown input、resource bounds、failure UX 和 rollback 都有位置。
- 每個 change 能指定 authoritative code、fixture、positive test 與 negative test。
- Checklist 不含 Deductree、Gentle、OxDM 或 Diolama domain type。
- Scenario C 能表達 secret storage 與 ordinary persistence 的不同 safety boundary。

## Failure conditions

- 填寫者需要新增大量自由欄位才能描述第二個產品。
- Template 強制一個 format、serializer、path、migration framework 或 SemVer policy。
- Version map 無法表達同一 package 內多個獨立 persisted identities。
- Checklist 只產生文件，沒有具體 fixture、test 或 failure-path verification。
- Consumer 必須降低既有 compatibility 或 security guarantee 才能套用。

## Outputs

- Updated `templates/PERSISTED_DATA_CHANGE_CHECKLIST.md`.
- 三份完成的 validation examples，存於本文件相鄰目錄，且不得包含 private data。
- Catalog decision：維持 `Validating`、升為 `Planned`、拆分或 `Rejected`。
- 若升為 `Planned`，另建 `docs/active/DCA-011-persisted-data-evolution.md`；不要把本
  validation brief 假裝成 implementation plan。

## Current progress

- [x] Candidate、hypothesis、opposing cases 與 acceptance criteria 已固定。
- [x] Checklist prototype 已建立。
- [x] Deductree additive + structural scenarios 已填寫並對照 authoritative code。
- [x] Gentle additive + structural scenarios 已填寫並對照 authoritative code。
- [x] OxDM secret-split scenario 已填寫並對照 authoritative code。
- [x] 根據前兩份實填結果，將 resource bounds 拆為 raw、archive、parsed 三層。
- [x] 根據 OxDM opposing case，加入 secret migration ordering 與 observable write outcome。
- [x] Lifecycle decision：維持 `Validating`；回溯案例已支持 boundary，但仍需在一個真實
  future persisted-data change 中 prospective 使用，才能升為 `Planned`。

## Validation decision

三個 opposing cases 都能使用同一份 template，同時保留完全不同的產品政策：

- Deductree：exact public document-version refusal；
- Gentle：additive DB/backup tolerance 加 structural transform；
- OxDM：unversioned ordinary config 與 keychain secret split。

因此 common invariant 成立，template boundary 暫時接受。`DCA-011` 不升為 `Planned`：
本輪是對既有 decisions 的 retrospective reconstruction，尚未證明 template 能在實作前
攔住一次真實 change。下一個涉及上述三個產品任一 persisted contract 的工作，應先填
template；若它在 coding 前產生可執行的 acceptance criteria 且沒有 product-specific
欄位，才建立 `docs/active/DCA-011-persisted-data-evolution.md`。
