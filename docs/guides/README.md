# AN ユーザーガイド

AN (安装) パッケージマネージャーのユーザーガイドです。

## ガイド一覧

| ガイド | 説明 |
|--------|------|
| [getting-started.md](./getting-started.md) | クイックスタート |
| [installation.md](./installation.md) | ANのインストール方法 |
| [commands.md](./commands.md) | コマンドリファレンス |

## ANとは

AN (安装 - アン) は、Debian/Ubuntu系Linux向けの統合型パッケージマネージャーです。

### 特徴

- **統一インターフェース**: .deb、AppImage、Flatpakを単一のコマンドで管理
- **完全削除**: 設定ファイル、依存関係、ユーザーデータまで完全に削除
- **透明性**: ダウンロードURLを明示し、ユーザー確認を求める
- **クリーン**: システムを汚さないインストール管理

### クイックスタート

```bash
# ANをインストール
curl -fsSL https://raw.githubusercontent.com/clearclown/AN/main/install.sh | bash

# アプリをインストール
an install firefox

# アプリを削除
an remove firefox

# Flatpakエイリアスを生成
an link

# ANを更新
an update
```

## 関連ドキュメント

- [機能仕様書](../spec/README.md)
- [設計ドキュメント](../design/README.md)
