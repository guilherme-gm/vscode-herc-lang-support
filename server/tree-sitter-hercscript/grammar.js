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
            $.number,
            $.identifier,
            $.string
            // TODO: other kinds of expressions
        ),

        number: $ => /\d+/,
        string: $ => /".*?"/, // TODO : Escaped strings
        identifier: $ => $._identifier,
        _identifier: $ => /[a-zA-Z_0-9]+/
    }
});