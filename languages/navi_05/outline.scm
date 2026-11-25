(function_declaration
  ["fn" "method" "property" "staticmethod" "staticproperty"] @context
  name: (identifier) @name) @item

(struct_declaration "struct" @context name: (type_name) @name) @item
(enum_declaration "enum" @context name: (type_name) @name) @item
(type_declaration "type" @context name: (type_name) @name) @item
