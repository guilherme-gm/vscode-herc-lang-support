module.exports = grammar({
	name: 'hercscript',

	rules: {
		source_file: $ => repeat($._header),

		_header: $ => choice(
			$.script_def
			// TODO: other kinds of definitions
		),

		script_def: $ => seq(
			$.position,
			//'\t',
			'script',
			//'\t',
			'Test', //npc name
			//'\t',
			$.npc_sprite,
			optional(seq(',', $.npc_area)),
			',',
			$.block
		),

		position: $ => choice(
			seq($.identifier, ',', $.number, ',', $.number, ',', $.number),
			'-'
		), // TODO : direction is optional for portals

		npc_sprite: $ => seq(
			$._identifier
		),

		npc_area: $ => seq(
			$.number, ',', $.number
		),

		block: $ => seq(
			'{',
			'}'
		),

		identifier: $ => $._identifier,
		_identifier: $ => seq(
			optional(choice(
				"$@", ".@", "##",
				"$", "@", ".", "'", "#"
			)),
			/[a-zA-Z_0-9]+/,
			optional('$')
		),
		number: $ => /\d+/,
        string: $ => seq(
            '"',
            repeat(token.immediate(prec(1, /[^\\"\n]+/))),
            '"'
		), // TODO : Escaped strings

	}
});
