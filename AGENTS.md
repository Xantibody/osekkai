# AGENTS.md

## コンセプト

- r-aizawa にわかりやすい文にまとめるための道具。PR 本文、issue、作業報告、調査メモ、議論の要約の下敷き
- 手順は r-aizawa 個人のまとめ方に依存する。一般的な文章術に寄せない。本人が例題を書いて言語化した作業順と判断を、そのまま skill に持つ
- 意味を読む判断は `skills/osekkai/SKILL.md` に、文字と構造で判定できる規則と守るべき順序は Rust のバイナリに置く

## 構成

| 場所 | 持つもの |
| --- | --- |
| `src/procedure.rs` | 守るべき 4 段。`osekkai steps` が印字し、`--check` が skill のアンカー順を確かめる |
| `src/state.rs` | 読み手のステートの次元と値。guidance（skill が読む）と rules（lint が有効にする）を属性に持つ |
| `src/rules/` | 1 ルール 1 ファイル。しきい値はここにだけ書く |
| `tests/fixtures/<rule>/{ok,ng}/` | ルールごとの fixture。`ng` は `rule@line` を `.expect` に書く |
| `tests/fixtures/examples/` | 本人が受け入れた架空の例題。sidecar が残っているものは、例を直すかルールを直すかが未決 |
| `skills/osekkai/` | skill 本文。4 段のアンカーを持つ |
| `.ai/` | 設計の経緯、plan、例題の原文。gitignore 下で commit しない |
| `~/.osekkai/` | skill が書くまとめ 1 件ごとの作業の記録。リポジトリの外 |

## 守ること

- 新しい lint ルールは、文字と構造だけで判定できるものに限る。意味を読まないと決まらない判断は skill の guidance に書く
- ステートの次元を足すのは、既存の次元で説明がつかない残す・消すの判断が例題に出て、guidance か rule を 1 つは変えるときだけ。読み手の目録（preset）は作らない
- 会話の進み具合を記録する状態機械は作らない。成果物に残る順序だけを検証する
- 実際の仕事の文章はこのリポジトリに置かない。例題は架空のものだけにし、実物は `~/.osekkai/` の記録に残す
- ルールは TDD で 1 ルール 1 コミット。fixture を先に書き、`nix develop -c cargo test` が落ちるのを見てから実装する

## 言語

README、コミット、PR と issue は日本語。コードのコメントとアンカーは英語。skill 本文は英語、`when_to_use` は日本語（dotfiles の skill の慣習）。

## 検査

```sh
nix develop -c cargo test
nix develop -c sh -c 'cargo fmt && cargo clippy --all-targets -- -D warnings'
nix flake check   # ビルド、テスト、rustfmt、skill のアンカー順
```
