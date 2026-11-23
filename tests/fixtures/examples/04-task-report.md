<!-- osekkai: history=known -->

CI の失敗を直しました。
まだ push していません。

- 落ちていたのは、日付をまたぐときだけ結果が変わるテスト
  - テストが今の時刻を読んでいた
- 時刻を引数で渡す形にし、テストでは固定した
  - 同じ原因のテスト 2 件も同じ形に直した
- 手元では全部のテストが、日付が変わる直前の時刻でも通った
- CI の再実行は push のあと

https://github.com/example/notes/pull/34

https://github.com/example/notes/actions/runs/1234567890

commit して push しますか？
