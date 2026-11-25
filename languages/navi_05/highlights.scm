(comment) @comment
(string) @string
(escape_sequence) @string.escape
(number) @number
(boolean) @boolean
(color) @constant.color

(type_name) @type
(builtin_type) @type.builtin
(struct_field name: (identifier) @property)
(enum_variant name: (identifier) @variant)
(parameter name: (identifier) @variable.parameter)
(named_argument name: (identifier) @label)
(locale_entry key: (identifier) @property)
(intrinsic (identifier) @attribute)
(member_expression property: (identifier) @property)

(function_declaration name: (identifier) @function)
(call_expression function: (identifier) @function)
(call_expression function: (member_expression property: (identifier) @function.method))

((identifier) @constant.builtin
 (#any-of? @constant.builtin "na" "open" "high" "low" "close" "volume" "bar_index"))

[
  "let" "var" "varip" "export" "fn" "method" "property" "staticmethod" "staticproperty"
  "struct" "enum" "type" "const" "input" "simple" "series"
  "if" "else" "for" "in" "to" "by" "while" "switch" "break" "continue"
] @keyword

"@locale" @attribute

(unary_expression operator: _ @operator)
(binary_expression operator: _ @operator)
(assignment operator: _ @operator)
["=>" "?"] @operator

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
(type_arguments ["<" ">"] @punctuation.bracket)
(type_parameters ["<" ">"] @punctuation.bracket)
["," "." ":" ";"] @punctuation.delimiter
