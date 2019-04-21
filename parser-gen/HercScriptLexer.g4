lexer grammar HercScriptLexer;

COMMA : ',' ;
// DASH : '-' ; // MINUS
HASH : '#' ;
UNIQUE_NAME : '::' ;
COLON : ':' ;
SEMI_COL: ';' ;

OPEN_BRACE : '{';
CLOSE_BRACE : '}';
OPEN_PARENTHESIS : '(';
CLOSE_PARENTHESIS : ')';

MINUS : '-'; // MINUS is also DASH so we can't read it as OP or we lose semantics on headers..

OP : ( SUMEQ | MINEQ | MULEQ | DIVEQ | MODEQ | SUM /*| MINUS */ | MUL | DIV | MOD) ;
SUMEQ : '+=';
MINEQ : '-=';
MULEQ : '*=';
DIVEQ : '/=';
MODEQ : '%=';
SUM: '+';
MUL: '*';
DIV: '/';
MOD: '%';


SCRIPT : 'script' ;

ID : [a-zA-Z_0-9]+ ;             // match lower-case identifiers
WS : [ \t\r\n]+ -> skip ; // skip spaces, tabs, newlines

LINE_COMMENT :	'//' ~('\r' | '\n')* -> skip;
BLOCK_COMMENT : '/*' (.*?) '*/' -> skip;

DQUOTE : '"' -> pushMode(IN_STRING) ;

// Inside String
// ================
mode IN_STRING;

TEXT: ~[\\"]+ ;
DQUOTE_IN_STRING: '"' -> type(DQUOTE), popMode;
