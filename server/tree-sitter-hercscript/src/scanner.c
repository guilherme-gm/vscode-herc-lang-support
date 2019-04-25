#include <stdlib.h>
#include <string.h>
#include <tree_sitter/parser.h>

// Order must match grammar.js external
enum TokenType
{
    NPC_NAME,
};

struct npc_name_str {
    char name[24 * 2]; // 24 name + 24 unique
};

void *tree_sitter_hercscript_external_scanner_create()
{
    /* Initialize and return a structure used by the scanner */
    return malloc(sizeof(struct npc_name_str));
}

void tree_sitter_hercscript_external_scanner_destroy(void *payload)
{
    /* Destroys data allocated by create */
    free(payload);
}

unsigned tree_sitter_hercscript_external_scanner_serialize(
    void *payload,
    char *buffer)
{
    struct npc_name_str *name = (struct npc_name_str *) payload;
    memcpy(buffer, name->name, strlen(name->name));
    return strlen(name->name);
}

void tree_sitter_hercscript_external_scanner_deserialize(
    void *payload,
    const char *buffer,
    unsigned length)
{
    struct npc_name_str *name = (struct npc_name_str *) payload;
    memcpy(name->name, buffer, length);
    name->name[length] = '\0';
}

bool tree_sitter_hercscript_external_scanner_scan(
    void *payload,
    TSLexer *lexer,
    const bool *valid_symbols
) {
    struct npc_name_str *name = (struct npc_name_str *) payload;
    int i = 0;

    if (valid_symbols[NPC_NAME]) {
        while (lexer->lookahead) {
            if (lexer->lookahead == '\t' && i > 0) { // i > 0 or \t of script will end it
                name->name[i] = '\0';
                lexer->mark_end(lexer);
                lexer->result_symbol = NPC_NAME;

                return true;
            }
            
            name->name[i++] = lexer->lookahead;
            lexer->advance(lexer, false);
        }
    }
    
    return false;
}
