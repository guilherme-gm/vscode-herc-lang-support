const PREC = {
	PAREN_DECLARATOR: -10,
	ASSIGNMENT: -1,
	CONDITIONAL: -2,
	DEFAULT: 0,
	LOGICAL_OR: 1,
	LOGICAL_AND: 2,
	INCLUSIVE_OR: 3,
	EXCLUSIVE_OR: 4,
	BITWISE_AND: 5,
	EQUAL: 6,
	RELATIONAL: 7,
	SHIFT: 9,
	ADD: 10,
	MULTIPLY: 11,
	EXPONENTIAL: 12,
	UNARY: 13,
	FUNCTION: 14,
	SUBSCRIPT: 16 // Arrays []
};

module.exports = grammar({
	name: 'hercscript',

	// word: $ => $.identifier,

	// externals: $ => [
	// 	$.npc_name
	// ],

	extras: $ => [
		/\s/,
		$._comment,
	],

	rules: {
		source_file: $ => repeat($._definition),

		_definition: $ => choice(
			$.script_def
			// TODO: other kinds of definitions
		),

		script_def: $ => seq(
			$.position,
			/\t/,
			'script',
			/\t/,
			$.identifier, //$.npc_name
			/\t/,
			field('sprite', $.identifier),
			optional($.trigger_area),
			',',
			field('body', $.block),
		),

		position: $ => choice(
			seq(
				field('map', $.identifier), ',',
				field('x', $.number_literal), ',',
				field('y', $.number_literal), ',',
				field('dir', $.number_literal)
			),
			'-'
		), // TODO : direction is optional for portals

		trigger_area: $ => seq(',', field('spanX', $.number_literal), ',', field('spanY', $.number_literal)),

		block: $ => seq(
			'{',
			repeat($._statement),
			'}'
		),

		init_declarator: $ => seq(
			field('declarator', $._identifier),
			'=',
			field('value', $._expression)
		),

		// Statements

		_statement: $ => choice(
			$.case_statement,
			$._non_case_statement
		),

		_non_case_statement: $ => choice(
			$.labeled_statement,
			$.block,
			$.expression_statement,
			$.if_statement,
			$.switch_statement,
			$.do_statement,
			$.while_statement,
			$.for_statement,
			$.return_statement,
			$.break_statement,
			$.continue_statement,
			$.goto_statement
		),

		labeled_statement: $ => seq(
			field('label', $._statement_identifier),
			':',
			$._statement
		),

		expression_statement: $ => seq(
			optional($._expression),
			';'
		),

		if_statement: $ => prec.right(seq(
			'if',
			field('condition', $.parenthesized_expression),
			field('consequence', $._statement),
			optional(seq(
				'else',
				field('alternative', $._statement)
			))
		)),

		switch_statement: $ => seq(
			'switch',
			field('condition', $.parenthesized_expression),
			field('body', $.block)
		),

		case_statement: $ => prec.right(seq(
			choice(
				seq('case', field('value', $.number_literal)),
				'default',
			),
			':',
			repeat($._non_case_statement)
		)),

		while_statement: $ => seq(
			'while',
			field('condition', $.parenthesized_expression),
			field('body', $._statement)
		),

		do_statement: $ => seq(
			'do',
			field('body', $._statement),
			'while',
			field('condition', $.parenthesized_expression),
			';'
		),

		for_statement: $ => seq(
			'for',
			'(',
			field('initializer', optional($._expression)), ';',
			field('condition', optional($._expression)), ';',
			field('update', optional($._expression)),
			')',
			$._statement
		),

		return_statement: $ => seq(
			'return',
			optional(choice($._expression, $.comma_expression)),
			';'
		),

		break_statement: $ => seq(
			'break', ';'
		),

		continue_statement: $ => seq(
			'continue', ';'
		),

		goto_statement: $ => seq(
			'goto',
			field('label', $._statement_identifier),
			';'
		),


		// Expressions
		_expression: $ => choice(
			$.conditional_expression,
			$.assignment_expression,
			$.binary_expression,
			$.unary_expression,
			$.update_expression,
			$.subscript_expression,
			$.function_expression,
			$.identifier,
			$.number_literal,
			$.string_literal,
			$.true,
			$.false,
			$.parenthesized_expression
		),

		comma_expression: $ => seq(
			field('left', $._expression),
			',',
			field('right', choice($._expression, $.comma_expression))
		),

		conditional_expression: $ => prec.right(PREC.CONDITIONAL, seq(
			field('condition', $._expression),
			'?',
			field('consequence', $._expression),
			':',
			field('alternative', $._expression)
		)),

		_assignment_left_expression: $ => choice(
			$.identifier,
			$.subscript_expression,
			$.parenthesized_expression
		),

		assignment_expression: $ => prec.right(PREC.ASSIGNMENT, seq(
			field('left', $._assignment_left_expression),
			choice(
				'=',
				'+=',
				'-=',
				'*=',
				'**=',
				'/=',
				'%=',
				'<<=',
				'>>=',
				'&=',
				'^=',
				'|='
			),
			field('right', $._expression)
		)),

		unary_expression: $ => prec.left(PREC.UNARY, seq(
			field('operator', choice('!', '~', '-', '+')),
			field('argument', $._expression)
		)),

		binary_expression: $ => {
			const table = [
				['+', PREC.ADD],
				['-', PREC.ADD],
				['*', PREC.MULTIPLY],
				['/', PREC.MULTIPLY],
				['%', PREC.MULTIPLY],
				['**', PREC.EXPONENTIAL],
				['||', PREC.LOGICAL_OR],
				['&&', PREC.LOGICAL_AND],
				['|', PREC.INCLUSIVE_OR],
				['^', PREC.EXCLUSIVE_OR],
				['&', PREC.BITWISE_AND],
				['==', PREC.EQUAL],
				['!=', PREC.EQUAL],
				['~=', PREC.EQUAL],
				['!=', PREC.EQUAL],
				['>', PREC.RELATIONAL],
				['>=', PREC.RELATIONAL],
				['<=', PREC.RELATIONAL],
				['<', PREC.RELATIONAL],
				['<<', PREC.SHIFT],
				['>>', PREC.SHIFT],
			];

			return choice(...table.map(([operator, precedence]) => {
				return prec.left(precedence, seq(
					field('left', $._expression),
					field('operator', operator),
					field('right', $._expression)
				))
			}));
		},

		update_expression: $ => {
			const argument = field('argument', $._expression);
			const operator = field('operator', choice('--', '++'));
			return prec.right(PREC.UNARY, choice(
				seq(operator, argument),
				seq(argument, operator),
			));
		},

		subscript_expression: $ => prec(PREC.SUBSCRIPT, seq(
			field('argument', $._expression),
			'[',
			field('index', $._expression),
			']'
		)),

		function_expression: $ => prec(PREC.FUNCTION, seq(
			field('function', $._expression),
			field('arguments', $.argument_list)
		)),

		argument_list: $ => seq('(', commaSep($._expression), ')'),

		parenthesized_expression: $ => seq(
			'(',
			choice($._expression, $.comma_expression),
			')'
		),

		//----------------


		true: $ => 'true',
		false: $ => 'false',

		number_literal: $ => /\d+/,
		string_literal: $ => seq(
			'"',
			repeat(token.immediate(prec(1, /[^\\"\n]+/))),
			'"'
		), // TODO : Escaped strings

		identifier: $ => $._identifier,
		_identifier: $ => seq(
			optional(choice(
				"$@", ".@", "##",
				"$", "@", ".", "'", "#"
			)),
			/[a-zA-Z_0-9]+/,
			optional('$')
		),

		_statement_identifier: $ => alias($.identifier, $.statement_identifier),
		// http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
		_comment: $ => token(choice(
			seq('//', /(\\[.\n]|[^\\\n])*/),
			seq(
				'/*',
				/[^*]*\*+([^/*][^*]*\*+)*/,
				'/'
			)
		)),

	}
});

function commaSep(rule) {
	return optional(commaSep1(rule))
}

function commaSep1(rule) {
	return seq(rule, repeat(seq(',', rule)))
}
