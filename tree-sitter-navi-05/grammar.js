const PREC = {
  ternary: 1,
  or: 2,
  and: 3,
  equality: 4,
  comparison: 5,
  additive: 6,
  multiplicative: 7,
  unary: 8,
  postfix: 9,
};

function commaSeparated(rule) {
  return optional(seq(rule, repeat(seq(",", rule)), optional(",")));
}

module.exports = grammar({
  name: "navi_05",

  extras: ($) => [/\s/, $.comment],
  word: ($) => $.identifier,
  conflicts: ($) => [
    [$.unary_expression, $.binary_expression, $.call_expression],
    [$.binary_expression, $.call_expression],
    [$.type_name, $._expression],
  ],

  rules: {
    source_file: ($) => repeat($._statement),

    _statement: ($) => choice(
      $.variable_declaration, $.function_declaration, $.struct_declaration,
      $.enum_declaration, $.type_declaration, seq($.assignment, ";"), $.expression_statement,
      $.break_statement, $.continue_statement, ";",
    ),

    block: ($) => seq("{", repeat($._statement), "}"),
    expression_statement: ($) => choice(
      seq($._expression, ";"),
      prec(1, $._control_expression),
    ),
    break_statement: () => prec.right(seq("break", optional(";"))),
    continue_statement: () => prec.right(seq("continue", optional(";"))),

    variable_declaration: ($) => seq(
      optional("export"), choice("let", "var", "varip"),
      field("name", $._binding), optional(seq(":", field("type", $.type))),
      "=", field("value", $._expression), ";",
    ),
    _binding: ($) => choice($.identifier, $.tuple_binding),
    tuple_binding: ($) => seq("(", $.identifier, ",", commaSeparated($.identifier), ")"),

    assignment: ($) => seq(
      field("left", choice($.identifier, $.member_expression, $.index_expression)),
      field("operator", choice("=", "+=", "-=", "*=", "/=", "%=")),
      field("right", $._expression),
    ),

    function_declaration: ($) => seq(
      optional("export"),
      choice("fn", "method", "property", seq(choice("staticmethod", "staticproperty"), "(", $.type, ")")),
      field("name", $.identifier), optional($.type_parameters),
      field("parameters", $.parameters),
      optional(seq(":", field("type", $.type))), field("body", $.block),
    ),
    type_parameters: ($) => seq("<", commaSeparated($.type_name), ">"),
    parameters: ($) => seq("(", commaSeparated($.parameter), ")"),
    parameter: ($) => seq(
      field("name", $.identifier), optional(seq(":", $.type)), optional(seq("=", $._expression)),
    ),

    struct_declaration: ($) => seq(
      optional("export"), "struct", field("name", $.type_name),
      "{", commaSeparated($.struct_field), "}",
    ),
    struct_field: ($) => seq(
      optional("varip"), field("name", $.identifier), ":", $.type, optional(seq("=", $._expression)),
    ),
    enum_declaration: ($) => seq(
      optional("export"), "enum", field("name", $.type_name),
      "{", commaSeparated($.enum_variant), "}",
    ),
    enum_variant: ($) => seq(field("name", $.identifier), optional(seq("=", $._expression))),
    type_declaration: ($) => seq(optional("export"), "type", field("name", $.type_name), "=", $.type, ";"),
    type: ($) => seq(
      optional(choice("const", "input", "simple", "series")),
      choice($.builtin_type, $.type_name, seq("(", commaSeparated($.type), ")")),
      optional($.type_arguments), repeat(seq("[", "]")),
    ),
    type_name: ($) => seq($.identifier, repeat(seq(".", $.identifier))),
    builtin_type: () => choice("int", "float", "bool", "color", "void", "String"),
    type_arguments: ($) => seq("<", commaSeparated($.type), ">"),

    _expression: ($) => choice(
      $.identifier, $.number, $.string, $.boolean, $.color, $.locale_expression,
      $.unary_expression, $.binary_expression, $.conditional_expression,
      $.call_expression, $.member_expression, $.index_expression, $.group_expression,
      $.tuple_expression, $.array_expression, $.intrinsic, $._control_expression,
    ),

    _control_expression: ($) => choice($.if_expression, $.for_expression, $.while_expression, $.switch_expression),
    if_expression: ($) => prec.right(seq(
      "if", field("condition", $._expression), field("consequence", $.block),
      optional(seq("else", field("alternative", choice($.if_expression, $.block)))),
    )),
    for_expression: ($) => seq(
      "for", field("binding", $._binding),
      choice(seq("in", $._expression), seq("=", $._expression, "to", $._expression, optional(seq("by", $._expression)))),
      field("body", $.block),
    ),
    while_expression: ($) => seq("while", field("condition", $._expression), field("body", $.block)),
    switch_expression: ($) => seq(
      "switch", optional($._expression), "{",
      repeat(choice(seq($.switch_arm, ","), seq($.switch_block_arm, optional(",")))),
      optional($.switch_arm), "}",
    ),
    switch_arm: ($) => seq(optional($._expression), "=>", choice($._expression, $.assignment)),
    switch_block_arm: ($) => seq(optional($._expression), "=>", $.block),

    unary_expression: ($) => prec(PREC.unary, seq(
      field("operator", choice("not", "-", "+")), field("operand", $._expression),
    )),
    binary_expression: ($) => choice(
      ...[
        [PREC.or, ["or"]],
        [PREC.and, ["and"]],
        [PREC.equality, ["==", "!="]],
        [PREC.comparison, ["<", "<=", ">", ">="]],
        [PREC.additive, ["+", "-"]],
        [PREC.multiplicative, ["*", "/", "%"]],
      ].map(([precedence, operators]) => prec.left(precedence, seq(
        field("left", $._expression), field("operator", (operators.length === 1 ? operators[0] : choice(...operators))), field("right", $._expression),
      ))),
    ),
    conditional_expression: ($) => prec.right(PREC.ternary, seq(
      field("condition", $._expression), "?", field("consequence", $._expression), ":", field("alternative", $._expression),
    )),

    call_expression: ($) => prec(PREC.postfix, seq(
      field("function", $._expression), optional($.type_arguments), field("arguments", $.arguments),
    )),
    arguments: ($) => seq("(", commaSeparated(choice($._expression, $.named_argument)), ")"),
    named_argument: ($) => seq(field("name", $.identifier), choice(seq(":", optional(field("value", $._expression))), seq("=", field("value", $._expression)))),
    member_expression: ($) => prec(PREC.postfix, seq(
      field("object", $._expression), ".", field("property", $.identifier),
    )),
    index_expression: ($) => prec(PREC.postfix, seq(
      field("object", $._expression), "[", field("index", $._expression), "]",
    )),
    group_expression: ($) => seq("(", $._expression, ")"),
    tuple_expression: ($) => seq("(", $._expression, ",", commaSeparated($._expression), ")"),
    array_expression: ($) => seq("[", commaSeparated($._expression), "]"),
    intrinsic: ($) => seq("@", $.identifier),

    locale_expression: ($) => seq("@locale", "{", commaSeparated($.locale_entry), "}"),
    locale_entry: ($) => seq(field("key", choice($.identifier, $.string)), ":", field("value", $.string)),

    identifier: () => /[\p{XID_Start}_][\p{XID_Continue}_]*/,
    number: () => /(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?/,
    boolean: () => choice("true", "false"),
    color: () => token(/#(?:[\da-fA-F]{8}|[\da-fA-F]{6})/),
    string: ($) => choice(
      seq('"', repeat(choice(token.immediate(/[^"\\]+/), $.escape_sequence)), '"'),
      seq("'", repeat(choice(token.immediate(/[^'\\]+/), $.escape_sequence)), "'"),
    ),
    escape_sequence: () => token.immediate(/\\./),
    comment: () => token(choice(seq("//", /[^\r\n]*/), seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/"))),
  },
});
