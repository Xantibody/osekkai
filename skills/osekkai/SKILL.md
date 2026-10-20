---
name: osekkai
description: The author's way of turning a source text — a chat log, an investigation, a meeting — into a short Markdown summary for a named reader. Declare who the reader is, draft under that reader's state, make `osekkai lint` pass, hand over. Other skills (explain, pull-request, issue) load it as the base for any prose a human reads.
when_to_use: 文章をまとめる、要約する、報告にする、別の読み手向けに書き直すとき — 「まとめて」「要約して」「報告書にして」「〇〇向けに書き直して」「調査メモにして」「議論を整理して」。PR 本文や作業報告など、explain skill が読み込むときも。
---

# osekkai

The reader decides the summary. The same source becomes four bullets with
no headings for someone who only needs to know what is going on, and a
decision request with options and open items for someone who has to
decide. So the reader is fixed before a word is written, and what the
reader knows is written into the document itself.

`osekkai` holds the part that must not be skipped — four steps, printed by
`osekkai steps` — and the vocabulary of reader states. Everything else in
this file is how the author drafts, and may be reordered or extended. The
steps carry an HTML-comment anchor each; `osekkai steps --check`
on this file keeps them in order.

## 0. Read the source and pick the reader

Read the whole source once before deciding anything. Then name the reader
in a sentence: who, and what they will do with the text (know about it,
decide something, do something). A PR body's reader is the reviewer; a
report's reader is the user who asked. If the request names no reader,
ask, or state the assumption in the hand-over.

<!-- osekkai-step: declare-state -->

## 1. Declare the reader's state

The state has three dimensions: the terms the reader does not know,
whether they know the history, and what they will do with the text.

| key | question |
| --- | --- |
| `unknown` | Which terms in the source would mean nothing to them? A comma-separated list |
| `history` | Do they know why this topic is up now? `known` or `unknown` |
| `purpose` | `inform`, `decide` or `act` |

The `unknown` list is decided per document, by you, for this reader. Go
through the source once with these questions and write down every term
that fails:

- **People.** Would the reader know who 鈴木 is? If not, the name goes on
  the list and only what was said stays.
- **Systems, tools, products.** Would the name alone mean something to
  them? A name that carries the point gets a one-line gloss and stays off
  the list; a name whose details they could not follow goes on the list,
  and its details go with it.
- **Amounts.** Do they have a feel for the budget? If not, each figure
  goes on the list and is written as a ratio (「今の半分ほど」).
- **Jargon.** Any abbreviation or in-house word they would have to look
  up.

A listed term must not appear in the text; the lint checks that. The
author lists fewer terms than an AI does. In the worked example the
outsider version keeps `Datadog`, `Grafana Cloud` and `840 万`: the names
are explained by the gloss on 「監視サービス」, and the current figure is
what the ratio is measured against. When in doubt, keep the term a gloss
already explains and list the ones that would need a gloss of their own.

Write the declaration as the first block of the document, then read the
guidance for it:

```markdown
<!-- osekkai: unknown=鈴木,佐藤,APM,310 万,450 万 history=unknown purpose=inform -->
```

```sh
osekkai state show unknown=鈴木,佐藤,APM,310 万,450 万 history=unknown purpose=inform
```

`state show` prints, per value, what to keep and what to cut, and which
lint rules it turns on. `osekkai state list` prints the whole vocabulary.
Declare only what the source touches; a text whose every term the reader
knows needs no `unknown`. The declaration is an HTML comment, so GitHub
does not render it and a PR body can keep it.

<!-- osekkai-step: draft -->

## 2. Draft

Drafting is a loop between the whole and the parts, not a single pass.
The author's own order:

1. **Place a working title.** Find the most abstract thing the source is
   about (「CI のビルド高速化調査」, not 「DB コンテナの立て直し」). It is
   provisional; the review at the end may change it.
2. **Choose the middle structure.** Much material becomes `##` sections;
   little becomes a bulleted list. When unsure, start wide (概要 / 詳細)
   and fill it in.
3. **Fold what did not grow.** If a section holds one or two bullets,
   collapse the sections into one list. The author's example went from
   two headings to a single nested list.
4. **Walk between concrete and abstract.** The first grouping is by
   whatever you noticed first and is allowed to be wrong; move items when
   the review shows a better axis. Do not expect to get it right once.
5. **Do not promote a weak cause to the conclusion.** The source may say
   X caused Y; check that the timing and the facts fit before leading
   with it. In the author's example the source blamed a container
   restart for a slowdown that started last week, but the restart was
   older than that, so it stayed a finding, not the conclusion. If the
   source gives no recommendation, write none of your own.
6. **Apply the reader's state.** For each fact, keep, cut, or add a
   one-line gloss, following the `state show` guidance:
   - `unknown=…`: a listed name is dropped and what was said stays; a
     listed figure becomes a ratio; a listed product or word is reworded,
     or cut with the detail that needed it. A name kept off the list gets
     its gloss once, as a child bullet (「監視サービスは、システムの状態を記録
     して障害を知らせるもの」).
   - No `unknown`: names alone suffice, and when opinions split, keep who
     said what so the reader knows whom to ask. List the amounts per
     option and omit the current figure.
   - `history=unknown`: say why this is being discussed now. `known`:
     conclusion and delta only.
   - `purpose=inform`: a few bullets on what is happening, no headings,
     and no decision material. `purpose=decide`: the request and its
     deadline first, then `## 案` and `## 未確定`. `purpose=act` has no
     worked example yet; ask the user what the reader needs to do.
7. **Move asides out.** Anything true but off the main line goes after
   the list, as a paragraph starting 「蛇足だが」. Only one, only last.
8. **Review the whole.** Reread as the reader. Is anything missing that
   they need? Did a fact the source itself lacks (an unstated
   assumption in an estimate) make it in as an open item? Does the title
   still fit?

### Sentence shape

- Every bullet has a subject: 「xx は○○」. 「xx が○○のため、△△は■■」 is
  still one bullet when it is one point.
- Prefer the clipped ending (「問題なし」, 「推奨」) over full sentences.
- Compress by default; add only when the reader cannot follow without it.
- One word per thing. A synonym reads as a second thing.
- When the same explanation appears twice, merge the two places or split
  them so each says it once.
- No colon inside a bullet (`- 調査結果: キャッシュは問題なし`), no second
  sentence inside a bullet (`- A。B。`), no ` — ` or `**label**:` joining a
  head to a body. The head becomes the bullet, the body its children.

### Density

The author's limit is what a reader takes in at a glance: about three
rendered lines per bullet, about twelve per section, five items under one
parent with fewer at each deeper level, and four top-level items when the
purpose is `inform`. The exact thresholds live in the binary; the lint
reports them. When it fires, the fix is to merge siblings, lift a level
up, or cut a section — never to shorten words until the count passes.

<!-- osekkai-step: lint -->

## 3. Lint until clean

```sh
osekkai lint <file>
```

Exit 0 means hand over. Each finding names the rule; this is how to fix
each:

| rule | fix |
| --- | --- |
| `state-first`, `state-declaration` | Move the declaration to the first block; a value it rejects is not in `osekkai state list` |
| `no-list-item-colon`, `no-list-item-separator` | The head becomes the bullet, what followed the colon becomes child bullets or one sentence |
| `no-sentence-chain-in-list-item` | Split at the 「。」 into sibling or child bullets, or rewrite as one point |
| `max-list-item-lines` | The bullet holds two points; split it, or move detail to a child |
| `max-list-block-items`, `max-top-items` | Merge siblings that share an axis, or lift a group into a section |
| `max-section-lines` | Split the section, or cut what the reader's state says to cut |
| `postscript-last` | Move the 「蛇足だが」 paragraph after the last list, outside any bullet |
| `no-unknown-term` | The listed term is still in the text: drop the name, write the figure as a ratio, reword or cut the word |
| `no-heading` | Flatten to a single list; the reader only needs to know |
| `decision-first` | Put the request and its deadline in the first bullet |
| `require-heading` | Add `## 案` and `## 未確定` and sort the items into them |

Japanese wording (ですます, 漢字の開き, spacing) is textlint's job, via
`lint-body` where it is installed; `osekkai` does not check it.

<!-- osekkai-step: hand-over -->

## 4. Hand over

Deliver the text with the declaration still in it when the destination
hides HTML comments (GitHub PR and issue bodies, most Markdown renderers).
Where the comment would show — a chat message, an email — delete the
first line by hand before pasting. State the reader you assumed if the
request did not name one.

Then keep a local record. Write the declaration, the final text and the
reader you wrote for to `.ai/osekkai/<YYYY-MM-DD>-<slug>.md` in the
repository you are working in (`$XDG_DATA_HOME/osekkai/` when there is
no repository). `.ai/` is gitignored on the author's machine and is
never committed: the source text may hold things that must not leave the
machine, and the record exists so the author can later reread what was
written and refine these steps. The osekkai repository itself only holds
fictional examples.

## Worked examples

`tests/fixtures/examples/` in the osekkai repository holds the author's
accepted outputs: one investigation memo for a teammate, and one
discussion summarized twice, for an outsider and for the manager who has
to decide. Read them to see the state change the shape before writing a
new kind of document.
