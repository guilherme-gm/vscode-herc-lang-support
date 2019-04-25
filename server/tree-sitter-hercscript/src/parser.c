#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 9
#define STATE_COUNT 38
#define SYMBOL_COUNT 27
#define ALIAS_COUNT 0
#define TOKEN_COUNT 12
#define EXTERNAL_TOKEN_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 7

enum {
  anon_sym_script = 1,
  anon_sym_COMMA = 2,
  anon_sym_LBRACE = 3,
  anon_sym_RBRACE = 4,
  anon_sym_SEMI = 5,
  anon_sym_LPAREN = 6,
  anon_sym_RPAREN = 7,
  anon_sym_return = 8,
  sym_number = 9,
  sym_string = 10,
  sym_identifier = 11,
  sym_source_file = 12,
  sym__header = 13,
  sym_script_def = 14,
  sym_position = 15,
  sym_npc_name = 16,
  sym_npc_sprite = 17,
  sym_block = 18,
  sym__statement = 19,
  sym_function_stmt = 20,
  sym_parameter_list = 21,
  sym__param = 22,
  sym_return_statement = 23,
  sym__expression = 24,
  aux_sym_source_file_repeat1 = 25,
  aux_sym_block_repeat1 = 26,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_script] = "script",
  [anon_sym_COMMA] = ",",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_SEMI] = ";",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_return] = "return",
  [sym_number] = "number",
  [sym_string] = "string",
  [sym_identifier] = "identifier",
  [sym_source_file] = "source_file",
  [sym__header] = "_header",
  [sym_script_def] = "script_def",
  [sym_position] = "position",
  [sym_npc_name] = "npc_name",
  [sym_npc_sprite] = "npc_sprite",
  [sym_block] = "block",
  [sym__statement] = "_statement",
  [sym_function_stmt] = "function_stmt",
  [sym_parameter_list] = "parameter_list",
  [sym__param] = "_param",
  [sym_return_statement] = "return_statement",
  [sym__expression] = "_expression",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_script] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_return] = {
    .visible = true,
    .named = false,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__header] = {
    .visible = false,
    .named = true,
  },
  [sym_script_def] = {
    .visible = true,
    .named = true,
  },
  [sym_position] = {
    .visible = true,
    .named = true,
  },
  [sym_npc_name] = {
    .visible = true,
    .named = true,
  },
  [sym_npc_sprite] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym__statement] = {
    .visible = false,
    .named = true,
  },
  [sym_function_stmt] = {
    .visible = true,
    .named = true,
  },
  [sym_parameter_list] = {
    .visible = true,
    .named = true,
  },
  [sym__param] = {
    .visible = false,
    .named = true,
  },
  [sym_return_statement] = {
    .visible = true,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  switch (state) {
    case 0:
      if (lookahead == 0)
        ADVANCE(1);
      if (lookahead == '"')
        ADVANCE(2);
      if (lookahead == '(')
        ADVANCE(3);
      if (lookahead == ')')
        ADVANCE(4);
      if (lookahead == ',')
        ADVANCE(5);
      if (lookahead == ';')
        ADVANCE(6);
      if (lookahead == 'r')
        ADVANCE(7);
      if (lookahead == 's')
        ADVANCE(8);
      if (lookahead == '{')
        ADVANCE(9);
      if (lookahead == '}')
        ADVANCE(10);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(0);
      if (('0' <= lookahead && lookahead <= '9'))
        ADVANCE(11);
      END_STATE();
    case 1:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 2:
      if (lookahead == '"')
        ADVANCE(12);
      if (lookahead != 0 &&
          lookahead != '\n')
        ADVANCE(2);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 7:
      if (lookahead == 'e')
        ADVANCE(13);
      END_STATE();
    case 8:
      if (lookahead == 'c')
        ADVANCE(14);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9'))
        ADVANCE(11);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym_string);
      if (lookahead == '"')
        ADVANCE(12);
      if (lookahead != 0 &&
          lookahead != '\n')
        ADVANCE(2);
      END_STATE();
    case 13:
      if (lookahead == 't')
        ADVANCE(15);
      END_STATE();
    case 14:
      if (lookahead == 'r')
        ADVANCE(16);
      END_STATE();
    case 15:
      if (lookahead == 'u')
        ADVANCE(17);
      END_STATE();
    case 16:
      if (lookahead == 'i')
        ADVANCE(18);
      END_STATE();
    case 17:
      if (lookahead == 'r')
        ADVANCE(19);
      END_STATE();
    case 18:
      if (lookahead == 'p')
        ADVANCE(20);
      END_STATE();
    case 19:
      if (lookahead == 'n')
        ADVANCE(21);
      END_STATE();
    case 20:
      if (lookahead == 't')
        ADVANCE(22);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_return);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_script);
      END_STATE();
    case 23:
      if (lookahead == 0)
        ADVANCE(1);
      if (lookahead == ')')
        ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(23);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 25:
      if (lookahead == ')')
        ADVANCE(4);
      if (lookahead == ',')
        ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(25);
      END_STATE();
    case 26:
      if (lookahead == 's')
        ADVANCE(8);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(26);
      END_STATE();
    case 27:
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(27);
      if (('0' <= lookahead && lookahead <= '9'))
        ADVANCE(11);
      END_STATE();
    case 28:
      if (lookahead == '{')
        ADVANCE(9);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(28);
      END_STATE();
    case 29:
      if (lookahead == 'r')
        ADVANCE(30);
      if (lookahead == '}')
        ADVANCE(10);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(29);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e')
        ADVANCE(31);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't')
        ADVANCE(32);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u')
        ADVANCE(33);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r')
        ADVANCE(34);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n')
        ADVANCE(35);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_return);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 36:
      if (lookahead == '(')
        ADVANCE(3);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(36);
      END_STATE();
    case 37:
      if (lookahead == '"')
        ADVANCE(2);
      if (lookahead == ')')
        ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(37);
      if (('0' <= lookahead && lookahead <= '9'))
        ADVANCE(38);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9'))
        ADVANCE(38);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(24);
      END_STATE();
    case 39:
      if (lookahead == ';')
        ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(39);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 23},
  [2] = {.lex_state = 25},
  [3] = {.lex_state = 26},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 23},
  [6] = {.lex_state = 27},
  [7] = {.lex_state = 23},
  [8] = {.lex_state = 23},
  [9] = {.lex_state = 25},
  [10] = {.lex_state = 23},
  [11] = {.lex_state = 23},
  [12] = {.lex_state = 27},
  [13] = {.lex_state = 25},
  [14] = {.lex_state = 25},
  [15] = {.lex_state = 25},
  [16] = {.lex_state = 28},
  [17] = {.lex_state = 27},
  [18] = {.lex_state = 29},
  [19] = {.lex_state = 23},
  [20] = {.lex_state = 26},
  [21] = {.lex_state = 23},
  [22] = {.lex_state = 36},
  [23] = {.lex_state = 37},
  [24] = {.lex_state = 29},
  [25] = {.lex_state = 37},
  [26] = {.lex_state = 39},
  [27] = {.lex_state = 39},
  [28] = {.lex_state = 23},
  [29] = {.lex_state = 29},
  [30] = {.lex_state = 39},
  [31] = {.lex_state = 23},
  [32] = {.lex_state = 25},
  [33] = {.lex_state = 29},
  [34] = {.lex_state = 29},
  [35] = {.lex_state = 39},
  [36] = {.lex_state = 37},
  [37] = {.lex_state = 23},
};

static uint16_t ts_parse_table[STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [anon_sym_SEMI] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_script] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_return] = ACTIONS(1),
  },
  [1] = {
    [sym_position] = STATE(3),
    [sym__header] = STATE(5),
    [sym_script_def] = STATE(5),
    [sym_source_file] = STATE(4),
    [aux_sym_source_file_repeat1] = STATE(5),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_identifier] = ACTIONS(5),
  },
  [2] = {
    [anon_sym_COMMA] = ACTIONS(7),
  },
  [3] = {
    [anon_sym_script] = ACTIONS(9),
  },
  [4] = {
    [ts_builtin_sym_end] = ACTIONS(11),
  },
  [5] = {
    [sym_position] = STATE(3),
    [sym__header] = STATE(8),
    [sym_script_def] = STATE(8),
    [aux_sym_source_file_repeat1] = STATE(8),
    [ts_builtin_sym_end] = ACTIONS(13),
    [sym_identifier] = ACTIONS(5),
  },
  [6] = {
    [sym_number] = ACTIONS(15),
  },
  [7] = {
    [sym_npc_name] = STATE(11),
    [sym_identifier] = ACTIONS(17),
  },
  [8] = {
    [sym_position] = STATE(3),
    [sym__header] = STATE(8),
    [sym_script_def] = STATE(8),
    [aux_sym_source_file_repeat1] = STATE(8),
    [sym_identifier] = ACTIONS(19),
    [ts_builtin_sym_end] = ACTIONS(22),
  },
  [9] = {
    [anon_sym_COMMA] = ACTIONS(24),
  },
  [10] = {
    [sym_identifier] = ACTIONS(26),
  },
  [11] = {
    [sym_npc_sprite] = STATE(14),
    [sym_identifier] = ACTIONS(28),
  },
  [12] = {
    [sym_number] = ACTIONS(30),
  },
  [13] = {
    [anon_sym_COMMA] = ACTIONS(32),
  },
  [14] = {
    [anon_sym_COMMA] = ACTIONS(34),
  },
  [15] = {
    [anon_sym_COMMA] = ACTIONS(36),
  },
  [16] = {
    [sym_block] = STATE(19),
    [anon_sym_LBRACE] = ACTIONS(38),
  },
  [17] = {
    [sym_number] = ACTIONS(40),
  },
  [18] = {
    [aux_sym_block_repeat1] = STATE(24),
    [sym_return_statement] = STATE(24),
    [sym__statement] = STATE(24),
    [sym_function_stmt] = STATE(24),
    [anon_sym_return] = ACTIONS(42),
    [sym_identifier] = ACTIONS(44),
    [anon_sym_RBRACE] = ACTIONS(46),
  },
  [19] = {
    [sym_identifier] = ACTIONS(48),
    [ts_builtin_sym_end] = ACTIONS(48),
  },
  [20] = {
    [anon_sym_script] = ACTIONS(50),
  },
  [21] = {
    [sym_identifier] = ACTIONS(52),
    [ts_builtin_sym_end] = ACTIONS(52),
  },
  [22] = {
    [sym_parameter_list] = STATE(26),
    [anon_sym_LPAREN] = ACTIONS(54),
  },
  [23] = {
    [sym__expression] = STATE(27),
    [sym_number] = ACTIONS(56),
    [sym_string] = ACTIONS(58),
    [sym_identifier] = ACTIONS(56),
  },
  [24] = {
    [aux_sym_block_repeat1] = STATE(29),
    [sym_return_statement] = STATE(29),
    [sym__statement] = STATE(29),
    [sym_function_stmt] = STATE(29),
    [anon_sym_return] = ACTIONS(42),
    [sym_identifier] = ACTIONS(44),
    [anon_sym_RBRACE] = ACTIONS(60),
  },
  [25] = {
    [sym__param] = STATE(31),
    [sym__expression] = STATE(32),
    [sym_string] = ACTIONS(62),
    [anon_sym_RPAREN] = ACTIONS(64),
    [sym_number] = ACTIONS(66),
    [sym_identifier] = ACTIONS(66),
  },
  [26] = {
    [anon_sym_SEMI] = ACTIONS(68),
  },
  [27] = {
    [anon_sym_SEMI] = ACTIONS(70),
  },
  [28] = {
    [sym_identifier] = ACTIONS(72),
    [ts_builtin_sym_end] = ACTIONS(72),
  },
  [29] = {
    [aux_sym_block_repeat1] = STATE(29),
    [sym_return_statement] = STATE(29),
    [sym__statement] = STATE(29),
    [sym_function_stmt] = STATE(29),
    [anon_sym_return] = ACTIONS(74),
    [sym_identifier] = ACTIONS(77),
    [anon_sym_RBRACE] = ACTIONS(80),
  },
  [30] = {
    [anon_sym_SEMI] = ACTIONS(82),
  },
  [31] = {
    [anon_sym_RPAREN] = ACTIONS(84),
  },
  [32] = {
    [anon_sym_COMMA] = ACTIONS(86),
    [anon_sym_RPAREN] = ACTIONS(88),
  },
  [33] = {
    [anon_sym_return] = ACTIONS(90),
    [sym_identifier] = ACTIONS(90),
    [anon_sym_RBRACE] = ACTIONS(92),
  },
  [34] = {
    [anon_sym_return] = ACTIONS(94),
    [sym_identifier] = ACTIONS(94),
    [anon_sym_RBRACE] = ACTIONS(96),
  },
  [35] = {
    [anon_sym_SEMI] = ACTIONS(98),
  },
  [36] = {
    [sym__param] = STATE(37),
    [sym__expression] = STATE(32),
    [sym_string] = ACTIONS(62),
    [anon_sym_RPAREN] = ACTIONS(100),
    [sym_number] = ACTIONS(66),
    [sym_identifier] = ACTIONS(66),
  },
  [37] = {
    [anon_sym_RPAREN] = ACTIONS(102),
  },
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.count = 0, .reusable = false},
  [1] = {.count = 1, .reusable = false}, RECOVER(),
  [3] = {.count = 1, .reusable = true}, REDUCE(sym_source_file, 0),
  [5] = {.count = 1, .reusable = true}, SHIFT(2),
  [7] = {.count = 1, .reusable = true}, SHIFT(6),
  [9] = {.count = 1, .reusable = true}, SHIFT(7),
  [11] = {.count = 1, .reusable = true},  ACCEPT_INPUT(),
  [13] = {.count = 1, .reusable = true}, REDUCE(sym_source_file, 1),
  [15] = {.count = 1, .reusable = true}, SHIFT(9),
  [17] = {.count = 1, .reusable = true}, SHIFT(10),
  [19] = {.count = 2, .reusable = true}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(2),
  [22] = {.count = 1, .reusable = true}, REDUCE(aux_sym_source_file_repeat1, 2),
  [24] = {.count = 1, .reusable = true}, SHIFT(12),
  [26] = {.count = 1, .reusable = true}, REDUCE(sym_npc_name, 1),
  [28] = {.count = 1, .reusable = true}, SHIFT(13),
  [30] = {.count = 1, .reusable = true}, SHIFT(15),
  [32] = {.count = 1, .reusable = true}, REDUCE(sym_npc_sprite, 1),
  [34] = {.count = 1, .reusable = true}, SHIFT(16),
  [36] = {.count = 1, .reusable = true}, SHIFT(17),
  [38] = {.count = 1, .reusable = true}, SHIFT(18),
  [40] = {.count = 1, .reusable = true}, SHIFT(20),
  [42] = {.count = 1, .reusable = false}, SHIFT(23),
  [44] = {.count = 1, .reusable = false}, SHIFT(22),
  [46] = {.count = 1, .reusable = true}, SHIFT(21),
  [48] = {.count = 1, .reusable = true}, REDUCE(sym_script_def, 6),
  [50] = {.count = 1, .reusable = true}, REDUCE(sym_position, 7),
  [52] = {.count = 1, .reusable = true}, REDUCE(sym_block, 2),
  [54] = {.count = 1, .reusable = true}, SHIFT(25),
  [56] = {.count = 1, .reusable = false}, SHIFT(27),
  [58] = {.count = 1, .reusable = true}, SHIFT(27),
  [60] = {.count = 1, .reusable = true}, SHIFT(28),
  [62] = {.count = 1, .reusable = true}, SHIFT(32),
  [64] = {.count = 1, .reusable = true}, SHIFT(30),
  [66] = {.count = 1, .reusable = false}, SHIFT(32),
  [68] = {.count = 1, .reusable = true}, SHIFT(33),
  [70] = {.count = 1, .reusable = true}, SHIFT(34),
  [72] = {.count = 1, .reusable = true}, REDUCE(sym_block, 3),
  [74] = {.count = 2, .reusable = false}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(23),
  [77] = {.count = 2, .reusable = false}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(22),
  [80] = {.count = 1, .reusable = true}, REDUCE(aux_sym_block_repeat1, 2),
  [82] = {.count = 1, .reusable = true}, REDUCE(sym_parameter_list, 2),
  [84] = {.count = 1, .reusable = true}, SHIFT(35),
  [86] = {.count = 1, .reusable = true}, SHIFT(36),
  [88] = {.count = 1, .reusable = true}, REDUCE(sym__param, 1),
  [90] = {.count = 1, .reusable = false}, REDUCE(sym_function_stmt, 3),
  [92] = {.count = 1, .reusable = true}, REDUCE(sym_function_stmt, 3),
  [94] = {.count = 1, .reusable = false}, REDUCE(sym_return_statement, 3),
  [96] = {.count = 1, .reusable = true}, REDUCE(sym_return_statement, 3),
  [98] = {.count = 1, .reusable = true}, REDUCE(sym_parameter_list, 3),
  [100] = {.count = 1, .reusable = true}, REDUCE(sym__param, 2),
  [102] = {.count = 1, .reusable = true}, REDUCE(sym__param, 3),
};

#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_hercscript() {
  static TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .symbol_metadata = ts_symbol_metadata,
    .parse_table = (const unsigned short *)ts_parse_table,
    .parse_actions = ts_parse_actions,
    .lex_modes = ts_lex_modes,
    .symbol_names = ts_symbol_names,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .lex_fn = ts_lex,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
  };
  return &language;
}
