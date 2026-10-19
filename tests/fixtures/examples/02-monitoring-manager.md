<!-- osekkai: people=known system=known cost=known history=unknown purpose=decide -->

# 監視基盤移行の判断依頼

- 11 月中旬までに次の 2 案から決めてほしい
  - Datadog の 12 月末の自動更新に間に合わせるため

## 案

- 全面移行案は年 310 万
  - ログ 90 日保存なら 430 万
  - 鈴木さんは APM の調査効率が落ちると懸念している
    - 佐藤さんは研修で解消できると見ている
- 併用案は年 450 万
  - APM だけ Datadog に残す
  - ログ保存期間の見積もり前提は未確認

## 未確定

- ログ 90 日保存の監査要件は山本さんが確認中

蛇足だが、Grafana OnCall を使えば PagerDuty も解約できる可能性がある
