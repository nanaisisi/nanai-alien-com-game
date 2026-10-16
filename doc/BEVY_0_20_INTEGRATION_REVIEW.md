# Bevy 0.20 仕様照合・技術検討書 (Bevy 0.20 Integration & Architecture Review)

本書は、ゲームエンジン [Bevy 0.20 (2026年10月8日リリース)](https://bevyengine.org/news/bevy-0-20/) の新機能・設計パラダイムの変更点を整理し、本プロジェクト（`nanai-alien-com-game`）における現行実装の適合状況、および今後の拡張方針を検討・記録した技術設計ドキュメントです。

---

## 1. Bevy 0.20 の主要なアップデート一覧

| 機能 / カテゴリ | 概要 | プロジェクトへの影響度 |
| :--- | :--- | :---: |
| **Weak System Ordering (`chain_weak`, `before_weak`, `after_weak`)** | 従来の `.chain()` が先行タスクの完全終了を待つ「強い直列化」であったのに対し、データアクセス競合（Read/Write）のあるシステム間でのみ順序を拘束し、非競合システムは並列稼働させる柔軟な順序付け。 | **高** (ターン処理・UI更新の並列性向上) |
| **UI: `Val::Em` / `Val::Rem` & フォントスケーリング** | CSSライクな相対単位の公式導入。現在ノードのフォントサイズ依存 (`Em`)、およびルートフォントサイズ依存 (`Rem`) による動的レスポンシブスケーリング。 | **高** (4K/FHD/ポータブル機UIスケーリング) |
| **UI: `FixedNode` (独立ビューポート配置)** | 親要素のレイアウト計算・クリッピング・Transform をバイパスし、対象カメラのビューポートを基準に絶対配置するマーカーコンポーネント。 | **中** (全画面オーバーレイ/モーダル/HUD) |
| **Feathers / Contextual Theming & 新規ウィジェット** | 親エンティティの階層文脈（`Base`, `Higher`, `Floating` 等）に応じた動的テーマ割り当て (`ThemeContext`, `SemanticToken`)。タブやカラーピッカー等の新UIパーツ。 | **中** (設定画面・百科事典UIの共通化) |
| **Pan Orbit Camera の標準内蔵** | CADやBlenderライクに注視点（Pivot）を中心に回転・パン・ズームする公式カメラコンポーネント。 | **低〜中** (戦略カメラとの比較検討) |
| **BSN (Bevy Scene) 構文刷新 & `Ready` Event** | `@scene` 接頭辞の明示的統一、およびエンティティツリー展開完了をフックできる Observable `Ready` イベント。 | **中** (将来的なシーン定義・プリセット配置) |
| **WESL Shaders & 2D Extended Materials** | WGSLの拡張標準である WESL の採用。Sprite マテリアルのカスタムシェーダー化や 2D Extended Material のサポート。 | **中** (ヘックス境界エフェクト・瘴気描画) |
| **Solari パストレーシング & DLSS-RR 4.5** | リアルタイムパストレーシング Solari の性能向上・Metal (macOS) 対応・DLSS-RR 4.5 統合。 | **低** (現行の軽量Hex 3Dレンダリングでは保留) |
| **Schedule Randomization** | システムの実行順序をランダムにシャッフルし、暗黙の依存関係やデータ競合バグを炙り出すデバッグ・検証機能。 | **中** (CI/ローカルテストでの品質向上) |

---

## 2. 現行コードベースにおける適合状況 (Status Quo)

本プロジェクトは `Cargo.toml` において `bevy = "0.20.0"` を採用しており、すでに主要な 0.20 API に適合しています。

### 適合確認済みの領域

1. **ECS メッセージング API (`MessageWriter` / `MessageReader`)**
   - 従来の `EventWriter` / `EventReader` から Bevy 0.20 推奨の `bevy::ecs::message::MessageWriter<'w, AppExit>` や `MessageReader<MouseWheel>` への移行が完了しています。
   - 参照: `src/ui/title/interaction.rs`, `src/ui/pause/interaction.rs`, `src/camera.rs`
2. **UI Widgets & Picking コンポーネント**
   - 従来の単一 `Interaction` enum 依存から脱却し、`bevy::ui_widgets::Button` および Picking 状態コンポーネント (`Option<&Hovered>`, `Option<&Pressed>`) を用いた疎結合な判定が行われています。
   - 参照: `src/ui/title/interaction.rs`, `src/ui/pause/interaction.rs`
3. **フラットな UI コンポーネント構造**
   - `NodeBundle` や `TextBundle` を廃止し、Bevy 0.20 準拠の個別コンポーネント（`Node`, `Text`, `TextFont`, `TextColor`, `BackgroundColor`, `BorderColor`）を直接 spawn する構造に完全移行しています。
   - 参照: `src/ui/title/view.rs`, `src/ui/pause/view.rs`
4. **メッシュプリミティブ・ビルダー API**
   - 六角柱のプロシージャル生成において、`Cylinder { radius, half_height }.mesh().resolution(6).build()` 構文を使用しています。
   - 参照: `src/map/generation.rs`

---

## 3. 具体的な技術検討と採用判断

### 3.1 スケジューリング: `chain_weak` による並列処理の最大化

#### 3.1.1 課題と背景

ゲームループやUI制御で従来の `.chain()` を多用すると、実質的なデータ競合（Read/Write コンフリクト）が存在しないシステム同士であっても前のシステムが完了するまで次のシステムが開始されず、マルチコア環境でスレッド待機（CPUアイドル）が発生します。

#### 3.1.2 適用方針

- **ターン進行システム**: リソース再計算、タイル領有の更新、ユニット移動、AI意思決定などのうち、依存関係が独立しているフェーズ間には `.chain_weak()` を適用し、並列実行スループットを向上させます。
- **UIステート更新**: キーボードナビゲーション、ボタンインタラクション、ハイライト色更新の順序保証において、競合のない処理を並列化します。

### 3.2 UIレイアウト: `Val::Em` / `Val::Rem` と `FixedNode` の導入

#### 3.2.1 課題と背景

現在 [src/ui/theme.rs](../src/ui/theme.rs) や各種 View では `Val::Px(16.0)`, `Val::Px(32.0)` 等の絶対ピクセル指定が多く、高DPIディスプレイ（4K）や携帯機（Steam Deck等）で文字やボタンのバランスが崩れやすい課題があります。

#### 3.2.2 適用方針

1. **`Val::Em` / `Val::Rem` によるUI比率の標準化**:
   - ボタンの内部パディングやアイコン間隔、行間（`row_gap`）をフォント基準（例: `Val::Em(0.5)`, `Val::Em(1.2)`）に再設計。
   - 設定画面の「UIスケール倍率」変更時に、ベースフォントサイズ（`RemSize`）を変更するだけで画面全体が破綻なく追従するようにリファクタリング。
2. **`FixedNode` によるモーダル・HUDの切り離し**:
   - 現在 [src/ui/pause/view.rs](../src/ui/pause/view.rs) の確認モーダル（`PauseConfirmModal`）やタイトルUIは親コンテナのレイアウト制約下にあります。
   - `FixedNode` を適用することで、カメラビューポート基準で最前面に完全固定配置し、親要素の Flexbox 配置やパディングの影響を受けない堅牢なオーバーレイを実現します。

### 3.3 カメラ制御: 独自 `MapCamera` vs 標準 `Pan Orbit Camera`

#### 比較検討

| 項目 | Bevy 0.20 標準 `Pan Orbit Camera` | 当プロジェクト `MapCamera` (現行) |
| :--- | :--- | :--- |
| **設計思想** | 単一オブジェクト（3Dモデル/CAD）を中心とした自由周回 | 4X/Civ型 広域マップの俯瞰スクロール・戦術見下ろし |
| **X軸円筒ラップ** | 非対応（標準の3D球面座標系） | **完全対応**（円筒世界端のシームレスワープ対応） |
| **移動制約** | 焦点（Pivot）への注視維持が前提 | WASD・矢印キー・画面端マウス・ドラッグによる平滑平面移動 |
| **ズーム挙動** | 焦点距離の縮小 | 正射投影/透視投影の視野高（Viewport Height）滑らか補間 |

#### 結論

当ゲーム特有の **「円筒形ラップ（Cylindrical Overworld Hex Map）」** や戦略ゲーム特有のパン操作を正確にハンドリングするため、汎用の Pan Orbit Camera への置換は行わず、**現行の独自 `MapCamera` を維持・拡張することが最適**であると判断します。

### 3.4 シェーダー標準: WESL への移行準備

#### 適用余地

現在、六角形タイルの二重境界線（Civilization風のツートン領土境界）やテクスチャブレンドは標準の `StandardMaterial` で描画しています。
将来的に計画されている「瘴気（ミズマ）の流体拡散」や「バイオーム境界の有機的ブレンディング」の実装時には、Bevy 0.20 で正式採用された **WESL** を用いたカスタムマテリアルパイプラインを構築します。

---

## 4. 今後のロードマップとアクションアイテム

- [ ] **Phase 1: UIレスポンシブ化検証**
  - [src/ui/pause/view.rs](../src/ui/pause/view.rs) のモーダルに `FixedNode` を導入し挙動確認。
  - [src/ui/theme.rs](../src/ui/theme.rs) における余白定義の `Val::Em` 化。
- [ ] **Phase 2: スケジューラの弱順序化**
  - 主要システムチェーンにおける `.chain()` から `.chain_weak()` への段階的移行とプロファイリング。
- [ ] **Phase 3: CI/テストへの Schedule Randomization 導入**
  - 暗黙のシステム実行順依存に起因する非決定論的バグの早期検知。
