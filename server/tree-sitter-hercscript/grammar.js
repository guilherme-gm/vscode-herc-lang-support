const PREC = {
	PAREN_DECLARATOR: -10,
	ASSIGNMENT: -1,
	CONDITIONAL: -2,
	OLD_FUNCTION: -1,
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
	SUBSCRIPT: 16, // Arrays []
	TOP_LEVEL: 30,
};

module.exports = grammar({
	name: 'hercscript',

	// word: $ => $.identifier,

	extras: $ => [
		/\s/,
		$.comment,
	],

	rules: {
		source_file: $ => repeat($._definition),

		_definition: $ => choice(
			// TODO: mapflag flags
			// $.mapflag_def,
			$.duplicate_def,
			$.function_def,
			$.script_def,
			$.shop_def,
			$.spawn_def,
			$.warp_def,
		),

		// mapflag_def: $ => seq(
		// 	field('map', $.identifier),
		// 	/\t/,
		// 	'mapflag',
		// 	/\t/,
		// 	field('flag', ),
		// ),

		duplicate_def: $ => prec(PREC.TOP_LEVEL, seq(
			field('position', $.position), /\t/,
			seq('duplicate', '(', field('src_npc', $.npc_name) ,')'), /\t/,
			field('new_npc', $.npc_name), /\t/,
			choice(
				seq(field('span_x', $.number_literal), ',', field('span_y', $.number_literal)),
				seq(field('sprite', $.identifier), field('trigger', optional($.trigger_area)))
			)
		)),

		function_def: $ => prec(PREC.TOP_LEVEL, seq(
			'function', /\t/, 'script', /\t/, field('func_name', $.identifier), /\t/,
			field('body', $.block),
		)),

		script_def: $ => prec(PREC.TOP_LEVEL, seq(
			field('position', $.position),
			/\t/,
			field('type', choice('script', 'trader')),
			/\t/,
			field('name', $.npc_name),
			/\t/,
			field('sprite', $.identifier),
			optional($.trigger_area),
			',',
			field('body', $.block),
		)),

		shop_def: $ => prec(PREC.TOP_LEVEL, seq(
			field('position', $.position),
			/\t/,
			field('type', choice('shop', 'cashshop')),
			/\t/,
			field('name', $.npc_name),
			/\t/,
			field('sprite', $.identifier),',',
			field('item', seq($.shop_item, repeat(seq(',', $.shop_item)))),
		)),

		shop_item: $ => seq($.number_literal, ':', $.number_literal),

		spawn_def: $ => prec(PREC.TOP_LEVEL, seq(
			field('map', $.identifier), ',',
			field('x1', $.number_literal), ',',
			field('y1', $.number_literal), ',',
			field('x2', $.number_literal), ',',
			field('y2', $.number_literal), /\t/,
			field('type', choice('monster', 'boss_monster', 'miniboss_monster')), /\t/,
			field('name', $.npc_name), /\t/, ///[^\t]+/), /\t/, // TODO: Level Name
			field('id', $.number_literal), ',',
			field('amount', $.number_literal), ',',
			field('delay1', $.number_literal), ',',
			field('delay2', $.number_literal), ',',
			field('event', choice($.string_literal, '0')),
			optional(seq(
				',', field('size', $.number_literal),
				',', field('ai', $.number_literal),
			)),
		)),

		warp_def: $ => prec(PREC.TOP_LEVEL, seq(
			field('position', $.position),
			/\t/, 'warp', /\t/,
			field('name', $.npc_name), /\t/,
			field('span_x', $.number_literal), ',',
			field('span_y', $.number_literal), ',',
			field('to_map', $.identifier), ',',
			field('to_x', $.number_literal), ',',
			field('to_y', $.number_literal),
		)),

		npc_name: $ => choice(
			seq($.visible_name, optional($.hidden_name), optional($.unique_name)),
			seq($.hidden_name, optional($.unique_name)),
			$.unique_name
		),

		visible_name: $ => $._name_part,
		hidden_name: $ => seq('#', $._name_part),
		unique_name: $ => seq('::', $._name_part),
		_name_part: $ => /[^\t(::)#]+/,

		position: $ => choice(
			seq(
				field('map', $.identifier), ',',
				field('x', $.number_literal), ',',
				field('y', $.number_literal), optional(seq(
					',', field('dir', $.number_literal)
				))
			),
			'-'
		),

		trigger_area: $ => seq(',', field('span_x', $.number_literal), ',', field('span_y', $.number_literal)),

		block: $ => seq(
			'{',
			repeat(field('stmt', $._statement)),
			'}'
		),

		init_declarator: $ => seq(
			field('declarator', choice($._scoped_identifier, $._identifier)),
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
			$.goto_statement,
			$.end_statement,
			$.function_declaration,
			$.function_statement,
			$.old_function,
		),

		labeled_statement: $ => seq(
			field('label', $._statement_identifier),
			':',
			field('body', $._statement)
		),

		expression_statement: $ => seq(
			field('expr', optional($._expression)),
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
			field('body', repeat($._non_case_statement))
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
			field("body", $._statement)
		),

		return_statement: $ => seq(
			'return',
			field('value', optional(choice($._expression, $.comma_expression))), // TODO: Is comma a thing?
			';'
		),

		break_statement: $ => seq(
			'break', ';'
		),

		end_statement: $ => seq(
			'end', ';'
		),

		function_declaration: $ => seq(
			'function',
			field('name', $.identifier),
			field('body', $.block),
		),

		function_statement: $ => seq(
			'function',
			field('name', $.identifier),
			';'
		),

		continue_statement: $ => seq(
			'continue', ';'
		),

		old_function: $ => prec(PREC.OLD_FUNCTION, seq(
			field('function', $.identifier),
			optional(field('arguments', $.old_function_args)),
			';'
		)),

		old_function_args: $ => seq(
			field('param', $._expression),
			optional(repeat(seq(
				',', field('param', $._expression)
			)))
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
			$.scoped_identifier,
			$.identifier,
			$.number_literal,
			$.string,
			$.true,
			$.false,
			$.parenthesized_expression,
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
			$.scoped_identifier,
			$.identifier,
			$.subscript_expression,
			$.parenthesized_expression
		),

		assignment_expression: $ => prec.right(PREC.ASSIGNMENT, seq(
			field('left', $._assignment_left_expression),
			field('operator', choice(
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
			)),
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

		argument_list: $ => seq(
			'(', commaSep($._expression, 'param'), ')'
		),

		parenthesized_expression: $ => seq(
			'(',
			field("expr", choice($._expression, $.comma_expression)), // TODO: Is comma expression a thing?
			')'
		),

		//----------------


		true: $ => 'true',
		false: $ => 'false',

		number_literal: $ => choice(/0x\d+/, /\d+/),

		string: $ => seq(field('line', $.string_literal), repeat(field('line', $.string_literal))),

		string_literal: $ => seq(
			'"',
			repeat(token.immediate(prec(1, /[^\\"\n]+/))),
			'"'
		), // TODO : Escaped strings

		// _func_name: $ => prec(100, /[a-zA-Z_0-9]+/),

		scoped_identifier: $ => prec(10, $._scoped_identifier),
		_scoped_identifier: $ => prec(10, choice(
			seq(choice(
				"$@", ".@", "##",
				"$", "@", ".", "'", "#"
			),
			$._identifier,
			optional('$')
			),
			seq($._identifier, '$')
		)),

		identifier: $ => $._identifier,
		_identifier: $ => /[a-zA-Z_0-9]+/,

		_statement_identifier: $ => alias($.identifier, $.statement_identifier),
		// http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
		comment: $ => token(choice(
			seq('//', /(\\[.\n]|[^\\\n])*/),
			seq(
				'/*',
				/[^*]*\*+([^/*][^*]*\*+)*/,
				'/'
			)
		)),

	}
});

function commaSep(rule, name) {
	return optional(commaSep1(rule, name))
}

function commaSep1(rule, name) {
	return seq(
		field(name, rule),
		repeat(
			choice(
				',',
				seq(',', field(name, rule))
			)
		)
	)
}
