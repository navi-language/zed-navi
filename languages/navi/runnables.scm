(
    (test_item name: (_) @run)
) @navi-test

(
    (bench_item name: (_) @run)
) @navi-bench

(
    (source_file) @run
) @navi-file

(
    (function_item
        name: (_) @_name
        (#match? @_name "main")) @run
) @navi-run
