# Cookpad Spring 1day Internship 2018 Rust プログラミングコース 事前準備手順書

*これは受講者に事前配布した手順書の抜粋です。手元で再現する場合の参考にしてください。*

## コンパイラの準備

ワークショップではバージョン 1.25 (stable)のコンパイラを用います。

未インストールの方は下記に示すRust 公式のインストール手順に従い、インストールをしてください。`rustc`, `cargo`, `rustup` などのコマンドがインストールされます。
 [インストール · プログラミング言語Rust](https://www.rust-lang.org/ja-JP/install.html)

上記ページにもある通り、シェルをカスタマイズしていると `$HOME/.cargo/bin/`  への PATH が自動で通らないことが多いです。
各位 **いい感じに** しておいてください。

### 既にインストール済みの方向け

その他の方法でインストールしている場合でも、`rustc` や `cargo` が正しく動作すれば受講上の問題はないはずですが、講師・TA がサポートできないかもしれません。

## Visual Studio Code + Rust(rls) の準備

**ワークショップで使うエディタは自由です**。
テキストを編集できるものであれば使い慣れているものを使っていただいて構いません。

しかし、Rust の開発に適切なエディタがわからない・普段使っているエディタが Rust を書くのに不向きであるといった場合にはこの環境構築手順を参考にしてください。

### RLS のインストール

※ `コンパイラの準備` にて rustup をインストールしてある環境を想定しています。

RLS とは Rust Language Server の略で、Visual Studio Code 等の Language Server プロトコルに対応したエディタでコード補完などを行うためのソフトウェアです。

RLS をインストールするには、シェルで以下のコマンドを実行します。

```
rustup component add --toolchain stable rls-preview
```

`rls --version` を実行し、次のように表示されればインストールは成功しています。

```
rls-preview 0.125.1-stable (cebf188 2018-03-19)
```

### Visual Studio Code のインストール

まず、以下の公式サイトより、Visual Studio Code (以下 VS Code)をダウンロードし、インストールしてください。
[Visual Studio Code - Code Editing. Redefined](https://code.visualstudio.com/)

### Rust (rls) のインストール

次に Rust 用の plugin `Rust (rls)` をインストールします。
以下のリンクから Visual Studio Marketplace を開き、インストールしてください。

[Rust (rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

`Install` ボタンをクリックして plugin のインストールを完了させた後、VS Code を再起動してください。

以上でエディタのセットアップは完了です。

## Hello, world!

環境構築が成功しているかどうか確認するため、Hello, world! をしましょう。

Hello, world! 用に適当なディレクトリを作成し、その中で以下のコマンドを実行します。

```
cargo init --bin
```

するといくつかのファイルとディレクトリが生成されます。

```
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs

1 directory, 3 files
```

`./src/main.rs` には初期状態で Hello, world! のためのコードが記述されています。

```
fn main() {
    println!("Hello, world!");
}
```

このプログラムを実行するためには次のコマンドを使います。

```
cargo run
```

このコマンドを実行すると、上記のコードがコンパイルされ、生成されたバイナリが実行されます。

以下のように表示されれば、正しく実行できています。

```
   Compiling rust-hello v0.1.0 (file:///Users/username/src/rust-hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `target/debug/rust-hello`
Hello, world!
```
