# chapter02
## 実行環境
```
$ cargo --version
cargo 1.60.0-nightly (358e79fe5 2022-01-04)
$ rustc --version
rustc 1.60.0-nightly (092e1c9d2 2022-01-09)
$ rustdoc --version
rustdoc 1.60.0-nightly (092e1c9d2 2022-01-09)
```

## hello
* Cargoは...
  * Rustコンパイラrustcを起動
  * コンパイルの結果できたプログラムを実行
  * 実行ファイルをtargetサブディレクトリに格納
* `cargo clean`で消すことが可能

## actix-gcd 
* crates.io：公開されいてる無償のライブラリ群
  * cargoで簡単にダウンロードできる
  * Rustのパッケージは、ライブラリであれ実行ファイルであれ、**クレート**

* actix-web:webフレームワーク
* serde:シリアライズを行う
* serdeを使うとwebフォームで入力されたデータを簡単に扱うことができる
  * この機能を使うためにderive機能を選択する


rustのバージョンを上げる
```
$ rustup update
```

```
error[E0277]: the trait bound `fn() -> HttpResponse {get_index}: Handler<_>` is not satisfied
```
