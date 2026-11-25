# Navi language support for Zed

[![Zed Extension](https://img.shields.io/badge/-Zed_Extension-blue?style=flat&logo=zedindustries&logoColor=%23FFFFFF&logoSize=auto&labelColor=%23111111&color=%23084CCF)](https://zed.dev/extensions/navi)

This extension provides language support for the [Navi](https://navi-lang.org).

## Usage

For Navi 0.5 scripts, Zed uses `navi-lsp` from PATH when available. Otherwise,
it downloads the official [Navi language server](https://navi-lang.org/docs/install)
and runs its WebAssembly backend with Zed's Node.js runtime.

Legacy Navi and Navi Stream use `navi-lsp-server` from PATH or download it automatically.
For older projects that use `.nv`, add `"file_types": {"Navi": ["nv"]}` to the
Zed project settings.

## Features

- Syntax highlighting.
- Outline view.
- LSP support.
- Formatting.
- Code actions.

## Languages

- Navi 0.5 (.nv)
- Navi (.navi)
- Navi Stream (.nvs)

## Screenshot

<img width="1058" alt="image" src="https://github.com/navi-language/zed-navi/assets/5518/cb98c2bf-f6f7-4f98-94c4-014c1fd164b2">

<img width="1110" alt="image" src="https://github.com/navi-language/zed-navi/assets/5518/5fda451e-e930-4da2-b4d7-ee8bd873d9ad">

## License

MIT
