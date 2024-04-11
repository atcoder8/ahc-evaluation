# ahc-evaluation

AHC (AtCoder Heuristic Contest) の提出コードを評価します。

このプログラムは以下の作業を自動化します。
  - 提出コードとローカルテスタのビルドを行います。
  - 各シードについて提出コードとローカルテスタを実行し、スコアと実行時間を収集します。
    - プログラムとファイルの入出力を行います。
    - ローカルテスタの出力からスコアを取得します。

## 使用方法

```
AHC (AtCoder Heuristic Contest) の提出コードを評価します。

使用方法: ahc-evaluation [OPTIONS]

オプション:
  -c, --config <CONFIG>  構成ファイルのパス [デフォルト: evaluation/config.toml]
  -h, --help             ヘルプの表示
  -V, --version          バージョンの表示
```

## 構成

カレントディレクトリの下に構成ファイルとして`evaluation/config.toml`を置きます。`--config`オプションを使用して構成ファイルへのパスを指定することもできます。

以下は設定ファイルの例です。キーは変更できませんが、値は必要に応じて変更する必要があります。

```toml
[thread]
# 評価に使用するスレッド数
# 指定しない場合は自動で決定されます
thread_num = 8

[path]
# 評価に使用されるシードリスト
seed_file = "tools/seeds.txt"

# 入力ファイルのディレクトリ
input_dir = "tools/in"

# 出力ファイルのディレクトリ
output_dir = "evaluation/out"

# 各シードに対するスコアと実行時間をまとめたリストを出力するファイル
evaluation_record = "evaluation/summary.csv"

[command]
# 提出コードのビルドコマンド
# ビルドが必要ない場合は空の配列を指定します
build.submission = ["cargo", "build", "--release"]

# ローカルテスタのビルドコマンド
# ビルドが必要ない場合は空の配列を指定します
build.tester = []

# 提出コードの実行コマンド
execute.submission = ["target/release/submission"]

# ローカルテスタの実行コマンド
# 以下のプレースホルダが使用できます (プレースホルダは個別に引用符で囲んでください)
# - `{input}`: シードに対応する入力ファイルのパス
# - `{output}`: シードに対応する出力ファイルのパス
# - `{cmd}`: 出力ファイルの実行コマンド
execute.tester = ["tools/tester", "{input}", "{output}"]

# 提出コードを単独ではなくローカルテスタを介して実行する場合はこのフラグを`true`にします
execute.integrated = false
```
