# install コマンド ステートマシン図

## 概要

`an install <target>` コマンドの状態遷移を定義します。

## 状態一覧

| 状態 | 説明 |
|------|------|
| Start | 初期状態 |
| ParseInput | 入力解析 |
| LocalFile | ローカルファイル処理分岐 |
| RemoteName | リモートアプリ名処理分岐 |
| InvalidInput | 無効な入力 |
| ProcessDeb | .deb処理 |
| ProcessAppImage | AppImage処理 |
| FetchToml | TOML情報取得 |
| ShowUrl | URL表示・確認 |
| Confirm | ユーザー確認 |
| Download | ダウンロード |
| Success | 成功終了 |
| Error | エラー終了 |
| Cancel | キャンセル終了 |

## 状態遷移図

```
                           ┌───────────┐
                           │   Start   │
                           └─────┬─────┘
                                 │
                           ┌─────▼─────┐
                           │ParseInput │
                           └─────┬─────┘
                                 │
             ┌───────────────────┼───────────────────┐
             │                   │                   │
       ┌─────▼─────┐       ┌─────▼─────┐       ┌─────▼─────┐
       │ LocalFile │       │RemoteName │       │InvalidInput│
       └─────┬─────┘       └─────┬─────┘       └─────┬─────┘
             │                   │                   │
    ┌────────┴────────┐          │             ┌─────▼─────┐
    │                 │          │             │   Error   │
┌───▼───┐       ┌─────▼─────┐    │             │  (E002)   │
│  .deb │       │ .AppImage │    │             └───────────┘
└───┬───┘       └─────┬─────┘    │
    │                 │          │
┌───▼───┐       ┌─────▼─────┐  ┌─▼─────────┐
│Process│       │  Process  │  │ FetchToml │
│  Deb  │       │ AppImage  │  └─────┬─────┘
└───┬───┘       └─────┬─────┘        │
    │                 │              │
    │           ┌─────▼─────┐  ┌─────▼─────┐
    │           │  Create   │  │  ShowUrl  │
    │           │  Symlink  │  └─────┬─────┘
    │           └─────┬─────┘        │
    │                 │        ┌─────▼─────┐
    │                 │        │  Confirm  │
    │                 │        └─────┬─────┘
    │                 │              │
    │                 │        ┌─────┴─────┐
    │                 │        │           │
    │                 │   [Yes]│      [No] │
    │                 │        │           │
    │                 │  ┌─────▼─────┐ ┌───▼───┐
    │                 │  │ Download  │ │Cancel │
    │                 │  └─────┬─────┘ └───────┘
    │                 │        │
    │                 │        │
    └─────────────────┴────────┤
                               │
                         ┌─────▼─────┐
                         │  Success  │
                         └───────────┘
```

## 遷移条件

### Start → ParseInput
- 常に遷移

### ParseInput → LocalFile
- 条件: `target` がファイルパスとして存在する

### ParseInput → RemoteName
- 条件: `target` がファイルとして存在しない

### ParseInput → InvalidInput
- 条件: `target` が空または無効

### LocalFile → .deb
- 条件: 拡張子が `.deb`

### LocalFile → .AppImage
- 条件: 拡張子が `.AppImage` または `.appimage`

### LocalFile → Error
- 条件: 上記以外の拡張子

### RemoteName → FetchToml
- 常に遷移

### FetchToml → ShowUrl
- 条件: TOMLエントリが見つかった

### FetchToml → Error
- 条件: TOMLエントリが見つからない (E003)

### Confirm → Download
- 条件: ユーザーが `y` または `yes` を入力

### Confirm → Cancel
- 条件: ユーザーが `n` または `no` を入力、または Ctrl+C

## 各状態の処理

### ProcessDeb
```rust
fn process_deb(path: &Path) -> Result<()> {
    // 1. sudo権限確認
    // 2. dpkg -i <path> 実行
    // 3. apt -f install 実行（依存解決）
    // 4. 元ファイル削除の提案
}
```

### ProcessAppImage
```rust
fn process_appimage(path: &Path) -> Result<()> {
    // 1. ~/Applications/ ディレクトリ確認/作成
    // 2. ファイル移動
    // 3. 実行権限付与 (chmod +x)
    // 4. ~/.local/bin/ にシンボリックリンク作成
}
```

### Download
```rust
fn download(url: &str, app: &App) -> Result<()> {
    // 1. 一時ファイルにダウンロード
    // 2. ファイルタイプに応じた処理へ
    // 3. 一時ファイル削除
}
```

## 関連ドキュメント

- [install コマンド仕様](../../spec/install.md)
- [エラーハンドリング設計](../error-handling.md)
