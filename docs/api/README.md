# AN 内部API仕様

AN (安装) パッケージマネージャーの内部API仕様です。

## ドキュメント一覧

| ファイル | 説明 |
|----------|------|
| [handlers.md](./handlers.md) | Handler層API |
| [db.md](./db.md) | DB層API |

## レイヤー構成

```
CLI Layer → Command Layer → Handler Layer → Utils Layer
                  ↓
              DB Layer
```

各レイヤーは明確なインターフェースを持ち、独立してテスト可能です。

## 関連ドキュメント

- [機能仕様書](../spec/README.md)
- [設計ドキュメント](../design/README.md)
- [アーキテクチャ](../design/architecture.md)
