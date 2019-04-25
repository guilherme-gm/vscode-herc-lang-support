#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 9
#define STATE_COUNT 38
#define SYMBOL_COUNT 28
#define ALIAS_COUNT 0
#define TOKEN_COUNT 13
#define EXTERNAL_TOKEN_COUNT 1
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
  sym__identifier = 11,
  sym_npc_name = 12,
  sym_source_file = 13,
  sym__header = 14,
  sym_script_def = 15,
  sym_position = 16,
  sym_npc_sprite = 17,
  sym_block = 18,
  sym__statement = 19,
  sym_function_stmt = 20,
  sym_parameter_list = 21,
  sym__param = 22,
  sym_return_statement = 23,
  sym__expression = 24,
  sym_identifier = 25,
  aux_sym_source_file_repeat1 = 26,
  aux_sym_block_repeat1 = 27,
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
  [sym__identifier] = "_identifier",
  [sym_npc_name] = "npc_name",
  [sym_source_file] = "source_file",
  [sym__header] = "_header",
  [sym_script_def] = "script_def",
  [sym_position] = "position",
  [sym_npc_sprite] = "npc_sprite",
  [sym_block] = "block",
  [sym__statement] = "_statement",
  [sym_function_stmt] = "function_stmt",
  [sym_parameter_list] = "parameter_list",
  [sym__param] = "_param",
  [sym_return_statement] = "return_statement",
  [sym__expression] = "_expression",
  [sym_identifier] = "identifier",
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
  [sym__identifier] = {
    .visible = false,
    .named = true,
  },
  [sym_npc_name] = {
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
  [sym_identifier] = {
    .visible = true,
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
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 1:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 2:
      if (lookahead == '"')
        ADVANCE(13);
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
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == 'e')
        ADVANCE(14);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == 'c')
        ADVANCE(15);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
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
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym__identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_string);
      if (lookahead == '"')
        ADVANCE(13);
      if (lookahead != 0 &&
          lookahead != '\n')
        ADVANCE(2);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == 't')
        ADVANCE(16);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == 'r')
        ADVANCE(17);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == 'u')
        ADVANCE(18);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == 'i')
        ADVANCE(19);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == 'r')
        ADVANCE(20);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == 'p')
        ADVANCE(21);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == 'n')
        ADVANCE(22);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(sym__identifier);
      if (lookahead == 't')
        ADVANCE(23);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_return);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_script);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 24:
      if (lookahead == 0)
        ADVANCE(1);
      if (lookahead == ')')
        ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(24);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 25:
      if (lookahead == '(')
        ADVANCE(3);
      if (lookahead == ')')
        ADVANCE(4);
      if (lookahead == ',')
        ADVANCE(5);
      if (lookahead == ';')
        ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(25);
      END_STATE();
    case 26:
      if (lookahead == 's')
        ADVANCE(27);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(26);
      END_STATE();
    case 27:
      if (lookahead == 'c')
        ADVANCE(28);
      END_STATE();
    case 28:
      if (lookahead == 'r')
        ADVANCE(29);
      END_STATE();
    case 29:
      if (lookahead == 'i')
        ADVANCE(30);
      END_STATE();
    case 30:
      if (lookahead == 'p')
        ADVANCE(31);
      END_STATE();
    case 31:
      if (lookahead == 't')
        ADVANCE(32);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_script);
      END_STATE();
    case 33:
      if (lookahead == ')')
        ADVANCE(4);
      if (lookahead == ',')
        ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(33);
      END_STATE();
    case 34:
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(34);
      if (('0' <= lookahead && lookahead <= '9'))
        ADVANCE(35);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9'))
        ADVANCE(35);
      END_STATE();
    case 36:
      if (lookahead == '{')
        ADVANCE(9);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(36);
      END_STATE();
    case 37:
      if (lookahead == 'r')
        ADVANCE(7);
      if (lookahead == '}')
        ADVANCE(10);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(37);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 38:
      if (lookahead == '"')
        ADVANCE(2);
      if (lookahead == ')')
        ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(38);
      if (('0' <= lookahead && lookahead <= '9'))
        ADVANCE(11);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z'))
        ADVANCE(12);
      END_STATE();
    case 39:
      if (lookahead == '(')
        ADVANCE(3);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(39);
      END_STATE();
    case 40:
      if (lookahead == ';')
        ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ')
        SKIP(40);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 24},
  [2] = {.lex_state = 25},
  [3] = {.lex_state = 26},
  [4] = {.lex_state = 33},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 24},
  [7] = {.lex_state = 0, .external_lex_state = 1},
  [8] = {.lex_state = 34},
  [9] = {.lex_state = 24},
  [10] = {.lex_state = 24},
  [11] = {.lex_state = 33},
  [12] = {.lex_state = 33},
  [13] = {.lex_state = 33},
  [14] = {.lex_state = 34},
  [15] = {.lex_state = 36},
  [16] = {.lex_state = 33},
  [17] = {.lex_state = 37},
  [18] = {.lex_state = 24},
  [19] = {.lex_state = 34},
  [20] = {.lex_state = 24},
  [21] = {.lex_state = 38},
  [22] = {.lex_state = 37},
  [23] = {.lex_state = 39},
  [24] = {.lex_state = 26},
  [25] = {.lex_state = 40},
  [26] = {.lex_state = 24},
  [27] = {.lex_state = 37},
  [28] = {.lex_state = 38},
  [29] = {.lex_state = 40},
  [30] = {.lex_state = 37},
  [31] = {.lex_state = 40},
  [32] = {.lex_state = 33},
  [33] = {.lex_state = 24},
  [34] = {.lex_state = 37},
  [35] = {.lex_state = 38},
  [36] = {.lex_state = 40},
  [37] = {.lex_state = 24},
};

enum {
  ts_external_token_npc_name = 0,
};

static TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token_npc_name] = sym_npc_name,
};

static bool ts_external_scanner_states[2][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token_npc_name] = true,
  },
};

static uint16_t ts_parse_table[STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [anon_sym_SEMI] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_npc_name] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_script] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [sym__identifier] = ACTIONS(1),
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_return] = ACTIONS(1),
  },
  [1] = {
    [sym_position] = STATE(3),
    [sym_identifier] = STATE(4),
    [sym__header] = STATE(6),
    [sym_script_def] = STATE(6),
    [sym_source_file] = STATE(5),
    [aux_sym_source_file_repeat1] = STATE(6),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym__identifier] = ACTIONS(5),
  },
  [2] = {
    [anon_sym_SEMI] = ACTIONS(7),
    [anon_sym_COMMA] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(7),
    [anon_sym_LPAREN] = ACTIONS(7),
  },
  [3] = {
    [anon_sym_script] = ACTIONS(9),
  },
  [4] = {
    [anon_sym_COMMA] = ACTIONS(11),
  },
  [5] = {
    [ts_builtin_sym_end] = ACTIONS(13),
  },
  [6] = {
    [sym_position] = STATE(3),
    [sym_identifier] = STATE(4),
    [sym__header] = STATE(9),
    [sym_script_def] = STATE(9),
    [aux_sym_source_file_repeat1] = STATE(9),
    [ts_builtin_sym_end] = ACTIONS(15),
    [sym__identifier] = ACTIONS(5),
  },
  [7] = {
    [sym_npc_name] = ACTIONS(17),
  },
  [8] = {
    [sym_number] = ACTIONS(19),
  },
  [9] = {
    [sym_position] = STATE(3),
    [sym_identifier] = STATE(4),
    [sym__header] = STATE(9),
    [sym_script_def] = STATE(9),
    [aux_sym_source_file_repeat1] = STATE(9),
    [sym__identifier] = ACTIONS(21),
    [ts_builtin_sym_end] = ACTIONS(24),
  },
  [10] = {
    [sym_npc_sprite] = STATE(13),
    [sym__identifier] = ACTIONS(26),
  },
  [11] = {
    [anon_sym_COMMA] = ACTIONS(28),
  },
  [12] = {
    [anon_sym_COMMA] = ACTIONS(30),
  },
  [13] = {
    [anon_sym_COMMA] = ACTIONS(32),
  },
  [14] = {
    [sym_number] = ACTIONS(34),
  },
  [15] = {
    [sym_block] = STATE(18),
    [anon_sym_LBRACE] = ACTIONS(36),
  },
  [16] = {
    [anon_sym_COMMA] = ACTIONS(38),
  },
  [17] = {
    [sym__statement] = STATE(22),
    [aux_sym_block_repeat1] = STATE(22),
    [sym_identifier] = STATE(23),
    [sym_return_statement] = STATE(22),
    [sym_function_stmt] = STATE(22),
    [anon_sym_return] = ACTIONS(40),
    [sym__identifier] = ACTIONS(42),
    [anon_sym_RBRACE] = ACTIONS(44),
  },
  [18] = {
    [sym__identifier] = ACTIONS(46),
    [ts_builtin_sym_end] = ACTIONS(46),
  },
  [19] = {
    [sym_number] = ACTIONS(48),
  },
  [20] = {
    [sym__identifier] = ACTIONS(50),
    [ts_builtin_sym_end] = ACTIONS(50),
  },
  [21] = {
    [sym__expression] = STATE(25),
    [sym_identifier] = STATE(25),
    [sym_number] = ACTIONS(52),
    [sym_string] = ACTIONS(54),
    [sym__identifier] = ACTIONS(42),
  },
  [22] = {
    [sym__statement] = STATE(27),
    [aux_sym_block_repeat1] = STATE(27),
    [sym_identifier] = STATE(23),
    [sym_return_statement] = STATE(27),
    [sym_function_stmt] = STATE(27),
    [anon_sym_return] = ACTIONS(40),
    [sym__identifier] = ACTIONS(42),
    [anon_sym_RBRACE] = ACTIONS(56),
  },
  [23] = {
    [sym_parameter_list] = STATE(29),
    [anon_sym_LPAREN] = ACTIONS(58),
  },
  [24] = {
    [anon_sym_script] = ACTIONS(60),
  },
  [25] = {
    [anon_sym_SEMI] = ACTIONS(62),
  },
  [26] = {
    [sym__identifier] = ACTIONS(64),
    [ts_builtin_sym_end] = ACTIONS(64),
  },
  [27] = {
    [sym__statement] = STATE(27),
    [aux_sym_block_repeat1] = STATE(27),
    [sym_identifier] = STATE(23),
    [sym_return_statement] = STATE(27),
    [sym_function_stmt] = STATE(27),
    [anon_sym_return] = ACTIONS(66),
    [sym__identifier] = ACTIONS(69),
    [anon_sym_RBRACE] = ACTIONS(72),
  },
  [28] = {
    [sym__expression] = STATE(32),
    [sym_identifier] = STATE(32),
    [sym__param] = STATE(33),
    [sym_string] = ACTIONS(74),
    [anon_sym_RPAREN] = ACTIONS(76),
    [sym_number] = ACTIONS(78),
    [sym__identifier] = ACTIONS(42),
  },
  [29] = {
    [anon_sym_SEMI] = ACTIONS(80),
  },
  [30] = {
    [anon_sym_return] = ACTIONS(82),
    [sym__identifier] = ACTIONS(82),
    [anon_sym_RBRACE] = ACTIONS(84),
  },
  [31] = {
    [anon_sym_SEMI] = ACTIONS(86),
  },
  [32] = {
    [anon_sym_COMMA] = ACTIONS(88),
    [anon_sym_RPAREN] = ACTIONS(90),
  },
  [33] = {
    [anon_sym_RPAREN] = ACTIONS(92),
  },
  [34] = {
    [anon_sym_return] = ACTIONS(94),
    [sym__identifier] = ACTIONS(94),
    [anon_sym_RBRACE] = ACTIONS(96),
  },
  [35] = {
    [sym__expression] = STATE(32),
    [sym_identifier] = STATE(32),
    [sym__param] = STATE(37),
    [sym_string] = ACTIONS(74),
    [anon_sym_RPAREN] = ACTIONS(98),
    [sym_number] = ACTIONS(78),
    [sym__identifier] = ACTIONS(42),
  },
  [36] = {
    [anon_sym_SEMI] = ACTIONS(100),
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
  [7] = {.count = 1, .reusable = true}, REDUCE(sym_identifier, 1),
  [9] = {.count = 1, .reusable = true}, SHIFT(7),
  [11] = {.count = 1, .reusable = true}, SHIFT(8),
  [13] = {.count = 1, .reusable = true},  ACCEPT_INPUT(),
  [15] = {.count = 1, .reusable = true}, REDUCE(sym_source_file, 1),
  [17] = {.count = 1, .reusable = true}, SHIFT(10),
  [19] = {.count = 1, .reusable = true}, SHIFT(11),
  [21] = {.count = 2, .reusable = true}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(2),
  [24] = {.count = 1, .reusable = true}, REDUCE(aux_sym_source_file_repeat1, 2),
  [26] = {.count = 1, .reusable = true}, SHIFT(12),
  [28] = {.count = 1, .reusable = true}, SHIFT(14),
  [30] = {.count = 1, .reusable = true}, REDUCE(sym_npc_sprite, 1),
  [32] = {.count = 1, .reusable = true}, SHIFT(15),
  [34] = {.count = 1, .reusable = true}, SHIFT(16),
  [36] = {.count = 1, .reusable = true}, SHIFT(17),
  [38] = {.count = 1, .reusable = true}, SHIFT(19),
  [40] = {.count = 1, .reusable = false}, SHIFT(21),
  [42] = {.count = 1, .reusable = false}, SHIFT(2),
  [44] = {.count = 1, .reusable = true}, SHIFT(20),
  [46] = {.count = 1, .reusable = true}, REDUCE(sym_script_def, 6),
  [48] = {.count = 1, .reusable = true}, SHIFT(24),
  [50] = {.count = 1, .reusable = true}, REDUCE(sym_block, 2),
  [52] = {.count = 1, .reusable = false}, SHIFT(25),
  [54] = {.count = 1, .reusable = true}, SHIFT(25),
  [56] = {.count = 1, .reusable = true}, SHIFT(26),
  [58] = {.count = 1, .reusable = true}, SHIFT(28),
  [60] = {.count = 1, .reusable = true}, REDUCE(sym_position, 7),
  [62] = {.count = 1, .reusable = true}, SHIFT(30),
  [64] = {.count = 1, .reusable = true}, REDUCE(sym_block, 3),
  [66] = {.count = 2, .reusable = false}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(21),
  [69] = {.count = 2, .reusable = false}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(2),
  [72] = {.count = 1, .reusable = true}, REDUCE(aux_sym_block_repeat1, 2),
  [74] = {.count = 1, .reusable = true}, SHIFT(32),
  [76] = {.count = 1, .reusable = true}, SHIFT(31),
  [78] = {.count = 1, .reusable = false}, SHIFT(32),
  [80] = {.count = 1, .reusable = true}, SHIFT(34),
  [82] = {.count = 1, .reusable = false}, REDUCE(sym_return_statement, 3),
  [84] = {.count = 1, .reusable = true}, REDUCE(sym_return_statement, 3),
  [86] = {.count = 1, .reusable = true}, REDUCE(sym_parameter_list, 2),
  [88] = {.count = 1, .reusable = true}, SHIFT(35),
  [90] = {.count = 1, .reusable = true}, REDUCE(sym__param, 1),
  [92] = {.count = 1, .reusable = true}, SHIFT(36),
  [94] = {.count = 1, .reusable = false}, REDUCE(sym_function_stmt, 3),
  [96] = {.count = 1, .reusable = true}, REDUCE(sym_function_stmt, 3),
  [98] = {.count = 1, .reusable = true}, REDUCE(sym__param, 2),
  [100] = {.count = 1, .reusable = true}, REDUCE(sym_parameter_list, 3),
  [102] = {.count = 1, .reusable = true}, REDUCE(sym__param, 3),
};

void *tree_sitter_hercscript_external_scanner_create();
void tree_sitter_hercscript_external_scanner_destroy(void *);
bool tree_sitter_hercscript_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_hercscript_external_scanner_serialize(void *, char *);
void tree_sitter_hercscript_external_scanner_deserialize(void *, const char *, unsigned);

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
    .external_scanner = {
      (const bool *)ts_external_scanner_states,
      ts_external_scanner_symbol_map,
      tree_sitter_hercscript_external_scanner_create,
      tree_sitter_hercscript_external_scanner_destroy,
      tree_sitter_hercscript_external_scanner_scan,
      tree_sitter_hercscript_external_scanner_serialize,
      tree_sitter_hercscript_external_scanner_deserialize,
    },
  };
  return &language;
}
