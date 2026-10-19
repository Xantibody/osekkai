<!-- osekkai: people=known system=known cost=known history=known purpose=inform -->

- 調査結果
    - キャッシュ
        - hit率は90%のため問題なし
    - ランナースペック
        - コンテナを毎回建てなおしていた
        - 共有テストにすると早くなる
        - だがデータ干渉の可能性があるためトランザクション推奨

蛇足だがlintも並列化できそう
