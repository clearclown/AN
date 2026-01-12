# install コマンド仕様

## 概要

ローカルファイル（.deb, .AppImage）またはリモートアプリをインストールします。

## シグネチャ

```bash
an install <target>
an i <target>
```

## ユースケース

### UC1: ローカル.debファイルのインストール

```bash
# ダウンロードした.debファイルをインストール
an install ~/Downloads/vscode.deb
```

### UC2: ローカルAppImageのインストール

```bash
# AppImageを適切な場所に配置
an install ~/Downloads/Obsidian.AppImage
```

### UC3: リモートアプリのインストール

```bash
# アプリ名を指定してインストール
an install firefox
```

## 入力

| パラメータ | 型 | 必須 | 説明 |
|-----------|-----|------|------|
| target | String | Yes | ファイルパスまたはアプリ名 |

### targetの解釈

1. ファイルパスとして存在する → ローカルファイル処理
2. ファイルとして存在しない → リモートアプリ名として処理

## 出力

### 成功時

```
Installing firefox...
Source: https://github.com/.../Firefox.AppImage
Continue? [y/N]: y

Downloading... [████████████████████████████████] 100%
Moving to ~/Applications/...
Creating symlink in ~/.local/bin/...

✓ firefox installed successfully
```

### 失敗時

```
Error: E003: アプリDBにエントリが見つかりません: unknownapp

Hint: 'an search unknownapp' でアプリを検索してみてください
```

## 処理フロー

→ [ステートマシン図](../design/state-machines/install-flow.md) 参照

### ローカル.deb処理

1. sudo権限の確認
2. `dpkg -i <file>` 実行
3. `apt -f install` で依存関係解決
4. 元ファイル削除の提案 (オプション)

### ローカルAppImage処理

1. `~/Applications/` ディレクトリ確認・作成
2. ファイルを移動
3. 実行権限付与 (`chmod +x`)
4. `~/.local/bin/` にシンボリックリンク作成
5. デスクトップエントリ生成 (オプション)

### リモートアプリ処理

1. `apps/<name>.toml` からアプリ情報取得
2. ソースURLを表示
3. ユーザー確認 (`y/N`)
4. ダウンロード
5. ファイルタイプに応じた処理

## エラーケース

| コード | 説明 | 対処 |
|--------|------|------|
| E001 | ファイルが見つからない | パスを確認 |
| E002 | 不明なファイル形式 | .deb または .AppImage のみ対応 |
| E003 | アプリDBにエントリなし | `an search` で検索 |
| E004 | ダウンロード失敗 | ネットワーク確認、URLの有効性確認 |
| E005 | dpkg/権限エラー | sudo権限確認、依存関係確認 |

## テストケース

### ユニットテスト

| ID | テスト内容 | 入力 | 期待結果 |
|----|-----------|------|----------|
| TC001 | ファイル存在確認 | 存在するパス | true |
| TC002 | ファイル存在確認 | 存在しないパス | false |
| TC003 | 拡張子判定 (.deb) | "package.deb" | FileType::Deb |
| TC004 | 拡張子判定 (.AppImage) | "app.AppImage" | FileType::AppImage |
| TC005 | 拡張子判定 (不明) | "file.xyz" | Error |
| TC006 | TOML検索 (存在) | "firefox" | Some(App) |
| TC007 | TOML検索 (不在) | "unknown" | None |

### 統合テスト

| ID | テスト内容 | 前提条件 | 期待結果 |
|----|-----------|----------|----------|
| IT001 | ローカル.debインストール | テスト用.debファイル | インストール成功 |
| IT002 | ローカルAppImageインストール | テスト用AppImage | ~/Applications/に配置、リンク作成 |
| IT003 | リモートインストール (承認) | TOMLエントリ存在 | ダウンロード・インストール成功 |
| IT004 | リモートインストール (拒否) | TOMLエントリ存在 | キャンセル終了 |
| IT005 | 存在しないファイル | 不正なパス | E001エラー |
| IT006 | 不明な拡張子 | .xyz ファイル | E002エラー |
| IT007 | DBにないアプリ | 未登録名 | E003エラー |

## セキュリティ考慮事項

1. **URL表示と確認**: リモートインストール時は必ずURLを表示し、ユーザー確認を求める
2. **sudo最小化**: sudo権限は.debインストール時のみ要求
3. **ダウンロード検証**: 将来的にチェックサム検証を実装予定

## 関連ドキュメント

- [ステートマシン図](../design/state-machines/install-flow.md)
- [アプリDBスキーマ](./app-db-schema.md)
- [エラーハンドリング](../design/error-handling.md)
