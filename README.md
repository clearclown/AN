承知いたしました。
既存の `am` (App Manager) プロジェクトの思想を受け継ぎつつ、それを「統合型パッケージマネージャー」へと進化（フォーク）させる、**Project "An" (安装)** の要件定義書および仕様書を作成しました。

これを開発のブループリント（設計図）とします。

---

# 📜 Project An (安装) - 開発ドキュメント

## 1. プロジェクト概要

* **プロジェクト名:** `An` (An Package Manager)
* **読み:** アン
* **由来:** 中国語の「安装 (Install)」および「安 (Peace/Stability)」
* **ベースプロジェクト:** Fork of / Wrapper around **"am" (App Manager)**
* **ターゲットOS:** Debian/Ubuntu系 Linux (特に Linux Mint)
* **開発言語:** Bash (Shell Script)
* **コアコンセプト:** "Unified & Clean"
* あらゆる形式（Deb, AppImage, Flatpak, Remote Script）を単一のインターフェースで扱う。
* システムを汚さない（完全削除・パス管理の自動化）。



---

## 2. 要件定義書 (Requirement Definition)

### 2.1. 解決したい課題

1. **管理の断片化:** `.deb`は `apt/dpkg`、AppImageは手動移動、Flatpakは長いコマンド…と管理方法がバラバラである。
2. **インストールの手間:** AppImageをダウンロードした後、「移動→権限付与→パス通し」の作業が面倒。
3. **不透明性:** `am` などの便利ツールは、どこのURLからダウンロードしているか直感的に分からず不安。
4. **削除の不完全さ:** `apt remove` や `flatpak uninstall` では設定ファイルや依存ゴミが残り、ディスクを圧迫する。

### 2.2. 機能要件 (Functional Requirements)

#### **F1. 万能インストール機能 (`an install`)**

* **ローカルファイル処理:**
* `.deb`: `apt` を使用してインストールし、依存関係を解決する。完了後、元のファイルの削除を提案する。
* `.AppImage`: 所定のディレクトリへ移動し、実行権限を与え、パスの通った場所にシンボリックリンクを作成する。


* **リモートインストール:**
* アプリ名を指定した場合、`am` のデータベースを利用してインストールする。
* **セキュリティ要件:** ダウンロード開始前に必ずソースURLを表示し、ユーザーの承認 (`y/N`) を求める。



#### **F2. 完全削除機能 (`an remove / uninstall`)**

* 指定されたアプリがどの形式でインストールされているか自動判別する。
* **パージ処理 (Purge):**
* Debian系: `apt purge` + `autoremove` を実行し、設定ファイルと不要な依存関係を全消去する。
* Flatpak: `--delete-data` オプション付きで削除し、ユーザーデータを残さない。
* AppImage: 実体ファイルとシンボリックリンクの両方を削除する。



#### **F3. エイリアス統合機能 (`an link`)**

* システム内の Flatpak アプリをスキャンし、短いコマンド名（例: `org.gimp.GIMP` -> `gimp`）で起動できるラッパースクリプトを自動生成する。

#### **F4. メンテナンス機能 (`an update`)**

* `an` 自体のアップデートおよび、`am` データベースの更新を行う。

### 2.3. 非機能要件 (Non-Functional Requirements)

* **ポータビリティ:** 追加の依存パッケージを極力減らし、標準的なコマンド (`curl`, `sed`, `awk`, `grep`) で動作させる。
* **透過性:** 実行しているコマンド（`apt` や `flatpak`）のログを隠蔽せず、ユーザーが見えるようにする。
* **安全性:** `sudo` は必要な操作（apt, システム領域への書き込み）の時のみ要求する。

---

## 3. 技術仕様書 (Technical Specification)

### 3.1. システムアーキテクチャ

`an` は **"Intelligent Wrapper"** として動作します。

```text
User Input (an install xxx)
      ⬇
[ Argument Parser ] 判別ロジック
      ⬇
-------------------------------------------------------
|  Local File?  |  Remote Name?  |  Flatpak ID?       |
-------------------------------------------------------
      ⬇                ⬇                 ⬇
[ Local Module ] [ Remote Module ] [ Link Module ]
  - .deb handler   - am wrapper      - flatpak scanner
  - .AppImage      - URL checker
    handler
      ⬇                ⬇                 ⬇
[ System Exec ]  [ am Core ]       [ Symlink Gen ]
 (apt / mv / ln)

```

### 3.2. ディレクトリ構成

Linuxの標準的なマナー（XDG Base Directory Specification）に準拠します。

| 用途 | パス | 備考 |
| --- | --- | --- |
| **実行ファイル** | `/usr/local/bin/an` | または `~/.local/bin/an` |
| **AppImage格納** | `~/Applications/` | 実体ファイルの置き場所 |
| **リンク置き場** | `~/.local/bin/` | パスの通った場所 |
| **設定/キャッシュ** | `~/.config/an/` | 将来的な設定ファイル用 |

### 3.3. コマンドインターフェース仕様

#### `an install <target>` (エイリアス: `i`)

* **引数:** ファイルパス または アプリ名
* **フロー:**
1. `target` がファイルとして存在するかチェック。
2. **Yes (ファイル):** 拡張子で分岐処理。
3. **No (ファイルなし):** `am` コマンドへ移譲フローへ。
* `am -u <target>` でURL取得。
* ユーザー確認プロンプト表示。
* 承認なら `am install <target>` 実行。





#### `an remove <target>` (エイリアス: `rm`, `uninstall`)

* **引数:** アプリ名
* **フロー (優先順位順に検索):**
1. `~/.local/bin/<target>` (AppImageリンク) をチェック → 削除。
2. `dpkg -l` (Debパッケージ) をチェック → `apt purge`。
3. `flatpak list` をチェック (ID逆引き) → `flatpak uninstall --delete-data`。
4. `am` 管理下かチェック → `am remove`。



#### `an link` (エイリアス: `l`)

* **機能:** Flatpak全スキャン & エイリアス生成。
* **命名規則:** アプリ名 (`Name`) を小文字化・スペース除去して使用。重複時は連番またはID末尾を使用。

---

## 4. 開発ロードマップ

### Phase 1: コア実装 (MVP)

* **目標:** `.zshrc` に書けるレベルではなく、独立したスクリプトファイルとしてリポジトリを作成する。
* `install` (Local Deb/AppImage, Remote am), `remove` (Purge), `link` の実装。

### Phase 2: インストーラー作成

* **目標:** ユーザーがワンライナーで `an` をインストールできるようにする。
* `am` が未インストールの場合は自動で入れる処理の追加。

### Phase 3: UI/UX改善

* 出力メッセージのカラー化（成功＝緑、警告＝黄、エラー＝赤）。
* TAB補完ファイルの作成（zsh用）。

ただし、`AN`のインストールを楽にしたい。将来的には、`apt install`や`cargo`などで簡単にできるようにしたい。

