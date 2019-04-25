module.exports = grammar({
    name: 'hercscript',

    externals: $ => [
        $.npc_name
    ],

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
            $.npc_name,
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
            repeat($._statement),
            '}'
        ),

        _statement: $ => choice(
            $.return_statement,
            $.function_stmt
            // TODO: other kinds of statements
        ),

        function_stmt: $ => seq(
            $.identifier,
            $.parameter_list,
            ';'
        ),

        parameter_list: $ => seq(
            '(',
            optional($._param),
            ')'
        ),

        _param: $ => seq(
            $._expression, optional(seq(',', optional($._param)))
        ),

        return_statement: $ => seq(
            'return',
            $._expression,
            ';'
        ),

        _expression: $ => choice(
            $.mulop,
            $.plusop,
            $.number,
            $.identifier,
            $.string,
            seq('(', $._expression, ')')
            // TODO: other kinds of expressions
        ),

        mulop: $ => prec.left(1, seq($._expression, choice('*', '/'), $._expression)),
        plusop: $ => prec.left(2, seq($._expression, choice('+', '-'), $._expression)),

        number: $ => /\d+/,
        string: $ => /".*?"/, // TODO : Escaped strings
        identifier: $ => $._identifier,
        _identifier: $ => /[a-zA-Z_0-9]+/
    }
});