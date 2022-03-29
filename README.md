# kvs-rs

Rustで実装したKey Value Storeです．

非同期ランタイムのTokioの学習とミドルウェアのようなソフトウェアを作りたいという思いから開発しました．

# Quick Start
## Local
```
cargo run --release
```
## Docker
```
docker build -t kvs-rs:latest .
docker run -it -p 8080:8080 kvs-rs:latest
```

# Demo 
Client
```
$ nc 127.0.0.1 8080
GET hello
error: no key hello
SET hello world
hello = world
GET hello
world
DELETE hello
world
GET hello
error: no key hello
```


# 機能
- Command
  - GET
  - SET
  - DELETE

# TODO
- Graceful Shutdownの実装
- クライアントの実装
- ベンチマークの実装
- エラーハンドリングの改善
- テストコードの追加
- TLS化

