module.exports = grammar({
	name: "cr8",
	extras: ($) => [$.comment, $._space],

	rules: {
		source_file: ($) => repeat($._item),
		_space: ($) => /[ \t]+?/,
		_newline: ($) => prec.left(3, repeat1("\n")),
		static_word: ($) => /[A-Z_][A-Z_0-9]*/,
		comment: ($) => /;.*?\n/,
		_item: ($) =>
			choice(
				prec.left(11, $.meta),
				prec.left(13, $.instruction),
				prec.left(12, $.label_assignment),
				prec.left(1, repeat1("\n")),
			),
		meta: ($) =>
			seq(
				$.meta_open,
				choice($.boot, $.const, $.dyn, $.macro, $.static, $.use),
			),
		instruction: ($) =>
			seq(
				$.word,
				optional(seq($.value, repeat(seq($.comma, $.value)))),
				choice(prec(90, /\s*?\n/), $.comment),
			),

		word: ($) => /[a-z_][a-z_0-9]*/i,
		label: ($) => /\.?[a-z_][a-z_0-9]*/,
		label_assignment: ($) =>
			token(
				seq(
					alias(/\.?[a-z][a-z_0-9]*/, $.label),
					token.immediate(alias(":", $.colon)),
				),
			),
		macro_variable: ($) => /\$\w[\w_\d]*/,
		number: ($) => choice(/[0-9_]+/, /0x[a-f0-9A-F_]+/i, /0b[01_]+/),
		comma: ($) => token(","),
		slash: ($) => token("/"),
		asterisk: ($) => token("*"),
		plus: ($) => token("+"),
		minus: ($) => token("-"),
		equals: ($) => token("="),
		arrow: ($) => token("=>"),
		paren_open: ($) => token("("),
		paren_close: ($) => token(")"),
		meta_open: ($) => token("#["),
		meta_close: ($) => token("]"),
		bracket_open: ($) => token("["),
		bracket_close: ($) => token("]"),
		mustache_open: ($) => token("{"),
		mustache_close: ($) => token("}"),
		angle_open: ($) => token("<"),
		angle_close: ($) => token(">"),
		angle_angle_left: ($) => token("<<"),
		angle_angle_right: ($) => token(">>"),
		pipe: ($) => token("|"),
		union: ($) => token("|"),
		ampersand: ($) => token("&"),
		hash: ($) => token("#"),
		exclamation: ($) => token("!"),
		percent: ($) => token("%"),
		dot: ($) => token("."),
		colon: ($) => token(":"),
		semi: ($) => token(";"),
		coloncolon: ($) => token("::"),
		dollar: ($) => token("$"),

		boot: ($) =>
			seq(
				token("boot"),
				$.meta_close,
				optional($._newline),
				$.label_assignment,
			),
		const: ($) =>
			seq(
				token("const"),
				$.paren_open,
				$.static_word,
				$.paren_close,
				$.meta_close,
				optional($._newline),
				$.mustache_open,
				optional($._newline),
				seq(
					$.number,
					optional($._newline),
					repeat(
						seq($.comma, optional($._newline), $.number, optional($._newline)),
					),
					optional($.comma),
					optional($._newline),
				),
				$.mustache_close,
			),
		dyn: ($) =>
			seq(
				token("dyn"),
				$.paren_open,
				choice(
					seq($.static_word, $.colon, $.number),
					seq(
						$.ampersand,
						token.immediate(choice(/0b[01_]+/, /0x[a-f0-9_A-F]+/i, /[0-9_]+/)),
					),
				),
				$.paren_close,
				$.meta_close,
			),
		static: ($) =>
			seq(
				token("static"),
				$.paren_open,
				$.static_word,
				$.colon,
				$.number,
				$.paren_close,
				$.meta_close,
			),
		mod: ($) =>
			seq(
				alias($.word, $.path_segment),
				repeat(
					seq(
						token.immediate(alias("::", $.path_sep)),
						alias($.word, $.path_segment),
					),
				),
			),

		use_arg: ($) => choice($.mod, $.string),

		use: ($) =>
			seq(token("use"), $.paren_open, $.use_arg, $.paren_close, $.meta_close),
		string: ($) => token(seq('"', /[^"]*/, '"')),

		macro: ($) =>
			seq(
				token("macro"),
				$.meta_close,
				optional($._newline),
				$.label_assignment,
				$.mustache_open,
				optional($._newline),
				repeat($.macro_capture),
				$.mustache_close,
			),

		macro_capture: ($) =>
			seq(
				$.macro_capture_args,
				$.arrow,
				optional($._newline),
				$.mustache_open,
				optional($._newline),
				repeat($.instruction),
				$.mustache_close,
				optional($._newline),
			),

		macro_capture_args: ($) =>
			seq(
				$.paren_open,
				optional($._newline),
				optional(
					seq(
						$.macro_capture_arg,
						optional($._newline),
						repeat(
							seq(
								$.comma,
								optional($._newline),
								$.macro_capture_arg,
								optional($._newline),
							),
						),
					),
				),
				$.paren_close,
			),

		macro_capture_arg: ($) =>
			seq($.macro_variable, $.colon, $.macro_variable_assignment),

		macro_variable_assignment: ($) =>
			seq($.macro_variable_type, repeat(seq($.union, $.macro_variable_type))),

		macro_variable_type: ($) =>
			choice(token("reg"), token("imm8"), token("imm16")),

		value: ($) =>
			choice(
				$.expr,
				$.number,
				$.register,
				alias(
					token(
						seq(
							/\$[a-z_][a-z_0-9]*/,
							optional(
								seq(
									token.immediate(alias(".", $.dot)),
									choice(token.immediate("l"), token.immediate("h")),
								),
							),
						),
					),
					$.macro_variable,
				),
			),
		register: ($) => seq($.percent, token.immediate(/\w+/)),
		expr: ($) => prec(10, seq($.bracket_open, $.expr_group, $.bracket_close)),
		expr_group: ($) =>
			choice(
				prec.left(0, $.expr_term),
				prec.left(9, seq($.expr_term, $.asterisk, $.expr_term)),
				prec.left(8, seq($.expr_term, $.slash, $.expr_term)),
				prec.left(7, seq($.expr_term, $.angle_angle_left, $.expr_term)),
				prec.left(6, seq($.expr_term, $.angle_angle_right, $.expr_term)),
				prec.left(5, seq($.expr_term, $.ampersand, $.expr_term)),
				prec.left(3, seq($.expr_term, $.pipe, $.expr_term)),
				prec.left(2, seq($.expr_term, $.plus, $.expr_term)),
				prec.left(1, seq($.expr_term, $.minus, $.expr_term)),
			),
		expr_term: ($) =>
			choice(
				prec(70, $.dollar),
				prec(60, $.static_word),
				prec(50, $.number),
				prec(0, $.label),
				$.expr_group,
				prec.left(10, seq($.paren_open, $.expr_group, $.paren_close)),
			),
	},
});
