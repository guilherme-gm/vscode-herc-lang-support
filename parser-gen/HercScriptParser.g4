parser grammar HercScriptParser;

options { tokenVocab=HercScriptLexer; }

// Entry Points

// Parsing a file
file
: script*// | function | warp | shop)*
;

// Parsing a line
/*line
:
;*/

// Header Helpers
// ========================
npcPos
: ID COMMA ID COMMA ID COMMA ID # OnMap
| MINUS # Floating
;

npcName
: ID+ hiddenName? uniqueName? // Has a name, maybe a hiddenName, maybe a uniqueName
| hiddenName uniqueName?      // Doesn't have name, has hiddenName, maybe a uniqueName
| uniqueName                  // Doesn't have name nor hiddenName, has a uniqueName
;

hiddenName
: HASH ID+
;

uniqueName
: UNIQUE_NAME ID+
;

npcSprite
: ID
;

npcTrigger
: COMMA ID COMMA ID
;

// Scripts
// ========================
script
: npcPos SCRIPT npcName npcSprite npcTrigger? COMMA OPEN_BRACE scriptBody* CLOSE_BRACE
;

// FIXME : better checking for semi-colons
scriptBody
: scriptExpr SEMI_COL?
| scriptStmt SEMI_COL?
| label
;

scriptExpr
: ID ((OP | MINUS) scriptExpr)?    #mathExpr
| ID (COMMA scriptExpr)? #paramExpr
;

scriptStmt
: ID OPEN_PARENTHESIS scriptExpr CLOSE_PARENTHESIS
;

label
: ID COLON
;
