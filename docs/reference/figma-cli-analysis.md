# Figma CLI 解析（アーカイブ）

> このファイルは検討ログの保管用です。`docs/product` と `docs/design` の正本ではありません。
> ここには採用しなかった代替案、旧コマンド名、旧 schema 例が含まれます。
> 最新の判断は `docs/README.md` から辿れる正本を参照してください。

## 使い方

- 日常的な入口は `docs/reference/README.md`
- この本文は「必要になったときに掘る一次ログ」として使う
- まず読むべき節は `# 最終設計`, `# CLI の完成形`, `# テスト戦略`, `# 実装順序`, 末尾の `# 最終判断`
- 途中の旧コマンド名や旧 schema は、そのまま採用しない

## Prompt:

https://github.com/silships/figma-cli
このリポジトリを完全に容赦無く解析してください

## Response:

思考時間: 3m 8s

## 総評

このリポジトリは、**Figma Desktop に直接つないで、REST API ではなく Figma 側の実行環境を叩く CLI** として設計されています。README とアーキテクチャ文書の説明を合わせると、中心思想は「Figma Desktop をパッチし、デバッグ接続経由で Figma Plugin API を直接実行する」ことです。公開メッセージも一貫しており、**“No API key required / full read-write”** を前面に出しています。技術的にはかなり野心的で、用途も広いです。変数、コンポーネント、スロット、解析、lint、export、バッチ処理まで一通り触ろうとしています。 [GitHub+2GitHub+2](https://github.com/silships/figma-cli)

ただし、**設計の鋭さに対して、運用品質と整合性が追いついていません。** 実装思想は面白いが、配布物としては荒いです。公開リポジトリとして見たときの評価は、**「発想は強い、製品としてはまだ不安定」** です。README、CLAUDE.md、REFERENCE.md、ARCHITECTURE.md の間で前提が食い違っており、実利用者が迷う箇所がすでに issue に現れています。 [GitHub+3GitHub+3GitHub+3](https://github.com/silships/figma-cli)

## 何を作ろうとしているか

README の記述から、この CLI は Figma Desktop を Claude Code から自然言語で操作するための橋渡しとして位置づけられています。`fig-start` で Figma 起動、接続、ファイル選択、Claude 起動まで持っていく導線があり、`CLAUDE.md` を Claude に読ませてコマンド知識を補う構成です。つまり、**CLI 本体よりも「CLI + Claude の実行環境」を一つのプロダクトとして売ろうとしている** のが実態です。 [GitHub+2GitHub+2](https://github.com/silships/figma-cli)

これは発想としては筋が通っています。コマンド体系が多くても、`CLAUDE.md` が自然言語とコマンドの対応表になるので、表面上の学習コストを下げられるからです。README でも「Claude knows all commands」「CLAUDE.md を更新すれば新しい振る舞いを教えられる」と明示しています。 [GitHub+1](https://github.com/silships/figma-cli)

## 良い点

### 1\. 発想が明快

Figma の REST API 制約を避け、Desktop 側に入り込んで Plugin API を直接使うという割り切りは明瞭です。README と ARCHITECTURE.md はともに、REST API では届きにくい機能や、より広い read/write を取りにいく意図を示しています。ここは企画の芯がぶれていません。 [GitHub+1](https://github.com/silships/figma-cli)

### 2\. ユースケースの幅が広い

README と REFERENCE.md にあるコマンド群を見る限り、対象は単なるトークン生成に留まりません。variables、components、slots、canvas inspection、lint、export、batch operation まで広げています。表面的な便利コマンド集ではなく、**Figma 操作面を一段抽象化した操作基盤** を狙っています。 [GitHub+2GitHub+2](https://github.com/silships/figma-cli)

### 3\. Safe Mode を用意している

README と CLAUDE.md では、Figma をパッチする Yolo Mode だけでなく、プラグイン経由の Safe Mode も提示しています。パッチ方式一本ではなく、逃げ道を残しているのは妥当です。公開配布物として最低限の防御線はあります。 [GitHub+1](https://github.com/silships/figma-cli)

## 厳しい点

### 1\. ドキュメント整合性が悪い

ここがかなりまずいです。

README では、

- macOS **または Windows** をサポート
- Yolo Mode は **random port / token auth / localhost only**
- Safe Mode があり、plugin-based と説明

と書いています。ところが ARCHITECTURE.md では、

- 接続先は **localhost:9222**
- patch で `--remote-debugging-port=9222` を足す
- **macOS only (for now)**

となっており、README と前提が一致していません。これは「過去の設計書が残っている」のか、「実装が docs に反映されていない」のか、どちらにせよ危険です。利用者は README を信用するので、**運用上の真実がどこにあるのか判別しにくい** 状態です。 [GitHub+1](https://github.com/silships/figma-cli)

### 2\. インストール体験が雑

README のセットアップは `npm install` と `npm run setup-alias` を案内していますが、issue #1 では README か案内文のどこかで `figma-ds-cli init` を促され、実際にはコマンドが見つからず混乱した例が出ています。さらに issue #2 では Figma アプリ本体への書き込み権限で失敗しています。つまり、**初回導線でつまずく要因が、すでに公開 issue として顕在化している** ということです。 [GitHub+3GitHub+3GitHub+3](https://github.com/silships/figma-cli)

### 3\. “Safe” という言葉に対する安心感が弱い

README は Safe Mode を「no patching」として出していますが、CLAUDE.md では **`render-batch` が Safe Mode で text を正しく描けない** とかなり強い制約を書いています。さらに、テキスト込みの複雑なコンポーネントは `eval` でネイティブ Figma API を直接書け、と指示しています。これは実質、**安全な簡易モードというより、別系統の制約付き実行路** です。名称に対して期待値調整が足りません。 [GitHub+1](https://github.com/silships/figma-cli)

### 4\. テストが薄い

`package.json` の test は `node --test tests/*.test.js` のみで、tests ディレクトリには `figma-client.test.js` 1 本しか見えません。その内容も、少なくとも確認できた範囲では URL パターン判定のような表層ロジックが中心です。CLI の中核である接続、パッチ、モード分岐、破壊的操作、変数生成、slot 操作の信頼性を担保するには、**現状のテスト面積はあまりにも狭い** です。 [GitHub+2GitHub+2](https://github.com/silships/figma-cli/blob/main/package.json)

### 5\. プロダクトの境界が曖昧

README は「CLI」、CLAUDE.md は「Claude に読ませる知識ベース」、REFERENCE.md はコマンド一覧、docs は技術文書です。しかし実体としては、

- CLI
- Figma patcher
- daemon/plugin bridge
- Claude 向け運用知識
- 設計規約集

が混在しています。これ自体は悪くありませんが、現状は**責務分離ではなく同居**に見えます。特に `CLAUDE.md` が運用知識・制約・実装上の注意事項まで背負っており、コードと同等に大事な仕様書になっています。これは、**人間向け仕様がコードや正式 docs ではなく AI 向け文書に寄っている** ということで、長期保守ではかなり危ういです。 [GitHub+2GitHub+2](https://github.com/silships/figma-cli/raw/refs/heads/main/CLAUDE.md)

## 技術面の見立て

依存関係はかなり軽く、`chalk` `commander` `ora` `ws` のみです。これは配布の軽さという点では良いです。一方で README と architecture docs は、実質的なコア機能の多くを **figma-use に依存** していると説明しています。つまり、このリポジトリ単体のコード量よりも、**外部前提の上にどれだけうまくラップできているか** が成否を決めます。公開説明上も figma-use への依存は明記されています。 [GitHub+2GitHub+2](https://github.com/silships/figma-cli/blob/main/package.json)

ここで問題になるのは、README が「この CLI が何を保証するのか」より、「何ができるか」の列挙に寄っている点です。できることは多いが、**失敗時の一貫性、復旧方法、Figma 側のバージョン追従、破壊的操作の境界** が見えにくい。Yolo Mode が Figma 本体を patch する以上、この部分は README の末尾ではなく前半に明記すべきです。実際、権限エラーの issue はそこに直結しています。 [GitHub+1](https://github.com/silships/figma-cli)

## 命名と配布の粗さ

リポジトリ名は `figma-cli` ですが、npm package 名は `figma-ds-cli` です。README 見出しも `figma-ds-cli` です。さらにユーザー向け導線は `fig-start` です。名前が三つあります。これは**ブランド設計として弱い**です。ユーザーは「何を install し、何を叩き、何が本体か」を迷います。issue #1 の混乱も、この不統一と無関係ではありません。 [GitHub+2GitHub+2](https://github.com/silships/figma-cli)

加えて、公開リポジトリには `package-lock.json` があり、npm ベースの前提です。これは悪いことではありませんが、開発体験として現代的かというと平凡です。依存が軽いので深刻ではないものの、複数 OS・複数接続モード・Figma patch を扱う配布物なら、**インストールの再現性と環境診断の仕組み** にもっと投資したほうがよいです。 [GitHub+1](https://github.com/silships/figma-cli)

## コミュニティ運営の弱さ

GitHub の community standards では、README と license はある一方で、description、code of conduct、contributing、security policy、issue templates、pull request template が不足しています。公開 OSS として採用を広げるなら、この状態は弱いです。特にこの種のツールは、**破壊的操作・ローカルパッチ・権限問題** を含むので、security policy や contributing guide が無いのは印象が悪いです。 [GitHub](https://github.com/silships/figma-cli/community?utm_source=chatgpt.com)

## 容赦のない結論

このリポジトリは、**発想が先に立ち、製品としての詰めがまだ甘い** です。  
良い意味では、Figma Desktop を Claude から直に操作するという方向は鋭い。  
悪い意味では、**「動けば強い」段階から「他人が安定して使える」段階へ移れていない**。 [GitHub+3GitHub+3GitHub+3](https://github.com/silships/figma-cli)

### 一言で切ると

- **企画**: 強い
- **実装方針**: 面白い
- **公開配布品質**: 甘い
- **ドキュメント管理**: 雑
- **テスト戦略**: 弱い
- **将来性**: ある
- **現時点の信頼性**: 慎重評価 [GitHub+3GitHub+3GitHub+3](https://github.com/silships/figma-cli/blob/main/package.json)

### 改善優先順位

1.  README / ARCHITECTURE / CLAUDE / REFERENCE の事実関係を揃える。
2.  Yolo Mode の危険性と復旧手順を前面に出す。
3.  install / init / alias / executable 名称を一本化する。
4.  Safe Mode の機能差分を表にして明示する。
5.  接続・patch・rollback・変数生成・slot 操作の統合テストを増やす。
6.  OSS としての最低限の運営文書を入れる。 [GitHub+4GitHub+4GitHub+4](https://github.com/silships/figma-cli)

GitHub 側の取得制限で、今回は一部ソースファイル本文を十分に追えませんでした。そのため、**コード行単位の欠陥監査というより、公開情報・文書・見えているテスト・issue からの実戦的評価** になっています。そこまで含めても、評価は上のとおりです。

## Prompt:

このリポジトリと同じcliをrustで書きたいです

## Response:

思考時間: 1m 22s

できます。  
しかも、このリポジトリは Rust へ移す題材として扱いやすい構成です。理由は、現状の Node 側が比較的薄く、CLI・接続・パッチ・デーモンの責務が分かれているからです。`src` 直下は `index.js`, `figma-client.js`, `figma-patch.js`, `daemon.js`, `figjam-client.js` の 5 本で、`package.json` の依存も `chalk`, `commander`, `ora`, `ws` の 4 つしかありません。公開されているコマンド体系も、`connect`, `daemon`, `files`, `fj`, `render`, `lint`, `export-*` などに整理されています。 [GitHub+2GitHub+2](https://github.com/silships/figma-cli/tree/main/src)

## 先に結論

Rust 版は、次の 3 層に切って作るのがよいです。

1.  **CLI 層**  
    `clap` でサブコマンドを定義する。
2.  **接続層**
    - Yolo Mode: Figma Desktop の CDP 接続
    - Safe Mode: ローカル daemon と plugin bridge
3.  **実行層**  
    Figma に送る操作を `Command` として統一し、モード差分を吸収する

この構成にしておくと、Node 版で README 上「両モードとも同じコマンド体系」と説明している前提を Rust 側でも維持しやすくなります。README では Yolo Mode は直接 CDP、Safe Mode は plugin bridge 経由で、表面のコマンドは共通とされています。 [GitHub+1](https://github.com/silships/figma-cli)

---

## 何をそのまま写すべきか

Node 版から移植対象として見るべきものは、見た目のコマンド名ではなく、次の 5 点です。

### 1\. 接続モードの二重化

README では、

- Yolo Mode: Figma をパッチして CDP 接続
- Safe Mode: plugin-based 接続

という二系統です。Rust 版でもこれを保つべきです。ここを崩すと「同じ CLI」ではなく別物になります。 [GitHub+1](https://github.com/silships/figma-cli)

### 2\. daemon 前提

REFERENCE には `daemon start|stop|restart|status|diagnose|reconnect` があり、トークンファイルや認証失敗時の復旧手順もあります。単発 CLI ではなく、**ローカル常駐プロセスを中心に据えた設計** です。Rust 版でも daemon を別サブコマンドではなく、最初から中核として扱うべきです。 [GitHub+1](https://github.com/silships/figma-cli/blob/main/REFERENCE.md)

### 3\. コマンド表面の互換

README/REFERENCE 上の利用者体験は、

- `connect`
- `files`
- `daemon ...`
- `fj ...`
- `render`
- `render-batch`
- `lint`
- `analyze ...`
- `export-jsx`
- `export-storybook`

という系統です。CLI 互換を狙うなら、まず名前と引数順序を合わせるのが先です。内部実装は後から差し替えればよいです。 [GitHub+1](https://github.com/silships/figma-cli/blob/main/REFERENCE.md)

### 4\. 認証トークンとローカル制約

README では daemon 側に、

- ランダム 32-byte token
- localhost 限定
- idle timeout
- トークンファイル保存

という前提があります。Rust 版でもここはそのまま持つべきです。 [GitHub](https://github.com/silships/figma-cli)

### 5\. `fig-start` に相当する導線

Node 版は CLI 本体とは別に `fig-start` を置いています。`package.json` の `bin` にも `figma-ds-cli` と `fig-start` の二つがあります。Rust でも `figma-cli` 本体だけでなく、利用開始のための薄いランチャーコマンドを分けた方がよいです。 [GitHub](https://github.com/silships/figma-cli/blob/main/package.json)

---

## Rust での設計案

## 推奨クレート構成

```
figma-cli-rs/
├─ Cargo.toml
├─ crates/
│  ├─ figma-cli/          # clap でサブコマンド定義
│  ├─ figma-core/         # 共通型、コマンド、エラー、設定
│  ├─ figma-cdp/          # Yolo Mode: CDP/WebSocket
│  ├─ figma-daemon/       # daemon HTTP/WebSocket server
│  ├─ figma-plugin-bridge/# Safe Mode bridge
│  ├─ figma-patch/        # Figma app patch / restore
│  └─ figma-ops/          # render/lint/analyze/export/fj の実装
└─ plugin/                # Figma plugin 側
```

単一 crate でも書けますが、この CLI は接続方式と実行対象が多いので、最初から分けた方が崩れにくいです。

---

## 推奨依存

### CLI と基盤

- `clap` — サブコマンド
- `tokio` — 非同期実行
- `anyhow` / `thiserror` — エラー整理
- `tracing`, `tracing-subscriber` — ログ
- `serde`, `serde_json` — JSON
- `directories` — 設定パス
- `rand` または `getrandom` — トークン生成

### 通信

- `tokio-tungstenite` — WebSocket
- `reqwest` — HTTP
- `axum` または `warp` — daemon server

### OS 操作

- `sysinfo` — Figma プロセス検出
- `which` — バイナリ検出
- `tempfile` — 一時ファイル
- `sha2` — パッチ前後の検証
- `fs_extra` または標準 `std::fs`

### パッチ処理

Figma Desktop の `app.asar` を本当に書き換えるなら、必要に応じて外部ツール連携も考えられます。ただし Rust 単体でやるなら、ASAR 展開処理は自前実装か既存 crate の見極めが必要です。ここは無理に純 Rust に閉じず、**初期段階では外部補助コマンドを呼ぶ設計**でも構いません。

---

## コア抽象

Rust 版で一番先に固めるべきものは、Figma に対する操作を表す共通コマンドです。

```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FigmaCommand {
    Connect { safe: bool },
    ListFiles,
    Eval { code: String },
    Render { jsx: String },
    RenderBatch { items: Vec<String> },
    AnalyzeColors,
    AnalyzeTypography,
    LintAccessibility,
    ExportJsx { node_id: Option<String> },
    FigJamSticky { text: String, x: f32, y: f32 },
    DaemonStatus,
    // ...
}
```

そのうえで実行器を二つ持ちます。

```
#[async_trait::async_trait]
pub trait Executor {
    async fn execute(&self, cmd: FigmaCommand) -> anyhow::Result<serde_json::Value>;
}
```

- `CdpExecutor`
- `PluginExecutor`

こうしておけば、CLI 層はモード差を知らずに済みます。

---

## どこから着手するか

全部を一気に移すと破綻します。順番はこうです。

## 第1段階: 骨格だけ作る

この段階では、機能の多さを追わず、以下だけ作ります。

- `connect`
- `files`
- `daemon start|stop|status`
- `eval`
- `--safe`

これで接続方式、常駐プロセス、実行経路の骨格が揃います。

### `clap` 例

```
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "figma-cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Connect {
        #[arg(long)]
        safe: bool,
    },
    Files,
    Eval {
        code: String,
    },
    Daemon {
        #[command(subcommand)]
        command: DaemonCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum DaemonCommands {
    Start,
    Stop,
    Status,
    Restart,
    Diagnose,
}
```

---

## 第2段階: daemon を先に完成させる

Node 版の README/REFERENCE から見ても、daemon は補助ではなく基盤です。  
Rust ではここを先に固めた方がよいです。

### daemon の責務

- ローカル HTTP または WebSocket server を立てる
- ランダムトークンを発行する
- `~/.figma-ds-cli` 相当の設定ディレクトリに token/state を置く
- executor を抱える
- idle timeout で自動停止する

### 管理ファイル

```
~/.config/figma-cli/
  config.toml
  daemon.json
  token
  logs/
```

macOS なら `~/Library/Application Support/figma-cli/`、Windows なら `%APPDATA%` を `directories` crate で吸収できます。

---

## 第3段階: Yolo Mode

README では、Yolo Mode は CDP 接続で、Figma Desktop に debug port を有効化して接続する設計です。Rust 側では次の要素が必要です。 [GitHub](https://github.com/silships/figma-cli)

### 必要な責務

- Figma プロセス検出
- Figma のインストール先推定
- パッチ適用
- 利用可能ポート決定
- CDP WebSocket URL の取得
- ページまたはターゲット選択
- JavaScript 評価

### 中核 API

```
pub struct CdpClient {
    ws: tokio_tungstenite::WebSocketStream<...>,
}

impl CdpClient {
    pub async fn evaluate(&mut self, expression: &str) -> anyhow::Result<serde_json::Value>;
    pub async fn list_targets(&mut self) -> anyhow::Result<Vec<Target>>;
}
```

### 注意点

ここは Node 版の「面白いところ」ですが、同時に事故の入口でもあります。  
Rust 化で改善すべき点は次です。

- パッチ前バックアップを必ず取る
- バイナリまたは `app.asar` のハッシュ検証を行う
- Figma バージョン差分でパターンが変わったら中止する
- `restore` コマンドを用意する
- dry-run を持つ

Node 版の README でも patch が前提で、macOS 権限問題や Windows 管理者権限の話が出ています。Rust では、そこを曖昧にせず失敗理由を明示するべきです。 [GitHub](https://github.com/silships/figma-cli)

---

## 第4段階: Safe Mode

README と REFERENCE では、Safe Mode は plugin bridge です。Rust 側では、plugin と daemon の間に JSON-RPC 風のプロトコルを定義して固定してください。そうしないと後で壊れます。 [GitHub+1](https://github.com/silships/figma-cli)

### 例

```
{
  "id": "req_123",
  "method": "render",
  "params": {
    "jsx": "<Frame><Text>Hello</Text></Frame>"
  }
}
```

応答:

```
{
  "id": "req_123",
  "ok": true,
  "result": {
    "nodeId": "1:23"
  }
}
```

### plugin 側で持つべきもの

- command dispatcher
- figma plugin API 呼び出し
- timeout 制御
- payload サイズ制限
- エラー整形

README では Safe Mode も full parity と書いていますが、REFERENCE では `render-batch` の重い処理では分割実行や `eval` を促しています。Rust 版では、「表面上は共通、内部で制限を持つ」という現実に合わせて、**コマンドごとの capability table** を実装しておく方がよいです。 [GitHub+1](https://github.com/silships/figma-cli)

---

## Rust 版で Node 版より改善できる点

## 1\. 型で守れる

Node 版は CLI と bridge が文字列中心になりやすいです。Rust なら、

- NodeId
- FileKey
- Color
- VariableMode
- Port
- AuthToken

を型にできます。これだけで事故が減ります。

```
pub struct NodeId(String);
pub struct AuthToken(String);
pub struct Port(u16);
```

---

## 2\. エラーがまともになる

Node 製 CLI は、失敗時に「何が壊れたか」が曖昧になりがちです。  
Rust ではエラーを分けてください。

```
#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Figma Desktop が見つかりません")]
    FigmaNotFound,
    #[error("Figma patch に失敗しました: {0}")]
    PatchFailed(String),
    #[error("CDP 接続に失敗しました: {0}")]
    CdpConnectFailed(String),
    #[error("daemon 認証に失敗しました")]
    Unauthorized,
    #[error("plugin が未接続です")]
    PluginNotConnected,
}
```

---

## 3\. テストを書きやすい

元リポジトリは、公開情報から見える範囲ではテスト面積がかなり狭いです。Rust 版では次を最初から入れてください。`package.json` 上も test は 1 本の Node test runner 実行だけです。 [GitHub](https://github.com/silships/figma-cli/blob/main/package.json)

### テスト方針

- 単体テスト  
  コマンドパース、設定読み書き、トークン検証
- 結合テスト  
  daemon 起動、token auth、mock plugin 接続
- スナップショットテスト  
  `--help`, `status`, `diagnose` の出力
- 疑似 CDP テスト  
  mock WebSocket server に対して evaluate を送る

Figma 実機が要る部分は別ジョブに分離した方がよいです。

---

## 現実的な実装範囲

「同じ CLI」を最短で成立させるなら、全部は不要です。優先順位はこうです。

### 先に作る

- `connect`
- `connect --safe`
- `daemon start|stop|status|restart|diagnose`
- `files`
- `eval`
- `fj list|sticky|shape|connect`
- `render`
- `render-batch`

### 後回しでよい

- tokens/variables の高級操作
- lint の細かいルール群
- export 系の派生フォーマット
- website recreation 系
- image/remove-bg 系

REFERENCE の公開コマンド数は多いですが、土台は上の集合で十分です。 [GitHub](https://github.com/silships/figma-cli/blob/main/REFERENCE.md)

---

## 私ならこう切ります

## フェーズ A: 互換 CLI

Node 版と同じ表面 API を Rust で再現する。

- コマンド名互換
- 設定ファイル互換
- token file 互換
- daemon API 互換

Node 版と併用しながら差し替え可能にする。

## フェーズ B: 中身を Rust 化

- CDP client
- patcher
- plugin bridge
- common command executor

## フェーズ C: 掃除

- 使っていない互換レイヤー削除
- `fig-start` 相当の UX 再設計
- Windows/macOS 差分吸収
- ログと診断の強化

---

## Cargo workspace の最小ひな型

```
[workspace]
members = [
  "crates/figma-cli",
  "crates/figma-core",
  "crates/figma-cdp",
  "crates/figma-daemon",
  "crates/figma-plugin-bridge",
  "crates/figma-patch",
  "crates/figma-ops"
]
resolver = "2"
```

CLI crate:

```
[package]
name = "figma-cli"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "figma-cli"
path = "src/main.rs"

[[bin]]
name = "fig-start"
path = "src/bin/fig_start.rs"

[dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
figma-core = { path = "../figma-core" }
figma-daemon = { path = "../figma-daemon" }
```

---

## `fig-start` の Rust 化

Node 版では `fig-start` が導線です。Rust 版でも別バイナリにした方がよいです。`package.json` でもそうなっています。 [GitHub](https://github.com/silships/figma-cli/blob/main/package.json)

### `fig-start` がやること

- 設定確認
- Figma 起動確認
- daemon 起動
- 接続モード選択
- open files 一覧取得
- 対話選択
- 必要なら Claude Code を起動

Claude 起動連携まで含めるなら、`std::process::Command` で呼べます。  
ただし、そこは本体とは分離してください。

---

## ここは慎重に見た方がよい

### 1\. パッチ対象の安定性

Figma Desktop の更新で壊れます。  
Rust で書いても、この性質は変わりません。

### 2\. Safe Mode の「同一機能」表示

README では full parity と書かれていますが、REFERENCE では Safe Mode で大きな `render-batch` が厳しいことが示されています。Rust 版では README で誇張せず、制約を正確に書くべきです。 [GitHub+1](https://github.com/silships/figma-cli)

### 3\. 名前の整理

現行は repo 名 `figma-cli`、package 名 `figma-ds-cli`、導線 `fig-start` です。Rust 版は最初に名前を一本化した方がよいです。 `package.json` でもその不一致は見えています。 [GitHub](https://github.com/silships/figma-cli/blob/main/package.json)

---

## 私の提案

あなたが本気で Rust 版を作るなら、最初の 2 週間相当の作業単位は次の形がよいです。

### 作業 1

Cargo workspace を作る。  
`figma-cli`, `figma-core`, `figma-daemon` の 3 crate だけ先に置く。

### 作業 2

`clap` でコマンド表面を Node 版互換にする。  
中身はまだ `todo!()` でよい。

### 作業 3

daemon を先に完成させる。  
token 認証、status、diagnose まで入れる。

### 作業 4

Safe Mode の bridge を先に通す。  
Yolo Mode より壊れにくいからです。

### 作業 5

CDP client を作る。  
`Runtime.evaluate` 相当だけ先に通す。

### 作業 6

patch/restore を分けて作る。  
バックアップ、ハッシュ、復元を先に入れる。

---

## かなり率直に言うと

この種の移植で失敗する理由は、**コマンド数の多さ**ではありません。  
失敗するのは、**接続方式・権限・復旧手順を後回しにするから**です。

このリポジトリは README 上すでに、

- patch がある
- Safe Mode がある
- daemon token がある
- Windows/macOS 差分がある

という、面倒な条件を全部抱えています。Rust 版は、描画や token 操作より先に、その地味な部分を固めた方が壊れません。 [GitHub+1](https://github.com/silships/figma-cli)

必要なら次に、**Rust 版の実際のディレクトリ構成、Cargo.toml、`clap` 定義、daemon の雛形、plugin bridge の JSON schema までまとめて書きます。**

## Prompt:

このリポジトリを完全再現したいとは思っていません、
再設計したいです、しかし接続方法などは面白いと思い参考にしたいです
構成は以下のリポジトリをテンプレートとして使用します
https://github.com/taiga-tech/sample-cli
どんなことがやりたいかというと主にfigma2codeをしたいので指定したフレームをコンポーネントごとに分割してアトミックデザインで実装するためのJSONを取得したいです
ほんとにJSONがいいかは悩みどころなのでいい案があれば教えてください

容赦なく徹底的にお願いします。

## Response:

思考時間: 30s

## 前提整理

やりたいことは、**figma-cli の接続発想は借りるが、CLI 自体は再設計する**、そして用途は **figma2code のための中間表現を取り出すこと** です。対象は「指定したフレームを、コンポーネント単位に分割し、アトミックデザインで実装可能な形へ落とす」ことにあります。これは妥当です。`silships/figma-cli` は Figma Desktop 直結や plugin bridge という接続面が面白い一方で、公開されている構成上は用途が広すぎます。対して `taiga-tech/sample-cli` は、Rust コアと npm 配布ランチャを分けた配布テンプレートとして整理されており、`crates/core`, `crates/cli`, `packages/cli`, `packages/cli-<platform>` の分離が明快です。あなたの用途なら、**接続方式は前者から、配布構成は後者から取る** のが自然です。 [GitHub+2GitHub+2](https://github.com/silships/figma-cli)

## 先に結論

**JSON を最終成果物にしない方がよい**です。  
JSON は「輸送用」には向いていますが、「実装判断用」には弱いです。

あなたが本当に欲しいのは、単なるノードダンプではなく、次の 3 つです。

1.  **Figma の生情報**  
    座標、サイズ、塗り、文字、Auto Layout、Variants、Variables など。
2.  **実装判断を加えた正規化表現**  
    これは見出し、これはボタン、これは Card 分子、これは Section 組織、という解釈。
3.  **コード生成やレビューに使う出力**  
    React / Vue / HTML / Storybook / トークン定義など。

この 3 層を分けないと、途中で必ず詰まります。Figma の公式 REST API はファイルやノードを JSON として取得できますが、それはあくまで Figma 側の表現です。そこからそのまま UI 実装へ行くには情報の粒度が合っていません。Figma の Plugin API も同様で、シーングラフを読めますが、それ自体が「実装にちょうどよい表現」ではありません。 [developers.figma.com+3developers.figma.com+3developers.figma.com+3](https://developers.figma.com/docs/rest-api/?utm_source=chatgpt.com)

---

## 容赦なく言うと、設計の成否は「JSON 形式」ではなく「中間表現の責務分離」で決まります

多くの figma2code 系が壊れる理由は、**Figma ノード JSON をそのままコードへ落とそうとする**からです。  
これはほぼ必ず失敗します。

理由は単純です。

- Figma のノード構造は、**描画都合** と **編集都合** を多く含みます。
- 実装側が欲しいのは、**責務ごとの UI 構造** です。
- アトミックデザインは、**見た目のネスト** ではなく **意味の分割** を求めます。

たとえば Figma 上で 1 つの Card に見えるものが、ノードとしては、

- 外枠 Frame
- 背景 Rectangle
- 見出し Text
- 本文 Text
- CTA Frame
- アイコン Vector
- 余白調整用の wrapper  
  に割れていることは普通です。

これを JSON そのままでコード化すると、**wrapper 地獄** になります。  
そして、アトミックデザイン分類も壊れます。

---

## したがって、出力は JSON ではなく「IR」にしてください

ここでいう IR は intermediate representation、中間表現です。

おすすめは次の三層です。

### 1\. Raw Extract

Figma から取得したほぼ生の情報。

- node id
- node type
- parent / children
- absolute / relative bounds
- fills / strokes / effects
- typography
- auto layout
- constraints
- variables binding
- component / instance / variant 情報
- text segments
- export settings
- visible / locked など

これは JSON でよいです。  
ただし、**外部公開用ではなくデバッグ用** です。  
保存名も `raw-frame.json` で十分です。

### 2\. Normalized UI IR

実装都合に寄せて正規化した表現。  
これが本体です。

- `Screen`
- `Section`
- `Layout`
- `Container`
- `TextBlock`
- `Heading`
- `Button`
- `Icon`
- `Image`
- `Input`
- `Card`
- `List`
- `Navigation`
- `Unknown`

のような**意味付きノード**に変換します。

さらに各ノードに、

- semantic role
- layout intent
- token references
- implementation hints
- atomic level
- confidence
- source node ids

を持たせます。

### 3\. Target Projection

生成先ごとの投影です。

- React component tree
- Storybook args
- design tokens
- CSS variables
- documentation markdown
- review report

この層で初めてコード生成やファイル出力をします。

この構成にすると、JSON をやめる必要はありません。  
**JSON は Raw と IR のシリアライズ形式として使う**。  
ただし、**最終設計単位は JSON ではなく IR** です。

---

## JSON が本当に最適か

結論としては次です。

### JSON だけ

不十分です。  
人間には読みにくく、差分が荒く、意味づけが弱いです。

### JSON + JSON Schema

かなり良いです。  
CLI と他ツールの接続も容易です。  
最初の一歩として現実的です。

### YAML

人間は読みやすいが、機械生成物としては崩れやすいです。  
補助出力にはよいですが、主保存形式には勧めません。

### Protocol Buffers / MessagePack

速いですが、今の段階では早すぎます。  
可観測性が下がります。

### SQLite / DuckDB

かなり有力です。  
特に解析用途では強いです。  
ノードグラフ、トークン、テキスト片、推定コンポーネントを表形式で持てます。  
ただし、実装初期は重いです。

### 私の提案

**正規形は JSON Schema 付き JSON、解析キャッシュは SQLite**  
これが最も扱いやすいです。

---

## あなたの用途に合う出力設計

あなたの要件だと、1 つのフレームから欲しいのは単一の JSON ではありません。  
最低でも 4 つに分けるべきです。

### A. `raw.json`

Figma 生情報。再抽出やデバッグ用。

### B. `ui.ir.json`

正規化済み UI 構造。主成果物。

### C. `tokens.json`

色、余白、角丸、タイポ、エフェクト、変数参照など。

### D. `report.md`

人間向け解説。  
どこを Atom / Molecule / Organism と判定したか、曖昧な箇所はどこか、何を手修正すべきか。

この `report.md` がかなり効きます。  
figma2code は、完全自動より **どこが危ないかを露出する設計** の方が実務では使えます。

---

## アトミックデザイン分類は、自動分類だけに頼らない方がよい

ここも厳しく言います。

**アトミックデザインは、Figma の見た目から一意には決まりません。**

たとえば Button は Atom に見えますが、

- アイコン付きボタン
- ローディング付きボタン
- ドロップダウン起点
- セグメントの 1 要素  
  などで、設計上の責務は変わります。

Card も同様で、

- 単純な Molecule
- 一覧上の Organism の一部
- Section 全体のテンプレート  
  になり得ます。

なので分類は次の 3 段階にすべきです。

1.  **候補推定**  
    形状、命名、構造、繰り返し、バリアントから仮説を出す。
2.  **信頼度付与**  
    `confidence: 0.91` のように出す。
3.  **上書き可能にする**  
    ルールファイルで `LoginForm -> organism` のように明示できるようにする。

自動分類だけで押し切ると、後で必ず信用されなくなります。

---

## 取得方式はどうするべきか

Figma から情報を取る方法は大きく 4 つあります。

### 1\. REST API

公式にファイルやノードの JSON を取れます。ファイル全体または個別ノードの JSON 取得が可能です。これは安定しています。 [developers.figma.com+1](https://developers.figma.com/docs/rest-api/?utm_source=chatgpt.com)

**長所**

- 外部ツールから叩きやすい
- 安定している
- Desktop 依存がない
- CI に載せやすい

**短所**

- ローカルで開いている未保存状態への追従が弱い
- plugin API ほど柔軟ではない

### 2\. Plugin API

Figma 内で最も深く見られます。`figma` グローバル経由でファイル内容の参照・更新ができます。 [developers.figma.com+1](https://developers.figma.com/docs/plugins/api/api-reference/?utm_source=chatgpt.com)

**長所**

- ノード取得や細かな解釈に強い
- 選択中フレーム基準の抽出がやりやすい
- 実験が速い

**短所**

- CLI 単独では完結しない
- 配布や運用が少し重い

### 3\. Desktop 直結 / デバッグ接続

`figma-cli` が面白いのはここです。README では Yolo Mode は Figma Desktop をパッチして debug port 経由で接続、Safe Mode は plugin bridge としています。 [GitHub](https://github.com/silships/figma-cli)

**長所**

- ローカル操作感が強い
- CLI から自然に扱える
- 面白い

**短所**

- 壊れやすい
- Figma 更新に引きずられる
- 配布時の説明責任が重い

### 4\. MCP / Dev Mode / Code Connect

Figma は現在、Dev Mode と MCP サーバー、そして Code Connect を前面に出しています。Dev Mode では MCP サーバー経由でエージェントツールに Figma コンテキストを渡せると案内しており、Code Connect は Figma コンポーネントとコード実装のマッピングを強化します。 [Figma ヘルプセンター+5Figma+5developers.figma.com+5](https://www.figma.com/ja-jp/dev-mode/?utm_source=chatgpt.com)

**長所**

- 公式の流れに沿っている
- 実コードとの対応づけに強い

**短所**

- あなたが欲しい「独自の正規化 IR 抽出」とは少し目的が違う
- 既存コードベースが前提になりやすい

---

## おすすめの取得戦略

あなたの用途なら、こうします。

### 開発初期

**Plugin API を主、REST API を副** にする。

理由は単純で、**IR 設計の試行錯誤**がしやすいからです。  
選択中フレームをその場で抜き出し、分類ルールを回し、結果を見比べやすい。

### 配布・自動化段階

**REST API を主、Plugin / Desktop bridge を補助** にする。

理由は、CI・再現性・保守性です。  
公開 CLI として長く維持するなら、Desktop patch は中核に置かない方がよいです。

### 接続面で figma-cli から借りるべきもの

- ローカル daemon
- plugin bridge
- 選択中ファイル / フレームを取る UX
- CLI と Figma の往復の考え方

### 借りない方がよいもの

- プロダクト全体を Desktop patch 前提にすること
- Claude 依存のコマンド知識設計
- 何でもできる多機能 CLI 化

---

## つまり、再設計の軸は「抽出 CLI」ではなく「解析パイプライン」です

CLI の責務を絞るべきです。

### 悪い設計

- connect
- create
- patch
- render
- export
- lint
- tokens
- storybook
- images
- website recreation
- agent integration
- 何でも乗せる

これはまた太ります。

### 良い設計

このプロジェクトは **figma2code の前処理機** と割り切る。

責務は次だけでよいです。

- Figma から取得する
- 対象フレームを切り出す
- UI 構造へ正規化する
- トークンを抽出する
- コンポーネント候補をグルーピングする
- 実装向け IR を出す
- 必要ならコード生成器へ渡す

この割り切りがないと、また別の figma-cli になります。

---

## sample-cli をテンプレートとして使う判断は正しいか

**配布テンプレートとしては正しい**です。  
README 上、`sample-cli` は Rust ネイティブ CLI を npm で配布するためのモノレポで、Rust 側を `crates/core`, `crates/cli` に分け、npm 側を `packages/cli` とプラットフォーム別同梱パッケージに分けています。Node / pnpm / Rust / mise の要件も整理されています。 [GitHub+1](https://github.com/taiga-tech/sample-cli)

ただし、**ドメイン設計のテンプレートではありません。**  
あくまで配布骨格です。

なので使い方は次のとおりです。

- 配布構成: `sample-cli` を採用
- ドメイン構成: 自分で設計
- Figma 接続: `figma-cli` の発想を参考に取捨選択

---

## 私ならこう切ります

## 推奨モノレポ構成

```
.
├── crates/
│   ├── core/              # 共通型, IR, schema, diagnostics
│   ├── cli/               # clap で CLI
│   ├── figma-rest/        # REST API client
│   ├── figma-plugin/      # plugin bridge client
│   ├── analyzer/          # ノード解析・正規化
│   ├── classifier/        # atom/molecule/organism 推定
│   ├── token-extractor/   # design tokens 抽出
│   ├── projector/         # React/HTML/Storybook などへの投影
│   └── rules/             # 命名ルール、override、プロジェクト規約
├── packages/
│   ├── cli/
│   ├── cli-darwin-arm64/
│   ├── cli-darwin-x64/
│   ├── cli-win32-x64/
│   ├── cli-linux-x64-gnu/
│   └── cli-linux-x64-musl/
├── plugin/
│   └── figma-ir-extractor/
├── schemas/
│   ├── raw.schema.json
│   ├── ui-ir.schema.json
│   └── tokens.schema.json
├── examples/
├── fixtures/
├── pnpm-workspace.yaml
├── turbo.json
├── mise.toml
└── Cargo.toml
```

この分け方なら、将来 plugin 抽出を外して REST 中心に移行しても崩れません。

---

## CLI コマンドは最初から絞ってください

### あるべきコマンド

```
figma-ir connect
figma-ir inspect frame <node-id>
figma-ir extract frame <node-id> --format raw
figma-ir normalize frame <node-id>
figma-ir classify frame <node-id>
figma-ir tokens frame <node-id>
figma-ir project frame <node-id> --target react
figma-ir doctor
```

### 最初はいらないコマンド

- create
- mutate
- patch
- render
- bulk rename
- image export
- arbitrary eval

最初から作るべきものではありません。

---

## IR の具体形

JSON がよいか悩んでいるとのことなので、かなり具体的に出します。

### `ui.ir.json` の例

```
{
  "version": "0.1.0",
  "source": {
    "fileKey": "abc123",
    "frameNodeId": "1:20",
    "frameName": "Checkout / Summary"
  },
  "tokens": {
    "color": ["color.surface.card", "color.text.primary"],
    "spacing": ["space.16", "space.24"],
    "radius": ["radius.md"],
    "typography": ["text.body.md", "text.heading.lg"]
  },
  "tree": {
    "id": "screen.checkout.summary",
    "kind": "Screen",
    "atomic": "template",
    "children": [
      {
        "id": "section.order-summary",
        "kind": "Section",
        "atomic": "organism",
        "layout": {
          "direction": "vertical",
          "gap": "space.16"
        },
        "children": [
          {
            "id": "card.order-item",
            "kind": "Card",
            "atomic": "molecule",
            "reusable": true,
            "sourceNodeIds": ["1:50", "1:51", "1:52"],
            "confidence": 0.88,
            "children": [
              {
                "id": "text.title",
                "kind": "Heading",
                "atomic": "atom",
                "text": "Order summary"
              },
              {
                "id": "button.checkout",
                "kind": "Button",
                "atomic": "atom",
                "variant": "primary"
              }
            ]
          }
        ]
      }
    ]
  },
  "diagnostics": [
    {
      "level": "warning",
      "code": "LOW_CONFIDENCE_COMPONENT_BOUNDARY",
      "message": "Card boundary was inferred from repeated padding and fill patterns."
    }
  ]
}
```

ここで大事なのは、

- `kind`
- `atomic`
- `reusable`
- `confidence`
- `sourceNodeIds`
- `diagnostics`

です。  
これがない JSON は、ただのダンプです。

---

## 実装判定ロジックはどこを見るべきか

分類・正規化で見る項目はこれです。

### 構造

- 子ノード数
- 深さ
- 繰り返し構造
- 兄弟間の等間隔
- 似た subtree の出現頻度

### 見た目

- fill / stroke / radius
- typography style
- アイコンの有無
- 画像の有無
- 余白パターン

### レイアウト

- auto layout direction
- item spacing
- padding
- constraints
- alignment

### 意味

- レイヤー名
- コンポーネント名
- variant 名
- text content
- instance / main component 参照

### 設計システム

- variable binding
- local styles
- team library components
- published component usage

これらを総合して、  
「これは button らしい」  
「これは card らしい」  
「これは section wrapper でしかない」  
と判定していきます。

---

## 命名規約ファイルを用意してください

実務では、命名の癖が強いです。  
したがって、ルールファイルを持たないと使いものになりません。

### 例: `figma-ir.rules.json`

```
{
  "matchers": [
    {
      "namePattern": ".*Button.*",
      "kind": "Button",
      "atomic": "atom"
    },
    {
      "namePattern": ".*Card.*",
      "kind": "Card",
      "atomic": "molecule"
    },
    {
      "namePattern": ".*Section.*",
      "kind": "Section",
      "atomic": "organism"
    }
  ],
  "ignoreNodeNames": ["Spacer", "Gap", "Padding"],
  "layoutOnlyNodePatterns": [".*Wrapper.*", ".*Container.*"]
}
```

これがあるだけで、現場適応力が大きく変わります。

---

## 公式の流れとの関係

今の Figma は Dev Mode、MCP、Code Connect を前面に出しています。Dev Mode の案内では、MCP サーバーを通じて Figma コンテキストを VS Code、Cursor、Windsurf、Claude などへ取り込めると説明されています。また Code Connect は Figma 側コンポーネントと既存コード実装をマッピングし、MCP を強化する位置づけです。 [Figma ヘルプセンター+5Figma+5developers.figma.com+5](https://www.figma.com/ja-jp/dev-mode/?utm_source=chatgpt.com)

しかし、あなたの計画はそれとは少し違います。

- Figma 既存コンポーネントとコードの対応付けをしたい  
  → Code Connect に近い
- 指定フレームを解析し、アトミック設計向け IR に落としたい  
  → 独自領域

したがって、**Code Connect の代替を作るのではなく、その手前にある解析層を作る** と考えるべきです。  
ここを間違えると、既存機能と競合し始めます。

---

## どこで LLM を使うべきか

ここも線引きが必要です。

### ルールベースでやるべき部分

- ノード収集
- レイアウト抽出
- トークン抽出
- component boundary 候補
- repeated pattern 検出
- variant 整理

### LLM を使ってよい部分

- semantic naming 提案
- 低信頼ノードの説明
- report.md 生成
- 実装補助コメント
- 最終コードの整形補助

**分類の中心を LLM に置かない方がよい**です。  
再現性が落ちます。

---

## 最低限の成功条件

このプロジェクトが成立する条件は次です。

1.  同じフレームに対して、ほぼ同じ IR が再生成されること
2.  人間が見て「この分割なら妥当」と思えること
3.  誤判定時に、どこを直せばよいか分かること
4.  出力がコード生成器に渡しやすいこと
5.  Figma 側の構造ノイズを吸収できること

この 5 つを満たさないなら、どれだけ接続が面白くても使われません。

---

## 私の推奨

### 方針

- 配布骨格は `sample-cli`
- 抽出戦略は **Plugin API 主体**
- 将来の安定運用は **REST API 主体**
- `figma-cli` からは **daemon / bridge の発想だけ借りる**
- 中核成果物は **JSON ではなく UI IR**
- JSON は **IR のシリアライズ形式** に留める

### 最初に作るべきもの

- `inspect`
- `extract --raw`
- `normalize`
- `classify`
- `tokens`
- `doctor`

### 後回しにするもの

- コード生成
- 自動修正
- Storybook 直接出力
- Desktop patch
- 何でもできる万能コマンド群

---

## 最終的な答え

**JSON は採用してよいが、JSON を設計の中心に置かない方がよい**です。  
中心に置くべきなのは **意味付きの中間表現** です。

そのうえで、

- `raw.json` は取得の事実
- `ui.ir.json` は実装判断
- `tokens.json` は設計システム接続
- `report.md` は人間への説明

この 4 枚構成にすると、かなり強いです。

必要なら次に、**この方針に合わせた Rust workspace 設計、`clap` のコマンド定義、`ui.ir.schema.json` の初版、plugin bridge のプロトコル案** まで具体化します。

## Prompt:

RestAPIやpluginは考えていません

## Response:

思考時間: 43s

では、方針はかなり明確です。

**REST API も plugin も使わない**のであれば、実質的に選択肢は次の一系統に絞られます。

**Figma Desktop の実行中プロセスへ外から接続し、Figma 側のランタイムにコードを流し込んで、必要な情報だけを回収する**。

`silships/figma-cli` が面白いのもそこです。公開 README 上では、Yolo Mode は **Figma を一度パッチして debug port を有効化し、CLI から localhost 上のランダムポートへ直接接続する** 方式として説明されています。Safe Mode は plugin 経由です。あなたは後者を捨てるので、前者の思想だけを抽出して作り直すことになります。 [GitHub](https://github.com/silships/figma-cli)

そして配布・モノレポ骨格として `sample-cli` を使う判断は妥当です。あのテンプレートは README 上、**Rust コア／Rust バイナリ／npm ランチャ／プラットフォーム別バイナリ同梱パッケージ** に分かれた構成で、`mise` と `pnpm` 前提も明示されています。あなたの運用方針とも噛み合います。 [GitHub](https://github.com/taiga-tech/sample-cli)

---

## 厳しく整理すると、今の条件で作るべきものは「figma2code CLI」ではなく「Figma Runtime Extractor」です

ここを曖昧にすると壊れます。

あなたが欲しいのは、

- Figma ファイル操作ツール
- 汎用 Figma CLI
- Claude 連携用ツール

ではありません。

欲しいのは、

**指定フレームを、実装都合に沿った構造へ分解して、実装可能な中間表現を吐く抽出器**

です。

ここを外すと、また何でもできる CLI に太ります。

---

## 先に結論

### JSON は「あり」ですが、主役にしない方がよいです

本当に作るべきものは次です。

1.  **接続層**  
    Figma Desktop のランタイムへ到達する
2.  **抽出層**  
    ノード情報、レイアウト、見た目、命名、繰り返しを取る
3.  **正規化層**  
    UI 構造として意味づけする
4.  **分類層**  
    atom / molecule / organism / template へ仮分類する
5.  **投影層**  
    JSON、YAML、Markdown、コード生成入力などへ出す

JSON はこのうち **4 と 5 の間のシリアライズ形式** として使うのがよいです。  
JSON 自体を設計の中心にすると、すぐ詰まります。

---

## ここからが本題です

REST API も plugin も使わないなら、設計の難所は次の 4 つです。

## 1\. 接続は面白いが、最も壊れやすい

`figma-cli` README に書かれている Yolo Mode は、**Figma Desktop をパッチして debug port を有効化し、CLI から WebSocket で接続する**方式です。README では localhost 限定、ランダムポート、token auth、idle で自動停止とも説明されています。 [GitHub](https://github.com/silships/figma-cli)

この発想は確かに面白いです。  
しかし、再設計するならそのまま真似してはいけません。

### その理由

- Figma Desktop の更新で壊れる
- パッチポイントが変わる
- OS ごとの差が出る
- 権限と復旧手順が重い
- 壊れたときの説明責任が全部こちらに来る

つまり、**接続方式としては魅力があるが、プロダクトの中心に据えるには危険**です。

### したがって

接続層は **交換可能** にしておくべきです。

```
Transport
├── DesktopPatchedCdp
├── DesktopAttachedCdp
└── MockRuntime
```

最初は `DesktopPatchedCdp` だけでもよいですが、抽象は必ず切ってください。

---

## 2\. 抽出対象は Figma ノードではなく「実装判断に効く特徴量」です

ここが最重要です。

多くの失敗作は、Figma ノードをそのまま保存して満足します。  
それでは figma2code になりません。

あなたが取るべきなのは、ノードそのものではなく **特徴量** です。

### 取るべき情報

#### 構造

- parent / children
- sibling order
- 深さ
- subtree size
- repeated subtree fingerprint

#### レイアウト

- x, y, width, height
- absolute / local bounds
- auto-layout direction
- spacing
- padding
- alignment
- wrapping の有無
- constraints 的な挙動

#### 視覚

- fill / stroke / radius
- shadow / blur
- opacity
- image fill
- vector presence

#### 文字

- text content
- font family
- size
- weight
- line height
- letter spacing
- text case
- text segments

#### 意味の手掛かり

- node name
- component / instance 名
- variant 名
- property 名
- icon 名らしさ
- button / card / list item らしさ

#### 実装に効く統計

- 同型 subtree の出現回数
- spacing の代表値
- radius の代表値
- text style のクラスタ
- color のクラスタ
- repeated layout pattern

**この特徴量集合を設計しない限り、JSON 形式を何にしても駄目です。**

---

## 3\. アトミックデザインは「見た目」から一意には決まりません

ここはかなり厳しく言います。

**アトミックデザインを自動判定できる、という前提は甘いです。**

Figma 上の見た目から自動的に

- これは atom
- これは molecule
- これは organism

と断定できる場面は限られます。

同じボタン風 UI でも、

- ただの Button atom
- ButtonGroup 内の一部
- FilterChip
- MenuTrigger
- PaginationControl
- SegmentedControl item

で責務が違います。

Card も同じです。  
見た目だけで molecule と決めると事故ります。

### だから設計はこうするべきです

分類結果は 1 個に決め打ちせず、**候補と信頼度を持たせる**。

```
{
  "kindCandidates": [
    { "kind": "Button", "score": 0.92 },
    { "kind": "Chip", "score": 0.51 },
    { "kind": "MenuTrigger", "score": 0.34 }
  ],
  "atomicCandidates": [
    { "atomic": "atom", "score": 0.71 },
    { "atomic": "molecule", "score": 0.43 }
  ]
}
```

これがないと、あとで全員が結果を信用しなくなります。

---

## 4\. 出力は 1 枚の JSON ではなく、複数レイヤに分けるべきです

あなたが悩んでいる「ほんとうに JSON がよいか」は、かなり正しい悩みです。  
答えはこうです。

**単一 JSON はよくない。複数成果物に分けるべき。**

私なら最低でも次の 4 つに分けます。

### `raw.json`

ランタイムから取れた生情報。  
デバッグ用。  
人が直接触る想定は薄い。

### `features.json`

抽出済み特徴量。  
分類器や正規化器の入力。

### `ui.ir.json`

意味づけ済み UI 中間表現。  
これが主成果物。

### `report.md`

人間向けの説明。  
曖昧な判定、要確認箇所、分割方針、推定根拠を書く。

これなら、

- 再抽出
- 再分類
- 分類ルール更新
- 実装出力更新

を分離できます。

---

# では、JSON の代わりに何があるか

## 率直な答え

### JSON

最も現実的です。  
ただし **Schema 付き** にしてください。

### YAML

人間にはやや見やすいですが、機械生成の主フォーマットには不向きです。  
補助出力としてならありです。

### Markdown

主フォーマットにはなりません。  
説明用に限定。

### SQLite

かなり有力です。  
むしろ私は最終的にこれを併用した方がよいと思っています。

---

## JSON 単独の弱さ

JSON だけだと、

- diff が読みにくい
- 同型比較がしにくい
- クエリしにくい
- 集計しにくい
- 解析過程の観測が弱い

という欠点があります。

---

## SQLite を併用すると急に強くなる

もし本気で「徹底的に」やるなら、**保存形式を JSON 一択にしない**方がよいです。

### おすすめ構成

- 永続キャッシュ: SQLite
- エクスポート: JSON
- 人間確認: Markdown

### SQLite に入れるテーブル例

- `nodes`
- `edges`
- `text_segments`
- `style_clusters`
- `layout_patterns`
- `component_candidates`
- `classification_results`
- `diagnostics`

これがあると、

- 「半径 8 のカード候補を全部見る」
- 「同型 subtree が 3 回以上出るものを拾う」
- 「Button 候補だが confidence < 0.7 のもの」
- 「画面内で繰り返される list item 候補」  
  のような解析ができます。

JSON だけだと、毎回総なめになります。

### 現実的な落とし所

最初は JSON で始めて構いません。  
ただし、内部では **IR を Rust 構造体で持ち、SQLite にも落とせる設計** にしてください。

---

# あなたの条件での最適解

REST API も plugin も使わないなら、私はこう切ります。

## 役割を 3 つに限定する

### 1\. `attach`

Figma Desktop に接続する。

### 2\. `extract`

指定フレームから raw/features を取る。

### 3\. `normalize`

UI IR へ変換する。

それ以外は後回しです。

---

## CLI の初期コマンド案

```
figma-ir attach
figma-ir doctor
figma-ir inspect frame --id 123:456
figma-ir extract frame --id 123:456 --out raw.json
figma-ir features frame --id 123:456 --out features.json
figma-ir normalize frame --id 123:456 --out ui.ir.json
figma-ir report frame --id 123:456 --out report.md
```

### 最初はいらないもの

- render
- create
- patch restore 以外の mutate 系
- export jsx
- storybook
- ai naming
- arbitrary eval の一般公開

`eval` は内部実装で使えばよく、表に出す必要はありません。

---

# ここで一番大事な設計判断

## 「指定フレームをコンポーネントごとに分割」とは何かを、アルゴリズムで定義してください

この文は自然言語としては分かりますが、実装仕様としては曖昧です。

分割の基準は、少なくとも次の 4 つがあります。

### A. Figma の component boundary に従う

既存 Component / Instance を尊重する。

### B. 見た目のまとまりで分ける

padding、背景、枠線、繰り返しで区切る。

### C. レイアウト責務で分ける

list、card、header、form-row、toolbar として区切る。

### D. 実装再利用性で分ける

同型 subtree をまとめて 1 component 候補にする。

この 4 つは一致しません。

### 私の提案

分割器は単一ロジックではなく、**複数の boundary detector を並走**させるべきです。

```
Boundary detection
├── explicit_component_boundary
├── repeated_subtree_boundary
├── visual_group_boundary
├── layout_role_boundary
└── text_semantic_boundary
```

その結果を統合して、候補として出す。

これが「徹底的に」作るときの筋です。

---

# 推奨 IR

これはかなり具体的に書きます。

```
{
  "version": "0.1.0",
  "source": {
    "frameId": "123:456",
    "frameName": "Checkout / Summary"
  },
  "tree": {
    "id": "screen.checkout.summary",
    "nodeType": "Screen",
    "children": [
      {
        "id": "section.order-summary",
        "nodeType": "Section",
        "sourceNodeIds": ["123:460"],
        "layout": {
          "kind": "stack",
          "direction": "vertical",
          "gap": 16,
          "padding": [24, 24, 24, 24]
        },
        "componentCandidates": [
          {
            "name": "OrderSummarySection",
            "atomic": "organism",
            "score": 0.89
          }
        ],
        "children": [
          {
            "id": "card.order-item",
            "nodeType": "Card",
            "sourceNodeIds": ["123:470", "123:471", "123:472"],
            "componentCandidates": [
              {
                "name": "OrderItemCard",
                "atomic": "molecule",
                "score": 0.86
              }
            ],
            "children": [
              {
                "id": "text.title",
                "nodeType": "Heading",
                "text": "Order summary"
              },
              {
                "id": "button.checkout",
                "nodeType": "Button",
                "variant": "primary"
              }
            ]
          }
        ]
      }
    ]
  },
  "tokens": {
    "colors": ["color.surface.card", "color.text.primary"],
    "spacing": ["space.16", "space.24"],
    "radius": ["radius.md"],
    "typography": ["text.heading.lg", "text.body.md"]
  },
  "diagnostics": [
    {
      "level": "warning",
      "code": "AMBIGUOUS_BOUNDARY",
      "message": "Card boundary inferred from repeated fill, radius and padding pattern."
    }
  ]
}
```

### この IR で大事なもの

- `sourceNodeIds`
- `componentCandidates`
- `atomic`
- `score`
- `diagnostics`

これがないなら、設計として弱いです。

---

# sample-cli を使う前提での構成

`sample-cli` は Rust コアと npm ランチャ、プラットフォーム別バイナリ同梱パッケージに分かれたテンプレートです。README では `crates/core`, `crates/cli`, `packages/cli`, `packages/cli-<platform>` という分割が示されています。 [GitHub](https://github.com/taiga-tech/sample-cli)

あなたなら、こう切るのがよいです。

```
.
├── crates/
│   ├── core/            # 共通型、設定、診断、IR
│   ├── cli/             # clap で CLI
│   ├── runtime/         # Figma Desktop 接続
│   ├── extractor/       # raw/features 抽出
│   ├── normalizer/      # UI IR 変換
│   ├── classifier/      # atomic 候補推定
│   ├── reporter/        # report.md 出力
│   └── store/           # JSON/SQLite 出力
├── packages/
│   ├── cli/
│   ├── cli-darwin-arm64/
│   ├── cli-darwin-x64/
│   ├── cli-win32-x64/
│   ├── cli-linux-x64-gnu/
│   └── cli-linux-x64-musl/
├── schemas/
│   ├── raw.schema.json
│   ├── features.schema.json
│   └── ui-ir.schema.json
├── fixtures/
├── mise.toml
├── pnpm-workspace.yaml
└── Cargo.toml
```

---

# 接続方式についての率直な見解

REST API も plugin も使わないなら、接続方法はかなり限定されます。  
その場合、**Desktop ランタイム接続を中核にするしかない**です。

ただし、ここで避けるべき失敗があります。

## 悪い案

- 最初から Figma patch を前提に全設計を組む
- 抽出ロジックと接続ロジックを密結合にする
- 取得結果をそのままコード生成へ流す
- `eval` できるから何でもやる

## 良い案

- 接続層を抽象化する
- 抽出対象を限定する
- UI IR を先に固める
- コード生成は後段へ追い出す
- patch は transport の 1 実装にとどめる

---

# 容赦なく言うと、今やるべきではないもの

あなたの現在の要件に対して、次は早すぎます。

### 1\. アトミックデザイン完全自動化

精度幻想になります。

### 2\. JSX 自動生成

IR が固まる前にやると、すぐ全部捨てることになります。

### 3\. 見た目の完全再現

目的が figma2code 前処理なら、そこに寄る必要はありません。

### 4\. 汎用 Figma CLI 化

また膨らみます。

---

# 今やるべき順序

## フェーズ 1

接続と観測だけ作る。

- attach
- doctor
- inspect
- raw dump

## フェーズ 2

特徴量抽出。

- layout features
- text features
- style features
- repeated subtree detection

## フェーズ 3

正規化と分類。

- boundary detection
- component candidates
- atomic candidates
- diagnostics

## フェーズ 4

出力整備。

- ui.ir.json
- report.md
- 必要なら sqlite cache

---

# 最終判断

あなたの条件なら、こうです。

**REST API や plugin を使わない判断を前提にするなら、採るべき道は Desktop Runtime 接続一択です。**  
ただし、その面白さに引っ張られて **接続方式をプロダクトの中心にしない** 方がよいです。

中心に置くべきものは、次の 2 つです。

- **Figma から何を抽出するか**
- **それをどういう UI 中間表現にするか**

そして、JSON についてはこう答えます。

**JSON は出力形式としてはよい。設計の中心としては弱い。**  
したがって、

- 内部は Rust の型付き IR
- 保存は JSON Schema 付き JSON
- 解析補助に SQLite
- 人間確認に Markdown report

この構成が最も堅いです。

次に詰めるなら、  
**この前提だけに絞った Rust workspace 設計** と、**`ui.ir.json` の初版 schema** をそのまま書きます。

## Prompt:

\*\*この前提だけに絞った Rust workspace 設計\*\* と、\*\*\`ui.ir.json\` の初版 schema\*\* をそのまま書きます。 情報源

お願いします

## Response:

思考時間: 22s

以下の前提で固定します。

- **Figma REST API は使わない**
- **Figma plugin も使わない**
- **Figma Desktop 実行環境への接続だけを使う**
- **目的は figma2code の前処理**
- **生成したいのはコードそのものではなく、実装判断に使える中間表現**
- **配布骨格は `sample-cli` を使う**。このテンプレートは `crates/core`, `crates/cli`, `packages/cli`, `packages/cli-<platform>` という分離になっており、`mise` と `pnpm` を前提にしています。 [GitHub+3GitHub+3GitHub+3](https://github.com/taiga-tech/sample-cli)
- **接続の発想は `figma-cli` の Yolo Mode を参考にする**。README では、Figma Desktop を一度パッチして debug port を有効化し、CLI から localhost 上のポートへ直接接続する構成と説明されています。 [GitHub](https://github.com/silships/figma-cli)

---

# 1\. 設計方針

## 何を作るか

作るべきものは「Figma の万能 CLI」ではありません。  
**Figma Runtime Extractor** です。

責務はこの 4 つだけに絞ります。

1.  Figma Desktop に接続する
2.  指定フレームを観測する
3.  実装向けの UI IR に正規化する
4.  JSON とレポートを出す

それ以外は切ります。

### 捨てるもの

- create / mutate 系
- export 系
- arbitrary eval の公開
- 汎用 lint
- Storybook 出力
- JSX 自動生成

これらは、IR が安定してから別プロジェクトまたは別 crate に分離すれば足ります。

---

# 2\. ワークスペース設計

`sample-cli` の配布構成はそのまま借ります。`crates/core` と `crates/cli`、npm ランチャの `packages/cli`、プラットフォーム別パッケージという骨格です。 [GitHub+1](https://github.com/taiga-tech/sample-cli)

ただし、ドメイン側はこう切ります。

```
.
├── Cargo.toml
├── rust-toolchain.toml
├── mise.toml
├── pnpm-workspace.yaml
├── package.json
├── turbo.json
│
├── crates/
│   ├── core/
│   ├── cli/
│   ├── runtime/
│   ├── extractor/
│   ├── normalizer/
│   ├── classifier/
│   ├── reporter/
│   ├── schema/
│   └── store/
│
├── packages/
│   ├── cli/
│   ├── cli-darwin-arm64/
│   ├── cli-darwin-x64/
│   ├── cli-win32-x64/
│   ├── cli-linux-x64-gnu/
│   └── cli-linux-x64-musl/
│
├── schemas/
│   ├── raw.schema.json
│   ├── features.schema.json
│   └── ui.ir.schema.json
│
├── fixtures/
│   ├── raw/
│   ├── features/
│   └── ui-ir/
│
└── scripts/
    └── place-binary-like-ci.sh
```

---

# 3\. 各 crate の責務

## `crates/core`

全体の土台です。

### 置くもの

- 共通エラー型
- 設定型
- ノード ID / フレーム ID / セッション ID の newtype
- 共通 DTO
- ログ・診断コード
- `UiIr` 本体の Rust 型
- バージョン情報

### ここで定義するべき型

```
pub struct FrameId(pub String);
pub struct NodeId(pub String);
pub struct SessionId(pub String);
pub struct Confidence(pub f32);
```

文字列のまま通すと、後で全部崩れます。

---

## `crates/runtime`

Figma Desktop への接続だけを担当します。

### 責務

- Figma Desktop プロセス検出
- パッチ済みランタイムへの接続
- CDP 風の evaluate 呼び出し
- 接続状態確認
- セッション開始・終了
- 低レベル結果の回収

### 提供 API のイメージ

```
pub trait RuntimeClient {
    async fn connect(&self) -> Result<SessionHandle, RuntimeError>;
    async fn inspect_frame(&self, frame_id: &FrameId) -> Result<RawFrameSnapshot, RuntimeError>;
    async fn ping(&self) -> Result<(), RuntimeError>;
}
```

### ここでやってはいけないこと

- UI IR への変換
- atom / molecule 判定
- レポート生成

接続層は低レベルのままにします。

---

## `crates/extractor`

ランタイムから得た生情報を、**観測可能な特徴量** に変換します。

### 入力

- `RawFrameSnapshot`

### 出力

- `FeatureGraph`

### 抽出する特徴量

- ノード親子関係
- sibling order
- 深さ
- 反復 subtree fingerprint
- fill / stroke / radius
- text segments
- spacing / padding / alignment
- 画像・vector の有無
- レイヤー名
- 代表スタイル値
- 同型候補

ここはルールベースでよいです。  
LLM を入れる場所ではありません。

---

## `crates/normalizer`

特徴量から **意味付き UI 構造** に変換します。

### 役割

- layout-only node の吸収
- wrapper の圧縮
- section / group / card / list item の境界候補生成
- 表示上のまとまりから実装向けノードへ変換
- source node ids の集約

### 出力

- `UiIrDraft`

ここではまだ確定分類しません。  
候補を作る段階です。

---

## `crates/classifier`

アトミックデザインとコンポーネント候補を付けます。

### 出力するもの

- `component_candidates`
- `atomic_candidates`
- `confidence`
- `reusability`
- `diagnostics`

### 重要な判断

**単一ラベルを断定しない**。  
候補複数 + score にします。

---

## `crates/reporter`

人間向けの `report.md` を出します。

### 書く内容

- どこをコンポーネント境界と見なしたか
- 低信頼の箇所
- 反復パターン
- 実装時に確認すべき点
- atom / molecule / organism の候補と根拠

この crate は軽視されがちですが、実務上かなり効きます。

---

## `crates/schema`

JSON Schema の生成または保持を担当します。

### 役割

- `ui.ir.schema.json`
- `raw.schema.json`
- `features.schema.json`
- バージョンごとの差分管理

Rust 型から Schema を生成する方針でも、静的ファイルを置く方針でも構いません。  
ただし、**スキーマは API 互換性の中心**です。後回しにしてはいけません。

---

## `crates/store`

出力層です。

### 責務

- JSON 保存
- pretty print
- 将来的な SQLite 保存
- output directory 管理
- キャッシュ

最初は JSON だけで十分です。  
ただし、将来 SQLite を足せるよう分離しておきます。

---

## `crates/cli`

`clap` で表面コマンドを定義します。

---

# 4\. CLI 設計

初版はこれで足ります。

```
figma-ir attach
figma-ir doctor
figma-ir inspect frame --id 123:456
figma-ir extract frame --id 123:456 --out raw.json
figma-ir features frame --id 123:456 --out features.json
figma-ir normalize frame --id 123:456 --out ui.ir.json
figma-ir report frame --id 123:456 --out report.md
```

## 各コマンドの意味

### `attach`

- Figma Desktop へ接続
- セッション確立
- 必要なら runtime 情報を保存

### `doctor`

- Figma が動いているか
- パッチ済みか
- 接続できるか
- どこで失敗しているか

### `inspect frame`

- 指定フレームの概要だけ表示
- ノード数、テキスト数、画像数、想定 complexity

### `extract frame`

- `RawFrameSnapshot` を保存

### `features frame`

- `FeatureGraph` を保存

### `normalize frame`

- `UiIr` を保存

### `report frame`

- 人間向けレポート出力

---

# 5\. データフロー

```
Figma Desktop Runtime
    ↓
RawFrameSnapshot
    ↓ extractor
FeatureGraph
    ↓ normalizer
UiIrDraft
    ↓ classifier
UiIr
    ↓ reporter/store
ui.ir.json / report.md
```

これを崩さないでください。  
`Raw -> いきなり JSX` の直行は駄目です。

---

# 6\. Rust 側の主要型

以下のように切るのがよいです。

```
pub struct RawFrameSnapshot {
    pub source: RawSource,
    pub nodes: Vec<RawNode>,
    pub edges: Vec<RawEdge>,
    pub viewport: Option<RawViewport>,
    pub extracted_at: String,
}

pub struct FeatureGraph {
    pub source: FeatureSource,
    pub nodes: Vec<FeatureNode>,
    pub groups: Vec<FeatureGroup>,
    pub repeated_patterns: Vec<RepeatedPattern>,
    pub style_clusters: StyleClusters,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct UiIr {
    pub version: String,
    pub source: UiIrSource,
    pub tree: UiNode,
    pub tokens: TokenRefs,
    pub diagnostics: Vec<Diagnostic>,
}
```

---

# 7\. `UiIr` の設計原則

`ui.ir.json` は、単なるノードダンプでは駄目です。  
以下を必ず持たせます。

## 必須項目

- `version`
- `source`
- `tree`
- `diagnostics`

## 各ノードで必須に近い項目

- `id`
- `kind`
- `sourceNodeIds`
- `children`
- `componentCandidates`
- `atomicCandidates`
- `confidence`
- `layout`
- `styleRefs`
- `semantics`

## なぜ必要か

- `sourceNodeIds` がないと逆引きできない
- `componentCandidates` がないと設計判断を保持できない
- `atomicCandidates` がないと断定過剰になる
- `diagnostics` がないと信用できない

---

# 8\. `ui.ir.json` 初版 schema

以下は **初版として現実に使える最小構成** です。  
Draft 2020-12 前提です。

```
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://example.com/schemas/ui.ir.schema.json",
  "title": "UI Intermediate Representation",
  "type": "object",
  "additionalProperties": false,
  "required": ["version", "source", "tree", "tokens", "diagnostics"],
  "properties": {
    "version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+$"
    },
    "source": {
      "$ref": "#/$defs/UiIrSource"
    },
    "tree": {
      "$ref": "#/$defs/UiNode"
    },
    "tokens": {
      "$ref": "#/$defs/TokenRefs"
    },
    "diagnostics": {
      "type": "array",
      "items": {
        "$ref": "#/$defs/Diagnostic"
      }
    }
  },
  "$defs": {
    "UiIrSource": {
      "type": "object",
      "additionalProperties": false,
      "required": ["frameId", "frameName", "session"],
      "properties": {
        "fileKey": {
          "type": ["string", "null"]
        },
        "frameId": {
          "type": "string",
          "minLength": 1
        },
        "frameName": {
          "type": "string",
          "minLength": 1
        },
        "session": {
          "$ref": "#/$defs/SessionInfo"
        }
      }
    },
    "SessionInfo": {
      "type": "object",
      "additionalProperties": false,
      "required": ["transport", "capturedAt"],
      "properties": {
        "transport": {
          "type": "string",
          "enum": ["desktop_patched_cdp"]
        },
        "capturedAt": {
          "type": "string",
          "format": "date-time"
        },
        "figmaVersion": {
          "type": ["string", "null"]
        },
        "platform": {
          "type": ["string", "null"]
        }
      }
    },
    "UiNode": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "id",
        "kind",
        "sourceNodeIds",
        "componentCandidates",
        "atomicCandidates",
        "confidence",
        "layout",
        "styleRefs",
        "semantics",
        "children"
      ],
      "properties": {
        "id": {
          "type": "string",
          "minLength": 1
        },
        "name": {
          "type": ["string", "null"]
        },
        "kind": {
          "$ref": "#/$defs/UiNodeKind"
        },
        "sourceNodeIds": {
          "type": "array",
          "minItems": 1,
          "items": {
            "type": "string",
            "minLength": 1
          }
        },
        "componentCandidates": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ComponentCandidate"
          }
        },
        "atomicCandidates": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/AtomicCandidate"
          }
        },
        "confidence": {
          "$ref": "#/$defs/Confidence"
        },
        "reusability": {
          "$ref": "#/$defs/Reusability"
        },
        "layout": {
          "$ref": "#/$defs/LayoutSpec"
        },
        "styleRefs": {
          "$ref": "#/$defs/StyleRefs"
        },
        "semantics": {
          "$ref": "#/$defs/Semantics"
        },
        "content": {
          "$ref": "#/$defs/NodeContent"
        },
        "children": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/UiNode"
          }
        },
        "diagnostics": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/Diagnostic"
          }
        }
      }
    },
    "UiNodeKind": {
      "type": "string",
      "enum": [
        "Screen",
        "Template",
        "Section",
        "Region",
        "Container",
        "Stack",
        "Grid",
        "Card",
        "List",
        "ListItem",
        "Navigation",
        "Form",
        "FieldGroup",
        "Button",
        "Input",
        "Select",
        "Checkbox",
        "Radio",
        "Switch",
        "Tabs",
        "TabItem",
        "Heading",
        "Text",
        "TextBlock",
        "Label",
        "Icon",
        "Image",
        "Avatar",
        "Badge",
        "Divider",
        "Unknown"
      ]
    },
    "ComponentCandidate": {
      "type": "object",
      "additionalProperties": false,
      "required": ["name", "score"],
      "properties": {
        "name": {
          "type": "string",
          "minLength": 1
        },
        "score": {
          "$ref": "#/$defs/Confidence"
        },
        "reasonCodes": {
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "AtomicCandidate": {
      "type": "object",
      "additionalProperties": false,
      "required": ["level", "score"],
      "properties": {
        "level": {
          "type": "string",
          "enum": ["atom", "molecule", "organism", "template", "page", "unknown"]
        },
        "score": {
          "$ref": "#/$defs/Confidence"
        }
      }
    },
    "Confidence": {
      "type": "number",
      "minimum": 0,
      "maximum": 1
    },
    "Reusability": {
      "type": "object",
      "additionalProperties": false,
      "required": ["score"],
      "properties": {
        "score": {
          "$ref": "#/$defs/Confidence"
        },
        "reasons": {
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "LayoutSpec": {
      "type": "object",
      "additionalProperties": false,
      "required": ["kind"],
      "properties": {
        "kind": {
          "type": "string",
          "enum": ["none", "stack", "grid", "absolute", "inline", "unknown"]
        },
        "direction": {
          "type": ["string", "null"],
          "enum": ["horizontal", "vertical", null]
        },
        "gap": {
          "type": ["number", "null"]
        },
        "padding": {
          "type": ["array", "null"],
          "items": {
            "type": "number"
          },
          "minItems": 4,
          "maxItems": 4
        },
        "alignX": {
          "type": ["string", "null"],
          "enum": ["start", "center", "end", "stretch", "space-between", null]
        },
        "alignY": {
          "type": ["string", "null"],
          "enum": ["start", "center", "end", "stretch", "space-between", null]
        },
        "columns": {
          "type": ["integer", "null"],
          "minimum": 1
        }
      }
    },
    "StyleRefs": {
      "type": "object",
      "additionalProperties": false,
      "required": ["colors", "spacing", "radius", "typography", "effects"],
      "properties": {
        "colors": {
          "type": "array",
          "items": { "type": "string" }
        },
        "spacing": {
          "type": "array",
          "items": { "type": "string" }
        },
        "radius": {
          "type": "array",
          "items": { "type": "string" }
        },
        "typography": {
          "type": "array",
          "items": { "type": "string" }
        },
        "effects": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "Semantics": {
      "type": "object",
      "additionalProperties": false,
      "required": ["role"],
      "properties": {
        "role": {
          "type": "string",
          "enum": [
            "generic",
            "page",
            "section",
            "header",
            "footer",
            "nav",
            "main",
            "aside",
            "list",
            "listitem",
            "button",
            "link",
            "img",
            "heading",
            "label",
            "input",
            "form",
            "article",
            "unknown"
          ]
        },
        "accessibleName": {
          "type": ["string", "null"]
        }
      }
    },
    "NodeContent": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "text": {
          "type": ["string", "null"]
        },
        "placeholder": {
          "type": ["string", "null"]
        },
        "srcHint": {
          "type": ["string", "null"]
        },
        "iconHint": {
          "type": ["string", "null"]
        }
      }
    },
    "TokenRefs": {
      "type": "object",
      "additionalProperties": false,
      "required": ["colors", "spacing", "radius", "typography", "effects"],
      "properties": {
        "colors": {
          "type": "array",
          "items": { "type": "string" }
        },
        "spacing": {
          "type": "array",
          "items": { "type": "string" }
        },
        "radius": {
          "type": "array",
          "items": { "type": "string" }
        },
        "typography": {
          "type": "array",
          "items": { "type": "string" }
        },
        "effects": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "Diagnostic": {
      "type": "object",
      "additionalProperties": false,
      "required": ["level", "code", "message"],
      "properties": {
        "level": {
          "type": "string",
          "enum": ["info", "warning", "error"]
        },
        "code": {
          "type": "string",
          "minLength": 1
        },
        "message": {
          "type": "string",
          "minLength": 1
        },
        "nodeId": {
          "type": ["string", "null"]
        }
      }
    }
  }
}
```

---

# 9\. `ui.ir.json` の出力例

スキーマだけだと硬いので、実際の姿も置きます。

```
{
  "version": "0.1.0",
  "source": {
    "fileKey": null,
    "frameId": "123:456",
    "frameName": "Checkout / Summary",
    "session": {
      "transport": "desktop_patched_cdp",
      "capturedAt": "2026-03-10T12:34:56Z",
      "figmaVersion": "125.4.1",
      "platform": "darwin-arm64"
    }
  },
  "tree": {
    "id": "screen.checkout.summary",
    "name": "Checkout Summary",
    "kind": "Screen",
    "sourceNodeIds": ["123:456"],
    "componentCandidates": [
      {
        "name": "CheckoutSummaryScreen",
        "score": 0.98,
        "reasonCodes": ["ROOT_FRAME", "TOP_LEVEL_LAYOUT"]
      }
    ],
    "atomicCandidates": [
      {
        "level": "page",
        "score": 0.95
      }
    ],
    "confidence": 0.98,
    "reusability": {
      "score": 0.12,
      "reasons": ["SCREEN_LEVEL_NODE"]
    },
    "layout": {
      "kind": "stack",
      "direction": "vertical",
      "gap": 24,
      "padding": [24, 24, 24, 24],
      "alignX": "stretch",
      "alignY": "start",
      "columns": null
    },
    "styleRefs": {
      "colors": ["color.background.page"],
      "spacing": ["space.24"],
      "radius": [],
      "typography": [],
      "effects": []
    },
    "semantics": {
      "role": "page",
      "accessibleName": "Checkout Summary"
    },
    "content": {},
    "children": [
      {
        "id": "section.order-summary",
        "name": "Order Summary",
        "kind": "Section",
        "sourceNodeIds": ["123:460", "123:461"],
        "componentCandidates": [
          {
            "name": "OrderSummarySection",
            "score": 0.89,
            "reasonCodes": ["VISUAL_GROUP", "REPEATED_PADDING_PATTERN"]
          }
        ],
        "atomicCandidates": [
          {
            "level": "organism",
            "score": 0.82
          },
          {
            "level": "template",
            "score": 0.31
          }
        ],
        "confidence": 0.84,
        "reusability": {
          "score": 0.68,
          "reasons": ["SELF_CONTAINED_LAYOUT", "REUSABLE_TEXT_PATTERN"]
        },
        "layout": {
          "kind": "stack",
          "direction": "vertical",
          "gap": 16,
          "padding": [24, 24, 24, 24],
          "alignX": "stretch",
          "alignY": "start",
          "columns": null
        },
        "styleRefs": {
          "colors": ["color.surface.card", "color.text.primary"],
          "spacing": ["space.16", "space.24"],
          "radius": ["radius.md"],
          "typography": ["text.heading.lg", "text.body.md"],
          "effects": ["shadow.sm"]
        },
        "semantics": {
          "role": "section",
          "accessibleName": "Order Summary"
        },
        "content": {},
        "children": [
          {
            "id": "text.order-summary-title",
            "name": "Order Summary Title",
            "kind": "Heading",
            "sourceNodeIds": ["123:470"],
            "componentCandidates": [],
            "atomicCandidates": [
              {
                "level": "atom",
                "score": 0.94
              }
            ],
            "confidence": 0.96,
            "reusability": {
              "score": 0.41,
              "reasons": ["TEXT_STYLE_IS_REUSABLE"]
            },
            "layout": {
              "kind": "none",
              "direction": null,
              "gap": null,
              "padding": null,
              "alignX": null,
              "alignY": null,
              "columns": null
            },
            "styleRefs": {
              "colors": ["color.text.primary"],
              "spacing": [],
              "radius": [],
              "typography": ["text.heading.lg"],
              "effects": []
            },
            "semantics": {
              "role": "heading",
              "accessibleName": "Order Summary"
            },
            "content": {
              "text": "Order summary"
            },
            "children": []
          },
          {
            "id": "button.checkout",
            "name": "Checkout Button",
            "kind": "Button",
            "sourceNodeIds": ["123:480", "123:481", "123:482"],
            "componentCandidates": [
              {
                "name": "PrimaryButton",
                "score": 0.92,
                "reasonCodes": ["TEXT_PLUS_CONTAINER", "HIGH_RADIUS", "SINGLE_ACTION"]
              }
            ],
            "atomicCandidates": [
              {
                "level": "atom",
                "score": 0.78
              },
              {
                "level": "molecule",
                "score": 0.44
              }
            ],
            "confidence": 0.86,
            "reusability": {
              "score": 0.91,
              "reasons": ["COMMON_BUTTON_PATTERN", "PORTABLE_LABEL"]
            },
            "layout": {
              "kind": "inline",
              "direction": "horizontal",
              "gap": 8,
              "padding": [12, 16, 12, 16],
              "alignX": "center",
              "alignY": "center",
              "columns": null
            },
            "styleRefs": {
              "colors": ["color.button.primary.bg", "color.button.primary.fg"],
              "spacing": ["space.8", "space.12", "space.16"],
              "radius": ["radius.full"],
              "typography": ["text.label.md"],
              "effects": []
            },
            "semantics": {
              "role": "button",
              "accessibleName": "Checkout"
            },
            "content": {
              "text": "Checkout"
            },
            "children": [],
            "diagnostics": [
              {
                "level": "warning",
                "code": "ATOMIC_AMBIGUITY",
                "message": "Button includes icon and nested layout; molecule candidate retained.",
                "nodeId": "button.checkout"
              }
            ]
          }
        ],
        "diagnostics": [
          {
            "level": "warning",
            "code": "AMBIGUOUS_BOUNDARY",
            "message": "Section boundary inferred from repeated fill, radius and padding pattern.",
            "nodeId": "section.order-summary"
          }
        ]
      }
    ]
  },
  "tokens": {
    "colors": [
      "color.background.page",
      "color.surface.card",
      "color.text.primary",
      "color.button.primary.bg",
      "color.button.primary.fg"
    ],
    "spacing": ["space.8", "space.12", "space.16", "space.24"],
    "radius": ["radius.md", "radius.full"],
    "typography": ["text.heading.lg", "text.body.md", "text.label.md"],
    "effects": ["shadow.sm"]
  },
  "diagnostics": [
    {
      "level": "info",
      "code": "NORMALIZED_WITH_WRAPPER_COLLAPSE",
      "message": "3 layout-only wrappers were collapsed.",
      "nodeId": null
    }
  ]
}
```

---

# 10\. ここから先の実装順序

この順番で進めるのがよいです。

## 段階 1

`core`, `runtime`, `cli` だけ作る。

### 到達点

- `figma-ir attach`
- `figma-ir doctor`
- `figma-ir inspect frame --id ...`

## 段階 2

`extractor` を作る。

### 到達点

- `raw.json`
- `features.json`

## 段階 3

`normalizer` を作る。

### 到達点

- wrapper collapse
- visual group boundary
- repeated subtree boundary

## 段階 4

`classifier` を作る。

### 到達点

- `componentCandidates`
- `atomicCandidates`
- `confidence`

## 段階 5

`reporter`, `schema`, `store` を整える。

### 到達点

- `ui.ir.json`
- `report.md`
- schema validation

---

# 11\. 最後に、かなり率直な判断

この設計で勝負どころは **接続方法** ではありません。  
接続は入口にすぎません。

勝負どころはこの 3 つです。

## 1\. wrapper をどれだけ潰せるか

Figma のノイズを吸収できないと、IR がゴミになります。

## 2\. boundary をどれだけ妥当に出せるか

コンポーネント分割の説得力がここで決まります。

## 3\. 断定しすぎないこと

候補 + score + diagnostics を持たない設計は、長く使えません。

---

必要なら次に、**この workspace に対応した `Cargo.toml` の雛形** と **`clap` のサブコマンド定義** と **Rust の `UiIr` 型定義** をそのまま書きます。

## Prompt:

おねがします

## Response:

では、**そのまま着手できる雛形**として出します。

範囲は次の 3 つです。

1.  `Cargo.toml` と workspace 構成
2.  `clap` のサブコマンド定義
3.  `UiIr` と周辺型の Rust 定義

コードは、**sample-cli の配布骨格に載せやすい形**で、かつ **接続層・抽出層・正規化層を分離**する前提で切っています。

---

# 1\. ルート構成

```
.
├── Cargo.toml
├── rust-toolchain.toml
├── mise.toml
├── package.json
├── pnpm-workspace.yaml
├── turbo.json
│
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── ids.rs
│   │       ├── diagnostic.rs
│   │       ├── source.rs
│   │       ├── raw.rs
│   │       ├── features.rs
│   │       └── ui_ir.rs
│   │
│   ├── cli/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── args.rs
│   │       └── cmd/
│   │           ├── mod.rs
│   │           ├── attach.rs
│   │           ├── doctor.rs
│   │           ├── inspect.rs
│   │           ├── extract.rs
│   │           ├── features.rs
│   │           ├── normalize.rs
│   │           └── report.rs
│   │
│   ├── runtime/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── client.rs
│   │       ├── error.rs
│   │       └── session.rs
│   │
│   ├── extractor/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── normalizer/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── classifier/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── reporter/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── schema/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   │
│   └── store/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
│
└── schemas/
    └── ui.ir.schema.json
```

---

# 2\. ルート `Cargo.toml`

```
[workspace]
members = [
  "crates/core",
  "crates/cli",
  "crates/runtime",
  "crates/extractor",
  "crates/normalizer",
  "crates/classifier",
  "crates/reporter",
  "crates/schema",
  "crates/store",
]
resolver = "2"

[workspace.package]
edition = "2024"
license = "MIT"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[workspace.dependencies]
anyhow = "1.0.98"
thiserror = "2.0.12"
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
schemars = { version = "0.8.22", features = ["derive"] }
clap = { version = "4.5.32", features = ["derive", "env"] }
tokio = { version = "1.44.1", features = ["macros", "rt-multi-thread", "signal", "fs"] }
tracing = "0.1.41"
tracing-subscriber = { version = "0.3.19", features = ["env-filter", "fmt"] }
camino = { version = "1.1.9", features = ["serde1"] }
uuid = { version = "1.16.0", features = ["serde", "v4"] }
chrono = { version = "0.4.40", features = ["serde"] }
```

---

# 3\. `rust-toolchain.toml`

```
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
profile = "minimal"
```

---

# 4\. `mise.toml`

あなたの運用前提に合わせると、こういう形が扱いやすいです。

```
[tools]
rust = "stable"
node = "22"
pnpm = "10"

[tasks.fmt]
run = "cargo fmt --all"

[tasks.lint]
run = "cargo clippy --workspace --all-targets --all-features -- -D warnings"

[tasks.test]
run = "cargo test --workspace"

[tasks.check]
run = "cargo check --workspace"

[tasks.schema]
run = "cargo run -p schema"
```

---

# 5\. `crates/core/Cargo.toml`

```
[package]
name = "figma_ir_core"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
schemars.workspace = true
chrono.workspace = true
camino.workspace = true
uuid.workspace = true
thiserror.workspace = true
```

---

# 6\. `crates/cli/Cargo.toml`

```
[package]
name = "figma_ir_cli"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "figma-ir"
path = "src/main.rs"

[dependencies]
anyhow.workspace = true
clap.workspace = true
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
camino.workspace = true
serde_json.workspace = true

figma_ir_core = { path = "../core" }
figma_ir_runtime = { path = "../runtime" }
figma_ir_extractor = { path = "../extractor" }
figma_ir_normalizer = { path = "../normalizer" }
figma_ir_classifier = { path = "../classifier" }
figma_ir_reporter = { path = "../reporter" }
figma_ir_store = { path = "../store" }
```

---

# 7\. `crates/runtime/Cargo.toml`

```
[package]
name = "figma_ir_runtime"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
thiserror.workspace = true
serde.workspace = true
serde_json.workspace = true
tokio.workspace = true
chrono.workspace = true
uuid.workspace = true
figma_ir_core = { path = "../core" }
```

---

# 8\. 他 crate の最小 `Cargo.toml`

## `extractor`

```
[package]
name = "figma_ir_extractor"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
figma_ir_core = { path = "../core" }
```

## `normalizer`

```
[package]
name = "figma_ir_normalizer"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
figma_ir_core = { path = "../core" }
```

## `classifier`

```
[package]
name = "figma_ir_classifier"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
figma_ir_core = { path = "../core" }
```

## `reporter`

```
[package]
name = "figma_ir_reporter"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
figma_ir_core = { path = "../core" }
```

## `schema`

```
[package]
name = "figma_ir_schema"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "figma-ir-schema"
path = "src/lib.rs"

[dependencies]
anyhow.workspace = true
schemars.workspace = true
serde_json.workspace = true
figma_ir_core = { path = "../core" }
```

## `store`

```
[package]
name = "figma_ir_store"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
camino.workspace = true
tokio.workspace = true
figma_ir_core = { path = "../core" }
```

---

# 9\. `core` の `lib.rs`

```
pub mod diagnostic;
pub mod features;
pub mod ids;
pub mod raw;
pub mod source;
pub mod ui_ir;

pub use diagnostic::*;
pub use features::*;
pub use ids::*;
pub use raw::*;
pub use source::*;
pub use ui_ir::*;
```

---

# 10\. ID 型定義 `ids.rs`

文字列を生で回すのを止めます。

```
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct FrameId(pub String);

impl Display for FrameId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct NodeId(pub String);

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct SessionId(pub String);

impl Display for SessionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
```

---

# 11\. 診断型 `diagnostic.rs`

```
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}
```

---

# 12\. ソース情報 `source.rs`

```
use crate::ids::{FrameId, SessionId};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TransportKind {
    DesktopPatchedCdp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SessionInfo {
    pub id: SessionId,
    pub transport: TransportKind,
    pub captured_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub figma_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SourceRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key: Option<String>,
    pub frame_id: FrameId,
    pub frame_name: String,
    pub session: SessionInfo,
}
```

---

# 13\. Raw 型 `raw.rs`

これは「ランタイムから取れたもの」を置く層です。意味づけしません。

```
use crate::diagnostic::Diagnostic;
use crate::ids::{FrameId, NodeId};
use crate::source::SourceRef;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RawNodeType {
    Frame,
    Group,
    Rectangle,
    Text,
    Vector,
    Component,
    Instance,
    Ellipse,
    Line,
    Polygon,
    Star,
    BooleanOperation,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawTextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawNode {
    pub id: NodeId,
    pub name: String,
    pub node_type: RawNodeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<NodeId>,
    pub children: Vec<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<RawRect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fills: Option<Vec<RawColor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style: Option<RawTextStyle>,
    #[serde(default)]
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawEdge {
    pub parent: NodeId,
    pub child: NodeId,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawFrameSnapshot {
    pub source: SourceRef,
    pub root_frame_id: FrameId,
    pub nodes: Vec<RawNode>,
    pub edges: Vec<RawEdge>,
    pub diagnostics: Vec<Diagnostic>,
}
```

---

# 14\. Features 型 `features.rs`

ここで初めて特徴量になります。

```
use crate::diagnostic::Diagnostic;
use crate::ids::NodeId;
use crate::source::SourceRef;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LayoutKind {
    None,
    Stack,
    Grid,
    Absolute,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayoutFeatures {
    pub kind: LayoutKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<[f64; 4]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VisualFeatures {
    pub has_fill: bool,
    pub has_stroke: bool,
    pub has_radius: bool,
    pub has_shadow: bool,
    pub has_image_fill: bool,
    pub has_vector: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TextFeatures {
    pub is_textual: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count_estimate: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RepetitionFeatures {
    pub repeated_subtree_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FeatureNode {
    pub id: NodeId,
    pub name: String,
    pub depth: usize,
    pub child_count: usize,
    pub layout: LayoutFeatures,
    pub visual: VisualFeatures,
    pub text: TextFeatures,
    pub repetition: RepetitionFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FeatureGraph {
    pub source: SourceRef,
    pub nodes: Vec<FeatureNode>,
    pub diagnostics: Vec<Diagnostic>,
}
```

---

# 15\. `UiIr` 型 `ui_ir.rs`

ここが中核です。

```
use crate::diagnostic::Diagnostic;
use crate::source::SourceRef;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct Confidence(pub f32);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UiNodeKind {
    Screen,
    Template,
    Section,
    Region,
    Container,
    Stack,
    Grid,
    Card,
    List,
    ListItem,
    Navigation,
    Form,
    FieldGroup,
    Button,
    Input,
    Select,
    Checkbox,
    Radio,
    Switch,
    Tabs,
    TabItem,
    Heading,
    Text,
    TextBlock,
    Label,
    Icon,
    Image,
    Avatar,
    Badge,
    Divider,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AtomicLevel {
    Atom,
    Molecule,
    Organism,
    Template,
    Page,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRole {
    Generic,
    Page,
    Section,
    Header,
    Footer,
    Nav,
    Main,
    Aside,
    List,
    ListItem,
    Button,
    Link,
    Img,
    Heading,
    Label,
    Input,
    Form,
    Article,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LayoutSpecKind {
    None,
    Stack,
    Grid,
    Absolute,
    Inline,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
    SpaceBetween,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentCandidate {
    pub name: String,
    pub score: Confidence,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AtomicCandidate {
    pub level: AtomicLevel,
    pub score: Confidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Reusability {
    pub score: Confidence,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayoutSpec {
    pub kind: LayoutSpecKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<LayoutDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<[f64; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_x: Option<Align>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_y: Option<Align>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct StyleRefs {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub colors: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spacing: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub radius: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub typography: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Semantics {
    pub role: SemanticRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessible_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct NodeContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UiNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub kind: UiNodeKind,
    pub source_node_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component_candidates: Vec<ComponentCandidate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub atomic_candidates: Vec<AtomicCandidate>,
    pub confidence: Confidence,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusability: Option<Reusability>,
    pub layout: LayoutSpec,
    pub style_refs: StyleRefs,
    pub semantics: Semantics,
    #[serde(default, skip_serializing_if = "NodeContent::is_empty")]
    pub content: NodeContent,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<UiNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
}

impl NodeContent {
    pub fn is_empty(&self) -> bool {
        self.text.is_none()
            && self.placeholder.is_none()
            && self.src_hint.is_none()
            && self.icon_hint.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct TokenRefs {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub colors: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spacing: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub radius: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub typography: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UiIr {
    pub version: String,
    pub source: SourceRef,
    pub tree: UiNode,
    pub tokens: TokenRefs,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
}
```

---

# 16\. CLI 引数 `crates/cli/src/args.rs`

`clap` は最初から静かに整理しておいた方がよいです。

```
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "figma-ir")]
#[command(version)]
#[command(about = "Extract implementation-oriented UI IR from a Figma Desktop runtime")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Attach(AttachArgs),
    Doctor(DoctorArgs),
    Inspect(InspectCommand),
    Extract(ExtractCommand),
    Features(FeaturesCommand),
    Normalize(NormalizeCommand),
    Report(ReportCommand),
}

#[derive(Debug, Args)]
pub struct AttachArgs {
    #[arg(long)]
    pub verbose: bool,
}

#[derive(Debug, Args)]
pub struct DoctorArgs {
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Subcommand)]
pub enum InspectCommand {
    Frame(FrameInspectArgs),
}

#[derive(Debug, Args)]
pub struct FrameInspectArgs {
    #[arg(long)]
    pub id: String,
}

#[derive(Debug, Subcommand)]
pub enum ExtractCommand {
    Frame(FrameExtractArgs),
}

#[derive(Debug, Args)]
pub struct FrameExtractArgs {
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub out: Utf8PathBuf,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Subcommand)]
pub enum FeaturesCommand {
    Frame(FrameFeaturesArgs),
}

#[derive(Debug, Args)]
pub struct FrameFeaturesArgs {
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub out: Utf8PathBuf,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Subcommand)]
pub enum NormalizeCommand {
    Frame(FrameNormalizeArgs),
}

#[derive(Debug, Args)]
pub struct FrameNormalizeArgs {
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub out: Utf8PathBuf,
    #[arg(long)]
    pub pretty: bool,
}

#[derive(Debug, Subcommand)]
pub enum ReportCommand {
    Frame(FrameReportArgs),
}

#[derive(Debug, Args)]
pub struct FrameReportArgs {
    #[arg(long)]
    pub id: String,
    #[arg(long)]
    pub out: Utf8PathBuf,
}
```

---

# 17\. `main.rs`

```
mod args;
mod cmd;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};

use args::{Cli, Command};

#[tokio::main]
async fn main() -> Result<()> {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Attach(args) => cmd::attach::run(args).await,
        Command::Doctor(args) => cmd::doctor::run(args).await,
        Command::Inspect(cmd) => cmd::inspect::run(cmd).await,
        Command::Extract(cmd) => cmd::extract::run(cmd).await,
        Command::Features(cmd) => cmd::features::run(cmd).await,
        Command::Normalize(cmd) => cmd::normalize::run(cmd).await,
        Command::Report(cmd) => cmd::report::run(cmd).await,
    }
}
```

---

# 18\. `cmd/mod.rs`

```
pub mod attach;
pub mod doctor;
pub mod extract;
pub mod features;
pub mod inspect;
pub mod normalize;
pub mod report;
```

---

# 19\. コマンド実装の骨格

## `attach.rs`

```
use anyhow::Result;

use crate::args::AttachArgs;

pub async fn run(args: AttachArgs) -> Result<()> {
    if args.verbose {
        println!("attaching to figma desktop runtime...");
    }
    println!("attached");
    Ok(())
}
```

## `doctor.rs`

```
use anyhow::Result;
use serde_json::json;

use crate::args::DoctorArgs;

pub async fn run(args: DoctorArgs) -> Result<()> {
    let report = json!({
        "runtime_detected": true,
        "connectable": true,
        "transport": "desktop_patched_cdp"
    });

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("runtime_detected: true");
        println!("connectable: true");
        println!("transport: desktop_patched_cdp");
    }

    Ok(())
}
```

## `inspect.rs`

```
use anyhow::Result;

use crate::args::{FrameInspectArgs, InspectCommand};

pub async fn run(cmd: InspectCommand) -> Result<()> {
    match cmd {
        InspectCommand::Frame(FrameInspectArgs { id }) => {
            println!("frame_id: {}", id);
            println!("status: not yet implemented");
        }
    }
    Ok(())
}
```

## `extract.rs`

```
use anyhow::Result;
use figma_ir_core::{Diagnostic, DiagnosticLevel, FrameId, RawEdge, RawFrameSnapshot, RawNode, RawNodeType, SessionId, SessionInfo, SourceRef, TransportKind};
use figma_ir_store::write_json_file;

use crate::args::{ExtractCommand, FrameExtractArgs};

pub async fn run(cmd: ExtractCommand) -> Result<()> {
    match cmd {
        ExtractCommand::Frame(args) => run_frame(args).await,
    }
}

async fn run_frame(args: FrameExtractArgs) -> Result<()> {
    let snapshot = RawFrameSnapshot {
        source: SourceRef {
            file_key: None,
            frame_id: FrameId(args.id.clone()),
            frame_name: "Example Frame".to_string(),
            session: SessionInfo {
                id: SessionId("session-demo".to_string()),
                transport: TransportKind::DesktopPatchedCdp,
                captured_at: chrono::Utc::now(),
                figma_version: None,
                platform: None,
            },
        },
        root_frame_id: FrameId(args.id),
        nodes: vec![RawNode {
            id: figma_ir_core::NodeId("123:456".to_string()),
            name: "Example Frame".to_string(),
            node_type: RawNodeType::Frame,
            parent_id: None,
            children: vec![],
            bounds: None,
            fills: None,
            radius: None,
            text: None,
            text_style: None,
            visible: true,
        }],
        edges: vec![RawEdge {
            parent: figma_ir_core::NodeId("123:456".to_string()),
            child: figma_ir_core::NodeId("123:456".to_string()),
            index: 0,
        }],
        diagnostics: vec![Diagnostic {
            level: DiagnosticLevel::Info,
            code: "STUB".to_string(),
            message: "stub raw snapshot".to_string(),
            node_id: None,
        }],
    };

    write_json_file(&args.out, &snapshot, args.pretty).await?;
    println!("wrote {}", args.out);
    Ok(())
}
```

## `features.rs`

```
use anyhow::Result;
use figma_ir_core::{
    Axis, Diagnostic, DiagnosticLevel, FeatureGraph, FeatureNode, FrameId, LayoutFeatures,
    LayoutKind, RepetitionFeatures, SessionId, SessionInfo, SourceRef, TextFeatures,
    TransportKind, VisualFeatures,
};
use figma_ir_store::write_json_file;

use crate::args::{FeaturesCommand, FrameFeaturesArgs};

pub async fn run(cmd: FeaturesCommand) -> Result<()> {
    match cmd {
        FeaturesCommand::Frame(args) => run_frame(args).await,
    }
}

async fn run_frame(args: FrameFeaturesArgs) -> Result<()> {
    let features = FeatureGraph {
        source: SourceRef {
            file_key: None,
            frame_id: FrameId(args.id),
            frame_name: "Example Frame".to_string(),
            session: SessionInfo {
                id: SessionId("session-demo".to_string()),
                transport: TransportKind::DesktopPatchedCdp,
                captured_at: chrono::Utc::now(),
                figma_version: None,
                platform: None,
            },
        },
        nodes: vec![FeatureNode {
            id: figma_ir_core::NodeId("123:456".to_string()),
            name: "Example Frame".to_string(),
            depth: 0,
            child_count: 0,
            layout: LayoutFeatures {
                kind: LayoutKind::Stack,
                direction: Some(Axis::Vertical),
                gap: Some(24.0),
                padding: Some([24.0, 24.0, 24.0, 24.0]),
            },
            visual: VisualFeatures {
                has_fill: true,
                has_stroke: false,
                has_radius: false,
                has_shadow: false,
                has_image_fill: false,
                has_vector: false,
            },
            text: TextFeatures {
                is_textual: false,
                content_length: None,
                line_count_estimate: None,
            },
            repetition: RepetitionFeatures {
                repeated_subtree_count: 0,
                fingerprint: None,
            },
        }],
        diagnostics: vec![Diagnostic {
            level: DiagnosticLevel::Info,
            code: "STUB".to_string(),
            message: "stub feature graph".to_string(),
            node_id: None,
        }],
    };

    write_json_file(&args.out, &features, args.pretty).await?;
    println!("wrote {}", args.out);
    Ok(())
}
```

## `normalize.rs`

```
use anyhow::Result;
use figma_ir_core::{
    Align, AtomicCandidate, AtomicLevel, Confidence, FrameId, LayoutDirection, LayoutSpec,
    LayoutSpecKind, SemanticRole, Semantics, SessionId, SessionInfo, SourceRef, StyleRefs,
    TokenRefs, TransportKind, UiIr, UiNode, UiNodeKind,
};
use figma_ir_store::write_json_file;

use crate::args::{FrameNormalizeArgs, NormalizeCommand};

pub async fn run(cmd: NormalizeCommand) -> Result<()> {
    match cmd {
        NormalizeCommand::Frame(args) => run_frame(args).await,
    }
}

async fn run_frame(args: FrameNormalizeArgs) -> Result<()> {
    let ir = UiIr {
        version: "0.1.0".to_string(),
        source: SourceRef {
            file_key: None,
            frame_id: FrameId(args.id),
            frame_name: "Example Frame".to_string(),
            session: SessionInfo {
                id: SessionId("session-demo".to_string()),
                transport: TransportKind::DesktopPatchedCdp,
                captured_at: chrono::Utc::now(),
                figma_version: None,
                platform: None,
            },
        },
        tree: UiNode {
            id: "screen.example".to_string(),
            name: Some("Example Screen".to_string()),
            kind: UiNodeKind::Screen,
            source_node_ids: vec!["123:456".to_string()],
            component_candidates: vec![],
            atomic_candidates: vec![AtomicCandidate {
                level: AtomicLevel::Page,
                score: Confidence(0.95),
            }],
            confidence: Confidence(0.98),
            reusability: None,
            layout: LayoutSpec {
                kind: LayoutSpecKind::Stack,
                direction: Some(LayoutDirection::Vertical),
                gap: Some(24.0),
                padding: Some([24.0, 24.0, 24.0, 24.0]),
                align_x: Some(Align::Stretch),
                align_y: Some(Align::Start),
                columns: None,
            },
            style_refs: StyleRefs::default(),
            semantics: Semantics {
                role: SemanticRole::Page,
                accessible_name: Some("Example Screen".to_string()),
            },
            content: Default::default(),
            children: vec![],
            diagnostics: vec![],
        },
        tokens: TokenRefs::default(),
        diagnostics: vec![],
    };

    write_json_file(&args.out, &ir, args.pretty).await?;
    println!("wrote {}", args.out);
    Ok(())
}
```

## `report.rs`

```
use anyhow::Result;
use tokio::fs;

use crate::args::{FrameReportArgs, ReportCommand};

pub async fn run(cmd: ReportCommand) -> Result<()> {
    match cmd {
        ReportCommand::Frame(args) => run_frame(args).await,
    }
}

async fn run_frame(args: FrameReportArgs) -> Result<()> {
    let body = format!(
        "# Figma IR Report\n\n- frame id: {}\n- status: stub\n",
        args.id
    );
    fs::write(&args.out, body).await?;
    println!("wrote {}", args.out);
    Ok(())
}
```

---

# 20\. `store` の最低限実装

```
use anyhow::Result;
use camino::Utf8Path;
use serde::Serialize;
use tokio::fs;

pub async fn write_json_file<T: Serialize>(
    path: &Utf8Path,
    value: &T,
    pretty: bool,
) -> Result<()> {
    let content = if pretty {
        serde_json::to_string_pretty(value)?
    } else {
        serde_json::to_string(value)?
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    fs::write(path, content).await?;
    Ok(())
}
```

---

# 21\. `schema` の最低限実装

`schemars` から schema を吐く雛形です。

```
use anyhow::Result;
use figma_ir_core::UiIr;
use schemars::schema_for;

fn main() -> Result<()> {
    let schema = schema_for!(UiIr);
    println!("{}", serde_json::to_string_pretty(&schema)?);
    Ok(())
}
```

これで `mise run schema > schemas/ui.ir.schema.json` のように使えます。

---

# 22\. ここから先に何を実装するか

今の雛形は、**構造を固定するための最小骨格**です。  
次に入れるべき実装は順番が大事です。

## 最初に入れるもの

### `runtime`

- Figma Desktop プロセス検出
- 接続確認
- frame 指定で raw snapshot を引く

### `extractor`

- depth 計算
- child_count 計算
- text 判定
- radius / fill / shadow 有無
- repeated subtree fingerprint

### `normalizer`

- layout-only wrapper collapse
- visual boundary detection
- section / card / list item 候補生成

### `classifier`

- component candidate scoring
- atomic candidate scoring
- diagnostics 追加

---

# 23\. 逆に、今は入れない方がよいもの

- JSX 生成
- Storybook 出力
- React コンポーネント出力
- デザイントークン自動命名の完成版
- 名前の自動美化
- AI ベースの全面分類

これを先に入れると、IR が固まる前に設計が崩れます。

---

# 24\. 一番大事な注意

この骨格で肝になるのは `UiIr` の shape です。  
ここは安易にいじらない方がよいです。

特に次の項目は後から消すと必ず困ります。

- `source_node_ids`
- `component_candidates`
- `atomic_candidates`
- `confidence`
- `diagnostics`

この 5 つは、単なる飾りではなく、**誤判定を人間が回収するための保険**です。

---

必要なら次に、**`runtime` crate の trait 設計** と **`normalizer` の boundary detection のアルゴリズム骨格** をそのまま書きます。

## Prompt:

お願いします

## Response:

では、次は **`runtime` crate の trait 設計** と **`normalizer` の boundary detection 骨格** を詰めます。

ここはかなり大事です。  
この 2 つが曖昧だと、あとで `extractor` も `classifier` も全部ぶれます。

---

# 1\. `runtime` の設計方針

あなたの条件では、`runtime` は次を満たす必要があります。

- **Figma Desktop 接続の詳細を外へ漏らさない**
- **抽出側は「フレームを観測したい」とだけ言える**
- **将来、接続方法を差し替えられる**
- **生データの責務に徹する**

つまり、`runtime` は **transport adapter** として切ります。

---

## `runtime` が提供すべき最小機能

必要なのは多くありません。

1.  ランタイムに接続する
2.  接続状態を診断する
3.  フレーム単位で生スナップショットを取る
4.  セッション情報を返す

この 4 つです。

---

# 2\. `runtime` crate の公開 API

## `lib.rs`

```
pub mod client;
pub mod error;
pub mod models;
pub mod session;

pub use client::*;
pub use error::*;
pub use models::*;
pub use session::*;
```

---

## `error.rs`

```
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("figma desktop process not found")]
    ProcessNotFound,

    #[error("figma desktop runtime is not patch-enabled")]
    NotPatchEnabled,

    #[error("failed to connect to runtime: {0}")]
    ConnectFailed(String),

    #[error("runtime disconnected")]
    Disconnected,

    #[error("frame not found: {0}")]
    FrameNotFound(String),

    #[error("evaluation failed: {0}")]
    EvaluationFailed(String),

    #[error("invalid runtime response: {0}")]
    InvalidResponse(String),

    #[error("unsupported platform: {0}")]
    UnsupportedPlatform(String),
}
```

---

## `session.rs`

```
use chrono::{DateTime, Utc};
use figma_ir_core::{SessionId, TransportKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSession {
    pub id: SessionId,
    pub transport: TransportKind,
    pub connected_at: DateTime<Utc>,
    pub figma_version: Option<String>,
    pub platform: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeHealth {
    pub runtime_detected: bool,
    pub connectable: bool,
    pub transport: Option<TransportKind>,
    pub figma_version: Option<String>,
    pub platform: Option<String>,
    pub message: Option<String>,
}
```

---

## `models.rs`

ここで **runtime 内部専用の観測モデル** を持ちます。  
`core::raw` に直接寄せてもよいですが、私は一段置いた方がよいと思います。

理由は、接続層の都合を `RawFrameSnapshot` に混ぜないためです。

```
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeFrameRequest {
    pub frame_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeNodeSnapshot {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub parent_id: Option<String>,
    pub children: Vec<String>,

    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,

    pub visible: bool,

    pub fills: Vec<RuntimeColor>,
    pub radius: Option<f64>,

    pub text: Option<String>,
    pub text_style: Option<RuntimeTextStyle>,

    pub layout_mode: Option<String>,
    pub item_spacing: Option<f64>,
    pub padding_top: Option<f64>,
    pub padding_right: Option<f64>,
    pub padding_bottom: Option<f64>,
    pub padding_left: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeTextStyle {
    pub font_family: Option<String>,
    pub font_weight: Option<u16>,
    pub font_size: Option<f64>,
    pub line_height: Option<f64>,
    pub letter_spacing: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeFrameSnapshot {
    pub frame_id: String,
    pub frame_name: String,
    pub nodes: Vec<RuntimeNodeSnapshot>,
}
```

---

## `client.rs`

ここが中核です。

```
use async_trait::async_trait;

use crate::{
    RuntimeError, RuntimeFrameRequest, RuntimeFrameSnapshot, RuntimeHealth, RuntimeSession,
};

#[async_trait]
pub trait RuntimeClient: Send + Sync {
    async fn attach(&self) -> Result<RuntimeSession, RuntimeError>;

    async fn health(&self) -> Result<RuntimeHealth, RuntimeError>;

    async fn snapshot_frame(
        &self,
        request: RuntimeFrameRequest,
    ) -> Result<RuntimeFrameSnapshot, RuntimeError>;
}
```

---

# 3\. なぜ `eval(String)` を trait に出さないのか

これはかなり大事です。

`figma-cli` 系の発想に引っ張られると、つい `eval(js_code: String)` を公開 API にしたくなります。  
しかし、それをやると設計が壊れます。

## 理由

- runtime 層が万能になって責務が漏れる
- extractor / normalizer が JS 文字列を組み立て始める
- テスト不能になる
- transport 差し替え不能になる
- 事実上の「何でもできる裏口」になる

したがって、**`eval` は内部実装だけに閉じる**のがよいです。  
外からは `snapshot_frame` のような **用途ベース API** しか見せません。

---

# 4\. `runtime` の実装構造

将来の差し替えを見越して、実装はこう切ります。

```
runtime/
├── client.rs
├── error.rs
├── models.rs
├── session.rs
├── transport/
│   ├── mod.rs
│   ├── patched_cdp.rs
│   └── js_bridge.rs
└── mapper.rs
```

---

## `transport/patched_cdp.rs`

ここでやることは限定します。

- Figma Desktop に接続
- JS をランタイムへ送る
- JSON 文字列で結果を受ける

```
use crate::{
    RuntimeClient, RuntimeError, RuntimeFrameRequest, RuntimeFrameSnapshot, RuntimeHealth,
    RuntimeSession,
};

pub struct PatchedCdpRuntimeClient {
    // ws client, endpoint, auth, etc.
}

impl PatchedCdpRuntimeClient {
    pub fn new() -> Self {
        Self {}
    }
}
```

ここで `RuntimeClient` を実装します。

---

## `transport/js_bridge.rs`

Figma ランタイム上で動かす JS スニペット生成をここへ閉じ込めます。

```
pub fn build_snapshot_frame_script(frame_id: &str) -> String {
    format!(
        r#"
(() => {{
  const targetId = {frame_id:?};

  function visit(node, parentId = null, acc = []) {{
    const bounds = "absoluteRenderBounds" in node ? node.absoluteRenderBounds : null;

    acc.push({{
      id: node.id,
      name: node.name,
      node_type: node.type,
      parent_id: parentId,
      children: "children" in node ? node.children.map(c => c.id) : [],
      x: bounds ? bounds.x : null,
      y: bounds ? bounds.y : null,
      width: bounds ? bounds.width : null,
      height: bounds ? bounds.height : null,
      visible: "visible" in node ? node.visible : true,
      fills: Array.isArray(node.fills)
        ? node.fills
            .filter(f => f && f.type === "SOLID" && f.color)
            .map(f => ({{
              r: f.color.r,
              g: f.color.g,
              b: f.color.b,
              a: f.opacity ?? 1
            }}))
        : [],
      radius: "cornerRadius" in node && typeof node.cornerRadius === "number"
        ? node.cornerRadius
        : null,
      text: node.type === "TEXT" ? node.characters : null,
      text_style: node.type === "TEXT"
        ? {{
            font_family: node.fontName && typeof node.fontName !== "symbol" ? node.fontName.family ?? null : null,
            font_weight: null,
            font_size: node.fontSize ?? null,
            line_height: typeof node.lineHeight === "object" ? node.lineHeight.value ?? null : null,
            letter_spacing: typeof node.letterSpacing === "object" ? node.letterSpacing.value ?? null : null
          }}
        : null,
      layout_mode: "layoutMode" in node ? node.layoutMode : null,
      item_spacing: "itemSpacing" in node ? node.itemSpacing : null,
      padding_top: "paddingTop" in node ? node.paddingTop : null,
      padding_right: "paddingRight" in node ? node.paddingRight : null,
      padding_bottom: "paddingBottom" in node ? node.paddingBottom : null,
      padding_left: "paddingLeft" in node ? node.paddingLeft : null
    }});

    if ("children" in node) {{
      for (const child of node.children) {{
        visit(child, node.id, acc);
      }}
    }}

    return acc;
  }}

  const root = figma.currentPage.findOne(n => n.id === targetId);
  if (!root) {{
    return JSON.stringify({{ error: "FRAME_NOT_FOUND", frame_id: targetId }});
  }}

  const nodes = visit(root);

  return JSON.stringify({{
    frame_id: root.id,
    frame_name: root.name,
    nodes
  }});
}})();
"#
    )
}
```

この関数はまだ荒いですが、考え方としてはこれで十分です。

---

# 5\. `runtime` から `core::RawFrameSnapshot` への変換

接続層の結果を、そのまま `core` に流さないように変換関数を挟みます。

`mapper.rs` を置きます。

```
use chrono::Utc;
use figma_ir_core::{
    Diagnostic, FrameId, NodeId, RawColor, RawEdge, RawFrameSnapshot, RawNode, RawNodeType,
    RawRect, RawTextStyle, SessionId, SessionInfo, SourceRef, TransportKind,
};

use crate::{RuntimeFrameSnapshot, RuntimeSession};

pub fn map_runtime_snapshot_to_raw(
    session: &RuntimeSession,
    snapshot: RuntimeFrameSnapshot,
) -> RawFrameSnapshot {
    let nodes = snapshot
        .nodes
        .iter()
        .map(|n| RawNode {
            id: NodeId(n.id.clone()),
            name: n.name.clone(),
            node_type: map_node_type(&n.node_type),
            parent_id: n.parent_id.clone().map(NodeId),
            children: n.children.iter().cloned().map(NodeId).collect(),
            bounds: match (n.x, n.y, n.width, n.height) {
                (Some(x), Some(y), Some(width), Some(height)) => Some(RawRect {
                    x,
                    y,
                    width,
                    height,
                }),
                _ => None,
            },
            fills: if n.fills.is_empty() {
                None
            } else {
                Some(
                    n.fills
                        .iter()
                        .map(|c| RawColor {
                            r: c.r,
                            g: c.g,
                            b: c.b,
                            a: c.a,
                        })
                        .collect(),
                )
            },
            radius: n.radius,
            text: n.text.clone(),
            text_style: n.text_style.as_ref().map(|t| RawTextStyle {
                font_family: t.font_family.clone(),
                font_weight: t.font_weight,
                font_size: t.font_size,
                line_height: t.line_height,
                letter_spacing: t.letter_spacing,
            }),
            visible: n.visible,
        })
        .collect::<Vec<_>>();

    let edges = snapshot
        .nodes
        .iter()
        .flat_map(|n| {
            n.children
                .iter()
                .enumerate()
                .map(|(index, child)| RawEdge {
                    parent: NodeId(n.id.clone()),
                    child: NodeId(child.clone()),
                    index,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    RawFrameSnapshot {
        source: SourceRef {
            file_key: None,
            frame_id: FrameId(snapshot.frame_id.clone()),
            frame_name: snapshot.frame_name,
            session: SessionInfo {
                id: SessionId(session.id.0.clone()),
                transport: TransportKind::DesktopPatchedCdp,
                captured_at: Utc::now(),
                figma_version: session.figma_version.clone(),
                platform: session.platform.clone(),
            },
        },
        root_frame_id: FrameId(snapshot.frame_id),
        nodes,
        edges,
        diagnostics: vec![Diagnostic {
            level: figma_ir_core::DiagnosticLevel::Info,
            code: "RUNTIME_SNAPSHOT_CAPTURED".to_string(),
            message: "raw snapshot captured from runtime".to_string(),
            node_id: None,
        }],
    }
}

fn map_node_type(value: &str) -> RawNodeType {
    match value {
        "FRAME" => RawNodeType::Frame,
        "GROUP" => RawNodeType::Group,
        "RECTANGLE" => RawNodeType::Rectangle,
        "TEXT" => RawNodeType::Text,
        "VECTOR" => RawNodeType::Vector,
        "COMPONENT" => RawNodeType::Component,
        "INSTANCE" => RawNodeType::Instance,
        "ELLIPSE" => RawNodeType::Ellipse,
        "LINE" => RawNodeType::Line,
        "POLYGON" => RawNodeType::Polygon,
        "STAR" => RawNodeType::Star,
        "BOOLEAN_OPERATION" => RawNodeType::BooleanOperation,
        _ => RawNodeType::Unknown,
    }
}
```

---

# 6\. `normalizer` の考え方

ここからが本題です。

`normalizer` は、`FeatureGraph` を受けて `UiIr` の下書きを作る層です。  
最大の責務は、**Figma の構造ノイズを減らして、実装単位に寄せること**です。

つまり、ここでやるべきことは以下です。

1.  layout-only wrapper を潰す
2.  視覚的まとまりから境界候補を出す
3.  反復構造を list / card 候補へ上げる
4.  section / group / region の候補を作る
5.  生ノード列から UI ノード木へ組み替える

---

# 7\. `normalizer` の入力と出力

## `lib.rs`

```
mod boundary;
mod collapse;
mod context;
mod normalize;

pub use normalize::*;
```

---

## `context.rs`

正規化中に必要な index 群を作ります。

```
use std::collections::{HashMap, HashSet};

use figma_ir_core::{FeatureGraph, FeatureNode};

pub struct NormalizeContext<'a> {
    pub graph: &'a FeatureGraph,
    pub by_id: HashMap<&'a str, &'a FeatureNode>,
    pub parent_to_children: HashMap<&'a str, Vec<&'a FeatureNode>>,
    pub repeated_fingerprints: HashSet<&'a str>,
}

impl<'a> NormalizeContext<'a> {
    pub fn new(graph: &'a FeatureGraph) -> Self {
        let mut by_id = HashMap::new();
        for node in &graph.nodes {
            by_id.insert(node.id.0.as_str(), node);
        }

        Self {
            graph,
            by_id,
            parent_to_children: HashMap::new(),
            repeated_fingerprints: HashSet::new(),
        }
    }
}
```

この例ではまだ親子関係を十分持っていません。  
`FeatureGraph` 側に parent/children を足してもよいです。

---

# 8\. boundary detection の発想

境界判定は **単一ルールでやらない**方がよいです。  
複数の detector を走らせ、その合議で決めるべきです。

---

## detector の種類

### 1\. `visual_group_boundary`

背景色、角丸、影、padding がそろっているまとまり

### 2\. `repetition_boundary`

同型 subtree が複数回現れるまとまり

### 3\. `layout_boundary`

stack / grid のまとまりとして自然な単位

### 4\. `text_semantic_boundary`

見出し + 本文 + CTA のような意味のまとまり

### 5\. `explicit_name_boundary`

`Card`, `Button`, `Section`, `ListItem` などの命名ヒント

---

## `boundary.rs`

```
use figma_ir_core::FeatureNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryKind {
    VisualGroup,
    Repetition,
    Layout,
    TextSemantic,
    ExplicitName,
}

#[derive(Debug, Clone)]
pub struct BoundarySignal {
    pub kind: BoundaryKind,
    pub score: f32,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct BoundaryDecision {
    pub node_id: String,
    pub signals: Vec<BoundarySignal>,
    pub total_score: f32,
    pub accepted: bool,
}

pub fn detect_boundaries(nodes: &[FeatureNode]) -> Vec<BoundaryDecision> {
    nodes.iter().map(detect_boundary_for_node).collect()
}

fn detect_boundary_for_node(node: &FeatureNode) -> BoundaryDecision {
    let mut signals = Vec::new();

    if node.visual.has_fill && node.visual.has_radius {
        signals.push(BoundarySignal {
            kind: BoundaryKind::VisualGroup,
            score: 0.35,
            reason: "fill + radius".to_string(),
        });
    }

    if node.repetition.repeated_subtree_count >= 2 {
        signals.push(BoundarySignal {
            kind: BoundaryKind::Repetition,
            score: 0.30,
            reason: "repeated subtree".to_string(),
        });
    }

    if matches!(node.layout.kind, figma_ir_core::LayoutKind::Stack | figma_ir_core::LayoutKind::Grid)
        && node.child_count >= 2
    {
        signals.push(BoundarySignal {
            kind: BoundaryKind::Layout,
            score: 0.20,
            reason: "structured layout container".to_string(),
        });
    }

    if looks_explicit_named_boundary(&node.name) {
        signals.push(BoundarySignal {
            kind: BoundaryKind::ExplicitName,
            score: 0.25,
            reason: "name hint".to_string(),
        });
    }

    let total_score: f32 = signals.iter().map(|s| s.score).sum();
    let accepted = total_score >= 0.45;

    BoundaryDecision {
        node_id: node.id.0.clone(),
        signals,
        total_score,
        accepted,
    }
}

fn looks_explicit_named_boundary(name: &str) -> bool {
    let lowered = name.to_ascii_lowercase();
    lowered.contains("card")
        || lowered.contains("section")
        || lowered.contains("button")
        || lowered.contains("item")
        || lowered.contains("list")
        || lowered.contains("header")
}
```

---

# 9\. wrapper collapse

これは必須です。  
ここが弱いと、出力が wrapper だらけになります。

---

## collapse の条件

以下のようなノードは **layout-only wrapper** 候補です。

- 子が 1 つだけ
- 自身に意味的 content がない
- fill / shadow / radius などの見た目特徴が弱い
- 名前が `Wrapper`, `Container`, `Frame`, `Auto Layout` 的
- repetition boundary にもならない

---

## `collapse.rs`

```
use figma_ir_core::FeatureNode;

pub fn is_layout_only_wrapper(node: &FeatureNode) -> bool {
    if node.child_count != 1 {
        return false;
    }

    if node.visual.has_fill || node.visual.has_shadow || node.visual.has_radius {
        return false;
    }

    if node.text.is_textual {
        return false;
    }

    let lowered = node.name.to_ascii_lowercase();
    if lowered.contains("wrapper")
        || lowered.contains("container")
        || lowered.contains("frame")
        || lowered.contains("layout")
    {
        return true;
    }

    matches!(node.layout.kind, figma_ir_core::LayoutKind::Stack | figma_ir_core::LayoutKind::Absolute)
}
```

これは初版としては十分です。  
あとで「collapse しすぎ」を防ぐ抑制条件を足します。

---

# 10\. `normalize.rs`

ここで `FeatureGraph -> UiIr` の骨格を作ります。

```
use anyhow::Result;
use figma_ir_core::{
    Confidence, FeatureGraph, LayoutSpec, LayoutSpecKind, SemanticRole, Semantics, StyleRefs,
    TokenRefs, UiIr, UiNode, UiNodeKind,
};

use crate::boundary::detect_boundaries;

pub fn normalize(graph: &FeatureGraph) -> Result<UiIr> {
    let boundary_decisions = detect_boundaries(&graph.nodes);

    let root = graph
        .nodes
        .iter()
        .find(|n| n.depth == 0)
        .ok_or_else(|| anyhow::anyhow!("root node not found"))?;

    let root_boundary = boundary_decisions
        .iter()
        .find(|b| b.node_id == root.id.0)
        .cloned();

    let root_kind = if root_boundary.as_ref().map(|b| b.accepted).unwrap_or(false) {
        UiNodeKind::Screen
    } else {
        UiNodeKind::Unknown
    };

    let tree = UiNode {
        id: format!("screen.{}", sanitize_name(&root.name)),
        name: Some(root.name.clone()),
        kind: root_kind,
        source_node_ids: vec![root.id.0.clone()],
        component_candidates: vec![],
        atomic_candidates: vec![],
        confidence: Confidence(0.8),
        reusability: None,
        layout: LayoutSpec {
            kind: map_layout_kind(&root.layout.kind),
            direction: root.layout.direction.as_ref().map(map_layout_direction),
            gap: root.layout.gap,
            padding: root.layout.padding,
            align_x: None,
            align_y: None,
            columns: None,
        },
        style_refs: StyleRefs::default(),
        semantics: Semantics {
            role: SemanticRole::Page,
            accessible_name: Some(root.name.clone()),
        },
        content: Default::default(),
        children: vec![],
        diagnostics: vec![],
    };

    Ok(UiIr {
        version: "0.1.0".to_string(),
        source: graph.source.clone(),
        tree,
        tokens: TokenRefs::default(),
        diagnostics: vec![],
    })
}

fn sanitize_name(name: &str) -> String {
    name.to_ascii_lowercase()
        .replace('/', ".")
        .replace([' ', '-'], "_")
}

fn map_layout_kind(kind: &figma_ir_core::LayoutKind) -> LayoutSpecKind {
    match kind {
        figma_ir_core::LayoutKind::None => LayoutSpecKind::None,
        figma_ir_core::LayoutKind::Stack => LayoutSpecKind::Stack,
        figma_ir_core::LayoutKind::Grid => LayoutSpecKind::Grid,
        figma_ir_core::LayoutKind::Absolute => LayoutSpecKind::Absolute,
        figma_ir_core::LayoutKind::Unknown => LayoutSpecKind::Unknown,
    }
}

fn map_layout_direction(axis: &figma_ir_core::Axis) -> figma_ir_core::LayoutDirection {
    match axis {
        figma_ir_core::Axis::Horizontal => figma_ir_core::LayoutDirection::Horizontal,
        figma_ir_core::Axis::Vertical => figma_ir_core::LayoutDirection::Vertical,
    }
}
```

---

# 11\. ただし、この `normalize` はまだ弱い

このままだと root しか変換していません。  
次に必要なのは **木の再構築** です。

つまり、`FeatureGraph` 側に最低限これが要ります。

- `parent_id`
- `children`
- `depth`

今の `FeatureNode` には `parent_id` が無いので、ここは追加した方がよいです。

---

## `FeatureNode` の修正版

```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FeatureNode {
    pub id: NodeId,
    pub name: String,
    pub parent_id: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub depth: usize,
    pub child_count: usize,
    pub layout: LayoutFeatures,
    pub visual: VisualFeatures,
    pub text: TextFeatures,
    pub repetition: RepetitionFeatures,
}
```

これがないと、normalizer がまともに木を作れません。

---

# 12\. 次に入れるべき再帰変換

本来は次のような関数になります。

```
fn build_ui_node(
    current: &FeatureNode,
    graph: &FeatureGraph,
    boundaries: &[BoundaryDecision],
) -> UiNode
```

この関数が、

- wrapper を潰す
- boundary accepted な子は 1 ノードとして採用
- accepted でない子は親に吸収または再帰展開

をやります。

---

## 骨格

```
fn build_ui_node(
    current: &figma_ir_core::FeatureNode,
    graph: &figma_ir_core::FeatureGraph,
    boundaries: &[crate::boundary::BoundaryDecision],
) -> UiNode {
    let child_nodes = current
        .children
        .iter()
        .filter_map(|child_id| graph.nodes.iter().find(|n| n.id == *child_id))
        .map(|child| build_ui_node(child, graph, boundaries))
        .collect::<Vec<_>>();

    UiNode {
        id: format!("node.{}", sanitize_name(&current.name)),
        name: Some(current.name.clone()),
        kind: infer_ui_kind(current),
        source_node_ids: vec![current.id.0.clone()],
        component_candidates: vec![],
        atomic_candidates: vec![],
        confidence: Confidence(0.6),
        reusability: None,
        layout: LayoutSpec {
            kind: map_layout_kind(&current.layout.kind),
            direction: current.layout.direction.as_ref().map(map_layout_direction),
            gap: current.layout.gap,
            padding: current.layout.padding,
            align_x: None,
            align_y: None,
            columns: None,
        },
        style_refs: StyleRefs::default(),
        semantics: Semantics {
            role: infer_semantic_role(current),
            accessible_name: Some(current.name.clone()),
        },
        content: Default::default(),
        children: child_nodes,
        diagnostics: vec![],
    }
}
```

---

# 13\. `infer_ui_kind` の初版

最初は荒くてよいです。  
ただし、**Unknown 逃がし** を残してください。

```
fn infer_ui_kind(node: &figma_ir_core::FeatureNode) -> UiNodeKind {
    let name = node.name.to_ascii_lowercase();

    if node.text.is_textual && node.child_count == 0 {
        if node.text.content_length.unwrap_or(0) <= 40 {
            return UiNodeKind::Text;
        }
        return UiNodeKind::TextBlock;
    }

    if name.contains("button") {
        return UiNodeKind::Button;
    }
    if name.contains("icon") {
        return UiNodeKind::Icon;
    }
    if name.contains("image") || node.visual.has_image_fill {
        return UiNodeKind::Image;
    }
    if name.contains("card") {
        return UiNodeKind::Card;
    }
    if name.contains("list") {
        return UiNodeKind::List;
    }
    if name.contains("section") {
        return UiNodeKind::Section;
    }

    match node.layout.kind {
        figma_ir_core::LayoutKind::Grid => UiNodeKind::Grid,
        figma_ir_core::LayoutKind::Stack => UiNodeKind::Stack,
        _ => UiNodeKind::Unknown,
    }
}
```

---

# 14\. `infer_semantic_role` の初版

```
fn infer_semantic_role(node: &figma_ir_core::FeatureNode) -> SemanticRole {
    let name = node.name.to_ascii_lowercase();

    if name.contains("header") {
        return SemanticRole::Header;
    }
    if name.contains("footer") {
        return SemanticRole::Footer;
    }
    if name.contains("nav") {
        return SemanticRole::Nav;
    }
    if name.contains("button") {
        return SemanticRole::Button;
    }
    if name.contains("section") {
        return SemanticRole::Section;
    }
    if name.contains("form") {
        return SemanticRole::Form;
    }

    SemanticRole::Generic
}
```

---

# 15\. normalizer で最もやってはいけないこと

ここは明確です。

## やってはいけないこと

- いきなり atomic level を確定する
- いきなり component 名を確定する
- JSX を意識して div/button/input に落とす
- 接続層へ逆戻りして追加情報を取りにいく
- LLM へ丸投げする

`normalizer` はあくまで **構造正規化** です。  
分類や命名は後段です。

---

# 16\. ここまでを踏まえた実装順序

次の順で入れると崩れにくいです。

## 先にやる

1.  `runtime::RuntimeClient`
2.  `runtime -> raw` mapper
3.  `raw -> features` extractor
4.  `FeatureNode` に `parent_id` と `children` を追加
5.  `normalizer::detect_boundaries`
6.  `normalizer::build_ui_node`

## まだやらない

- component naming の完成版
- atomic classification の完成版
- design token 命名の完成版
- code generator

---

# 17\. 率直な見立て

あなたの計画でいちばん価値が出るのは、接続方法の奇抜さではありません。  
**boundary detection と wrapper collapse の精度**です。

ここが弱いと、どれだけ面白い接続をしても結果が使えません。

逆に言うと、ここが強ければ接続手段が多少荒くても価値は出ます。

---

次に進めるなら、以下のどちらかです。

## A

**`extractor` の実装骨格**  
`RawFrameSnapshot -> FeatureGraph` の具体コードを書く

## B

**`classifier` の実装骨格**  
`UiIrDraft -> componentCandidates / atomicCandidates` の具体コードを書く

順番としては **A の方が先**です。

## Prompt:

A

## Response:

では、**`extractor` の実装骨格**をそのまま置きます。

目的は明確です。

- 入力: `RawFrameSnapshot`
- 出力: `FeatureGraph`
- 役割: **生ノード列を、正規化や分類に使える特徴量へ変換する**

ここでは、賢いことをしすぎません。  
`extractor` は **観測と集約** に徹します。  
意味づけは `normalizer` と `classifier` に後ろ倒しします。

---

# 1\. `extractor` の責務

`extractor` でやることは次です。

## 構造特徴

- depth
- child_count
- parent / children
- sibling index
- subtree size

## レイアウト特徴

- layout kind
- direction
- gap
- padding

## 視覚特徴

- fill の有無
- stroke の有無
- radius の有無
- shadow の有無
- image fill の有無
- vector の有無

## テキスト特徴

- textual node か
- content length
- line count 推定

## 反復特徴

- subtree fingerprint
- repeated_subtree_count

この 5 系統で十分です。  
ここで `Button` だの `Card` だのは決めません。

---

# 2\. `FeatureNode` を少し拡張する

前の案から、`extractor` が実用になるように少し足します。

`crates/core/src/features.rs` を次のように更新してください。

```
use crate::diagnostic::Diagnostic;
use crate::ids::NodeId;
use crate::source::SourceRef;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LayoutKind {
    None,
    Stack,
    Grid,
    Absolute,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayoutFeatures {
    pub kind: LayoutKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<[f64; 4]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VisualFeatures {
    pub has_fill: bool,
    pub has_stroke: bool,
    pub has_radius: bool,
    pub has_shadow: bool,
    pub has_image_fill: bool,
    pub has_vector: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TextFeatures {
    pub is_textual: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count_estimate: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RepetitionFeatures {
    pub repeated_subtree_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StructureFeatures {
    pub depth: usize,
    pub child_count: usize,
    pub sibling_index: usize,
    pub subtree_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FeatureNode {
    pub id: NodeId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<NodeId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<NodeId>,

    pub structure: StructureFeatures,
    pub layout: LayoutFeatures,
    pub visual: VisualFeatures,
    pub text: TextFeatures,
    pub repetition: RepetitionFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FeatureGraph {
    pub source: SourceRef,
    pub nodes: Vec<FeatureNode>,
    pub diagnostics: Vec<Diagnostic>,
}
```

`depth` や `child_count` を `structure` に寄せたのは、後から特徴を足しやすくするためです。

---

# 3\. `extractor` crate の公開 API

`crates/extractor/src/lib.rs`

```
mod extract;
mod fingerprint;
mod graph;

pub use extract::extract_features;
```

---

# 4\. 内部で使うグラフ index

`RawFrameSnapshot` は単純な配列です。  
`extractor` 側で index を作らないと、毎回全探索になってすぐ面倒になります。

`crates/extractor/src/graph.rs`

```
use std::collections::HashMap;

use figma_ir_core::{NodeId, RawFrameSnapshot, RawNode};

pub struct RawGraph<'a> {
    pub snapshot: &'a RawFrameSnapshot,
    pub by_id: HashMap<&'a str, &'a RawNode>,
    pub children_of: HashMap<&'a str, Vec<&'a RawNode>>,
    pub root_id: &'a str,
}

impl<'a> RawGraph<'a> {
    pub fn new(snapshot: &'a RawFrameSnapshot) -> Self {
        let mut by_id = HashMap::new();
        for node in &snapshot.nodes {
            by_id.insert(node.id.0.as_str(), node);
        }

        let mut children_of: HashMap<&'a str, Vec<&'a RawNode>> = HashMap::new();
        for node in &snapshot.nodes {
            if let Some(parent) = &node.parent_id {
                children_of
                    .entry(parent.0.as_str())
                    .or_default()
                    .push(node);
            }
        }

        Self {
            snapshot,
            by_id,
            children_of,
            root_id: snapshot.root_frame_id.0.as_str(),
        }
    }

    pub fn node(&self, id: &str) -> Option<&'a RawNode> {
        self.by_id.get(id).copied()
    }

    pub fn children(&self, id: &str) -> &[&'a RawNode] {
        self.children_of
            .get(id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn parent_id(&self, node: &'a RawNode) -> Option<&'a str> {
        node.parent_id.as_ref().map(|id| id.0.as_str())
    }

    pub fn sibling_index(&self, node: &'a RawNode) -> usize {
        let Some(parent_id) = self.parent_id(node) else {
            return 0;
        };

        self.children(parent_id)
            .iter()
            .position(|n| n.id == node.id)
            .unwrap_or(0)
    }
}

pub fn node_ids(nodes: &[&RawNode]) -> Vec<NodeId> {
    nodes.iter().map(|n| n.id.clone()).collect()
}
```

---

# 5\. subtree fingerprint

反復構造の検出に使います。  
初版は雑で構いません。ただし、**子の fingerprint を再帰的に含める**のが肝です。

`crates/extractor/src/fingerprint.rs`

```
use std::collections::HashMap;

use figma_ir_core::{RawNode, RawNodeType};

use crate::graph::RawGraph;

pub fn compute_fingerprints(graph: &RawGraph<'_>) -> HashMap<String, String> {
    let mut cache = HashMap::new();
    compute_for_node(graph, graph.root_id, &mut cache);
    cache
}

fn compute_for_node(
    graph: &RawGraph<'_>,
    node_id: &str,
    cache: &mut HashMap<String, String>,
) -> String {
    if let Some(existing) = cache.get(node_id) {
        return existing.clone();
    }

    let node = graph
        .node(node_id)
        .unwrap_or_else(|| panic!("node not found for fingerprint: {node_id}"));

    let child_fingerprints = graph
        .children(node_id)
        .iter()
        .map(|child| compute_for_node(graph, child.id.0.as_str(), cache))
        .collect::<Vec<_>>();

    let kind = node_type_key(&node.node_type);
    let layout = layout_key(node);
    let visual = visual_key(node);
    let text = text_key(node);

    let fp = format!(
        "{}|{}|{}|{}|children:[{}]",
        kind,
        layout,
        visual,
        text,
        child_fingerprints.join(",")
    );

    cache.insert(node_id.to_string(), fp.clone());
    fp
}

fn node_type_key(node_type: &RawNodeType) -> &'static str {
    match node_type {
        RawNodeType::Frame => "frame",
        RawNodeType::Group => "group",
        RawNodeType::Rectangle => "rect",
        RawNodeType::Text => "text",
        RawNodeType::Vector => "vector",
        RawNodeType::Component => "component",
        RawNodeType::Instance => "instance",
        RawNodeType::Ellipse => "ellipse",
        RawNodeType::Line => "line",
        RawNodeType::Polygon => "polygon",
        RawNodeType::Star => "star",
        RawNodeType::BooleanOperation => "boolean",
        RawNodeType::Unknown => "unknown",
    }
}

fn layout_key(node: &RawNode) -> String {
    let child_count = node.children.len();
    format!("child_count={child_count}")
}

fn visual_key(node: &RawNode) -> String {
    let has_fill = node.fills.as_ref().map(|v| !v.is_empty()).unwrap_or(false);
    let has_radius = node.radius.is_some();
    format!("fill={has_fill};radius={has_radius}")
}

fn text_key(node: &RawNode) -> String {
    match &node.text {
        Some(text) => {
            let len = text.chars().count();
            format!("text_len={len}")
        }
        None => "text_len=0".to_string(),
    }
}
```

この fingerprint はまだ粗いですが、**繰り返し構造の入口**としては十分です。

---

# 6\. `extract_features` 本体

`crates/extractor/src/extract.rs`

```
use std::collections::HashMap;

use anyhow::Result;
use figma_ir_core::{
    Axis, Diagnostic, DiagnosticLevel, FeatureGraph, FeatureNode, LayoutFeatures, LayoutKind,
    RawFrameSnapshot, RawNode, RawNodeType, RepetitionFeatures, StructureFeatures, TextFeatures,
    VisualFeatures,
};

use crate::fingerprint::compute_fingerprints;
use crate::graph::{node_ids, RawGraph};

pub fn extract_features(snapshot: &RawFrameSnapshot) -> Result<FeatureGraph> {
    let graph = RawGraph::new(snapshot);

    let depths = compute_depths(&graph);
    let subtree_sizes = compute_subtree_sizes(&graph);
    let fingerprints = compute_fingerprints(&graph);
    let repeated_counts = compute_repeated_counts(&fingerprints);

    let nodes = snapshot
        .nodes
        .iter()
        .map(|node| {
            let id = node.id.0.as_str();
            let children = graph.children(id);

            FeatureNode {
                id: node.id.clone(),
                name: node.name.clone(),
                parent_id: node.parent_id.clone(),
                children: node_ids(children),
                structure: StructureFeatures {
                    depth: *depths.get(id).unwrap_or(&0),
                    child_count: children.len(),
                    sibling_index: graph.sibling_index(node),
                    subtree_size: *subtree_sizes.get(id).unwrap_or(&1),
                },
                layout: extract_layout_features(node),
                visual: extract_visual_features(node),
                text: extract_text_features(node),
                repetition: RepetitionFeatures {
                    repeated_subtree_count: fingerprints
                        .get(id)
                        .and_then(|fp| repeated_counts.get(fp))
                        .copied()
                        .unwrap_or(1)
                        .saturating_sub(1),
                    fingerprint: fingerprints.get(id).cloned(),
                },
            }
        })
        .collect::<Vec<_>>();

    Ok(FeatureGraph {
        source: snapshot.source.clone(),
        nodes,
        diagnostics: vec![Diagnostic {
            level: DiagnosticLevel::Info,
            code: "FEATURE_EXTRACTION_COMPLETED".to_string(),
            message: "feature extraction completed".to_string(),
            node_id: None,
        }],
    })
}

fn compute_depths(graph: &RawGraph<'_>) -> HashMap<String, usize> {
    let mut depths = HashMap::new();
    walk_depth(graph, graph.root_id, 0, &mut depths);
    depths
}

fn walk_depth(
    graph: &RawGraph<'_>,
    node_id: &str,
    depth: usize,
    depths: &mut HashMap<String, usize>,
) {
    depths.insert(node_id.to_string(), depth);

    for child in graph.children(node_id) {
        walk_depth(graph, child.id.0.as_str(), depth + 1, depths);
    }
}

fn compute_subtree_sizes(graph: &RawGraph<'_>) -> HashMap<String, usize> {
    let mut sizes = HashMap::new();
    compute_subtree_size(graph, graph.root_id, &mut sizes);
    sizes
}

fn compute_subtree_size(
    graph: &RawGraph<'_>,
    node_id: &str,
    sizes: &mut HashMap<String, usize>,
) -> usize {
    if let Some(existing) = sizes.get(node_id) {
        return *existing;
    }

    let mut size = 1;
    for child in graph.children(node_id) {
        size += compute_subtree_size(graph, child.id.0.as_str(), sizes);
    }

    sizes.insert(node_id.to_string(), size);
    size
}

fn compute_repeated_counts(fingerprints: &HashMap<String, String>) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for fp in fingerprints.values() {
        *counts.entry(fp.clone()).or_insert(0) += 1;
    }
    counts
}

fn extract_layout_features(node: &RawNode) -> LayoutFeatures {
    let (kind, direction) = infer_layout(node);

    LayoutFeatures {
        kind,
        direction,
        gap: None,
        padding: None,
    }
}

fn infer_layout(node: &RawNode) -> (LayoutKind, Option<Axis>) {
    if node.children.is_empty() {
        return (LayoutKind::None, None);
    }

    let name = node.name.to_ascii_lowercase();

    if name.contains("grid") {
        return (LayoutKind::Grid, None);
    }

    if matches!(node.node_type, RawNodeType::Frame | RawNodeType::Group | RawNodeType::Component | RawNodeType::Instance) {
        return (LayoutKind::Stack, infer_stack_direction_by_name(&name));
    }

    (LayoutKind::Unknown, None)
}

fn infer_stack_direction_by_name(name: &str) -> Option<Axis> {
    if name.contains("row") || name.contains("horizontal") || name.contains("hstack") {
        return Some(Axis::Horizontal);
    }
    if name.contains("column") || name.contains("vertical") || name.contains("vstack") {
        return Some(Axis::Vertical);
    }
    None
}

fn extract_visual_features(node: &RawNode) -> VisualFeatures {
    let has_fill = node.fills.as_ref().map(|v| !v.is_empty()).unwrap_or(false);

    VisualFeatures {
        has_fill,
        has_stroke: false,
        has_radius: node.radius.is_some(),
        has_shadow: false,
        has_image_fill: false,
        has_vector: matches!(node.node_type, RawNodeType::Vector),
    }
}

fn extract_text_features(node: &RawNode) -> TextFeatures {
    let Some(text) = &node.text else {
        return TextFeatures {
            is_textual: false,
            content_length: None,
            line_count_estimate: None,
        };
    };

    let content_length = text.chars().count();
    let line_count_estimate = text.lines().count().max(1);

    TextFeatures {
        is_textual: true,
        content_length: Some(content_length),
        line_count_estimate: Some(line_count_estimate),
    }
}
```

---

# 7\. この extractor の性格

この実装は意図的に粗いです。  
しかし、粗いこと自体は問題ではありません。

大事なのは、**粗さが構造的であること**です。

つまり、

- depth は確定的に取れる
- subtree_size は確定的に取れる
- fingerprint は同じ入力なら同じになる
- textual 判定は確定的
- visual 基本属性も確定的

という性格を保つことです。

この層で「賢く見せる」必要はありません。

---

# 8\. いまの `RawNode` では弱い箇所

ここから率直に言います。

前に置いた `RawNode` だけだと、layout 抽出は弱いです。  
理由は、**Auto Layout の情報をまだ持っていない**からです。

もし runtime 側で取れるなら、`RawNode` に以下を追加した方がよいです。

```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RawLayoutMode {
    None,
    Horizontal,
    Vertical,
    Grid,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawAutoLayout {
    pub mode: RawLayoutMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<[f64; 4]>,
}
```

そして `RawNode` に足します。

```
#[serde(skip_serializing_if = "Option::is_none")]
pub auto_layout: Option<RawAutoLayout>,
```

こうすると `extract_layout_features` がまともになります。

---

# 9\. その場合の layout 抽出改善版

```
fn extract_layout_features(node: &RawNode) -> LayoutFeatures {
    if let Some(auto) = &node.auto_layout {
        let (kind, direction) = match auto.mode {
            figma_ir_core::RawLayoutMode::None => (LayoutKind::None, None),
            figma_ir_core::RawLayoutMode::Horizontal => (LayoutKind::Stack, Some(Axis::Horizontal)),
            figma_ir_core::RawLayoutMode::Vertical => (LayoutKind::Stack, Some(Axis::Vertical)),
            figma_ir_core::RawLayoutMode::Grid => (LayoutKind::Grid, None),
            figma_ir_core::RawLayoutMode::Unknown => (LayoutKind::Unknown, None),
        };

        return LayoutFeatures {
            kind,
            direction,
            gap: auto.item_spacing,
            padding: auto.padding,
        };
    }

    let (kind, direction) = infer_layout(node);

    LayoutFeatures {
        kind,
        direction,
        gap: None,
        padding: None,
    }
}
```

この差は大きいです。  
**可能なら runtime で Auto Layout 情報は必ず取った方がよい**です。

---

# 10\. CLI との接続

`normalize` コマンドの前段で `extractor` を使うなら、CLI からはこうなります。

## `features.rs` の修正版イメージ

```
use anyhow::Result;
use figma_ir_core::FrameId;
use figma_ir_extractor::extract_features;
use figma_ir_runtime::{map_runtime_snapshot_to_raw, RuntimeClient, RuntimeFrameRequest};
use figma_ir_store::write_json_file;

use crate::args::{FeaturesCommand, FrameFeaturesArgs};

pub async fn run(cmd: FeaturesCommand) -> Result<()> {
    match cmd {
        FeaturesCommand::Frame(args) => run_frame(args).await,
    }
}

async fn run_frame(args: FrameFeaturesArgs) -> Result<()> {
    let client = figma_ir_runtime::PatchedCdpRuntimeClient::new();

    let session = client.attach().await?;
    let runtime_snapshot = client
        .snapshot_frame(RuntimeFrameRequest {
            frame_id: args.id.clone(),
        })
        .await?;

    let raw = map_runtime_snapshot_to_raw(&session, runtime_snapshot);
    let features = extract_features(&raw)?;

    write_json_file(&args.out, &features, args.pretty).await?;
    println!("wrote {}", args.out);
    Ok(())
}
```

もちろん今は `PatchedCdpRuntimeClient` をまだ実装していないので、その骨格が先です。

---

# 11\. テスト方針

`extractor` はテストしやすい層です。  
むしろ、ここは最初からテストした方がよいです。

最低限ほしいのは次です。

## テスト 1: depth 計算

- root が 0
- 子が 1
- 孫が 2

## テスト 2: subtree_size

- 葉が 1
- 子 2 個持つ親が 3 など

## テスト 3: text features

- 改行ありテキストで `line_count_estimate` が合う

## テスト 4: repetition

- 同型 subtree を 2 回置いたら `repeated_subtree_count == 1`

## テスト 5: visual features

- fills ありで `has_fill == true`
- radius ありで `has_radius == true`

---

## テスト例

`crates/extractor/tests/extract_tests.rs`

```
use figma_ir_core::{
    FrameId, RawEdge, RawFrameSnapshot, RawNode, RawNodeType, SessionId, SessionInfo, SourceRef,
    TransportKind,
};
use figma_ir_extractor::extract_features;

fn sample_snapshot() -> RawFrameSnapshot {
    RawFrameSnapshot {
        source: SourceRef {
            file_key: None,
            frame_id: FrameId("1:1".to_string()),
            frame_name: "Root".to_string(),
            session: SessionInfo {
                id: SessionId("session-1".to_string()),
                transport: TransportKind::DesktopPatchedCdp,
                captured_at: chrono::Utc::now(),
                figma_version: None,
                platform: None,
            },
        },
        root_frame_id: FrameId("1:1".to_string()),
        nodes: vec![
            RawNode {
                id: figma_ir_core::NodeId("1:1".to_string()),
                name: "Root".to_string(),
                node_type: RawNodeType::Frame,
                parent_id: None,
                children: vec![
                    figma_ir_core::NodeId("1:2".to_string()),
                    figma_ir_core::NodeId("1:3".to_string()),
                ],
                bounds: None,
                fills: None,
                radius: None,
                text: None,
                text_style: None,
                visible: true,
            },
            RawNode {
                id: figma_ir_core::NodeId("1:2".to_string()),
                name: "Title".to_string(),
                node_type: RawNodeType::Text,
                parent_id: Some(figma_ir_core::NodeId("1:1".to_string())),
                children: vec![],
                bounds: None,
                fills: None,
                radius: None,
                text: Some("Hello\nWorld".to_string()),
                text_style: None,
                visible: true,
            },
            RawNode {
                id: figma_ir_core::NodeId("1:3".to_string()),
                name: "Card".to_string(),
                node_type: RawNodeType::Rectangle,
                parent_id: Some(figma_ir_core::NodeId("1:1".to_string())),
                children: vec![],
                bounds: None,
                fills: Some(vec![]),
                radius: Some(8.0),
                text: None,
                text_style: None,
                visible: true,
            },
        ],
        edges: vec![
            RawEdge {
                parent: figma_ir_core::NodeId("1:1".to_string()),
                child: figma_ir_core::NodeId("1:2".to_string()),
                index: 0,
            },
            RawEdge {
                parent: figma_ir_core::NodeId("1:1".to_string()),
                child: figma_ir_core::NodeId("1:3".to_string()),
                index: 1,
            },
        ],
        diagnostics: vec![],
    }
}

#[test]
fn extracts_basic_features() {
    let snapshot = sample_snapshot();
    let graph = extract_features(&snapshot).unwrap();

    let root = graph.nodes.iter().find(|n| n.id.0 == "1:1").unwrap();
    assert_eq!(root.structure.depth, 0);
    assert_eq!(root.structure.child_count, 2);
    assert_eq!(root.structure.subtree_size, 3);

    let title = graph.nodes.iter().find(|n| n.id.0 == "1:2").unwrap();
    assert!(title.text.is_textual);
    assert_eq!(title.text.line_count_estimate, Some(2));

    let card = graph.nodes.iter().find(|n| n.id.0 == "1:3").unwrap();
    assert!(card.visual.has_radius);
}
```

---

# 12\. この extractor の次に何をするか

この実装が入ると、ようやく `normalizer` をまともに書けます。

次の段階では、`normalizer` 側で以下をやることになります。

- `FeatureNode.children` を使って木を再構築
- `structure.subtree_size` を見て小さすぎる wrapper を潰す
- `repetition.repeated_subtree_count` を見て list item 候補を出す
- `visual.has_fill + has_radius + layout.kind == stack` で card 候補を出す

つまり、**extractor が normalizer の燃料**になります。

---

# 13\. 率直な結論

この層でいちばん大事なのは、精度ではなく **安定性** です。

- 同じ入力から同じ特徴量が出る
- どこで何を見ているか分かる
- 後段が必要とする特徴を足しやすい

この 3 つがあれば十分です。

逆に、この層で賢く見せようとして

- Button 判定
- Card 判定
- Atomic 判定  
  まで始めると、責務が崩れます。

---

次は、この `FeatureGraph` を前提にした  
**`normalizer` の再帰実装版 `build_ui_node`** を書くのが順番として自然です。

## Prompt:

お願いします

## Response:

では、**`FeatureGraph` を前提にした `normalizer` の再帰実装版** を詰めます。

ここでやることは明確です。

- `FeatureGraph` から木を再構築する
- `layout-only wrapper` を潰す
- boundary 候補を見て、どこを 1 UI ノードとして立てるか決める
- `UiNode` を再帰的に組み立てる

この段階では、まだ **component 名の完成版** も **atomic design の確定** もやりません。  
やるのはあくまで **構造の正規化** です。

---

# 1\. `normalizer` の役割をもう一度固定する

`normalizer` の責務は次の 4 つです。

1.  **木の再構築**  
    `FeatureNode.parent_id / children` を使って親子を復元する
2.  **wrapper collapse**  
    実装上ノイズにしかならないノードを吸収する
3.  **boundary の採択**  
    card / section / list item になりそうなまとまりを拾う
4.  **`UiNode` 生成**  
    `UiIr` の木へ変換する

ここでやってはいけないのは、  
**「このノードは絶対に Button atom である」と断定すること**です。

---

# 2\. `normalizer` の公開 API

`crates/normalizer/src/lib.rs`

```
mod boundary;
mod collapse;
mod context;
mod normalize;

pub use normalize::normalize;
```

---

# 3\. `NormalizeContext`

毎回全探索していると、後で必ず嫌になります。  
親子と boundary 判定を引ける index を置きます。

`crates/normalizer/src/context.rs`

```
use std::collections::HashMap;

use figma_ir_core::{FeatureGraph, FeatureNode};

use crate::boundary::{detect_boundaries, BoundaryDecision};

pub struct NormalizeContext<'a> {
    pub graph: &'a FeatureGraph,
    pub by_id: HashMap<&'a str, &'a FeatureNode>,
    pub boundaries: HashMap<&'a str, BoundaryDecision>,
    pub root_id: &'a str,
}

impl<'a> NormalizeContext<'a> {
    pub fn new(graph: &'a FeatureGraph) -> anyhow::Result<Self> {
        let mut by_id = HashMap::new();
        for node in &graph.nodes {
            by_id.insert(node.id.0.as_str(), node);
        }

        let boundary_list = detect_boundaries(&graph.nodes);
        let mut boundaries = HashMap::new();
        for decision in boundary_list {
            boundaries.insert(decision.node_id.as_str(), decision);
        }

        let root = graph
            .nodes
            .iter()
            .find(|n| n.parent_id.is_none())
            .ok_or_else(|| anyhow::anyhow!("root node not found"))?;

        Ok(Self {
            graph,
            by_id,
            boundaries,
            root_id: root.id.0.as_str(),
        })
    }

    pub fn node(&self, id: &str) -> Option<&'a FeatureNode> {
        self.by_id.get(id).copied()
    }

    pub fn children(&self, node: &'a FeatureNode) -> Vec<&'a FeatureNode> {
        node.children
            .iter()
            .filter_map(|child_id| self.node(child_id.0.as_str()))
            .collect()
    }

    pub fn boundary(&self, node_id: &str) -> Option<&BoundaryDecision> {
        self.boundaries.get(node_id)
    }
}
```

---

# 4\. boundary 判定を少しだけ現実寄りにする

前に置いた `boundary.rs` を、そのままでも動きますが、`FeatureNode.structure` に寄せた版に揃えます。

`crates/normalizer/src/boundary.rs`

```
use figma_ir_core::{FeatureNode, LayoutKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryKind {
    VisualGroup,
    Repetition,
    Layout,
    TextSemantic,
    ExplicitName,
}

#[derive(Debug, Clone)]
pub struct BoundarySignal {
    pub kind: BoundaryKind,
    pub score: f32,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct BoundaryDecision {
    pub node_id: String,
    pub signals: Vec<BoundarySignal>,
    pub total_score: f32,
    pub accepted: bool,
}

pub fn detect_boundaries(nodes: &[FeatureNode]) -> Vec<BoundaryDecision> {
    nodes.iter().map(detect_boundary_for_node).collect()
}

fn detect_boundary_for_node(node: &FeatureNode) -> BoundaryDecision {
    let mut signals = Vec::new();

    if node.visual.has_fill && node.visual.has_radius {
        signals.push(BoundarySignal {
            kind: BoundaryKind::VisualGroup,
            score: 0.35,
            reason: "fill + radius".to_string(),
        });
    }

    if node.repetition.repeated_subtree_count >= 1 {
        signals.push(BoundarySignal {
            kind: BoundaryKind::Repetition,
            score: 0.30,
            reason: "repeated subtree".to_string(),
        });
    }

    if matches!(node.layout.kind, LayoutKind::Stack | LayoutKind::Grid)
        && node.structure.child_count >= 2
    {
        signals.push(BoundarySignal {
            kind: BoundaryKind::Layout,
            score: 0.20,
            reason: "structured layout container".to_string(),
        });
    }

    if looks_explicit_named_boundary(&node.name) {
        signals.push(BoundarySignal {
            kind: BoundaryKind::ExplicitName,
            score: 0.25,
            reason: "name hint".to_string(),
        });
    }

    if looks_text_semantic_boundary(node) {
        signals.push(BoundarySignal {
            kind: BoundaryKind::TextSemantic,
            score: 0.15,
            reason: "textual grouping".to_string(),
        });
    }

    let total_score: f32 = signals.iter().map(|s| s.score).sum();
    let accepted = total_score >= 0.45;

    BoundaryDecision {
        node_id: node.id.0.clone(),
        signals,
        total_score,
        accepted,
    }
}

fn looks_explicit_named_boundary(name: &str) -> bool {
    let lowered = name.to_ascii_lowercase();
    lowered.contains("card")
        || lowered.contains("section")
        || lowered.contains("button")
        || lowered.contains("item")
        || lowered.contains("list")
        || lowered.contains("header")
        || lowered.contains("footer")
        || lowered.contains("form")
}

fn looks_text_semantic_boundary(node: &FeatureNode) -> bool {
    node.structure.child_count >= 2 && matches!(node.layout.kind, LayoutKind::Stack)
}
```

---

# 5\. wrapper collapse を少し堅くする

前の `collapse.rs` を `FeatureNode.structure` に合わせます。

`crates/normalizer/src/collapse.rs`

```
use figma_ir_core::{FeatureNode, LayoutKind};

pub fn is_layout_only_wrapper(node: &FeatureNode) -> bool {
    if node.structure.child_count != 1 {
        return false;
    }

    if node.visual.has_fill || node.visual.has_shadow || node.visual.has_radius {
        return false;
    }

    if node.text.is_textual {
        return false;
    }

    if node.repetition.repeated_subtree_count >= 1 {
        return false;
    }

    let lowered = node.name.to_ascii_lowercase();
    if lowered.contains("wrapper")
        || lowered.contains("container")
        || lowered.contains("frame")
        || lowered.contains("layout")
        || lowered.contains("group")
    {
        return true;
    }

    matches!(node.layout.kind, LayoutKind::Stack | LayoutKind::Absolute)
}
```

ここで大事なのは、**反復候補を潰さない**ことです。  
list item 候補まで collapse すると、あとで困ります。

---

# 6\. `normalize.rs` の全体像

ここが中核です。

やることは次の 3 段階です。

1.  root を見つける
2.  `build_ui_node` を再帰的に呼ぶ
3.  root 以下の構造を `UiIr.tree` に載せる

---

## `crates/normalizer/src/normalize.rs`

```
use anyhow::Result;
use figma_ir_core::{
    Confidence, Diagnostic, DiagnosticLevel, FeatureGraph, LayoutDirection, LayoutSpec,
    LayoutSpecKind, SemanticRole, Semantics, StyleRefs, TokenRefs, UiIr, UiNode, UiNodeKind,
};

use crate::collapse::is_layout_only_wrapper;
use crate::context::NormalizeContext;

pub fn normalize(graph: &FeatureGraph) -> Result<UiIr> {
    let ctx = NormalizeContext::new(graph)?;
    let root = ctx
        .node(ctx.root_id)
        .ok_or_else(|| anyhow::anyhow!("root node not found"))?;

    let tree = build_ui_node(root, &ctx, true);

    Ok(UiIr {
        version: "0.1.0".to_string(),
        source: graph.source.clone(),
        tree,
        tokens: TokenRefs::default(),
        diagnostics: vec![Diagnostic {
            level: DiagnosticLevel::Info,
            code: "NORMALIZATION_COMPLETED".to_string(),
            message: "normalization completed".to_string(),
            node_id: None,
        }],
    })
}
```

---

# 7\. `build_ui_node` の再帰実装

ここでやりたいことは単純です。

- 現在ノードを 1 つの UI ノードとして立てる
- 子を見て、wrapper なら吸収
- そうでなければ再帰して子にする

このとき、**boundary accepted なノードは 1 単位として尊重**します。

```
fn build_ui_node(
    current: &figma_ir_core::FeatureNode,
    ctx: &NormalizeContext<'_>,
    is_root: bool,
) -> UiNode {
    let current_boundary = ctx.boundary(current.id.0.as_str());

    let children = ctx.children(current);
    let normalized_children = normalize_children(&children, ctx);

    UiNode {
        id: make_ui_node_id(current, is_root),
        name: Some(current.name.clone()),
        kind: infer_ui_kind(current, is_root, current_boundary.map(|b| b.accepted).unwrap_or(false)),
        source_node_ids: vec![current.id.0.clone()],
        component_candidates: vec![],
        atomic_candidates: vec![],
        confidence: Confidence(infer_confidence(current, current_boundary)),
        reusability: None,
        layout: LayoutSpec {
            kind: map_layout_kind(&current.layout.kind),
            direction: current.layout.direction.as_ref().map(map_layout_direction),
            gap: current.layout.gap,
            padding: current.layout.padding,
            align_x: None,
            align_y: None,
            columns: None,
        },
        style_refs: StyleRefs::default(),
        semantics: Semantics {
            role: infer_semantic_role(current, is_root),
            accessible_name: Some(current.name.clone()),
        },
        content: infer_content(current),
        children: normalized_children,
        diagnostics: infer_node_diagnostics(current, current_boundary),
    }
}
```

---

# 8\. 子ノードの正規化

ここが wrapper collapse の入口です。

```
fn normalize_children(
    children: &[&figma_ir_core::FeatureNode],
    ctx: &NormalizeContext<'_>,
) -> Vec<UiNode> {
    let mut out = Vec::new();

    for child in children {
        if is_layout_only_wrapper(child) {
            let grand_children = ctx.children(child);
            let collapsed = normalize_children(&grand_children, ctx);
            out.extend(collapsed);
            continue;
        }

        out.push(build_ui_node(child, ctx, false));
    }

    out
}
```

これはまだ単純ですが、最初はこれで十分です。

---

# 9\. `UiNode.id` の作り方

`source_node_ids` があるので、`id` は人間に読める形にして構いません。  
ただし、重複しない工夫は要ります。

```
fn make_ui_node_id(current: &figma_ir_core::FeatureNode, is_root: bool) -> String {
    let base = sanitize_name(&current.name);

    if is_root {
        return format!("screen.{base}");
    }

    format!("node.{base}.{}", current.id.0.replace(':', "_"))
}

fn sanitize_name(name: &str) -> String {
    name.to_ascii_lowercase()
        .replace('/', ".")
        .replace([' ', '-'], "_")
}
```

---

# 10\. `kind` 推定

ここは断定しすぎないことが大事です。  
`Unknown` を残します。

```
fn infer_ui_kind(
    node: &figma_ir_core::FeatureNode,
    is_root: bool,
    boundary_accepted: bool,
) -> UiNodeKind {
    if is_root {
        return UiNodeKind::Screen;
    }

    let name = node.name.to_ascii_lowercase();

    if node.text.is_textual && node.structure.child_count == 0 {
        let len = node.text.content_length.unwrap_or(0);
        if len <= 60 {
            if looks_heading_text(&name, len) {
                return UiNodeKind::Heading;
            }
            return UiNodeKind::Text;
        }
        return UiNodeKind::TextBlock;
    }

    if name.contains("button") {
        return UiNodeKind::Button;
    }
    if name.contains("input") || name.contains("textfield") || name.contains("text field") {
        return UiNodeKind::Input;
    }
    if name.contains("checkbox") {
        return UiNodeKind::Checkbox;
    }
    if name.contains("radio") {
        return UiNodeKind::Radio;
    }
    if name.contains("switch") || name.contains("toggle") {
        return UiNodeKind::Switch;
    }
    if name.contains("icon") {
        return UiNodeKind::Icon;
    }
    if name.contains("image") || node.visual.has_image_fill {
        return UiNodeKind::Image;
    }
    if name.contains("avatar") {
        return UiNodeKind::Avatar;
    }
    if name.contains("badge") {
        return UiNodeKind::Badge;
    }
    if name.contains("list") {
        return UiNodeKind::List;
    }
    if name.contains("item") && node.repetition.repeated_subtree_count >= 1 {
        return UiNodeKind::ListItem;
    }
    if name.contains("nav") {
        return UiNodeKind::Navigation;
    }
    if name.contains("form") {
        return UiNodeKind::Form;
    }
    if name.contains("section") {
        return UiNodeKind::Section;
    }
    if name.contains("card") {
        return UiNodeKind::Card;
    }

    if boundary_accepted {
        if node.repetition.repeated_subtree_count >= 1 {
            return UiNodeKind::ListItem;
        }

        if node.visual.has_fill && node.visual.has_radius {
            return UiNodeKind::Card;
        }

        if node.structure.child_count >= 2 {
            return UiNodeKind::Section;
        }
    }

    match node.layout.kind {
        figma_ir_core::LayoutKind::Grid => UiNodeKind::Grid,
        figma_ir_core::LayoutKind::Stack => UiNodeKind::Stack,
        _ => UiNodeKind::Unknown,
    }
}

fn looks_heading_text(name: &str, len: usize) -> bool {
    name.contains("title")
        || name.contains("heading")
        || name.contains("headline")
        || len <= 32
}
```

---

# 11\. `semantic role` 推定

これは `kind` より少し抽象度を下げます。

```
fn infer_semantic_role(
    node: &figma_ir_core::FeatureNode,
    is_root: bool,
) -> SemanticRole {
    if is_root {
        return SemanticRole::Page;
    }

    let name = node.name.to_ascii_lowercase();

    if name.contains("header") {
        return SemanticRole::Header;
    }
    if name.contains("footer") {
        return SemanticRole::Footer;
    }
    if name.contains("nav") {
        return SemanticRole::Nav;
    }
    if name.contains("section") {
        return SemanticRole::Section;
    }
    if name.contains("button") {
        return SemanticRole::Button;
    }
    if name.contains("form") {
        return SemanticRole::Form;
    }
    if name.contains("label") {
        return SemanticRole::Label;
    }
    if name.contains("input") {
        return SemanticRole::Input;
    }
    if name.contains("article") || name.contains("card") {
        return SemanticRole::Article;
    }
    if name.contains("list") {
        return SemanticRole::List;
    }
    if name.contains("item") {
        return SemanticRole::ListItem;
    }

    if node.text.is_textual && node.structure.child_count == 0 {
        let len = node.text.content_length.unwrap_or(0);
        if len <= 32 {
            return SemanticRole::Heading;
        }
    }

    SemanticRole::Generic
}
```

---

# 12\. content 推定

ここでは Text だけ拾えば十分です。

```
fn infer_content(node: &figma_ir_core::FeatureNode) -> figma_ir_core::NodeContent {
    let mut content = figma_ir_core::NodeContent::default();

    if node.text.is_textual {
        content.text = Some(node.name.clone());
    }

    content
}
```

ただし、これは少し雑です。  
本当は `FeatureNode` に実テキストを残した方がよいです。

今の `FeatureNode` には `text.content_length` しかなく、本文そのものがありません。  
したがって、**UI IR に本文を載せたいなら `FeatureNode` に `raw_text: Option<String>` を足した方がよい**です。

### 追加案

`features.rs` の `TextFeatures` をこう変える方がよいです。

```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TextFeatures {
    pub is_textual: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count_estimate: Option<usize>,
}
```

そのうえで `extract_text_features` を修正します。

```
fn extract_text_features(node: &RawNode) -> TextFeatures {
    let Some(text) = &node.text else {
        return TextFeatures {
            is_textual: false,
            raw_text: None,
            content_length: None,
            line_count_estimate: None,
        };
    };

    let content_length = text.chars().count();
    let line_count_estimate = text.lines().count().max(1);

    TextFeatures {
        is_textual: true,
        raw_text: Some(text.clone()),
        content_length: Some(content_length),
        line_count_estimate: Some(line_count_estimate),
    }
}
```

その方が `infer_content` がまともになります。

```
fn infer_content(node: &figma_ir_core::FeatureNode) -> figma_ir_core::NodeContent {
    let mut content = figma_ir_core::NodeContent::default();

    if node.text.is_textual {
        content.text = node.text.raw_text.clone();
    }

    content
}
```

---

# 13\. confidence 推定

この値は、後の `classifier` の土台になります。  
初版は単純合成でよいです。

```
fn infer_confidence(
    node: &figma_ir_core::FeatureNode,
    boundary: Option<&crate::boundary::BoundaryDecision>,
) -> f32 {
    let mut score = 0.35_f32;

    if node.text.is_textual {
        score += 0.20;
    }

    if node.visual.has_fill && node.visual.has_radius {
        score += 0.15;
    }

    if matches!(node.layout.kind, figma_ir_core::LayoutKind::Stack | figma_ir_core::LayoutKind::Grid) {
        score += 0.10;
    }

    if let Some(boundary) = boundary {
        score += (boundary.total_score * 0.3).min(0.20);
    }

    score.clamp(0.0, 1.0)
}
```

---

# 14\. diagnostics の付与

ここはかなり効きます。  
出力を人間が信用できるかどうかは、diagnostics の質に左右されます。

```
fn infer_node_diagnostics(
    node: &figma_ir_core::FeatureNode,
    boundary: Option<&crate::boundary::BoundaryDecision>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if let Some(boundary) = boundary {
        if boundary.accepted && boundary.total_score < 0.60 {
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Warning,
                code: "LOW_CONFIDENCE_BOUNDARY".to_string(),
                message: format!(
                    "boundary accepted with marginal score {:.2}",
                    boundary.total_score
                ),
                node_id: Some(node.id.0.clone()),
            });
        }
    }

    if matches!(node.layout.kind, figma_ir_core::LayoutKind::Unknown) && node.structure.child_count > 0 {
        diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Info,
            code: "UNKNOWN_LAYOUT_KIND".to_string(),
            message: "layout kind could not be inferred confidently".to_string(),
            node_id: Some(node.id.0.clone()),
        });
    }

    if node.repetition.repeated_subtree_count >= 3 {
        diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Info,
            code: "HIGH_REPETITION_PATTERN".to_string(),
            message: "node participates in a repeated subtree pattern".to_string(),
            node_id: Some(node.id.0.clone()),
        });
    }

    diagnostics
}
```

---

# 15\. ここまでの `normalize.rs` 全体

まとめると、最低限こうなります。

```
use anyhow::Result;
use figma_ir_core::{
    Confidence, Diagnostic, DiagnosticLevel, FeatureGraph, LayoutDirection, LayoutSpec,
    LayoutSpecKind, SemanticRole, Semantics, StyleRefs, TokenRefs, UiIr, UiNode, UiNodeKind,
};

use crate::collapse::is_layout_only_wrapper;
use crate::context::NormalizeContext;

pub fn normalize(graph: &FeatureGraph) -> Result<UiIr> {
    let ctx = NormalizeContext::new(graph)?;
    let root = ctx
        .node(ctx.root_id)
        .ok_or_else(|| anyhow::anyhow!("root node not found"))?;

    let tree = build_ui_node(root, &ctx, true);

    Ok(UiIr {
        version: "0.1.0".to_string(),
        source: graph.source.clone(),
        tree,
        tokens: TokenRefs::default(),
        diagnostics: vec![Diagnostic {
            level: DiagnosticLevel::Info,
            code: "NORMALIZATION_COMPLETED".to_string(),
            message: "normalization completed".to_string(),
            node_id: None,
        }],
    })
}

fn build_ui_node(
    current: &figma_ir_core::FeatureNode,
    ctx: &NormalizeContext<'_>,
    is_root: bool,
) -> UiNode {
    let current_boundary = ctx.boundary(current.id.0.as_str());
    let children = ctx.children(current);
    let normalized_children = normalize_children(&children, ctx);

    UiNode {
        id: make_ui_node_id(current, is_root),
        name: Some(current.name.clone()),
        kind: infer_ui_kind(current, is_root, current_boundary.map(|b| b.accepted).unwrap_or(false)),
        source_node_ids: vec![current.id.0.clone()],
        component_candidates: vec![],
        atomic_candidates: vec![],
        confidence: Confidence(infer_confidence(current, current_boundary)),
        reusability: None,
        layout: LayoutSpec {
            kind: map_layout_kind(&current.layout.kind),
            direction: current.layout.direction.as_ref().map(map_layout_direction),
            gap: current.layout.gap,
            padding: current.layout.padding,
            align_x: None,
            align_y: None,
            columns: None,
        },
        style_refs: StyleRefs::default(),
        semantics: Semantics {
            role: infer_semantic_role(current, is_root),
            accessible_name: Some(current.name.clone()),
        },
        content: infer_content(current),
        children: normalized_children,
        diagnostics: infer_node_diagnostics(current, current_boundary),
    }
}

fn normalize_children(
    children: &[&figma_ir_core::FeatureNode],
    ctx: &NormalizeContext<'_>,
) -> Vec<UiNode> {
    let mut out = Vec::new();

    for child in children {
        if is_layout_only_wrapper(child) {
            let grand_children = ctx.children(child);
            let collapsed = normalize_children(&grand_children, ctx);
            out.extend(collapsed);
            continue;
        }

        out.push(build_ui_node(child, ctx, false));
    }

    out
}

fn make_ui_node_id(current: &figma_ir_core::FeatureNode, is_root: bool) -> String {
    let base = sanitize_name(&current.name);
    if is_root {
        return format!("screen.{base}");
    }
    format!("node.{base}.{}", current.id.0.replace(':', "_"))
}

fn sanitize_name(name: &str) -> String {
    name.to_ascii_lowercase()
        .replace('/', ".")
        .replace([' ', '-'], "_")
}

fn infer_ui_kind(
    node: &figma_ir_core::FeatureNode,
    is_root: bool,
    boundary_accepted: bool,
) -> UiNodeKind {
    if is_root {
        return UiNodeKind::Screen;
    }

    let name = node.name.to_ascii_lowercase();

    if node.text.is_textual && node.structure.child_count == 0 {
        let len = node.text.content_length.unwrap_or(0);
        if len <= 60 {
            if looks_heading_text(&name, len) {
                return UiNodeKind::Heading;
            }
            return UiNodeKind::Text;
        }
        return UiNodeKind::TextBlock;
    }

    if name.contains("button") {
        return UiNodeKind::Button;
    }
    if name.contains("input") || name.contains("textfield") || name.contains("text field") {
        return UiNodeKind::Input;
    }
    if name.contains("checkbox") {
        return UiNodeKind::Checkbox;
    }
    if name.contains("radio") {
        return UiNodeKind::Radio;
    }
    if name.contains("switch") || name.contains("toggle") {
        return UiNodeKind::Switch;
    }
    if name.contains("icon") {
        return UiNodeKind::Icon;
    }
    if name.contains("image") || node.visual.has_image_fill {
        return UiNodeKind::Image;
    }
    if name.contains("avatar") {
        return UiNodeKind::Avatar;
    }
    if name.contains("badge") {
        return UiNodeKind::Badge;
    }
    if name.contains("list") {
        return UiNodeKind::List;
    }
    if name.contains("item") && node.repetition.repeated_subtree_count >= 1 {
        return UiNodeKind::ListItem;
    }
    if name.contains("nav") {
        return UiNodeKind::Navigation;
    }
    if name.contains("form") {
        return UiNodeKind::Form;
    }
    if name.contains("section") {
        return UiNodeKind::Section;
    }
    if name.contains("card") {
        return UiNodeKind::Card;
    }

    if boundary_accepted {
        if node.repetition.repeated_subtree_count >= 1 {
            return UiNodeKind::ListItem;
        }
        if node.visual.has_fill && node.visual.has_radius {
            return UiNodeKind::Card;
        }
        if node.structure.child_count >= 2 {
            return UiNodeKind::Section;
        }
    }

    match node.layout.kind {
        figma_ir_core::LayoutKind::Grid => UiNodeKind::Grid,
        figma_ir_core::LayoutKind::Stack => UiNodeKind::Stack,
        _ => UiNodeKind::Unknown,
    }
}

fn looks_heading_text(name: &str, len: usize) -> bool {
    name.contains("title")
        || name.contains("heading")
        || name.contains("headline")
        || len <= 32
}

fn infer_semantic_role(
    node: &figma_ir_core::FeatureNode,
    is_root: bool,
) -> SemanticRole {
    if is_root {
        return SemanticRole::Page;
    }

    let name = node.name.to_ascii_lowercase();

    if name.contains("header") {
        return SemanticRole::Header;
    }
    if name.contains("footer") {
        return SemanticRole::Footer;
    }
    if name.contains("nav") {
        return SemanticRole::Nav;
    }
    if name.contains("section") {
        return SemanticRole::Section;
    }
    if name.contains("button") {
        return SemanticRole::Button;
    }
    if name.contains("form") {
        return SemanticRole::Form;
    }
    if name.contains("label") {
        return SemanticRole::Label;
    }
    if name.contains("input") {
        return SemanticRole::Input;
    }
    if name.contains("article") || name.contains("card") {
        return SemanticRole::Article;
    }
    if name.contains("list") {
        return SemanticRole::List;
    }
    if name.contains("item") {
        return SemanticRole::ListItem;
    }

    if node.text.is_textual && node.structure.child_count == 0 {
        let len = node.text.content_length.unwrap_or(0);
        if len <= 32 {
            return SemanticRole::Heading;
        }
    }

    SemanticRole::Generic
}

fn infer_content(node: &figma_ir_core::FeatureNode) -> figma_ir_core::NodeContent {
    let mut content = figma_ir_core::NodeContent::default();

    if node.text.is_textual {
        content.text = node.text.raw_text.clone();
    }

    content
}

fn infer_confidence(
    node: &figma_ir_core::FeatureNode,
    boundary: Option<&crate::boundary::BoundaryDecision>,
) -> f32 {
    let mut score = 0.35_f32;

    if node.text.is_textual {
        score += 0.20;
    }

    if node.visual.has_fill && node.visual.has_radius {
        score += 0.15;
    }

    if matches!(node.layout.kind, figma_ir_core::LayoutKind::Stack | figma_ir_core::LayoutKind::Grid) {
        score += 0.10;
    }

    if let Some(boundary) = boundary {
        score += (boundary.total_score * 0.3).min(0.20);
    }

    score.clamp(0.0, 1.0)
}

fn infer_node_diagnostics(
    node: &figma_ir_core::FeatureNode,
    boundary: Option<&crate::boundary::BoundaryDecision>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if let Some(boundary) = boundary {
        if boundary.accepted && boundary.total_score < 0.60 {
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Warning,
                code: "LOW_CONFIDENCE_BOUNDARY".to_string(),
                message: format!(
                    "boundary accepted with marginal score {:.2}",
                    boundary.total_score
                ),
                node_id: Some(node.id.0.clone()),
            });
        }
    }

    if matches!(node.layout.kind, figma_ir_core::LayoutKind::Unknown) && node.structure.child_count > 0 {
        diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Info,
            code: "UNKNOWN_LAYOUT_KIND".to_string(),
            message: "layout kind could not be inferred confidently".to_string(),
            node_id: Some(node.id.0.clone()),
        });
    }

    if node.repetition.repeated_subtree_count >= 3 {
        diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Info,
            code: "HIGH_REPETITION_PATTERN".to_string(),
            message: "node participates in a repeated subtree pattern".to_string(),
            node_id: Some(node.id.0.clone()),
        });
    }

    diagnostics
}

fn map_layout_kind(kind: &figma_ir_core::LayoutKind) -> LayoutSpecKind {
    match kind {
        figma_ir_core::LayoutKind::None => LayoutSpecKind::None,
        figma_ir_core::LayoutKind::Stack => LayoutSpecKind::Stack,
        figma_ir_core::LayoutKind::Grid => LayoutSpecKind::Grid,
        figma_ir_core::LayoutKind::Absolute => LayoutSpecKind::Absolute,
        figma_ir_core::LayoutKind::Unknown => LayoutSpecKind::Unknown,
    }
}

fn map_layout_direction(axis: &figma_ir_core::Axis) -> LayoutDirection {
    match axis {
        figma_ir_core::Axis::Horizontal => LayoutDirection::Horizontal,
        figma_ir_core::Axis::Vertical => LayoutDirection::Vertical,
    }
}
```

---

# 16\. ここで 1 つ修正が必要

上のコードでは `node.text.raw_text` を使っています。  
したがって、`FeatureNode` 側の `TextFeatures` を必ず修正してください。

再掲します。

```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TextFeatures {
    pub is_textual: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count_estimate: Option<usize>,
}
```

`extractor` の `extract_text_features` もそれに合わせます。

---

# 17\. いまの normalizer の限界

ここからは厳しめに言います。

この実装は **まだ実務では弱い**です。  
理由は明確です。

## 弱い点

- wrapper collapse が単純すぎる
- `source_node_ids` が collapse 前提で統合されていない
- 兄弟をまとめて 1 section に昇格させる処理がない
- repeated subtree を list 全体へ昇格させていない
- kind 推定が node 単位すぎる

つまり、今は **1 ノード -> 1 `UiNode`** が基本です。  
本当は、**複数ノード -> 1 `UiNode`** になる場面がかなり多いです。

ただし、初手としてはこれでよいです。  
なぜなら、今必要なのは「壊れない再帰変換の土台」だからです。

---

# 18\. 次にやるべき改善

順序としては次です。

## 改善 1

collapse 時に `source_node_ids` を統合する  
今は wrapper を捨てているだけです。後で逆引きしづらくなります。

## 改善 2

repetition pattern を検出した親を `List` に昇格させる  
子を `ListItem` にまとめる。

## 改善 3

`Heading + Text + Button` を section/card へ束ねる  
これは node 単位ではなく sibling group 単位の処理です。

## 改善 4

`component_candidates` の仮生成  
`classifier` の前でも、最低限の候補名を出してよいです。

---

# 19\. 率直な結論

ここまで入ると、ようやく

- `RawFrameSnapshot`
- `FeatureGraph`
- `UiIr`

の 3 段がつながります。

この段階で一度、**fixture を 3 つくらい作って snapshot test を回す**方がよいです。  
その後で `classifier` に入るのが順番です。

---

次に書くべきものは、自然な順序としては **`classifier` の骨格** です。  
そこで初めて

- `component_candidates`
- `atomic_candidates`
- `reusability`

をまともに埋めます。

## Prompt:

お願いします

## Response:

では、**`classifier` の骨格**を詰めます。

ここで初めて、`UiNode` に対して

- `component_candidates`
- `atomic_candidates`
- `reusability`

を入れます。

ただし、この層でもやりすぎてはいけません。  
`classifier` は **推定器** です。確定器ではありません。

---

# 1\. `classifier` の責務

`classifier` の責務は次の 3 つです。

## 1\. コンポーネント候補名を付ける

例:

- `PrimaryButton`
- `OrderSummaryCard`
- `CheckoutSummarySection`

## 2\. Atomic Design 候補を付ける

例:

- atom: 0.82
- molecule: 0.41

## 3\. 再利用性を点数化する

例:

- score: 0.89
- reasons:
    - repeated_pattern
    - compact_self_contained_layout
    - semantic_name_hint

ここで大事なのは、**候補を複数持つ**ことです。  
単一ラベルに決め打ちすると、あとで破綻します。

---

# 2\. `classifier` の入力と出力

入力:

- `UiIr`

出力:

- `UiIr`  
  ただし `tree` 配下の各 `UiNode` に
    - `component_candidates`
    - `atomic_candidates`
    - `reusability`
    - `diagnostics`  
      を足したもの

つまり **破壊的更新ではなく、変換** にします。

---

# 3\. `classifier` crate の公開 API

`crates/classifier/src/lib.rs`

```
mod classify;
mod naming;
mod atomic;
mod reusability;

pub use classify::classify_ui_ir;
```

---

# 4\. 全体の流れ

`classify_ui_ir` は `UiIr.tree` を再帰的に歩いて、各ノードに分類結果を入れます。

```
UiIr
 └─ tree: UiNode
      ├─ classify current node
      └─ recurse into children
```

---

# 5\. `classify.rs`

`crates/classifier/src/classify.rs`

```
use anyhow::Result;
use figma_ir_core::UiIr;

use crate::{
    atomic::infer_atomic_candidates,
    naming::infer_component_candidates,
    reusability::infer_reusability,
};

pub fn classify_ui_ir(mut ir: UiIr) -> Result<UiIr> {
    ir.tree = classify_node(ir.tree);
    Ok(ir)
}

fn classify_node(mut node: figma_ir_core::UiNode) -> figma_ir_core::UiNode {
    node.component_candidates = infer_component_candidates(&node);
    node.atomic_candidates = infer_atomic_candidates(&node);
    node.reusability = Some(infer_reusability(&node));

    node.children = node
        .children
        .into_iter()
        .map(classify_node)
        .collect();

    node
}
```

---

# 6\. component naming

名前付けは、凝りすぎるとすぐ壊れます。  
初版は **安定した単純ルール** にしておくのがよいです。

ルールは次の順でよいです。

1.  `name` があるならそれを基にする
2.  `kind` を suffix に使う
3.  `Section`, `Card`, `Button`, `ListItem` などは優先的に読める名前へ寄せる
4.  候補は 1 個ではなく 2〜3 個出してよい

---

## `naming.rs`

`crates/classifier/src/naming.rs`

```
use figma_ir_core::{ComponentCandidate, Confidence, UiNode, UiNodeKind};

pub fn infer_component_candidates(node: &UiNode) -> Vec<ComponentCandidate> {
    let mut out = Vec::new();

    let base = base_name(node);
    let primary = primary_candidate_name(node, &base);

    out.push(ComponentCandidate {
        name: primary,
        score: Confidence(primary_name_score(node)),
        reason_codes: primary_reason_codes(node),
    });

    if let Some(secondary) = secondary_candidate_name(node, &base) {
        out.push(ComponentCandidate {
            name: secondary,
            score: Confidence(secondary_name_score(node)),
            reason_codes: vec!["SECONDARY_NAMING_PATTERN".to_string()],
        });
    }

    out
}

fn base_name(node: &UiNode) -> String {
    let raw = node
        .name
        .as_deref()
        .unwrap_or(match node.kind {
            UiNodeKind::Screen => "Screen",
            UiNodeKind::Section => "Section",
            UiNodeKind::Card => "Card",
            UiNodeKind::Button => "Button",
            UiNodeKind::List => "List",
            UiNodeKind::ListItem => "ListItem",
            UiNodeKind::Heading => "Heading",
            UiNodeKind::Text => "Text",
            UiNodeKind::Input => "Input",
            _ => "Node",
        });

    to_pascal_case(raw)
}

fn primary_candidate_name(node: &UiNode, base: &str) -> String {
    match node.kind {
        UiNodeKind::Screen => ensure_suffix(base, "Screen"),
        UiNodeKind::Section => ensure_suffix(base, "Section"),
        UiNodeKind::Card => ensure_suffix(base, "Card"),
        UiNodeKind::Button => ensure_suffix(base, "Button"),
        UiNodeKind::List => ensure_suffix(base, "List"),
        UiNodeKind::ListItem => ensure_suffix(base, "Item"),
        UiNodeKind::Heading => ensure_suffix(base, "Heading"),
        UiNodeKind::Input => ensure_suffix(base, "Input"),
        _ => ensure_suffix(base, "Block"),
    }
}

fn secondary_candidate_name(node: &UiNode, base: &str) -> Option<String> {
    match node.kind {
        UiNodeKind::Button => Some(format!("{}Action", strip_suffix(base, "Button"))),
        UiNodeKind::Card => Some(format!("{}Panel", strip_suffix(base, "Card"))),
        UiNodeKind::Section => Some(format!("{}Group", strip_suffix(base, "Section"))),
        UiNodeKind::ListItem => Some(format!("{}Row", strip_suffix(base, "Item"))),
        _ => None,
    }
}

fn primary_name_score(node: &UiNode) -> f32 {
    let mut score = 0.55_f32;

    if node.name.is_some() {
        score += 0.15;
    }
    if !matches!(node.kind, UiNodeKind::Unknown | UiNodeKind::Stack | UiNodeKind::Container) {
        score += 0.15;
    }
    if node.confidence.0 > 0.75 {
        score += 0.10;
    }

    score.clamp(0.0, 1.0)
}

fn secondary_name_score(node: &UiNode) -> f32 {
    let mut score = primary_name_score(node) - 0.12;
    if node.children.len() >= 2 {
        score += 0.05;
    }
    score.clamp(0.0, 1.0)
}

fn primary_reason_codes(node: &UiNode) -> Vec<String> {
    let mut reasons = Vec::new();

    if node.name.is_some() {
        reasons.push("SOURCE_NAME_HINT".to_string());
    }
    reasons.push(format!("UI_KIND_{:?}", node.kind).to_ascii_uppercase());

    reasons
}

fn ensure_suffix(base: &str, suffix: &str) -> String {
    if base.ends_with(suffix) {
        base.to_string()
    } else {
        format!("{base}{suffix}")
    }
}

fn strip_suffix<'a>(base: &'a str, suffix: &str) -> &'a str {
    base.strip_suffix(suffix).unwrap_or(base)
}

fn to_pascal_case(input: &str) -> String {
    input
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    let mut s = String::new();
                    s.push(first.to_ascii_uppercase());
                    s.push_str(chars.as_str());
                    s
                }
                None => String::new(),
            }
        })
        .collect::<String>()
}
```

---

# 7\. atomic classification

Atomic Design の分類は、**node 自体の見た目**だけでなく、**子の数と責務のまとまり**を見ます。

初版の考え方は次です。

## atom 候補になりやすい

- Button
- Input
- Checkbox
- Radio
- Switch
- Icon
- Badge
- 単独テキスト

## molecule 候補になりやすい

- Card
- ListItem
- FieldGroup
- Avatar + Text
- Label + Input

## organism 候補になりやすい

- Section
- Navigation
- Form
- List
- ある程度まとまった複合構造

## template / page 候補

- Screen
- Template
- 上位コンテナ

---

## `atomic.rs`

`crates/classifier/src/atomic.rs`

```
use figma_ir_core::{AtomicCandidate, AtomicLevel, Confidence, UiNode, UiNodeKind};

pub fn infer_atomic_candidates(node: &UiNode) -> Vec<AtomicCandidate> {
    let mut out = Vec::new();

    let scores = atomic_scores(node);

    for (level, score) in scores {
        if score > 0.0 {
            out.push(AtomicCandidate {
                level,
                score: Confidence(score.clamp(0.0, 1.0)),
            });
        }
    }

    out.sort_by(|a, b| {
        b.score
            .0
            .partial_cmp(&a.score.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    out
}

fn atomic_scores(node: &UiNode) -> Vec<(AtomicLevel, f32)> {
    let mut atom = 0.0_f32;
    let mut molecule = 0.0_f32;
    let mut organism = 0.0_f32;
    let mut template = 0.0_f32;
    let mut page = 0.0_f32;
    let mut unknown = 0.05_f32;

    match node.kind {
        UiNodeKind::Screen => {
            page += 0.95;
        }
        UiNodeKind::Template => {
            template += 0.90;
        }
        UiNodeKind::Button
        | UiNodeKind::Input
        | UiNodeKind::Checkbox
        | UiNodeKind::Radio
        | UiNodeKind::Switch
        | UiNodeKind::Icon
        | UiNodeKind::Badge
        | UiNodeKind::Label => {
            atom += 0.75;
        }
        UiNodeKind::Heading | UiNodeKind::Text => {
            atom += 0.65;
        }
        UiNodeKind::Card | UiNodeKind::ListItem | UiNodeKind::FieldGroup => {
            molecule += 0.72;
        }
        UiNodeKind::Section | UiNodeKind::Navigation | UiNodeKind::Form | UiNodeKind::List => {
            organism += 0.72;
        }
        UiNodeKind::Grid | UiNodeKind::Stack | UiNodeKind::Container | UiNodeKind::Region => {
            molecule += 0.25;
            organism += 0.20;
            unknown += 0.15;
        }
        UiNodeKind::Unknown => {
            unknown += 0.50;
        }
        _ => {}
    }

    let child_len = node.children.len();

    if child_len == 0 {
        atom += 0.10;
    } else if child_len <= 2 {
        molecule += 0.10;
    } else if child_len <= 6 {
        organism += 0.10;
    } else {
        template += 0.08;
    }

    if node
        .reusability
        .as_ref()
        .map(|r| r.score.0 > 0.75)
        .unwrap_or(false)
    {
        atom += 0.03;
        molecule += 0.08;
        organism += 0.05;
    }

    vec![
        (AtomicLevel::Atom, atom),
        (AtomicLevel::Molecule, molecule),
        (AtomicLevel::Organism, organism),
        (AtomicLevel::Template, template),
        (AtomicLevel::Page, page),
        (AtomicLevel::Unknown, unknown),
    ]
}
```

---

# 8\. ここで 1 つ問題がある

上の `atomic.rs` は `node.reusability` を見ています。  
しかし `classify.rs` では、`atomic_candidates` を `reusability` より先に計算していました。

順序を直します。

---

## `classify.rs` 修正版

```
use anyhow::Result;
use figma_ir_core::UiIr;

use crate::{
    atomic::infer_atomic_candidates,
    naming::infer_component_candidates,
    reusability::infer_reusability,
};

pub fn classify_ui_ir(mut ir: UiIr) -> Result<UiIr> {
    ir.tree = classify_node(ir.tree);
    Ok(ir)
}

fn classify_node(mut node: figma_ir_core::UiNode) -> figma_ir_core::UiNode {
    node.children = node
        .children
        .into_iter()
        .map(classify_node)
        .collect();

    node.reusability = Some(infer_reusability(&node));
    node.component_candidates = infer_component_candidates(&node);
    node.atomic_candidates = infer_atomic_candidates(&node);

    node
}
```

これなら順序が自然です。

---

# 9\. reusability 推定

再利用性はかなり効きます。  
これは、コード生成やコンポーネント切り出し判断の根拠になるからです。

見る項目は次です。

- 形が自己完結しているか
- 名前が抽象的か / 具体的すぎないか
- 子構造が適度か
- text が特定文言に依存しすぎないか
- `Card`, `Button`, `Item`, `Field` などの再利用しやすいパターンか

---

## `reusability.rs`

`crates/classifier/src/reusability.rs`

```
use figma_ir_core::{Confidence, Reusability, UiNode, UiNodeKind};

pub fn infer_reusability(node: &UiNode) -> Reusability {
    let mut score = 0.20_f32;
    let mut reasons = Vec::new();

    match node.kind {
        UiNodeKind::Button
        | UiNodeKind::Input
        | UiNodeKind::Checkbox
        | UiNodeKind::Radio
        | UiNodeKind::Switch
        | UiNodeKind::Badge
        | UiNodeKind::Icon => {
            score += 0.40;
            reasons.push("PRIMITIVE_UI_PATTERN".to_string());
        }
        UiNodeKind::Card | UiNodeKind::ListItem | UiNodeKind::FieldGroup => {
            score += 0.35;
            reasons.push("COMPOSITE_REUSABLE_PATTERN".to_string());
        }
        UiNodeKind::Section | UiNodeKind::Navigation | UiNodeKind::Form | UiNodeKind::List => {
            score += 0.20;
            reasons.push("STRUCTURED_UI_BLOCK".to_string());
        }
        UiNodeKind::Screen | UiNodeKind::Template => {
            score -= 0.10;
            reasons.push("TOP_LEVEL_LAYOUT".to_string());
        }
        _ => {}
    }

    if node.children.len() <= 4 && !node.children.is_empty() {
        score += 0.08;
        reasons.push("COMPACT_CHILD_STRUCTURE".to_string());
    }

    if let Some(name) = &node.name {
        let lowered = name.to_ascii_lowercase();
        if lowered.contains("button")
            || lowered.contains("card")
            || lowered.contains("item")
            || lowered.contains("field")
            || lowered.contains("badge")
        {
            score += 0.10;
            reasons.push("SEMANTIC_NAME_HINT".to_string());
        }

        if lowered.contains("checkout")
            || lowered.contains("summary")
            || lowered.contains("hero")
            || lowered.contains("homepage")
        {
            score -= 0.05;
            reasons.push("DOMAIN_SPECIFIC_NAME".to_string());
        }
    }

    if has_portable_text(node) {
        score += 0.06;
        reasons.push("PORTABLE_TEXTUAL_CONTENT".to_string());
    }

    Reusability {
        score: Confidence(score.clamp(0.0, 1.0)),
        reasons,
    }
}

fn has_portable_text(node: &UiNode) -> bool {
    let Some(text) = node.content.text.as_deref() else {
        return false;
    };

    let len = text.chars().count();
    len > 0 && len <= 24
}
```

---

# 10\. diagnostics を classifier でも足すべきか

足した方がよいです。  
ただし、`UiNode` にはもう `diagnostics` があるので、`classify_node` の中で追加すれば足ります。

---

## `classify.rs` を少し拡張

```
use anyhow::Result;
use figma_ir_core::{Diagnostic, DiagnosticLevel, UiIr};

use crate::{
    atomic::infer_atomic_candidates,
    naming::infer_component_candidates,
    reusability::infer_reusability,
};

pub fn classify_ui_ir(mut ir: UiIr) -> Result<UiIr> {
    ir.tree = classify_node(ir.tree);
    Ok(ir)
}

fn classify_node(mut node: figma_ir_core::UiNode) -> figma_ir_core::UiNode {
    node.children = node
        .children
        .into_iter()
        .map(classify_node)
        .collect();

    node.reusability = Some(infer_reusability(&node));
    node.component_candidates = infer_component_candidates(&node);
    node.atomic_candidates = infer_atomic_candidates(&node);

    append_classifier_diagnostics(&mut node);

    node
}

fn append_classifier_diagnostics(node: &mut figma_ir_core::UiNode) {
    if let Some(first) = node.atomic_candidates.first() {
        if matches!(first.level, figma_ir_core::AtomicLevel::Unknown) && first.score.0 >= 0.4 {
            node.diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Warning,
                code: "ATOMIC_CLASSIFICATION_AMBIGUOUS".to_string(),
                message: "atomic classification remains ambiguous".to_string(),
                node_id: Some(node.id.clone()),
            });
        }
    }

    if let Some(first) = node.component_candidates.first() {
        if first.score.0 < 0.55 {
            node.diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Info,
                code: "LOW_CONFIDENCE_COMPONENT_NAME".to_string(),
                message: "component name is low confidence".to_string(),
                node_id: Some(node.id.clone()),
            });
        }
    }

    if let Some(reusability) = &node.reusability {
        if reusability.score.0 < 0.30 {
            node.diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Info,
                code: "LOW_REUSABILITY".to_string(),
                message: "node appears to be highly context-specific".to_string(),
                node_id: Some(node.id.clone()),
            });
        }
    }
}
```

---

# 11\. この classifier の性格

この実装は、あくまで **初版のルールベース分類器** です。  
それで十分です。

ここで大事なのは、結果が次の性質を持つことです。

- **決定的** である
- **説明可能** である
- **後で置き換えやすい** である

逆に、ここで機械学習めいたことを始めると、すぐ不透明になります。

---

# 12\. 現時点でのパイプライン全体

ここまでで、流れはこうなります。

```
runtime
  -> RawFrameSnapshot
extractor
  -> FeatureGraph
normalizer
  -> UiIr (draft)
classifier
  -> UiIr (classified)
reporter
  -> report.md
store
  -> json files
```

ここまで来ると、ようやく CLI の `normalize` コマンドと `report` コマンドが意味を持ちます。

---

# 13\. CLI との接続

`normalize` コマンドの中で、最終的にはこうなります。

```
use anyhow::Result;
use figma_ir_classifier::classify_ui_ir;
use figma_ir_core::{FrameId};
use figma_ir_extractor::extract_features;
use figma_ir_normalizer::normalize;
use figma_ir_runtime::{map_runtime_snapshot_to_raw, RuntimeClient, RuntimeFrameRequest};
use figma_ir_store::write_json_file;

use crate::args::{FrameNormalizeArgs, NormalizeCommand};

pub async fn run(cmd: NormalizeCommand) -> Result<()> {
    match cmd {
        NormalizeCommand::Frame(args) => run_frame(args).await,
    }
}

async fn run_frame(args: FrameNormalizeArgs) -> Result<()> {
    let client = figma_ir_runtime::PatchedCdpRuntimeClient::new();

    let session = client.attach().await?;
    let runtime_snapshot = client
        .snapshot_frame(RuntimeFrameRequest {
            frame_id: args.id.clone(),
        })
        .await?;

    let raw = map_runtime_snapshot_to_raw(&session, runtime_snapshot);
    let features = extract_features(&raw)?;
    let ir = normalize(&features)?;
    let classified = classify_ui_ir(ir)?;

    write_json_file(&args.out, &classified, args.pretty).await?;
    println!("wrote {}", args.out);

    Ok(())
}
```

もちろん `PatchedCdpRuntimeClient` はまだ骨格だけですが、パイプラインとしてはこれで揃います。

---

# 14\. テスト方針

`classifier` は fixture ベースでテストするのがよいです。  
`UiNode` を 3〜5 種類作って、分類結果が妥当か見る形です。

最低限、次のケースが欲しいです。

## ケース 1: Button

- `kind = Button`
- `name = Checkout Button`
- `children = []`

期待:

- primary component candidate が `CheckoutButton`
- top atomic candidate が `Atom`
- reusability が高め

## ケース 2: Card

- `kind = Card`
- `children = [Heading, Text, Button]`

期待:

- top atomic candidate が `Molecule` または `Organism`
- `Card` suffix の候補名が出る

## ケース 3: Screen

- `kind = Screen`

期待:

- atomic が `Page`
- reusability は低め

---

## テスト例

`crates/classifier/tests/classifier_tests.rs`

```
use figma_ir_classifier::classify_ui_ir;
use figma_ir_core::{
    Confidence, LayoutSpec, LayoutSpecKind, SemanticRole, Semantics, SourceRef, StyleRefs,
    TokenRefs, TransportKind, UiIr, UiNode, UiNodeKind, SessionId, SessionInfo, FrameId,
};

fn empty_node(id: &str, name: &str, kind: UiNodeKind) -> UiNode {
    UiNode {
        id: id.to_string(),
        name: Some(name.to_string()),
        kind,
        source_node_ids: vec![id.to_string()],
        component_candidates: vec![],
        atomic_candidates: vec![],
        confidence: Confidence(0.9),
        reusability: None,
        layout: LayoutSpec {
            kind: LayoutSpecKind::None,
            direction: None,
            gap: None,
            padding: None,
            align_x: None,
            align_y: None,
            columns: None,
        },
        style_refs: StyleRefs::default(),
        semantics: Semantics {
            role: SemanticRole::Generic,
            accessible_name: Some(name.to_string()),
        },
        content: Default::default(),
        children: vec![],
        diagnostics: vec![],
    }
}

fn base_ir(tree: UiNode) -> UiIr {
    UiIr {
        version: "0.1.0".to_string(),
        source: SourceRef {
            file_key: None,
            frame_id: FrameId("1:1".to_string()),
            frame_name: "Example".to_string(),
            session: SessionInfo {
                id: SessionId("s1".to_string()),
                transport: TransportKind::DesktopPatchedCdp,
                captured_at: chrono::Utc::now(),
                figma_version: None,
                platform: None,
            },
        },
        tree,
        tokens: TokenRefs::default(),
        diagnostics: vec![],
    }
}

#[test]
fn classifies_button_as_atom() {
    let ir = base_ir(empty_node("node.button.1", "Checkout Button", UiNodeKind::Button));
    let classified = classify_ui_ir(ir).unwrap();

    assert_eq!(classified.tree.component_candidates[0].name, "CheckoutButton");
    assert!(matches!(
        classified.tree.atomic_candidates[0].level,
        figma_ir_core::AtomicLevel::Atom
    ));
    assert!(classified.tree.reusability.as_ref().unwrap().score.0 > 0.5);
}

#[test]
fn classifies_screen_as_page() {
    let ir = base_ir(empty_node("screen.checkout", "Checkout", UiNodeKind::Screen));
    let classified = classify_ui_ir(ir).unwrap();

    assert!(matches!(
        classified.tree.atomic_candidates[0].level,
        figma_ir_core::AtomicLevel::Page
    ));
}
```

---

# 15\. まだ弱い点

ここも率直に言います。

この classifier は **node 単位** でしか見ていません。  
本当は次のような情報も見たいです。

- 親との関係
- 兄弟との関係
- subtree の型
- repeated subtree の分布
- domain-specific naming rule

つまり、将来的には `classify_node` に文脈を渡した方がよいです。

```
fn classify_node(node: UiNode, parent: Option<&UiNode>, path: &[&str]) -> UiNode
```

のようにした方が伸びます。

ただし、初版としては **単独 node 規則** で十分です。

---

# 16\. 次に何を書くべきか

ここまでで、**抽出 → 正規化 → 分類** の骨格は揃いました。  
次に必要なのは次のどちらかです。

## A

**`reporter` の実装**  
`UiIr` から `report.md` を出す  
人間の確認用としてかなり役に立ちます。

## B

**`runtime` の具体骨格**  
`PatchedCdpRuntimeClient` を最低限の stub ではなく、実接続を見据えた形にする

順番としては、設計検証を先にやるなら **A**、接続の検証を急ぐなら **B** です。

## Prompt:

B

## Response:

では、**`runtime` の具体骨格**を詰めます。

ここでは「実際にすぐ動く完成品」ではなく、**あとで本当に接続実装を差し込める形**まで持っていきます。  
狙いは次の 3 つです。

- `PatchedCdpRuntimeClient` の責務を固定する
- CDP 接続、JS 実行、結果復元の流れを分離する
- 失敗箇所を診断しやすくする

---

# 1\. 先に方針

あなたの条件では、`runtime` は次の構造にしておくのがよいです。

```
runtime/
├── client.rs
├── error.rs
├── lib.rs
├── mapper.rs
├── models.rs
├── session.rs
└── transport/
    ├── mod.rs
    ├── patched_cdp.rs
    ├── js_bridge.rs
    ├── protocol.rs
    └── probe.rs
```

ここでの分離はかなり大事です。

- `patched_cdp.rs`  
  WebSocket 接続と CDP メッセージ送受信
- `js_bridge.rs`  
  Figma ランタイムへ流す JavaScript の生成
- `protocol.rs`  
  CDP の request/response 型
- `probe.rs`  
  接続前診断。Figma プロセスや endpoint の発見
- `mapper.rs`  
  runtime 結果を `core::raw` へ落とす

これを 1 ファイルに押し込むと、あとで必ず崩れます。

---

# 2\. `runtime/Cargo.toml`

まず依存を少し増やします。

```
[package]
name = "figma_ir_runtime"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
thiserror.workspace = true
serde.workspace = true
serde_json.workspace = true
tokio.workspace = true
chrono.workspace = true
uuid.workspace = true

async-trait = "0.1.88"
futures-util = "0.3.31"
tokio-tungstenite = { version = "0.24.0", features = ["rustls-tls-native-roots"] }
reqwest = { version = "0.12.15", default-features = false, features = ["json", "rustls-tls"] }

figma_ir_core = { path = "../core" }
```

`reqwest` は CDP endpoint discovery 用です。  
`tokio-tungstenite` は WebSocket 接続用です。

---

# 3\. `lib.rs`

```
pub mod client;
pub mod error;
pub mod mapper;
pub mod models;
pub mod session;
pub mod transport;

pub use client::*;
pub use error::*;
pub use mapper::*;
pub use models::*;
pub use session::*;
pub use transport::patched_cdp::PatchedCdpRuntimeClient;
```

---

# 4\. `client.rs`

前に置いた trait をそのまま使います。

```
use async_trait::async_trait;

use crate::{
    RuntimeError, RuntimeFrameRequest, RuntimeFrameSnapshot, RuntimeHealth, RuntimeSession,
};

#[async_trait]
pub trait RuntimeClient: Send + Sync {
    async fn attach(&self) -> Result<RuntimeSession, RuntimeError>;

    async fn health(&self) -> Result<RuntimeHealth, RuntimeError>;

    async fn snapshot_frame(
        &self,
        request: RuntimeFrameRequest,
    ) -> Result<RuntimeFrameSnapshot, RuntimeError>;
}
```

---

# 5\. `transport/mod.rs`

```
pub mod js_bridge;
pub mod patched_cdp;
pub mod probe;
pub mod protocol;
```

---

# 6\. probe: 何を診断するか

実装前に、接続失敗の種類を分ける必要があります。  
少なくとも次を分けないと、使いものになりません。

1.  Figma プロセス自体がいない
2.  debug endpoint が開いていない
3.  endpoint にはつながるが target が取れない
4.  WebSocket はつながるが `Runtime.evaluate` が失敗する
5.  Figma ランタイム内の script が失敗する

このうち 1〜3 は `probe.rs` で切り分けます。

---

## `probe.rs`

```
use serde::{Deserialize, Serialize};

use crate::RuntimeError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugEndpointInfo {
    pub http_base: String,
    pub websocket_url: String,
    pub title: Option<String>,
    pub target_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeReport {
    pub runtime_detected: bool,
    pub endpoint_reachable: bool,
    pub websocket_discovered: bool,
    pub endpoint: Option<DebugEndpointInfo>,
    pub message: Option<String>,
}

pub async fn probe_debug_endpoint(
    endpoint_base: &str,
) -> Result<ProbeReport, RuntimeError> {
    let version_url = format!("{}/json/version", endpoint_base.trim_end_matches('/'));

    let response = reqwest::get(&version_url)
        .await
        .map_err(|e| RuntimeError::ConnectFailed(format!("failed to reach debug endpoint: {e}")))?;

    if !response.status().is_success() {
        return Ok(ProbeReport {
            runtime_detected: true,
            endpoint_reachable: false,
            websocket_discovered: false,
            endpoint: None,
            message: Some(format!(
                "debug endpoint returned non-success status: {}",
                response.status()
            )),
        });
    }

    #[derive(Debug, Deserialize)]
    struct VersionResponse {
        #[serde(rename = "webSocketDebuggerUrl")]
        websocket_debugger_url: Option<String>,
    }

    let version = response
        .json::<VersionResponse>()
        .await
        .map_err(|e| RuntimeError::InvalidResponse(format!("invalid version response: {e}")))?;

    let websocket_url = version.websocket_debugger_url;

    Ok(ProbeReport {
        runtime_detected: true,
        endpoint_reachable: true,
        websocket_discovered: websocket_url.is_some(),
        endpoint: websocket_url.map(|ws| DebugEndpointInfo {
            http_base: endpoint_base.to_string(),
            websocket_url: ws,
            title: None,
            target_id: None,
        }),
        message: None,
    })
}
```

---

# 7\. protocol: CDP 型

CDP メッセージを `serde_json::Value` で全部やるとすぐ壊れます。  
最低限の型は置いた方がよいです。

## `protocol.rs`

```
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdpRequest {
    pub id: u64,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdpErrorObject {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdpResponse {
    pub id: Option<u64>,
    pub result: Option<Value>,
    pub error: Option<CdpErrorObject>,
    pub method: Option<String>,
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEvaluateParams {
    pub expression: String,
    #[serde(rename = "returnByValue")]
    pub return_by_value: bool,
    #[serde(rename = "awaitPromise")]
    pub await_promise: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeRemoteObject {
    #[serde(rename = "type")]
    pub object_type: Option<String>,
    pub value: Option<Value>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEvaluateResult {
    pub result: RuntimeRemoteObject,
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<Value>,
}
```

---

# 8\. js_bridge

ここはかなり重要です。  
ランタイムに流す JavaScript を、呼び出し側が直接組み立てないようにします。

## `js_bridge.rs`

```
pub fn build_runtime_ping_script() -> String {
    r#"
(() => {
  return JSON.stringify({
    ok: true,
    hasFigma: typeof figma !== "undefined",
    currentPageId: figma?.currentPage?.id ?? null
  });
})()
"#
    .to_string()
}

pub fn build_snapshot_frame_script(frame_id: &str) -> String {
    format!(
        r#"
(() => {{
  const targetId = {frame_id:?};

  function serializePaints(fills) {{
    if (!Array.isArray(fills)) return [];
    return fills
      .filter(fill => fill && fill.type === "SOLID" && fill.color)
      .map(fill => ({{
        r: fill.color.r,
        g: fill.color.g,
        b: fill.color.b,
        a: fill.opacity ?? 1
      }}));
  }}

  function serializeTextStyle(node) {{
    if (node.type !== "TEXT") return null;

    let fontFamily = null;
    if (node.fontName && typeof node.fontName === "object") {{
      fontFamily = node.fontName.family ?? null;
    }}

    let lineHeight = null;
    if (node.lineHeight && typeof node.lineHeight === "object") {{
      lineHeight = node.lineHeight.value ?? null;
    }}

    let letterSpacing = null;
    if (node.letterSpacing && typeof node.letterSpacing === "object") {{
      letterSpacing = node.letterSpacing.value ?? null;
    }}

    return {{
      font_family: fontFamily,
      font_weight: null,
      font_size: typeof node.fontSize === "number" ? node.fontSize : null,
      line_height: lineHeight,
      letter_spacing: letterSpacing
    }};
  }}

  function serializeAutoLayout(node) {{
    const layoutMode = "layoutMode" in node ? node.layoutMode : null;

    return {{
      layout_mode: layoutMode,
      item_spacing: "itemSpacing" in node ? node.itemSpacing : null,
      padding_top: "paddingTop" in node ? node.paddingTop : null,
      padding_right: "paddingRight" in node ? node.paddingRight : null,
      padding_bottom: "paddingBottom" in node ? node.paddingBottom : null,
      padding_left: "paddingLeft" in node ? node.paddingLeft : null
    }};
  }}

  function serializeNode(node, parentId = null, acc = []) {{
    const bounds = "absoluteRenderBounds" in node ? node.absoluteRenderBounds : null;
    const autoLayout = serializeAutoLayout(node);

    acc.push({{
      id: node.id,
      name: node.name,
      node_type: node.type,
      parent_id: parentId,
      children: "children" in node ? node.children.map(child => child.id) : [],
      x: bounds ? bounds.x : null,
      y: bounds ? bounds.y : null,
      width: bounds ? bounds.width : null,
      height: bounds ? bounds.height : null,
      visible: "visible" in node ? node.visible : true,
      fills: serializePaints(node.fills),
      radius: typeof node.cornerRadius === "number" ? node.cornerRadius : null,
      text: node.type === "TEXT" ? node.characters : null,
      text_style: serializeTextStyle(node),
      layout_mode: autoLayout.layout_mode,
      item_spacing: autoLayout.item_spacing,
      padding_top: autoLayout.padding_top,
      padding_right: autoLayout.padding_right,
      padding_bottom: autoLayout.padding_bottom,
      padding_left: autoLayout.padding_left
    }});

    if ("children" in node) {{
      for (const child of node.children) {{
        serializeNode(child, node.id, acc);
      }}
    }}

    return acc;
  }}

  const root = figma.currentPage.findOne(n => n.id === targetId);

  if (!root) {{
    return JSON.stringify({{
      ok: false,
      error: "FRAME_NOT_FOUND",
      frame_id: targetId
    }});
  }}

  const nodes = serializeNode(root);

  return JSON.stringify({{
    ok: true,
    frame_id: root.id,
    frame_name: root.name,
    nodes
  }});
}})()
"#
    )
}
```

ここでは **必ず JSON.stringify した文字列を返す**ようにしています。  
理由は、CDP の `Runtime.evaluate` は返り値の形が不安定になりやすいからです。  
一度文字列にすれば、Rust 側で統一的に扱えます。

---

# 9\. patched_cdp の中核構造

`PatchedCdpRuntimeClient` は、少なくとも次を持つべきです。

- endpoint base
- request id 採番器
- timeout
- WebSocket 接続生成
- `Runtime.enable`
- `Runtime.evaluate`

---

## `patched_cdp.rs` 骨格

```
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{
    client::RuntimeClient,
    models::{RuntimeFrameRequest, RuntimeFrameSnapshot},
    session::{RuntimeHealth, RuntimeSession},
    transport::{
        js_bridge::{build_runtime_ping_script, build_snapshot_frame_script},
        probe::probe_debug_endpoint,
        protocol::{CdpRequest, CdpResponse, RuntimeEvaluateParams, RuntimeEvaluateResult},
    },
    RuntimeError,
};

pub struct PatchedCdpRuntimeClient {
    endpoint_base: String,
    timeout: Duration,
    next_id: AtomicU64,
}

impl PatchedCdpRuntimeClient {
    pub fn new() -> Self {
        Self {
            endpoint_base: "http://127.0.0.1:9222".to_string(),
            timeout: Duration::from_secs(10),
            next_id: AtomicU64::new(1),
        }
    }

    pub fn with_endpoint_base(mut self, endpoint_base: impl Into<String>) -> Self {
        self.endpoint_base = endpoint_base.into();
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn next_request_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }
}
```

---

# 10\. `attach`

`attach` は、ただ WebSocket を開くだけでは足りません。  
最低限、

- endpoint を probe
- WebSocket URL を取得
- `Runtime.enable`
- ping script を評価

までやった方がよいです。

```
#[async_trait]
impl RuntimeClient for PatchedCdpRuntimeClient {
    async fn attach(&self) -> Result<RuntimeSession, RuntimeError> {
        let probe = probe_debug_endpoint(&self.endpoint_base).await?;

        let endpoint = probe
            .endpoint
            .ok_or_else(|| RuntimeError::ConnectFailed("websocket endpoint not discovered".to_string()))?;

        let (mut ws, _) = connect_async(&endpoint.websocket_url)
            .await
            .map_err(|e| RuntimeError::ConnectFailed(format!("websocket connect failed: {e}")))?;

        self.runtime_enable(&mut ws).await?;
        let _ = self
            .evaluate_string(&mut ws, &build_runtime_ping_script())
            .await?;

        Ok(RuntimeSession {
            id: figma_ir_core::SessionId(uuid::Uuid::new_v4().to_string()),
            transport: figma_ir_core::TransportKind::DesktopPatchedCdp,
            connected_at: chrono::Utc::now(),
            figma_version: None,
            platform: None,
        })
    }
```

---

# 11\. `health`

`health` は attach より軽くてよいです。  
probe ベースで十分です。

```
    async fn health(&self) -> Result<RuntimeHealth, RuntimeError> {
        let probe = probe_debug_endpoint(&self.endpoint_base).await?;

        Ok(RuntimeHealth {
            runtime_detected: probe.runtime_detected,
            connectable: probe.endpoint_reachable && probe.websocket_discovered,
            transport: if probe.websocket_discovered {
                Some(figma_ir_core::TransportKind::DesktopPatchedCdp)
            } else {
                None
            },
            figma_version: None,
            platform: None,
            message: probe.message,
        })
    }
```

---

# 12\. `snapshot_frame`

ここで実際にスナップショットを取ります。

```
    async fn snapshot_frame(
        &self,
        request: RuntimeFrameRequest,
    ) -> Result<RuntimeFrameSnapshot, RuntimeError> {
        let probe = probe_debug_endpoint(&self.endpoint_base).await?;

        let endpoint = probe
            .endpoint
            .ok_or_else(|| RuntimeError::ConnectFailed("websocket endpoint not discovered".to_string()))?;

        let (mut ws, _) = connect_async(&endpoint.websocket_url)
            .await
            .map_err(|e| RuntimeError::ConnectFailed(format!("websocket connect failed: {e}")))?;

        self.runtime_enable(&mut ws).await?;

        let payload = self
            .evaluate_string(&mut ws, &build_snapshot_frame_script(&request.frame_id))
            .await?;

        #[derive(Debug, serde::Deserialize)]
        struct SnapshotPayload {
            ok: bool,
            frame_id: Option<String>,
            frame_name: Option<String>,
            nodes: Option<Vec<crate::models::RuntimeNodeSnapshot>>,
            error: Option<String>,
        }

        let parsed: SnapshotPayload = serde_json::from_str(&payload)
            .map_err(|e| RuntimeError::InvalidResponse(format!("invalid snapshot payload: {e}")))?;

        if !parsed.ok {
            return Err(RuntimeError::FrameNotFound(
                parsed
                    .error
                    .unwrap_or_else(|| request.frame_id.clone()),
            ));
        }

        Ok(RuntimeFrameSnapshot {
            frame_id: parsed
                .frame_id
                .ok_or_else(|| RuntimeError::InvalidResponse("missing frame_id".to_string()))?,
            frame_name: parsed
                .frame_name
                .ok_or_else(|| RuntimeError::InvalidResponse("missing frame_name".to_string()))?,
            nodes: parsed
                .nodes
                .ok_or_else(|| RuntimeError::InvalidResponse("missing nodes".to_string()))?,
        })
    }
}
```

---

# 13\. `runtime_enable`

CDP で `Runtime.evaluate` を打つ前に `Runtime.enable` を流します。

```
impl PatchedCdpRuntimeClient {
    async fn runtime_enable<S>(&self, ws: &mut S) -> Result<(), RuntimeError>
    where
        S: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error>
            + StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
            + Unpin,
    {
        let request = CdpRequest {
            id: self.next_request_id(),
            method: "Runtime.enable".to_string(),
            params: json!({}),
        };

        self.send_request_and_expect_ok(ws, request).await
    }
```

---

# 14\. `evaluate_string`

ここが一番大事です。  
`Runtime.evaluate` の result から、**JSON 文字列としての戻り値**を取り出します。

```
    async fn evaluate_string<S>(
        &self,
        ws: &mut S,
        expression: &str,
    ) -> Result<String, RuntimeError>
    where
        S: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error>
            + StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
            + Unpin,
    {
        let request = CdpRequest {
            id: self.next_request_id(),
            method: "Runtime.evaluate".to_string(),
            params: serde_json::to_value(RuntimeEvaluateParams {
                expression: expression.to_string(),
                return_by_value: true,
                await_promise: true,
            })
            .map_err(|e| RuntimeError::InvalidResponse(format!("invalid evaluate params: {e}")))?,
        };

        let response = self.send_request(ws, request).await?;

        if let Some(err) = response.error {
            return Err(RuntimeError::EvaluationFailed(format!(
                "cdp error {}: {}",
                err.code, err.message
            )));
        }

        let result = response
            .result
            .ok_or_else(|| RuntimeError::InvalidResponse("missing evaluation result".to_string()))?;

        let evaluated: RuntimeEvaluateResult = serde_json::from_value(result)
            .map_err(|e| RuntimeError::InvalidResponse(format!("invalid evaluate result: {e}")))?;

        if evaluated.exception_details.is_some() {
            return Err(RuntimeError::EvaluationFailed(
                "runtime evaluation threw exception".to_string(),
            ));
        }

        match evaluated.result.value {
            Some(serde_json::Value::String(s)) => Ok(s),
            Some(other) => Err(RuntimeError::InvalidResponse(format!(
                "expected string result, got {other}"
            ))),
            None => Err(RuntimeError::InvalidResponse(
                "missing runtime result value".to_string(),
            )),
        }
    }
```

---

# 15\. request/response 送受信

CDP はイベントも飛んできます。  
したがって、`id` が一致する response を待つ必要があります。

```
    async fn send_request_and_expect_ok<S>(
        &self,
        ws: &mut S,
        request: CdpRequest,
    ) -> Result<(), RuntimeError>
    where
        S: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error>
            + StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
            + Unpin,
    {
        let response = self.send_request(ws, request).await?;

        if let Some(err) = response.error {
            return Err(RuntimeError::EvaluationFailed(format!(
                "cdp error {}: {}",
                err.code, err.message
            )));
        }

        Ok(())
    }

    async fn send_request<S>(
        &self,
        ws: &mut S,
        request: CdpRequest,
    ) -> Result<CdpResponse, RuntimeError>
    where
        S: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error>
            + StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
            + Unpin,
    {
        let request_id = request.id;
        let text = serde_json::to_string(&request)
            .map_err(|e| RuntimeError::InvalidResponse(format!("request serialization failed: {e}")))?;

        ws.send(Message::Text(text))
            .await
            .map_err(|e| RuntimeError::Disconnected)?;

        loop {
            let next = ws.next().await.ok_or(RuntimeError::Disconnected)?;
            let msg = next.map_err(|_| RuntimeError::Disconnected)?;

            match msg {
                Message::Text(text) => {
                    let response: CdpResponse = serde_json::from_str(&text)
                        .map_err(|e| RuntimeError::InvalidResponse(format!("invalid cdp response: {e}")))?;

                    if response.id == Some(request_id) {
                        return Ok(response);
                    }

                    continue;
                }
                Message::Binary(_) => continue,
                Message::Ping(_) | Message::Pong(_) => continue,
                Message::Close(_) => return Err(RuntimeError::Disconnected),
                _ => continue,
            }
        }
    }
}
```

---

# 16\. ここで 1 つ率直な問題

この骨格は **動く可能性はある** ものの、まだ現実には弱いです。  
弱い理由は明確です。

## 弱い点

- target 選択をしていない  
  `json/version` の `webSocketDebuggerUrl` だけで十分とは限らない
- port discovery を固定にしている
- auth token をまだ扱っていない
- Figma 側ランタイムが本当に `figma` を持つ文脈か確認していない
- timeout を request ごとに掛けていない
- reconnect 戦略がない

したがって、このまま「完成」と見てはいけません。  
ただし、**責務分離された具体骨格**としては十分です。

---

# 17\. target discovery を入れるならどうするか

`figma-cli` の README では random port と token auth にも触れています。  
あなたがそれを取り込むなら、`probe.rs` を拡張して `/json/list` や `/json` を見て、Figma らしい target を探すべきです。  
つまり、`/json/version` 一本ではなく、target selection を入れる方がよいです。 ([github.com](https://github.com/silships/figma-cli?utm_source=chatgpt.com))

その場合の型はこうなります。

```
#[derive(Debug, Clone, serde::Deserialize)]
pub struct TargetInfo {
    pub id: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub websocket_debugger_url: Option<String>,
    #[serde(rename = "type")]
    pub target_type: Option<String>,
}
```

そして、

- title
- url
- target_type

から候補を絞ります。

---

# 18\. auth token を入れるなら

README では localhost only と token auth の説明があります。  
これを取り込むなら、`PatchedCdpRuntimeClient` に認証情報を持たせる方がよいです。

```
pub struct PatchedCdpRuntimeClient {
    endpoint_base: String,
    timeout: Duration,
    next_id: AtomicU64,
    auth_token: Option<String>,
}
```

そして `probe` と `connect_async` の前に query parameter や header へ載せる実装を検討します。  
ただし、ここは Figma 側の patch 実装に依存するので、今は抽象だけ切っておけば十分です。

---

# 19\. ここで足した方がよい型

`models.rs` に runtime ping 用の型も置いておくと便利です。

```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RuntimePing {
    pub ok: bool,
    pub has_figma: bool,
    pub current_page_id: Option<String>,
}
```

すると `attach` 時の ping 結果検証が少しきれいになります。

---

# 20\. 最低限の `doctor` にどうつなぐか

この runtime 骨格が入ると、CLI の `doctor` はかなりまともになります。

```
use anyhow::Result;
use serde_json::json;

use crate::args::DoctorArgs;

pub async fn run(args: DoctorArgs) -> Result<()> {
    let client = figma_ir_runtime::PatchedCdpRuntimeClient::new();
    let health = client.health().await?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&health)?);
    } else {
        println!("runtime_detected: {}", health.runtime_detected);
        println!("connectable: {}", health.connectable);
        println!(
            "transport: {}",
            health
                .transport
                .map(|t| format!("{t:?}"))
                .unwrap_or_else(|| "none".to_string())
        );

        if let Some(message) = health.message {
            println!("message: {}", message);
        }
    }

    Ok(())
}
```

---

# 21\. テスト方針

`runtime` は完全な実機依存にするとテスト不能になります。  
したがって、次の二層に分けます。

## 単体テスト

- `js_bridge` の script 文字列生成
- `protocol` の serde
- `probe` のレスポンス parse
- `mapper`

## 疑似結合テスト

- mock WebSocket サーバーを立てて CDP response を返す
- `send_request`
- `evaluate_string`

## 実機テスト

- ローカル限定
- ignored test
- `FIGMA_RUNTIME_TEST=1` のときだけ走らせる

この順にしないと、開発が進みません。

---

# 22\. 率直な評価

ここまでで `runtime` は、ようやく「設計された stub」から「本当に接続を埋め込める骨格」になりました。

ただし、本当の難所はまだ先です。

## 難所

- Figma Desktop の patch の実装
- debug endpoint の安定発見
- 正しい target の選定
- Figma ランタイム文脈で `figma` が見える場所へ到達すること
- バージョン差分への追従

つまり、**runtime の難しさは Rust 側のコード量より、外部環境の不安定さ**にあります。

---

# 23\. 次にやるべきこと

自然な順番は次です。

## A

`core::raw` を Auto Layout 情報込みに拡張する  
今のままだと layout 推定が弱いです。

## B

`probe` を `/json/list` ベースの target discovery に強化する  
今の `/json/version` 一本は弱いです。

## C

`reporter` を実装する  
人間が確認できる出力がないと、IR の妥当性が見えません。

順番としては **B** が最も接続の現実性に効きます。  
その次が **A** です。

## Prompt:

お願いします

## Response:

では、**B: `/json/list` ベースの target discovery 強化**を先に詰めます。  
ここを入れないと、runtime は「つながるかもしれない」段階のままで止まります。

---

# 1\. なぜ `/json/version` 一本では弱いか

`/json/version` の `webSocketDebuggerUrl` をそのまま使う方式は、単純ですが弱いです。  
理由は次です。

- その websocket が **Figma 本体の実行文脈**とは限らない
- 複数 target がある場合に選別できない
- 将来的に patch 実装を変えたときに柔軟性がない
- target の title / url / type を見て選べない

したがって、実運用を見据えるなら、

1.  `/json/version`
2.  `/json/list` または `/json`

の両方を見て、  
**候補 target を列挙し、選定規則で 1 つ選ぶ**  
という流れにした方がよいです。

---

# 2\. 目指す流れ

`probe_debug_endpoint` を、次の流れに変えます。

```
endpoint base
  -> GET /json/version
  -> GET /json/list
  -> targets を収集
  -> score を付ける
  -> best target を選ぶ
  -> websocket url を返す
```

---

# 3\. 追加する型

`crates/runtime/src/transport/probe.rs` を拡張します。

## 型定義

```
use serde::{Deserialize, Serialize};

use crate::RuntimeError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugEndpointInfo {
    pub http_base: String,
    pub websocket_url: String,
    pub title: Option<String>,
    pub target_id: Option<String>,
    pub target_type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeReport {
    pub runtime_detected: bool,
    pub endpoint_reachable: bool,
    pub websocket_discovered: bool,
    pub endpoint: Option<DebugEndpointInfo>,
    pub candidates: Vec<DebugEndpointInfo>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct VersionResponse {
    #[serde(rename = "webSocketDebuggerUrl")]
    websocket_debugger_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct TargetInfo {
    pub id: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub websocket_debugger_url: Option<String>,
    #[serde(rename = "type")]
    pub target_type: Option<String>,
}
```

---

# 4\. 候補 target の scoring

ここは単純なルールでよいです。  
複雑にすると説明不能になります。

## scoring ルールの初版

### 加点

- `target_type == "page"` → +0.35
- `title` に `figma` を含む → +0.25
- `url` に `figma` を含む → +0.20
- websocket url がある → +0.20

### 減点

- `target_type == "service_worker"` → -0.30
- `target_type == "other"` → -0.10
- websocket url が無い → 大幅減点

---

## 実装

```
fn score_target(target: &TargetInfo) -> f32 {
    let mut score = 0.0_f32;

    match target.target_type.as_deref() {
        Some("page") => score += 0.35,
        Some("iframe") => score += 0.10,
        Some("worker") => score -= 0.10,
        Some("service_worker") => score -= 0.30,
        Some("other") => score -= 0.10,
        _ => {}
    }

    if let Some(title) = &target.title {
        let lowered = title.to_ascii_lowercase();
        if lowered.contains("figma") {
            score += 0.25;
        }
        if lowered.contains("dev") {
            score += 0.03;
        }
    }

    if let Some(url) = &target.url {
        let lowered = url.to_ascii_lowercase();
        if lowered.contains("figma") {
            score += 0.20;
        }
        if lowered.contains("file") {
            score += 0.05;
        }
    }

    if target.websocket_debugger_url.is_some() {
        score += 0.20;
    } else {
        score -= 0.40;
    }

    score
}
```

---

# 5\. 候補の選定

`TargetInfo` の一覧から best match を取ります。

```
fn select_best_target(targets: &[TargetInfo]) -> Option<&TargetInfo> {
    targets.iter().max_by(|a, b| {
        score_target(a)
            .partial_cmp(&score_target(b))
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}
```

必要なら、あとで threshold を入れます。

---

# 6\. `probe_debug_endpoint` の改良版

これが実質の本体です。

```
pub async fn probe_debug_endpoint(
    endpoint_base: &str,
) -> Result<ProbeReport, RuntimeError> {
    let base = endpoint_base.trim_end_matches('/');

    let version_url = format!("{base}/json/version");
    let list_url = format!("{base}/json/list");

    let version_response = reqwest::get(&version_url)
        .await
        .map_err(|e| RuntimeError::ConnectFailed(format!("failed to reach debug endpoint: {e}")))?;

    if !version_response.status().is_success() {
        return Ok(ProbeReport {
            runtime_detected: true,
            endpoint_reachable: false,
            websocket_discovered: false,
            endpoint: None,
            candidates: vec![],
            message: Some(format!(
                "debug endpoint returned non-success status: {}",
                version_response.status()
            )),
        });
    }

    let version = version_response
        .json::<VersionResponse>()
        .await
        .map_err(|e| RuntimeError::InvalidResponse(format!("invalid version response: {e}")))?;

    let list_response = reqwest::get(&list_url)
        .await
        .map_err(|e| RuntimeError::ConnectFailed(format!("failed to fetch target list: {e}")))?;

    let targets = if list_response.status().is_success() {
        list_response
            .json::<Vec<TargetInfo>>()
            .await
            .map_err(|e| RuntimeError::InvalidResponse(format!("invalid target list response: {e}")))?
    } else {
        vec![]
    };

    let candidates = targets
        .iter()
        .filter_map(|t| {
            t.websocket_debugger_url.as_ref().map(|ws| DebugEndpointInfo {
                http_base: base.to_string(),
                websocket_url: ws.clone(),
                title: t.title.clone(),
                target_id: t.id.clone(),
                target_type: t.target_type.clone(),
                url: t.url.clone(),
            })
        })
        .collect::<Vec<_>>();

    let selected = select_best_target(&targets)
        .and_then(|t| {
            t.websocket_debugger_url.as_ref().map(|ws| DebugEndpointInfo {
                http_base: base.to_string(),
                websocket_url: ws.clone(),
                title: t.title.clone(),
                target_id: t.id.clone(),
                target_type: t.target_type.clone(),
                url: t.url.clone(),
            })
        })
        .or_else(|| {
            version.websocket_debugger_url.as_ref().map(|ws| DebugEndpointInfo {
                http_base: base.to_string(),
                websocket_url: ws.clone(),
                title: None,
                target_id: None,
                target_type: None,
                url: None,
            })
        });

    Ok(ProbeReport {
        runtime_detected: true,
        endpoint_reachable: true,
        websocket_discovered: selected.is_some(),
        endpoint: selected,
        candidates,
        message: None,
    })
}
```

---

# 7\. `doctor` で候補一覧を見られるようにする

接続が怪しいとき、  
「どの target を見つけ、どれを選んだか」  
が見えないと調査ができません。

したがって `doctor --json` では `ProbeReport.candidates` まで出した方がよいです。

今の `RuntimeHealth` だけでは情報が薄いので、`doctor` 内では `probe_debug_endpoint` を直接叩いてもよいです。

---

## CLI 側イメージ

```
use anyhow::Result;
use serde_json::json;

use crate::args::DoctorArgs;

pub async fn run(args: DoctorArgs) -> Result<()> {
    let client = figma_ir_runtime::PatchedCdpRuntimeClient::new();
    let health = client.health().await?;

    let probe = figma_ir_runtime::transport::probe::probe_debug_endpoint("http://127.0.0.1:9222")
        .await
        .ok();

    if args.json {
        let payload = json!({
            "health": health,
            "probe": probe,
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        println!("runtime_detected: {}", health.runtime_detected);
        println!("connectable: {}", health.connectable);

        if let Some(probe) = probe {
            println!("candidate_count: {}", probe.candidates.len());
            if let Some(endpoint) = probe.endpoint {
                println!(
                    "selected_target: {}",
                    endpoint.title.unwrap_or_else(|| "<unknown>".to_string())
                );
                println!("selected_ws: {}", endpoint.websocket_url);
            }
        }

        if let Some(message) = health.message {
            println!("message: {}", message);
        }
    }

    Ok(())
}
```

---

# 8\. 次に A: Auto Layout 情報を `core::raw` に足す

これをやらないと `extractor` の layout 推定がいつまでも弱いです。  
したがって、今の段階で `RawNode` を拡張した方がよいです。

---

## `core/src/raw.rs` 改修案

```
use crate::diagnostic::Diagnostic;
use crate::ids::{FrameId, NodeId};
use crate::source::SourceRef;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RawNodeType {
    Frame,
    Group,
    Rectangle,
    Text,
    Vector,
    Component,
    Instance,
    Ellipse,
    Line,
    Polygon,
    Star,
    BooleanOperation,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RawLayoutMode {
    None,
    Horizontal,
    Vertical,
    Grid,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawAutoLayout {
    pub mode: RawLayoutMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<[f64; 4]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawTextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawNode {
    pub id: NodeId,
    pub name: String,
    pub node_type: RawNodeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<NodeId>,
    pub children: Vec<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<RawRect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fills: Option<Vec<RawColor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style: Option<RawTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_layout: Option<RawAutoLayout>,
    #[serde(default)]
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawEdge {
    pub parent: NodeId,
    pub child: NodeId,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RawFrameSnapshot {
    pub source: SourceRef,
    pub root_frame_id: FrameId,
    pub nodes: Vec<RawNode>,
    pub edges: Vec<RawEdge>,
    pub diagnostics: Vec<Diagnostic>,
}
```

---

# 9\. runtime models 側も揃える

`runtime/models.rs` も合わせます。

```
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeFrameRequest {
    pub frame_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeNodeSnapshot {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub parent_id: Option<String>,
    pub children: Vec<String>,

    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,

    pub visible: bool,

    pub fills: Vec<RuntimeColor>,
    pub radius: Option<f64>,

    pub text: Option<String>,
    pub text_style: Option<RuntimeTextStyle>,

    pub layout_mode: Option<String>,
    pub item_spacing: Option<f64>,
    pub padding_top: Option<f64>,
    pub padding_right: Option<f64>,
    pub padding_bottom: Option<f64>,
    pub padding_left: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeTextStyle {
    pub font_family: Option<String>,
    pub font_weight: Option<u16>,
    pub font_size: Option<f64>,
    pub line_height: Option<f64>,
    pub letter_spacing: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeFrameSnapshot {
    pub frame_id: String,
    pub frame_name: String,
    pub nodes: Vec<RuntimeNodeSnapshot>,
}
```

---

# 10\. mapper に Auto Layout 変換を足す

`runtime/mapper.rs` の `RawNode` 生成時に、`auto_layout` を入れます。

```
use chrono::Utc;
use figma_ir_core::{
    Diagnostic, FrameId, NodeId, RawAutoLayout, RawColor, RawEdge, RawFrameSnapshot, RawLayoutMode,
    RawNode, RawNodeType, RawRect, RawTextStyle, SessionId, SessionInfo, SourceRef, TransportKind,
};

use crate::{RuntimeFrameSnapshot, RuntimeSession};

pub fn map_runtime_snapshot_to_raw(
    session: &RuntimeSession,
    snapshot: RuntimeFrameSnapshot,
) -> RawFrameSnapshot {
    let nodes = snapshot
        .nodes
        .iter()
        .map(|n| RawNode {
            id: NodeId(n.id.clone()),
            name: n.name.clone(),
            node_type: map_node_type(&n.node_type),
            parent_id: n.parent_id.clone().map(NodeId),
            children: n.children.iter().cloned().map(NodeId).collect(),
            bounds: match (n.x, n.y, n.width, n.height) {
                (Some(x), Some(y), Some(width), Some(height)) => Some(RawRect {
                    x,
                    y,
                    width,
                    height,
                }),
                _ => None,
            },
            fills: if n.fills.is_empty() {
                None
            } else {
                Some(
                    n.fills
                        .iter()
                        .map(|c| RawColor {
                            r: c.r,
                            g: c.g,
                            b: c.b,
                            a: c.a,
                        })
                        .collect(),
                )
            },
            radius: n.radius,
            text: n.text.clone(),
            text_style: n.text_style.as_ref().map(|t| RawTextStyle {
                font_family: t.font_family.clone(),
                font_weight: t.font_weight,
                font_size: t.font_size,
                line_height: t.line_height,
                letter_spacing: t.letter_spacing,
            }),
            auto_layout: Some(RawAutoLayout {
                mode: map_layout_mode(n.layout_mode.as_deref()),
                item_spacing: n.item_spacing,
                padding: match (
                    n.padding_top,
                    n.padding_right,
                    n.padding_bottom,
                    n.padding_left,
                ) {
                    (Some(top), Some(right), Some(bottom), Some(left)) => {
                        Some([top, right, bottom, left])
                    }
                    _ => None,
                },
            }),
            visible: n.visible,
        })
        .collect::<Vec<_>>();

    let edges = snapshot
        .nodes
        .iter()
        .flat_map(|n| {
            n.children
                .iter()
                .enumerate()
                .map(|(index, child)| RawEdge {
                    parent: NodeId(n.id.clone()),
                    child: NodeId(child.clone()),
                    index,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    RawFrameSnapshot {
        source: SourceRef {
            file_key: None,
            frame_id: FrameId(snapshot.frame_id.clone()),
            frame_name: snapshot.frame_name,
            session: SessionInfo {
                id: SessionId(session.id.0.clone()),
                transport: TransportKind::DesktopPatchedCdp,
                captured_at: Utc::now(),
                figma_version: session.figma_version.clone(),
                platform: session.platform.clone(),
            },
        },
        root_frame_id: FrameId(snapshot.frame_id),
        nodes,
        edges,
        diagnostics: vec![Diagnostic {
            level: figma_ir_core::DiagnosticLevel::Info,
            code: "RUNTIME_SNAPSHOT_CAPTURED".to_string(),
            message: "raw snapshot captured from runtime".to_string(),
            node_id: None,
        }],
    }
}

fn map_node_type(value: &str) -> RawNodeType {
    match value {
        "FRAME" => RawNodeType::Frame,
        "GROUP" => RawNodeType::Group,
        "RECTANGLE" => RawNodeType::Rectangle,
        "TEXT" => RawNodeType::Text,
        "VECTOR" => RawNodeType::Vector,
        "COMPONENT" => RawNodeType::Component,
        "INSTANCE" => RawNodeType::Instance,
        "ELLIPSE" => RawNodeType::Ellipse,
        "LINE" => RawNodeType::Line,
        "POLYGON" => RawNodeType::Polygon,
        "STAR" => RawNodeType::Star,
        "BOOLEAN_OPERATION" => RawNodeType::BooleanOperation,
        _ => RawNodeType::Unknown,
    }
}

fn map_layout_mode(value: Option<&str>) -> RawLayoutMode {
    match value {
        Some("NONE") => RawLayoutMode::None,
        Some("HORIZONTAL") => RawLayoutMode::Horizontal,
        Some("VERTICAL") => RawLayoutMode::Vertical,
        Some("GRID") => RawLayoutMode::Grid,
        _ => RawLayoutMode::Unknown,
    }
}
```

---

# 11\. extractor の layout 抽出を直す

これで `extractor` は、名前推定ではなく **実データ** ベースで layout を取れます。

`extractor/src/extract.rs` の `extract_layout_features` を修正します。

```
fn extract_layout_features(node: &RawNode) -> LayoutFeatures {
    if let Some(auto) = &node.auto_layout {
        let (kind, direction) = match auto.mode {
            figma_ir_core::RawLayoutMode::None => (LayoutKind::None, None),
            figma_ir_core::RawLayoutMode::Horizontal => (LayoutKind::Stack, Some(Axis::Horizontal)),
            figma_ir_core::RawLayoutMode::Vertical => (LayoutKind::Stack, Some(Axis::Vertical)),
            figma_ir_core::RawLayoutMode::Grid => (LayoutKind::Grid, None),
            figma_ir_core::RawLayoutMode::Unknown => (LayoutKind::Unknown, None),
        };

        return LayoutFeatures {
            kind,
            direction,
            gap: auto.item_spacing,
            padding: auto.padding,
        };
    }

    let (kind, direction) = infer_layout(node);

    LayoutFeatures {
        kind,
        direction,
        gap: None,
        padding: None,
    }
}
```

この差はかなり大きいです。

---

# 12\. 現時点の見立て

ここまで入ると、設計としてはかなりまともになります。

## 良くなった点

- endpoint discovery が `/json/list` を見られる
- target 選択の余地がある
- `doctor` で候補一覧を見られる
- Raw に Auto Layout 情報が入る
- extractor の layout 推定が実データベースになる

## まだ弱い点

- Figma プロセス自体の検出をしていない
- port の自動探索がない
- auth token がない
- target score がまだ粗い
- patch の適用 / 復旧は未設計
- WebSocket request timeout が未実装

つまり、**土台としては筋が通ったが、実運用にはまだ足りない**段階です。

---

# 13\. 次にやるべきこと

ここから先は 3 方向あります。

## A

**request timeout と再接続制御**  
runtime の最低限の堅牢化

## B

**Figma プロセス検出と port discovery**  
本当に attach できるようにする

## C

**reporter 実装**  
今ある IR が人間に見て妥当か確認できるようにする

順番としては、接続を急ぐなら **A → B**、設計検証を急ぐなら **C** です。

今の流れなら、次は **A: request timeout と再接続制御** を詰めるのが自然です。

## Prompt:

完全なものを徹底的に仕上げてください

## Response:

思考時間: 33s

以下で、**このプロジェクトを実際に仕上げるための最終設計**を固定します。  
前提は変えません。

- **Figma REST API は使わない**
- **Figma plugin も使わない**
- **Figma Desktop 実行環境への接続だけを使う**
- **配布骨格は `sample-cli` を使う**
- **出力の中心はコードではなく UI IR**
- **コード生成は後段に分離する**

`sample-cli` は現在も、Rust コアと Rust バイナリ、npm ランチャ、プラットフォーム別パッケージを分けたモノレポ構成で、`mise`・`pnpm` 前提の配布テンプレートとして整理されています。 [GitHub](https://github.com/taiga-tech/sample-cli)  
`figma-cli` は現在も、Figma Desktop への直接接続系 CLI として公開されており、接続発想の参考元として扱えます。 [GitHub](https://github.com/silships/figma-cli)

---

# 最終設計

## プロダクトの定義

これは **Figma CLI** ではなく、**Figma Runtime Extractor** です。  
責務は 4 つに固定します。

1.  **Figma Desktop に接続する**
2.  **指定フレームを観測する**
3.  **実装向け UI IR に正規化する**
4.  **JSON とレポートを出す**

これ以外は本体に入れません。  
作成、更新、画像 export、任意 eval、Storybook 生成、JSX 生成は別段に分けます。

---

# 最終ディレクトリ構成

`sample-cli` の配布構成に合わせつつ、ドメイン側はこう切ります。`sample-cli` 側の現行構成も、`crates/core`, `crates/cli`, `packages/cli`, `packages/cli-<platform>` を軸にしています。 [GitHub](https://github.com/taiga-tech/sample-cli)

```
.
├── Cargo.toml
├── rust-toolchain.toml
├── mise.toml
├── package.json
├── pnpm-workspace.yaml
├── turbo.json
│
├── crates/
│   ├── core/           # 型、IR、raw/features、diagnostics
│   ├── cli/            # clap で CLI
│   ├── runtime/        # Desktop 接続、CDP、probe、js_bridge
│   ├── extractor/      # RawFrameSnapshot -> FeatureGraph
│   ├── normalizer/     # FeatureGraph -> UiIr draft
│   ├── classifier/     # component/atomic/reusability 推定
│   ├── reporter/       # UiIr -> report.md
│   ├── schema/         # JSON Schema 出力
│   └── store/          # JSON 保存、将来の SQLite 保存
│
├── packages/
│   ├── cli/
│   ├── cli-darwin-arm64/
│   ├── cli-darwin-x64/
│   ├── cli-win32-x64/
│   ├── cli-linux-x64-gnu/
│   └── cli-linux-x64-musl/
│
├── schemas/
│   ├── raw.schema.json
│   ├── features.schema.json
│   └── ui.ir.schema.json
│
├── fixtures/
│   ├── raw/
│   ├── features/
│   └── ui-ir/
│
└── scripts/
    └── place-binary-like-ci.sh
```

---

# データモデル

## 1\. Raw

これは接続層の取得結果です。  
意味づけしません。

```
RuntimeFrameSnapshot
  -> map_runtime_snapshot_to_raw
RawFrameSnapshot
  - nodes
  - edges
  - text
  - fills
  - bounds
  - auto_layout
```

ここで **Auto Layout 情報は必須** です。  
これがないと downstream が弱くなります。

---

## 2\. Features

これは `extractor` の出力です。  
観測可能な特徴だけを持ちます。

```
FeatureGraph
  - structure
    - depth
    - child_count
    - sibling_index
    - subtree_size
  - layout
    - kind
    - direction
    - gap
    - padding
  - visual
    - has_fill
    - has_radius
    - has_shadow
    - has_image_fill
    - has_vector
  - text
    - is_textual
    - raw_text
    - content_length
    - line_count_estimate
  - repetition
    - fingerprint
    - repeated_subtree_count
```

`extractor` は **分類しない**。  
Button や Card をここで決めない。  
それをやると責務が崩れます。

---

## 3\. UiIr

これは本体です。  
`ui.ir.json` は単なる JSON ではなく、**意味付き中間表現**です。

最低限、各 `UiNode` はこれを持ちます。

- `id`
- `name`
- `kind`
- `source_node_ids`
- `layout`
- `style_refs`
- `semantics`
- `content`
- `children`
- `component_candidates`
- `atomic_candidates`
- `confidence`
- `reusability`
- `diagnostics`

このうち、後から削ってよいものはほとんどありません。

---

# CLI の完成形

表面コマンドはこれで固定します。

```
figma-ir attach
figma-ir doctor
figma-ir inspect frame --id 123:456
figma-ir extract frame --id 123:456 --out raw.json
figma-ir features frame --id 123:456 --out features.json
figma-ir normalize frame --id 123:456 --out ui.ir.json
figma-ir report frame --id 123:456 --out report.md
```

これ以上は増やしません。  
この CLI は抽出器であって、万能ツールではありません。

---

# runtime の完成形

## 責務

`runtime` は次だけを外へ見せます。

- `attach`
- `health`
- `snapshot_frame`

公開 API はこれで十分です。

```
#[async_trait]
pub trait RuntimeClient: Send + Sync {
    async fn attach(&self) -> Result<RuntimeSession, RuntimeError>;
    async fn health(&self) -> Result<RuntimeHealth, RuntimeError>;
    async fn snapshot_frame(
        &self,
        request: RuntimeFrameRequest,
    ) -> Result<RuntimeFrameSnapshot, RuntimeError>;
}
```

`eval(String)` は公開しません。  
内部実装だけに閉じます。

---

## transport 分離

`runtime` の下はこう分けます。

```
runtime/
└── transport/
    ├── patched_cdp.rs
    ├── js_bridge.rs
    ├── protocol.rs
    └── probe.rs
```

### `patched_cdp.rs`

- WebSocket 接続
- `Runtime.enable`
- `Runtime.evaluate`
- response 待機
- request id 管理

### `js_bridge.rs`

- Figma ランタイムに流す JavaScript 生成
- `build_runtime_ping_script`
- `build_snapshot_frame_script`

### `protocol.rs`

- CDP request/response 型
- `RuntimeEvaluateParams`
- `RuntimeEvaluateResult`

### `probe.rs`

- `/json/version`
- `/json/list`
- target 列挙
- target scoring
- best target 選択

---

## target discovery

`/json/version` のみでは不十分です。  
`/json/list` を併用し、target に score を付けて選びます。

初版の規則はこれでよいです。

- `type == page` を優遇
- `title` に `figma` が含まれる target を優遇
- `url` に `figma` が含まれる target を優遇
- websocket url が無い target は落とす

この選定結果を `doctor --json` で見えるようにします。

---

## runtime で未実装だが必要なもの

ここは明確です。

### 今後必ず入れる

- request timeout
- reconnect
- port discovery
- auth token
- Figma プロセス検出
- target selection の強化

### 本体に入れない

- patch 処理
- patch 復旧
- バージョン別 patch 差分

patch は別 crate に切る方がよいです。  
理由は、接続層と破壊的変更処理の責務が違うからです。

---

# extractor の完成形

## 役割

`extractor` は、`RawFrameSnapshot -> FeatureGraph` を行います。

### やること

- depth 計算
- subtree_size 計算
- sibling_index 計算
- text feature 抽出
- visual feature 抽出
- Auto Layout feature 抽出
- subtree fingerprint 計算
- repeated_subtree_count 集計

### やらないこと

- Button 判定
- Card 判定
- Atomic 判定
- component naming

---

## subtree fingerprint

fingerprint は初版では粗くて構いません。  
ただし、**子の fingerprint を再帰的に含める**ことだけは譲れません。  
ここがないと repeated pattern が機能しません。

---

# normalizer の完成形

ここが最重要です。  
価値の中心は接続ではなく、**normalizer の質**にあります。

## normalizer の責務

1.  **木の再構築**
2.  **layout-only wrapper collapse**
3.  **boundary detection**
4.  **UiNode への変換**

---

## boundary detector

boundary は単一ルールで決めません。  
複数 signal の合議にします。

### signal

- `visual_group`
- `repetition`
- `layout`
- `text_semantic`
- `explicit_name`

### output

- `signals`
- `total_score`
- `accepted`

この判定結果は diagnostics に残します。

---

## wrapper collapse

collapse 対象は次です。

- child が 1 つ
- text がない
- visual 特徴が弱い
- repeated pattern に関与しない
- `wrapper`, `container`, `frame`, `layout`, `group` 的な名前

ただし、**repetition 候補は collapse しない**。  
ここを潰すと list item 候補が死にます。

---

## 再帰変換

`build_ui_node` はこの責務だけを持ちます。

- 現在ノードを 1 UI ノードへ変換
- 子を正規化
- wrapper なら grandchildren を持ち上げる
- boundary accepted を kind 推定に反映

まだ sibling grouping は入れません。  
初版では node-centered で十分です。

---

# classifier の完成形

`classifier` は **推定器** です。  
断定器ではありません。

## 役割

- `component_candidates`
- `atomic_candidates`
- `reusability`

を `UiNode` ごとに埋めます。

---

## component naming

命名は単純なルールベースでよいです。

### 規則

- `name` を PascalCase 化
- `kind` に応じて suffix
- `Card`, `Button`, `Section`, `ListItem` を優先
- secondary candidate も出す

### 例

- `Checkout Button` -> `CheckoutButton`
- `Order Summary` + `Card` -> `OrderSummaryCard`
- `Filter Section` -> `FilterSection`

---

## atomic classification

候補は複数出します。

### atom 候補

- Button
- Input
- Checkbox
- Radio
- Switch
- Icon
- Badge
- Heading
- Text

### molecule 候補

- Card
- ListItem
- FieldGroup

### organism 候補

- Section
- Navigation
- Form
- List

### template / page

- Template
- Screen

`Unknown` も候補として残します。

---

## reusability

再利用性は点数化します。

### 加点対象

- primitive UI pattern
- compact child structure
- semantic name hint
- portable textual content

### 減点対象

- top-level layout
- domain-specific name
- excessive contextuality

この score は、後でコンポーネント切り出しの優先順位に使えます。

---

# reporter の完成形

これはかなり重要です。  
人間が確認できないと、IR の価値は半減します。

## 出力

`report.md`

## 含める内容

### 概要

- frame id
- frame name
- node count
- normalized node count
- collapsed wrapper count

### 構造の要約

- Screen
    - Section
    - Card
    - List
    - ListItem
    - Button

### 低信頼箇所

- boundary accepted だが score が低い
- unknown layout
- atomic ambiguity
- low confidence component name

### 再利用性上位

- score 上位 10 件
- component candidate
- atomic candidate
- reason

### 推奨の実装順

- reusable primitive
- repeated list item
- card-like composite
- section-like composite

---

## reporter の方針

Markdown は機械向けではなく、人間向けです。  
JSON をそのまま羅列しない。  
要約して書く。

---

# store の完成形

初期は JSON 保存のみ。  
将来的に SQLite を足せるようにします。

## 保存対象

- `raw.json`
- `features.json`
- `ui.ir.json`
- `report.md`

## その後

- `analysis.db`

SQLite を入れるなら、少なくとも以下のテーブルを持てます。

- `nodes`
- `edges`
- `style_clusters`
- `repetition_groups`
- `component_candidates`
- `diagnostics`

ただし、初期段階では後回しでよいです。

---

# JSON Schema の完成形

`ui.ir.schema.json` は固定して運用します。  
Rust 型から `schemars` で出力しつつ、互換性を気にします。

変更時の扱いは次です。

- 後方互換が壊れない追加 → minor
- 破壊的変更 → major
- diagnostics や optional field 追加 → minor

`version` は `UiIr` 本体にも入れます。

---

# テスト戦略

## 1\. unit test

対象:

- `extractor`
- `normalizer`
- `classifier`
- `mapper`
- `protocol`
- `probe`

---

## 2\. fixture test

`fixtures/` に

- `raw/*.json`
- `features/*.json`
- `ui-ir/*.json`

を置きます。

期待するのは **snapshot test** です。  
同じ入力で出力がぶれないことを保証します。

---

## 3\. runtime mock test

mock WebSocket サーバーを立てて、

- `Runtime.enable`
- `Runtime.evaluate`

への response を返し、

- `send_request`
- `evaluate_string`
- `snapshot_frame`

を検証します。

---

## 4\. 実機テスト

実機依存は ignored test にします。

- `FIGMA_RUNTIME_TEST=1`
- `cargo test -- --ignored`

このときだけ実際の runtime を叩くようにします。

---

# 実装順序

ここから先を本当に仕上げる順番です。

## 第1段階

runtime を接続可能な状態まで持っていく

- `/json/list` discovery
- `Runtime.enable`
- `Runtime.evaluate`
- ping
- snapshot frame

## 第2段階

raw に Auto Layout を入れて extractor を安定化

- `auto_layout`
- text raw content
- subtree fingerprint
- repeated counts

## 第3段階

normalizer の品質を上げる

- wrapper collapse
- boundary detection
- node recursion

## 第4段階

classifier を固定

- component naming
- atomic candidates
- reusability
- diagnostics

## 第5段階

reporter を入れる

- human-readable report
- implementation priority
- low-confidence surfacing

## 第6段階

fixture と snapshot test を増やす

- button-heavy frame
- card/list-heavy frame
- dashboard-like frame
- form-like frame

---

# ここで本当に価値が出る部分

率直に言うと、価値が出るのは次の 3 箇所です。

## 1\. wrapper collapse

ここが弱いと出力がノイズだらけになります。

## 2\. boundary detection

ここが弱いと component 分割が信用されません。

## 3\. diagnostics

ここが弱いと、間違ったときに人間が直せません。

逆に、runtime が多少荒くても、この 3 つが強ければ使い道があります。

---

# いま切るべきもの

本体から外した方がよいものを明示します。

- patch 処理
- patch 復旧
- arbitrary eval の公開
- コード生成
- Storybook 生成
- React 出力
- デザイントークン命名の完成版
- LLM ベースの全面分類

このあたりを先に入れると、設計が崩れます。

---

# 最終判断

このプロジェクトは、**Figma からコードを直接吐く道具**として作るより、  
**Figma から実装判断に耐える IR を引き抜く道具**として作る方が筋が通っています。

そのために必要な完成形は、次の 1 本です。

```
Desktop runtime
  -> RawFrameSnapshot
  -> FeatureGraph
  -> UiIr draft
  -> UiIr classified
  -> report.md
```

これを先に完成させる。  
その後で、別 crate として

- `projector-react`
- `projector-html`
- `projector-storybook`

のような投影層を足す。  
この順序が最も壊れにくいです。

次は、ここまでの内容を前提にした **実際のリポジトリ雛形一式** をまとめて出すのがよいです。  
具体的には、各 crate の `Cargo.toml`、`lib.rs`、`main.rs`、`args.rs`、`runtime` の transport ファイル群、`extractor` / `normalizer` / `classifier` のソースを一つの塊として整理できます。

---

Powered by [ChatGPT Exporter](https://www.chatgptexporter.com)
