# osekkai

文章のまとめ方を Claude Code（と codex）に持たせるための skill と、その行動を支えるガードレールの lint。

- `osekkai` は Rust の単体バイナリ。Markdown の箇条の構造を数える lint と、読み手のステートに応じた lint、守るべき手順の印字をする
- `skills/` は文章をまとめる手順を書いた skill

設計の経緯は `.ai/`（gitignore 下）にある。
