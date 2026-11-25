# Navi 0.5 Tree-sitter grammar

This grammar parses the .nv dialect documented at https://navi-lang.org/docs/.
It is used for editor features such as highlighting and outline. Navi's compiler
and language server handle language validation.

Run `tree-sitter generate --abi 14` from this directory after editing
`grammar.js`. Zed builds the checked-in parser from `src/parser.c`.
