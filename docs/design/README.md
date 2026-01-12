# AN 設計ドキュメント

AN (安装) パッケージマネージャーの設計ドキュメントです。

## ドキュメント一覧

| ファイル | 説明 |
|----------|------|
| [architecture.md](./architecture.md) | システムアーキテクチャ |
| [error-handling.md](./error-handling.md) | エラーハンドリング設計 |
| [state-machines/](./state-machines/) | ステートマシン図 |

## ステートマシン図

| ファイル | 説明 |
|----------|------|
| [install-flow.md](./state-machines/install-flow.md) | installコマンド状態遷移 |
| [remove-flow.md](./state-machines/remove-flow.md) | removeコマンド状態遷移 |
| [link-flow.md](./state-machines/link-flow.md) | linkコマンド状態遷移 |
| [update-flow.md](./state-machines/update-flow.md) | updateコマンド状態遷移 |
| [list-flow.md](./state-machines/list-flow.md) | listコマンド状態遷移 |
| [search-flow.md](./state-machines/search-flow.md) | searchコマンド状態遷移 |
| [info-flow.md](./state-machines/info-flow.md) | infoコマンド状態遷移 |
| [sync-flow.md](./state-machines/sync-flow.md) | syncコマンド状態遷移 |

## 関連ドキュメント

- [機能仕様書](../spec/README.md)
- [内部API仕様](../api/README.md)
- [ユーザーガイド](../guides/README.md)
