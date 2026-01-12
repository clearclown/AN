# link コマンド仕様

## 概要

Flatpakアプリをスキャンし、短い名前で実行できるエイリアス（ラッパースクリプト）を自動生成します。

例: `flatpak run org.gimp.GIMP` → `gimp`

## シグネチャ

```bash
an link
an l
```

## ユースケース

### UC1: Flatpakエイリアス一括生成

```bash
an link
# 全てのFlatpakアプリに対してエイリアスを生成
```

## 入力

このコマンドは引数を取りません。

## 出力

### 成功時

```
Scanning Flatpak applications...

Created links:
  gimp         → org.gimp.GIMP
  firefox      → org.mozilla.firefox
  spotify      → com.spotify.Client
  vscode       → com.visualstudio.code

Skipped (already exists):
  code         → /usr/bin/code (system binary)

Summary: 4 links created, 1 skipped
```

### Flatpak未インストール時

```
Error: E201: Flatpakがインストールされていません

Hint: sudo apt install flatpak でインストールしてください
```

### アプリなし時

```
No Flatpak applications found.

Hint: flatpak install <app> でアプリをインストールしてください
```

## 処理フロー

→ [ステートマシン図](../design/state-machines/link-flow.md) 参照

### 名前正規化ルール

| Flatpak ID | 生成されるコマンド名 |
|------------|---------------------|
| org.gimp.GIMP | gimp |
| org.mozilla.firefox | firefox |
| com.spotify.Client | spotify |
| com.visualstudio.code | vscode |
| org.kde.kdenlive | kdenlive |

### 正規化アルゴリズム

1. ドット区切りの最後の部分を取得
2. 小文字に変換
3. 一般的な接尾辞を除去 (`Client`, `App`, `Desktop`)
4. 衝突がある場合は接尾辞を付加

### 衝突解決

1. **システムバイナリと衝突**: スキップ（警告表示）
2. **AN既存リンクと衝突**: 上書き更新
3. **同一セッション内で衝突**: 2番目以降に連番付加

## ラッパースクリプト仕様

### 生成されるスクリプト

```bash
#!/bin/bash
# AN-generated wrapper for org.gimp.GIMP
exec flatpak run org.gimp.GIMP "$@"
```

### 配置先

```
~/.local/bin/<normalized_name>
```

### 権限

- 実行権限: 755 (rwxr-xr-x)

## エラーケース

| コード | 説明 | 対処 |
|--------|------|------|
| E201 | Flatpakがインストールされていない | `sudo apt install flatpak` |
| E202 | シンボリックリンク作成失敗 | ~/.local/bin/ の書き込み権限確認 |

## テストケース

### ユニットテスト

| ID | テスト内容 | 入力 | 期待結果 |
|----|-----------|------|----------|
| TC201 | 名前正規化 (GIMP) | "org.gimp.GIMP" | "gimp" |
| TC202 | 名前正規化 (firefox) | "org.mozilla.firefox" | "firefox" |
| TC203 | 名前正規化 (Client接尾辞) | "com.spotify.Client" | "spotify" |
| TC204 | 衝突検出 | 既存バイナリ名 | true |
| TC205 | ラッパースクリプト生成 | アプリID | 正しいスクリプト内容 |

### 統合テスト

| ID | テスト内容 | 前提条件 | 期待結果 |
|----|-----------|----------|----------|
| IT201 | link実行 | Flatpakアプリインストール済み | リンク作成成功 |
| IT202 | 重複実行 | 既にリンク作成済み | 上書き更新 |
| IT203 | 衝突回避 | システムバイナリと同名 | スキップ |
| IT204 | Flatpakなし | Flatpak未インストール | E201エラー |
| IT205 | アプリなし | Flatpakアプリなし | 正常終了（0件） |

## PATH設定

`~/.local/bin/` がPATHに含まれている必要があります。

### 確認方法

```bash
echo $PATH | grep -q "$HOME/.local/bin" && echo "OK" || echo "要設定"
```

### 設定方法 (bash/zsh)

```bash
# ~/.bashrc または ~/.zshrc に追加
export PATH="$HOME/.local/bin:$PATH"
```

## 関連ドキュメント

- [ステートマシン図](../design/state-machines/link-flow.md)
- [エラーハンドリング](../design/error-handling.md)
