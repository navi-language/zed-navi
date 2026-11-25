# Navi language support for Zed

[![Zed Extension](https://img.shields.io/badge/-Zed_Extension-blue?style=flat&logo=zedindustries&logoColor=%23FFFFFF&logoSize=auto&labelColor=%23111111&color=%23084CCF)](https://zed.dev/extensions/navi)

This extension provides language support for the [Navi](https://navi-lang.org).

## Navi 0.5

Open a `.nv` script to get syntax highlighting, document outline, diagnostics,
completion, hover, go-to-definition, and formatting. Zed uses `navi-lsp` if it
finds it on PATH. Otherwise, it downloads Longbridge's official WebAssembly
language server and runs it with Zed's Node.js runtime.

Install the [Navi CLI](https://navi-lang.org/docs/install) if you want to use the
`navi check` task. The language server works without the CLI.

## Legacy projects

`.navi` files open as **Navi Legacy** and `.nvs` files open as **Navi Stream**.
They use the original grammars and `navi-lsp-server`. Older projects that use
`.nv` can associate it with **Navi Legacy** in their Zed project settings:

```json
{
  "file_types": {
    "Navi Legacy": ["nv"]
  }
}
```

## License

MIT
