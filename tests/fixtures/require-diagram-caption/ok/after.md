## CI の仕組み

```mermaid
flowchart LR
  repo["リポジトリ"] -- "取得" --> runner["ランナー"]
  runner -- "テスト" --> db["DB コンテナ"]
```

ランナーがリポジトリから取得し、DB コンテナに対してテストを流す。
