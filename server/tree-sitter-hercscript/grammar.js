module.exports = grammar({
    name: 'hercscript',

    externals: $ => [
        $.npc_name
    ],

    extras: $ => [
        /\s/,
        $._comment,
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
            $.break_stmt,
            $.if_stmt,
            $.switch_stmt,
            seq($.function_stmt, ';'),
            $.block,
            seq($.assignment_stmt, ';'),
            // TODO: other kinds of statements
        ),

        function_stmt: $ => seq(
            $.identifier,
            $.parameter_list,
        ),

        parameter_list: $ => seq(
            '(',
            optional($._param),
            ')'
        ),

        _param: $ => seq(
            $._expression,
            optional(seq(',', optional($._param)))
        ),

        if_stmt: $ => prec.right(seq(
            'if',
            '(',
            $._expression,
            ')',
            $._statement,
            optional(seq('else', $._statement))
        )),

        switch_stmt: $ => seq(
            'switch',
            '(', $._expression, ')',
            alias($.switch_body, $.block)
        ),

        switch_body: $ => seq(
            '{',
            repeat(choice($.case_stmt, $._statement)),
            '}'
        ),

        case_stmt: $ => prec.right(seq(
            choice(
                seq('case', $.number),
                'default'
            ),
            ':',
            repeat($._statement)
        )),

        break_stmt: $ => seq(
            'break;'
        ),

        return_statement: $ => seq(
            'return',
            $._expression,
            ';'
        ),

        assignment_stmt: $ => prec.right(7, seq($.identifier, choice('=', '+=', '-=', '*=', '**=', '/=', '%=', '<<=', '>>=', '&=', '^=', "|="), $._expression)),

        _expression: $ => choice(
            $.function_stmt,
            $.mulop,
            $.plusop,
            $.compareop,
            $.bitwiseop,
            $.logicalop,
            $.ternary,
            $.number,
            $.identifier,
            $.string,
            seq('(', $._expression, ')')
            // TODO: other kinds of expressions
        ),

        mulop: $ => prec.left(1, seq($._expression, choice('*', '/'), $._expression)),
        plusop: $ => prec.left(2, seq($._expression, choice('+', '-'), $._expression)),
        compareop: $ => prec.left(3, seq($._expression, choice('<', '<=', '>', '>=', '==', '!=', '~=', '~!'), $._expression)),
        bitwiseop: $ => prec.left(4, seq($._expression, choice('&', '^', '|'), $._expression)),
        logicalop: $ => prec.left(5, seq($._expression, choice('&&', '||'), $._expression)),
        ternary: $ => prec.right(6, seq($._expression, '?', $._expression, ':', $._expression)),


        number: $ => /\d+/,
        string: $ => seq(
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