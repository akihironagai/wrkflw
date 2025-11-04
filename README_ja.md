# WRKFLW

[![Crates.io](https://img.shields.io/crates/v/wrkflw)](https://crates.io/crates/wrkflw)
[![Rust Version](https://img.shields.io/badge/rust-1.67%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/crates/l/wrkflw)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/bahdotsh/wrkflw/build.yml?branch=main)](https://github.com/bahdotsh/wrkflw/actions/workflows/build.yml)
[![Downloads](https://img.shields.io/crates/d/wrkflw)](https://crates.io/crates/wrkflw)

WRKFLWは、完全なGitHub環境を必要とせずに、GitHub Actionsワークフローをローカルで検証・実行するための強力なコマンドラインツールです。開発者がGitHubに変更をプッシュする前に、自分のマシンで直接ワークフローをテストできます。

![WRKFLW Demo](demo.gif)

## 機能

- **TUIインターフェース**: ワークフローの実行管理と監視のためのフル機能ターミナルユーザーインターフェース
- **ワークフローファイルの検証**: GitHub Actionsワークフローファイルの構文エラーや一般的なミスをCI/CD統合用の適切な終了コードでチェック
- **ローカルでのワークフロー実行**: DockerまたはPodmanコンテナを使用してマシン上で直接ワークフローを実行
- **複数のコンテナランタイム**: Docker、Podman、エミュレーションモードをサポートし、最大限の柔軟性を提供
- **ジョブ依存関係の解決**: ジョブの依存関係に基づいて正しい実行順序を自動的に決定
- **コンテナ統合**: 適切な環境設定で分離されたコンテナ内でワークフローステップを実行
- **GitHubコンテキスト**: GitHubライクな環境変数とワークフローコマンドを提供
- **ルートレス実行**: Podmanサポートにより、root権限なしでコンテナを実行可能
- **アクションサポート**: 様々なGitHub Actionsタイプをサポート：
  - Dockerコンテナアクション
  - JavaScriptアクション
  - コンポジットアクション
  - ローカルアクション
- **特別なアクション処理**: `actions/checkout`などの一般的に使用されるアクションのネイティブ処理
- **再利用可能ワークフロー（呼び出し元ジョブ）**: `jobs.<id>.uses`を介して再利用可能ワークフローを呼び出すジョブを実行（ローカルパスまたは`owner/repo/path@ref`）
- **出力キャプチャ**: ログ、ステップ出力、実行詳細の表示
- **並列ジョブ実行**: 独立したジョブを並列実行してワークフローの実行を高速化
- **リモートワークフロートリガー**: GitHubまたはGitLabでワークフロー実行を手動でトリガー

## 要件

### コンテナランタイム（オプション）

WRKFLWは分離実行のために複数のコンテナランタイムをサポートします：

- **Docker**: デフォルトのコンテナランタイム。[docker.com](https://docker.com)からインストール
- **Podman**: ルートレスコンテナランタイム。Dockerが利用できない、または許可されていない環境に最適。[podman.io](https://podman.io)からインストール
- **エミュレーション**: コンテナランタイム不要。ホストシステム上で直接コマンドを実行

### Podmanサポート

Podmanは以下のような環境で特に有用です：
- 組織によってDockerのインストールが許可されていない
- Dockerデーモン用のroot権限が利用できない
- ルートレスコンテナ実行を好む
- デーモンレスアーキテクチャによる強化されたセキュリティが望ましい

Podmanの使用方法:
```bash
# Podmanをインストール（OSによって異なる）
# macOSでHomebrewを使用:
brew install podman

# Ubuntu/Debian:
sudo apt-get install podman

# Podmanマシンを初期化（macOS/Windows）
podman machine init
podman machine start

# wrkflwで使用
wrkflw run --runtime podman .github/workflows/ci.yml
```

## インストール

`wrkflw`をインストールする推奨方法は、RustのパッケージマネージャーCargoを使用することです：

### Cargo Installを使用（推奨）
```bash
cargo install wrkflw
```

### ソースから

リポジトリをクローンしてCargoでビルド：

```bash
git clone https://github.com/bahdotsh/wrkflw.git
cd wrkflw
cargo build --release
```

コンパイルされたバイナリは`target/release/wrkflw`で利用できます。

## 使用方法

WRKFLWを使用する最も簡単な方法は、プロジェクトのルートディレクトリに移動して実行することです：

```bash
wrkflw
```

これにより、`.github/workflows`ディレクトリからすべてのワークフローが自動的に検出され、TUIインターフェースに読み込まれます。

WRKFLWは3つの主要なコマンドモードも提供します：

### ワークフローファイルの検証

```bash
# デフォルトの場所（.github/workflows）のすべてのワークフローファイルを検証
wrkflw validate

# 特定のワークフローファイルを検証
wrkflw validate path/to/workflow.yml

# 特定のディレクトリ内のワークフローを検証
wrkflw validate path/to/workflows

# 複数のファイルやディレクトリを検証（GitHubとGitLabは自動検出）
wrkflw validate path/to/flow-1.yml path/to/flow-2.yml path/to/workflows

# 提供されたすべてのパスでGitLab解析を強制
wrkflw validate --gitlab .gitlab-ci.yml other.gitlab-ci.yml

# 詳細出力で検証
wrkflw validate --verbose path/to/workflow.yml

# GitLab CIパイプラインを検証
wrkflw validate .gitlab-ci.yml --gitlab

# カスタムエラーハンドリング用の終了コードを無効化（デフォルト：有効）
wrkflw validate --no-exit-code path/to/workflow.yml
```

#### CI/CD統合用の終了コード

デフォルトで、`wrkflw validate`は検証が失敗した場合に終了コードを`1`に設定し、CI/CDパイプラインやスクリプトに最適です：

```bash
# CI/CDスクリプト内 - 検証失敗でスクリプトが終了
if ! wrkflw validate; then
    echo "❌ ワークフロー検証が失敗しました！"
    exit 1
fi
echo "✅ すべてのワークフローが有効です！"

# カスタムエラーハンドリングの場合、終了コードを無効化
wrkflw validate --no-exit-code
if [ $? -eq 0 ]; then
    echo "検証完了（詳細は出力を確認してください）"
fi
```

**終了コードの動作:**
- `0`: すべての検証が正常に完了
- `1`: 1つ以上の検証失敗を検出
- `2`: コマンド使用エラー（無効な引数、ファイルが見つからない等）

### CLIモードでのワークフロー実行

```bash
# Dockerでワークフローを実行（デフォルト）
wrkflw run .github/workflows/ci.yml

# DockerではなくPodmanでワークフローを実行
wrkflw run --runtime podman .github/workflows/ci.yml

# エミュレーションモードでワークフローを実行（コンテナなし）
wrkflw run --runtime emulation .github/workflows/ci.yml

# 詳細出力で実行
wrkflw run --verbose .github/workflows/ci.yml

# デバッグ用に失敗したコンテナを保持
wrkflw run --preserve-containers-on-failure .github/workflows/ci.yml
```

### TUIインターフェースの使用

```bash
# デフォルトディレクトリのワークフローでTUIを開く
wrkflw tui

# 特定のワークフローディレクトリでTUIを開く
wrkflw tui path/to/workflows

# 特定のワークフローを事前選択してTUIを開く
wrkflw tui path/to/workflow.yml

# PodmanランタイムでTUIを開く
wrkflw tui --runtime podman

# エミュレーションモードでTUIを開く
wrkflw tui --runtime emulation
```

### リモートワークフローのトリガー

```bash
# GitHubでワークフローをリモートトリガー
wrkflw trigger workflow-name --branch main --input key1=value1 --input key2=value2

# GitLabでパイプラインをリモートトリガー
wrkflw trigger-gitlab --branch main --variable key1=value1 --variable key2=value2
```

## TUIコントロール

ターミナルユーザーインターフェースは、ワークフローを管理するためのインタラクティブな方法を提供します：

- **Tab / 1-4**: タブ間の切り替え（ワークフロー、実行、ログ、ヘルプ）
- **Up/Down または j/k**: リストのナビゲーション
- **Space**: ワークフロー選択の切り替え
- **Enter**: 選択したワークフローの実行 / ジョブ詳細の表示
- **r**: 選択したすべてのワークフローを実行
- **a**: すべてのワークフローを選択
- **n**: すべてのワークフローの選択を解除
- **e**: ランタイムモードの循環（Docker → Podman → エミュレーション）
- **v**: 実行モードと検証モードの切り替え
- **Esc**: 戻る / 詳細ビューを終了
- **q**: アプリケーションを終了
## 例

### ワークフローの検証

```bash
$ wrkflw validate .github/workflows/rust.yml
Validating GitHub workflow file: .github/workflows/rust.yml... Validating 1 workflow file(s)...
✅ Valid: .github/workflows/rust.yml

Summary: 1 valid, 0 invalid

$ echo $?
0

# 検証失敗の例
$ wrkflw validate .github/workflows/invalid.yml
Validating GitHub workflow file: .github/workflows/invalid.yml... Validating 1 workflow file(s)...
❌ Invalid: .github/workflows/invalid.yml
   1. Job 'test' is missing 'runs-on' field
   2. Job 'test' is missing 'steps' section

Summary: 0 valid, 1 invalid

$ echo $?
1
```

### ワークフローの実行

```bash
$ wrkflw run .github/workflows/rust.yml

Executing workflow: .github/workflows/rust.yml
============================================================
Runtime: Docker
------------------------------------------------------------

✅ Job succeeded: build

------------------------------------------------------------
  ✅ Checkout code
  ✅ Set up Rust
  ✅ Build
  ✅ Run tests

✅ Workflow completed successfully!
```

### クイックTUI起動

```bash
# プロジェクトルートに移動してwrkflwを実行
$ cd my-project
$ wrkflw

# これにより.github/workflowsファイルが自動的にTUIに読み込まれます
```

## システム要件

- Rust 1.67以降
- コンテナランタイム（オプション、コンテナベース実行用）：
  - **Docker**: 従来のコンテナランタイム
  - **Podman**: Dockerのルートレス代替  
  - **なし**: エミュレーションモードはローカルシステムツールを使用してワークフローを実行

## 動作原理

WRKFLWはGitHub Actionsワークフローファイルを解析し、各ジョブとステップを正しい順序で実行します。コンテナモード（Docker/Podman）では、GitHubのランナー環境に近いコンテナを作成します。ワークフロー実行プロセス：

1. **解析**: ワークフローYAML構造の読み取りと検証
2. **依存関係解決**: ジョブ依存関係に基づく実行計画の作成
3. **環境設定**: GitHubライクな環境変数とコンテキストの準備
4. **実行**: 各ジョブとステップをコンテナ内（Docker/Podman）またはローカルエミュレーションで実行
5. **監視**: TUIまたはコマンドラインでの進行状況追跡と出力キャプチャ
## 高度な機能

### GitHub環境ファイルサポート

WRKFLWはGitHubの環境ファイルと特別なコマンドをサポートします：

- `GITHUB_OUTPUT`: ステップ出力の保存用（`echo "result=value" >> $GITHUB_OUTPUT`）
- `GITHUB_ENV`: 環境変数の設定用（`echo "VAR=value" >> $GITHUB_ENV`）
- `GITHUB_PATH`: PATHの変更用（`echo "/path/to/dir" >> $GITHUB_PATH`）
- `GITHUB_STEP_SUMMARY`: ステップサマリーの作成用（`echo "# Summary" >> $GITHUB_STEP_SUMMARY`）

### コンポジットアクション

WRKFLWは複数のステップで構成されるコンポジットアクションをサポートします。これには以下が含まれます：

- `./path/to/action`で参照されるローカルコンポジットアクション
- GitHubリポジトリからのリモートコンポジットアクション
- ネストされたコンポジットアクション（他のアクションを使用するコンポジットアクション）

### コンテナクリーンアップ

WRKFLWは、プロセスがCtrl+Cで中断された場合でも、ワークフロー実行中に作成されたコンテナ（Docker/Podman）を自動的にクリーンアップします。

失敗したワークフローのデバッグのために、`--preserve-containers-on-failure`フラグを使用して失敗したコンテナを保持できます：

```bash
# デバッグ用に失敗したコンテナを保持
wrkflw run --preserve-containers-on-failure .github/workflows/build.yml

# TUIモードでも利用可能
wrkflw tui --preserve-containers-on-failure
```

このフラグが有効な場合、コンテナが失敗すると、WRKFLWは：
- 失敗したコンテナを削除せずに実行状態を保持
- コンテナIDと検査手順をログに記録
- 次のようなメッセージを表示：`Preserving container abc123 for debugging (exit code: 1). Use 'docker exec -it abc123 bash' to inspect.`（Docker）
- または：`Preserving container abc123 for debugging (exit code: 1). Use 'podman exec -it abc123 bash' to inspect.`（Podman）

これにより、失敗が発生した時点でのコンテナの正確な状態を検査し、ファイルを調べ、環境変数をチェックし、問題をより効果的にデバッグできます。

### Podman固有の機能

Podmanをコンテナランタイムとして使用する場合、追加の利点があります：

**ルートレス操作:**
```bash
# root権限なしでワークフローを実行
wrkflw run --runtime podman .github/workflows/ci.yml
```

**強化されたセキュリティ:**
- デーモンレスアーキテクチャにより攻撃面を削減
- ユーザー名前空間により追加の分離を提供
- 特権デーモンが不要

**コンテナ検査:**
```bash
# 保持されたコンテナをリスト表示
podman ps -a --filter "name=wrkflw-"

# 保持されたコンテナのファイルシステムを検査（実行なし）
podman mount <container-id>

# または同じボリュームで新しいコンテナを実行
podman run --rm -it --volumes-from <failed-container> ubuntu:20.04 bash

# すべてのwrkflwコンテナをクリーンアップ
podman ps -a --filter "name=wrkflw-" --format "{{.Names}}" | xargs podman rm -f
```

**互換性:**
- Dockerワークフローのドロップイン代替
- 同じCLIオプションと動作
- 同一のコンテナ実行環境
## 制限事項

### サポートされている機能
- ✅ 基本的なワークフロー構文と検証（すべてのYAML構文チェック、必須フィールド、構造）とCI/CD統合用の適切な終了コード
- ✅ ジョブ依存関係の解決と並列実行（正しい'needs'関係を持つすべてのジョブが正しい順序で実行され、独立したジョブは並列実行）
- ✅ マトリックスビルド（合理的なマトリックスサイズでサポート；非常に大きなマトリックスは遅いかリソース集約的になる可能性）
- ✅ 環境変数とGitHubコンテキスト（すべての標準GitHub Actions環境変数とコンテキストオブジェクトがエミュレート）
- ✅ コンテナアクション（コンテナを使用するすべてのアクションがDockerとPodmanモードでサポート）
- ✅ JavaScriptアクション（JavaScriptを使用するすべてのアクションがサポート）
- ✅ コンポジットアクション（ネストされたものやローカルコンポジットアクションを含むすべてのコンポジットアクションがサポート）
- ✅ ローカルアクション（ローカルパスで参照されるアクションがサポート）
- ✅ 一般的なアクションの特別処理（例：`actions/checkout`がネイティブサポート）
- ✅ 再利用可能ワークフロー（呼び出し元）：`jobs.<id>.uses`を使用してローカルまたはリモートワークフローを呼び出すジョブが実行；入力とシークレットが呼び出されたワークフローに伝播
- ✅ `workflow_dispatch`によるワークフロートリガー（ワークフローの手動トリガーがサポート）
- ✅ GitLabパイプライントリガー（GitLabパイプラインの手動トリガーがサポート）
- ✅ 環境ファイル（`GITHUB_OUTPUT`、`GITHUB_ENV`、`GITHUB_PATH`、`GITHUB_STEP_SUMMARY`が完全サポート）
- ✅ ワークフロー管理と監視用のTUIインターフェース
- ✅ 検証、実行、リモートトリガー用のCLIインターフェース
- ✅ 出力キャプチャ（ログ、ステップ出力、実行詳細がTUIとCLIの両方で利用可能）
- ✅ コンテナクリーンアップ（wrkflwによって作成されたすべてのコンテナが中断時でも自動的にクリーンアップ）

### 制限またはサポートされていない機能（明示的リスト）
- ❌ GitHubシークレットと権限：基本的な環境変数のみサポート。GitHubの暗号化されたシークレットと細かい権限は利用不可。
- ❌ GitHub Actionsキャッシュ：キャッシュ機能（例：`actions/cache`）はエミュレーションモードでサポートされず、DockerとPodmanモードでも部分的サポートのみ（実行間の永続キャッシュなし）。
- ❌ GitHub API統合：基本的なワークフロートリガーのみサポート。ワークフロー状態レポート、アーティファクトアップロード/ダウンロード、APIベースのジョブ制御などの機能は利用不可。
- ❌ GitHub固有の環境変数：一部の高度または動的な環境変数（例：GitHubランナーやGitHub APIによって設定されるもの）は静的またはベストエフォート値でエミュレートされるが、すべてが完全に機能するわけではない。
- ❌ 大規模/複雑なマトリックスビルド：非常に大きなマトリックス（数百または数千のジョブ組み合わせ）は、パフォーマンスとリソース制限により実用的でない可能性。
- ❌ ネットワーク分離アクション：厳密なネットワーク分離やカスタムネットワーク設定を必要とするアクションは、そのままでは動作しない可能性があり、手動でのコンテナランタイム設定が必要な場合。
- ❌ 一部のイベントトリガー：`workflow_dispatch`（手動トリガー）のみ完全サポート。他のトリガー（例：`push`、`pull_request`、`schedule`、`release`等）はサポートされない。
- ❌ GitHubランナー固有の機能：正確なGitHubホストランナー環境に依存する機能（例：プリインストールツール、ランナーラベル、ハードウェア）は保証されない。ベストエフォートエミュレーションのみ提供。
- ❌ WindowsとmacOSランナー：Linuxベースのランナーのみ完全サポート。WindowsとmacOSジョブはサポートされない。
- ❌ サービスコンテナ：サービスコンテナ（例：`services:`で定義されるデータベース）はDockerとPodmanモードでのみサポート。エミュレーションモードではサポートされない。
- ❌ アーティファクト：ジョブ/ステップ間でのアーティファクトのアップロードとダウンロードはサポートされない。
- ❌ ジョブ/ステップタイムアウト：ジョブとステップのカスタムタイムアウトは強制されない。
- ❌ ジョブ/ステップ並行性とキャンセル：`concurrency`やジョブキャンセルなどの機能はサポートされない。
- ❌ 式と高度なYAML機能：一般的な式の多くはサポートされるが、一部の高度またはエッジケースの式は完全に実装されていない可能性。
- ⚠️ 再利用可能ワークフロー（制限）：
  - 呼び出されたワークフローからの出力は呼び出し元に伝播されない（`needs.<id>.outputs.*`はサポートされない）
  - `secrets: inherit`は特別扱いされない；シークレットを渡すにはマッピングを提供
  - リモート呼び出しはHTTPS経由でパブリックリポジトリをクローン；プライベートリポジトリには事前設定されたアクセスが必要（未実装）
  - 深くネストされた再利用可能呼び出しは動作するが、通常のジョブ依存関係チェック以外のサイクル検出はない
## 再利用可能ワークフロー

WRKFLWは再利用可能ワークフロー呼び出し元ジョブの実行をサポートします。

### 構文

```yaml
jobs:
  call-local:
    uses: ./.github/workflows/shared.yml

  call-remote:
    uses: my-org/my-repo/.github/workflows/shared.yml@v1
    with:
      foo: bar
    secrets:
      token: ${{ secrets.MY_TOKEN }}
```

### 動作
- ローカル参照は現在の作業ディレクトリからの相対パスで解決されます。
- リモート参照は指定された`@ref`で一時ディレクトリにシャロークローンされます。
- `with:`エントリは呼び出されたワークフローに環境変数`INPUT_<KEY>`として公開されます。
- `secrets:`マッピングエントリは環境変数`SECRET_<KEY>`として公開されます。
- 呼び出されたワークフローは独自の`jobs`/`needs`に従って実行され、そのジョブ結果のサマリーが呼び出し元ジョブの単一結果として報告されます。

### 現在の制限
- 呼び出されたワークフローからの出力は呼び出し元に戻されません。
- `secrets: inherit`はサポートされません；明示的なマッピングを指定してください。
- リモート`uses:`のプライベートリポジトリはまだサポートされていません。

### ランタイムモードの違い
- **Dockerモード**: GitHubの環境に最も近い一致を提供し、Dockerコンテナアクション、サービスコンテナ、Linuxベースジョブをサポート。一部の高度なコンテナ設定では手動設定が必要な場合があります。
- **Podmanモード**: Dockerモードと似ていますが、コンテナ実行にPodmanを使用。ルートレスコンテナサポートと強化されたセキュリティを提供。Dockerベースワークフローと完全互換。
- **🔒 セキュアエミュレーションモード**: セキュリティのための包括的なサンドボックス化でローカルシステム上でワークフローを実行。**ローカル開発に推奨**：
  - コマンド検証とフィルタリング（`rm -rf /`、`sudo`などの危険なコマンドをブロック）
  - リソース制限（CPU、メモリ、実行時間）
  - ファイルシステムアクセス制御
  - プロセス監視と制限
  - 信頼できないワークフローをローカルで安全に実行
- **⚠️ エミュレーションモード（レガシー）**: サンドボックス化なしでローカルシステムツールを使用してワークフローを実行。**推奨されません - 代わりにセキュアエミュレーションを使用**：
  - ローカルとJavaScriptアクションのみサポート（Dockerコンテナアクションなし）
  - サービスコンテナサポートなし
  - キャッシュサポートなし
  - **セキュリティ保護なし - 有害なコマンドを実行可能**
  - 一部のアクションはローカルで動作するように適応が必要な場合があります

### ベストプラクティス
- **ローカル開発にはセキュアエミュレーションモードを使用** - コンテナオーバーヘッドなしで安全性を提供
- 互換性を確保するために複数のランタイムモードでワークフローをテスト
- **本番環境にはDocker/Podmanモードを使用** - 最大限の分離と再現性を提供
- より良いパフォーマンスのためにマトリックスビルドを合理的なサイズに保つ
- 可能な場合はGitHubシークレットの代わりに環境変数を使用
- 複雑なカスタム機能にはローカルアクションの使用を検討
- **セキュリティ警告を確認** - セキュアエミュレーションモードでブロックされたコマンドに注意
- **セキュアモードから開始** - 必要な場合のみレガシーエミュレーションにフォールバック
## ロードマップ

以下のロードマップは、WRKFLWで現在サポートされていない、または部分的にサポートされている機能を実装するための計画されたアプローチを概説しています。進捗と優先順位は、ユーザーフィードバックとコミュニティの貢献に基づいて変更される可能性があります。

### 1. シークレットと権限
- **目標:** GitHub Actionsと同様の暗号化されたシークレットと細かい権限をサポート。
- **計画:** 
  - ワークフローステップ用のセキュアなシークレット保存と注入を実装。
  - 環境変数、ファイル、またはシークレットマネージャーからのシークレット読み取りサポートを追加。
  - ジョブとステップの権限スコープを調査。

### 2. GitHub Actionsキャッシュ
- **目標:** 特に依存関係について、ワークフロー実行間の永続キャッシュを有効化。
- **計画:** 
  - Dockerモード用のローカルキャッシュディレクトリを実装。
  - Dockerとエミュレーションモードの両方で`actions/cache`サポートを追加。
  - 実行間キャッシュ永続性を調査。

### 3. GitHub API統合
- **目標:** アーティファクトアップロード/ダウンロード、ワークフロー/ジョブ状態レポート、その他のAPIベース機能をサポート。
- **計画:** 
  - アーティファクトアップロード/ダウンロードエンドポイントを追加。
  - API経由でのGitHubへの状態レポートを実装。
  - ジョブ/ステップアノテーションとログアップロードサポートを追加。

### 4. 高度な環境変数
- **目標:** すべての動的GitHub提供環境変数をエミュレート。
- **計画:** 
  - 不足している変数を監査し、可能な場合は動的計算を追加。
  - ドキュメントに互換性テーブルを提供。

### 5. 大規模/複雑なマトリックスビルド
- **目標:** 大きなマトリックスのパフォーマンスとリソース管理を改善。
- **計画:** 
  - マトリックス展開とジョブスケジューリングを最適化。
  - 非常に大きなマトリックスにリソース制限と警告を追加。

### 6. ネットワーク分離アクション
- **目標:** カスタムネットワーク設定とアクションの厳密な分離をサポート。
- **計画:** 
  - DockerとPodman用の高度なコンテナネットワーク設定オプションを追加。
  - ネットワーク分離のベストプラクティスを文書化。

### 7. イベントトリガー
- **目標:** 追加のトリガー（`push`、`pull_request`、`schedule`等）をサポート。
- **計画:** 
  - 一般的なトリガーのイベントシミュレーションを実装。
  - ユーザーがローカル実行用のイベントペイロードを指定できるようにする。

### 8. WindowsとmacOSランナー
- **目標:** 非Linuxランナーのサポートを追加。
- **計画:** 
  - クロスプラットフォームコンテナ化とエミュレーションを調査。
  - プラットフォーム固有の制限についてドキュメントを追加。

### 9. エミュレーションモードでのサービスコンテナ
- **目標:** エミュレーションモードでサービスコンテナ（例：データベース）をサポート。
- **計画:** 
  - ローカルサービス起動と終了スクリプトを実装。
  - 一般的なサービスの設定を提供。

### 10. アーティファクト、タイムアウト、並行性、式
- **目標:** アーティファクト処理、ジョブ/ステップタイムアウト、並行性、高度なYAML式をサポート。
- **計画:** 
  - アーティファクト保存と取得を追加。
  - タイムアウトと並行性制限を強制。
  - 高度な使用例のために式パーサーを拡張。

---

**手伝いたいですか？** 貢献を歓迎します！始め方については[CONTRIBUTING.md](CONTRIBUTING.md)をご覧ください。

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細についてはLICENSEファイルをご覧ください。
## リモートワークフロートリガー

WRKFLWでは、コマンドラインインターフェース（CLI）とターミナルユーザーインターフェース（TUI）の両方を通じて、GitHubでワークフロー実行を手動でトリガーできます。

### 要件:

1. ワークフロー権限を持つGitHubトークンが必要です。`GITHUB_TOKEN`環境変数に設定してください：
   ```bash
   export GITHUB_TOKEN=ghp_your_token_here
   ```

2. ワークフローYAMLで`workflow_dispatch`トリガーが定義されている必要があります：
   ```yaml
   on:
     workflow_dispatch:
       inputs:
         name:
           description: 'Person to greet'
           default: 'World'
           required: true
         debug:
           description: 'Enable debug mode'
           required: false
           type: boolean
           default: false
   ```

### CLIからのトリガー:

```bash
# デフォルトブランチを使用してワークフローをトリガー
wrkflw trigger workflow-name

# 特定のブランチでワークフローをトリガー
wrkflw trigger workflow-name --branch feature-branch

# 入力パラメータ付きでトリガー
wrkflw trigger workflow-name --branch main --input name=Alice --input debug=true
```

トリガー後、WRKFLWはGitHubでトリガーされたワークフローを表示するURLを含むフィードバックを提供します。

### TUIからのトリガー:

1. TUIインターフェースを起動：
   ```bash
   wrkflw tui
   ```

2. "Workflows"タブに移動（`Tab`キーまたは`1`を押す）。

3. 矢印キー（`↑`/`↓`）または`j`/`k`を使用して目的のワークフローを選択。

4. `t`を押して選択したワークフローをトリガー。

5. ワークフローが正常にトリガーされると、UIに通知が表示されます。

6. 提供されたURLを使用してGitHubでトリガーされたワークフローの実行を監視できます。

### トリガーされたワークフローの確認:

ワークフローがトリガーされたことを確認するには：

1. WebブラウザでGitHubリポジトリにアクセス。
2. "Actions"タブに移動。
3. ワークフロー実行のリストでワークフローを探す。
4. クリックして実行の詳細を表示。
