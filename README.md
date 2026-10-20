# osekkai

文章のまとめ方を Claude Code（と codex）に持たせるための skill と、その行動を支えるガードレールの lint。

- `osekkai` は Rust の単体バイナリで、Markdown の箇条の構造と読み手のステートを lint し、守るべき手順を印字する
- `skills/osekkai/SKILL.md` は文章をまとめる手順の skill。4 段のアンカーを持ち、`nix flake check` が順序を確かめる。nix package は同じ skill を `share/osekkai/skills/osekkai` にも入れるので、home-manager はそこを `~/.claude/skills` に指せばバイナリと skill の版が揃う

設計の経緯は `.ai/`（gitignore 下）にある。

## 使い方

```sh
osekkai lint <file>...              # file:line:col: rule-id: message。指摘があれば exit 1
osekkai lint --format json <file>   # textlint と同じ ruleId キーの JSON
osekkai lint --require-state <file> # 読み手のステートが宣言されていなければ指摘する
osekkai state list                  # ステートの次元と値
osekkai state show unknown=鈴木,APM purpose=inform  # そのステートの guidance と有効になる lint
osekkai steps                       # 守るべき 4 段
osekkai steps --check SKILL.md      # skill 本文のアンカーが 4 段の順で並んでいるか
```

## 読み手のステート

本文の最初のブロックに HTML コメントで宣言する。GitHub は描画しないので PR 本文に残せる。

```markdown
<!-- osekkai: unknown=鈴木,APM,840 万 history=unknown purpose=inform -->
```

次元はバイナリが持つ。`unknown` は読み手が知らない語の列挙で、何がその語かは文書ごとに書き手が決める。人名、製品名、金額はどれもその例でしかないので、種類ごとの次元は持たない。`history` と `purpose` は enum の値。読み手の目録は作らず、文書ごとに値の組み合わせで宣言する。各値は意味を読む判断の guidance と、文字と構造で判定できる lint ルールの 2 つを持ち、`state show` が両方を印字する。

## lint ルール

常時有効なもの。

| ID | 見るもの |
| --- | --- |
| `no-list-item-colon` | 箇条の中のコロン |
| `no-sentence-chain-in-list-item` | 箇条の中で「。」で文を重ねる形 |
| `no-list-item-separator` | 行頭句を ` — ` などで本文に繋ぐ形 |
| `max-list-item-lines` | 1 箇条の見た目の行数（3 行まで） |
| `max-list-block-items` | 1 つの親の下の項目数（合計 5 まで、深い階層ほど少なく） |
| `max-section-lines` | 1 節の見た目の行数（12 行まで） |
| `postscript-last` | 「蛇足」の段落が箇条の外の最後にあるか |
| `state-first` | ステートの宣言が最初のブロックにあるか |

宣言されたステートで有効になるもの。

| ID | 有効になる条件 | 見るもの |
| --- | --- | --- |
| `no-unknown-term` | 知らない語を宣言 | 宣言した語が本文に残っていないか |
| `no-heading` | 知っておく | 題名以外の見出しがないか |
| `max-top-items` | 知っておく | 最上位の項目が 4 つまでか |
| `decision-first` | 判断する | 先頭に期限と依頼があるか |
| `require-heading` | 判断する | 「案」「未確定」の章があるか |

日本語の文の規則（表記ゆれ、ですます調）は textlint に任せ、ここでは持たない。

## 開発

```sh
nix develop -c cargo test     # tests/fixtures/ の Markdown を全部 lint し、sidecar の .expect と比べる
nix flake check               # ビルド、テスト、rustfmt
```

fixture は `tests/fixtures/<rule>/{ok,ng}/*.md`。`ng` は鳴るべき `rule@line` を同名の `.expect` に書く。`tests/fixtures/examples/` は本人が受け入れた出力で、いま鳴っている指摘も sidecar に記録してある。
