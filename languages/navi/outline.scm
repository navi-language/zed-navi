(struct_item
    (visibility_modifier)? @context
    "struct" @context
    name: (_) @name) @item

(enum_item
    (visibility_modifier)? @context
    "enum" @context
    name: (_) @name) @item

(impl_item
    "impl" @context
    interface: (_)? @name
    "for"? @context
    type: (_) @name) @item

(interface_item
    (visibility_modifier)? @context
    "interface" @context
    name: (_) @name) @item

(function_item
    (visibility_modifier)? @context
    "fn" @context
    name: (_) @name) @item

(type_item
    (visibility_modifier)? @context
    "type" @context
    name: (_) @name) @item

(associated_type
    "type" @context
    name: (_) @name) @item

(test_item
    "test" @context
    name: (_) @name) @item

(bench_item
    "bench" @context
    name: (_) @name) @item
