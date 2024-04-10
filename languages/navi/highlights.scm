; Identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume uppercase names are enum constructors
((identifier) @type
 (#match? @type "^[A-Z]"))

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

(closure_type [
    "|"
] @punctuation.bracket)
(closure_parameters [
  "|"
] @punctuation.bracket)

(type_arguments [
  "<"
  ">"
] @punctuation.bracket)

[
  "::"
  ":"
  "."
  ","
  ";"
] @punctuation.delimiter

(parameter (identifier) @variable.parameter)

[
  "alias"
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
  "is"
  "let"
  "loop"
  "panic"
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
] @keyword

(self) @variable.special
(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)

[
    (char_literal)
    (string_literal)
    (raw_string_literal)
    (string_template)
] @string

(string_template_substitution
    "${" @punctuation.special
     "}" @punctuation.special) @embedded

[
    (bool_literal)
    (nil_literal)
] @constant.builtin

[
    (integer_literal)
    (float_literal)
] @number

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
