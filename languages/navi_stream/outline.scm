(struct_item
    (visibility_modifier)? @context
    "struct" @context
    name: (_) @name) @item

(field_declaration
    (visibility_modifier)? @context
    name: (_) @name) @item

(enum_item
    (visibility_modifier)? @context
    "enum" @context
    name: (_) @name) @item

(enum_variant
    (visibility_modifier)? @context
    name: (_) @name) @item

(impl_item
    "impl" @context
    trait: (_)? @name
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

(const_item
    (visibility_modifier)? @context
    "const" @context
    name: (_) @name) @item
