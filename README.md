# ECDH demo in Rust
ECDH (Elliptic curve Diffie–Hellman key exchange)、
日本語では「楕円曲線ディフィー・ヘルマン鍵共有」と呼ばれる暗号鍵共有のプロトコルを、
Rust の練習の一環として実装したもの。

An implementation of a cryptographic key sharing protocol
called Elliptic curve Diffie–Hellman key exchange (ECDH)
as part of Rust's practice.

## Set up
Rust をインストールする。標数```p```を変更したい場合は```main.rs```の1行目の右辺の値を変更する(デフォルトは```863 = 2^5 * 3^3 - 1```)。
```p```は```p % 4 == 3```かつ```p >= 7```である素数である必要がある。

Install Rust.
To run the program with another prime number ```p```,
change RHS of the first line in ```main.rs``` (default value is ```863 = 2^5 * 3^3 - 1```).
```p``` must be a prime number with ```p % 4 == 3``` and ```p >= 7```.