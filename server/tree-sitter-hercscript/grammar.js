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
            $.npc_name,
            //'\t',
            $.npc_sprite,
            ',',
            $.block
        ),

        position: $ => seq(
            $.identifier,
            ',',
            $.number,
            ',',
            $.number,
            ',',
            $.number

            // TODO : Floating npc / Optional lookin (for portals)
        ),

        npc_name: $ => seq(
            $.identifier
            // TODO : Hidden name, unique name, spaces
        ),

        npc_sprite: $ => seq(
            $.identifier
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
        identifier: $ => /[a-zA-Z_0-9]+/
    }
});