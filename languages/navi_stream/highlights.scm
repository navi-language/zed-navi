; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume that uppercase names in paths are types
((scoped_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (identifier) @type)
 (#match? @type "^[A-Z]"))
((scoped_type_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (#match? @type "^[A-Z]"))

; Assume other uppercase names are enum constructors
((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

; Assume all qualified names in struct patterns are enum constructors. (They're
; either that, or struct names; highlighting both as constructors seems to be
; the less glaring choice of error, visually.)
(struct_pattern
  type: (scoped_type_identifier
    name: (type_identifier) @constructor))

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.method))
(call_expression
  function: (scoped_identifier
    "."
    name: (identifier) @function))

(keyword_argument key: (identifier) @attribute)

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function.method))

; Function definitions

(function_item (identifier) @function)
(function_signature_item (identifier) @function)

; Other identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

(i18n_key) @constant

(line_comment) @comment
(block_comment) @comment

[
  "("
  ")"
  "["
  "]"
  "{"
  "${"
  "}"
] @punctuation.bracket

(type_arguments [
  "<"
  ">"
] @punctuation.bracket)

(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

[
  "::"
  ":"
  "."
  ","
  ";"
] @punctuation.delimiter

(parameter (identifier) @variable.parameter)

[
  "as"
  "assert"
  "assert_eq"
  "assert_ne"
  "bench"
  "break"
  "case"
  "catch"
  "const"
  "continue"
  "default"
  "defer"
  "do"
  "else"
  "enum"
  "finally"
  "fn"
  "for"
  "if"
  "impl"
  "in"
  "interface"
  "let"
  "loop"
  "pub"
  "return"
  "select"
  "spawn"
  "struct"
  "switch"
  "test"
  "throw"
  "throws"
  "try!"
  "try"
  "type"
  "use"
  "while"
  "meta"
  "param"
  "export"
] @keyword

(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)

(self) @variable.special

[
    (char_literal)
    (string_literal)
    (raw_string_literal)
    (string_template)
] @string

(color_literal) @constant.color

[
    (integer_literal)
    (float_literal)
] @number

[
    (bool_literal)
    (nil_literal)
] @constant.builtin


(escape_sequence) @escape

(attribute_item) @attribute
(inner_attribute_item) @attribute

[
  "*"
  "&"
  "'"
  "?"
  "!"
] @operator
