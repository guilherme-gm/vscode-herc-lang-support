// Generated from ./parser-gen/HercScriptLexer.g4 by ANTLR 4.7.2
// jshint ignore: start
var antlr4 = require('antlr4/index');



var serializedATN = ["\u0003\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964",
    "\u0002\u001d\u00ad\b\u0001\b\u0001\u0004\u0002\t\u0002\u0004\u0003\t",
    "\u0003\u0004\u0004\t\u0004\u0004\u0005\t\u0005\u0004\u0006\t\u0006\u0004",
    "\u0007\t\u0007\u0004\b\t\b\u0004\t\t\t\u0004\n\t\n\u0004\u000b\t\u000b",
    "\u0004\f\t\f\u0004\r\t\r\u0004\u000e\t\u000e\u0004\u000f\t\u000f\u0004",
    "\u0010\t\u0010\u0004\u0011\t\u0011\u0004\u0012\t\u0012\u0004\u0013\t",
    "\u0013\u0004\u0014\t\u0014\u0004\u0015\t\u0015\u0004\u0016\t\u0016\u0004",
    "\u0017\t\u0017\u0004\u0018\t\u0018\u0004\u0019\t\u0019\u0004\u001a\t",
    "\u001a\u0004\u001b\t\u001b\u0004\u001c\t\u001c\u0004\u001d\t\u001d\u0003",
    "\u0002\u0003\u0002\u0003\u0003\u0003\u0003\u0003\u0004\u0003\u0004\u0003",
    "\u0004\u0003\u0005\u0003\u0005\u0003\u0006\u0003\u0006\u0003\u0007\u0003",
    "\u0007\u0003\b\u0003\b\u0003\t\u0003\t\u0003\n\u0003\n\u0003\u000b\u0003",
    "\u000b\u0003\f\u0003\f\u0003\f\u0003\f\u0003\f\u0003\f\u0003\f\u0003",
    "\f\u0003\f\u0005\f[\n\f\u0003\r\u0003\r\u0003\r\u0003\u000e\u0003\u000e",
    "\u0003\u000e\u0003\u000f\u0003\u000f\u0003\u000f\u0003\u0010\u0003\u0010",
    "\u0003\u0010\u0003\u0011\u0003\u0011\u0003\u0011\u0003\u0012\u0003\u0012",
    "\u0003\u0013\u0003\u0013\u0003\u0014\u0003\u0014\u0003\u0015\u0003\u0015",
    "\u0003\u0016\u0003\u0016\u0003\u0016\u0003\u0016\u0003\u0016\u0003\u0016",
    "\u0003\u0016\u0003\u0017\u0006\u0017|\n\u0017\r\u0017\u000e\u0017}\u0003",
    "\u0018\u0006\u0018\u0081\n\u0018\r\u0018\u000e\u0018\u0082\u0003\u0018",
    "\u0003\u0018\u0003\u0019\u0003\u0019\u0003\u0019\u0003\u0019\u0007\u0019",
    "\u008b\n\u0019\f\u0019\u000e\u0019\u008e\u000b\u0019\u0003\u0019\u0003",
    "\u0019\u0003\u001a\u0003\u001a\u0003\u001a\u0003\u001a\u0007\u001a\u0096",
    "\n\u001a\f\u001a\u000e\u001a\u0099\u000b\u001a\u0003\u001a\u0003\u001a",
    "\u0003\u001a\u0003\u001a\u0003\u001a\u0003\u001b\u0003\u001b\u0003\u001b",
    "\u0003\u001b\u0003\u001c\u0006\u001c\u00a5\n\u001c\r\u001c\u000e\u001c",
    "\u00a6\u0003\u001d\u0003\u001d\u0003\u001d\u0003\u001d\u0003\u001d\u0003",
    "\u0097\u0002\u001e\u0004\u0003\u0006\u0004\b\u0005\n\u0006\f\u0007\u000e",
    "\b\u0010\t\u0012\n\u0014\u000b\u0016\f\u0018\r\u001a\u000e\u001c\u000f",
    "\u001e\u0010 \u0011\"\u0012$\u0013&\u0014(\u0015*\u0016,\u0017.\u0018",
    "0\u00192\u001a4\u001b6\u001c8\u001d:\u0002\u0004\u0002\u0003\u0006\u0006",
    "\u00022;C\\aac|\u0005\u0002\u000b\f\u000f\u000f\"\"\u0004\u0002\f\f",
    "\u000f\u000f\u0004\u0002$$^^\u0002\u00b8\u0002\u0004\u0003\u0002\u0002",
    "\u0002\u0002\u0006\u0003\u0002\u0002\u0002\u0002\b\u0003\u0002\u0002",
    "\u0002\u0002\n\u0003\u0002\u0002\u0002\u0002\f\u0003\u0002\u0002\u0002",
    "\u0002\u000e\u0003\u0002\u0002\u0002\u0002\u0010\u0003\u0002\u0002\u0002",
    "\u0002\u0012\u0003\u0002\u0002\u0002\u0002\u0014\u0003\u0002\u0002\u0002",
    "\u0002\u0016\u0003\u0002\u0002\u0002\u0002\u0018\u0003\u0002\u0002\u0002",
    "\u0002\u001a\u0003\u0002\u0002\u0002\u0002\u001c\u0003\u0002\u0002\u0002",
    "\u0002\u001e\u0003\u0002\u0002\u0002\u0002 \u0003\u0002\u0002\u0002",
    "\u0002\"\u0003\u0002\u0002\u0002\u0002$\u0003\u0002\u0002\u0002\u0002",
    "&\u0003\u0002\u0002\u0002\u0002(\u0003\u0002\u0002\u0002\u0002*\u0003",
    "\u0002\u0002\u0002\u0002,\u0003\u0002\u0002\u0002\u0002.\u0003\u0002",
    "\u0002\u0002\u00020\u0003\u0002\u0002\u0002\u00022\u0003\u0002\u0002",
    "\u0002\u00024\u0003\u0002\u0002\u0002\u00026\u0003\u0002\u0002\u0002",
    "\u00038\u0003\u0002\u0002\u0002\u0003:\u0003\u0002\u0002\u0002\u0004",
    "<\u0003\u0002\u0002\u0002\u0006>\u0003\u0002\u0002\u0002\b@\u0003\u0002",
    "\u0002\u0002\nC\u0003\u0002\u0002\u0002\fE\u0003\u0002\u0002\u0002\u000e",
    "G\u0003\u0002\u0002\u0002\u0010I\u0003\u0002\u0002\u0002\u0012K\u0003",
    "\u0002\u0002\u0002\u0014M\u0003\u0002\u0002\u0002\u0016O\u0003\u0002",
    "\u0002\u0002\u0018Z\u0003\u0002\u0002\u0002\u001a\\\u0003\u0002\u0002",
    "\u0002\u001c_\u0003\u0002\u0002\u0002\u001eb\u0003\u0002\u0002\u0002",
    " e\u0003\u0002\u0002\u0002\"h\u0003\u0002\u0002\u0002$k\u0003\u0002",
    "\u0002\u0002&m\u0003\u0002\u0002\u0002(o\u0003\u0002\u0002\u0002*q\u0003",
    "\u0002\u0002\u0002,s\u0003\u0002\u0002\u0002.{\u0003\u0002\u0002\u0002",
    "0\u0080\u0003\u0002\u0002\u00022\u0086\u0003\u0002\u0002\u00024\u0091",
    "\u0003\u0002\u0002\u00026\u009f\u0003\u0002\u0002\u00028\u00a4\u0003",
    "\u0002\u0002\u0002:\u00a8\u0003\u0002\u0002\u0002<=\u0007.\u0002\u0002",
    "=\u0005\u0003\u0002\u0002\u0002>?\u0007%\u0002\u0002?\u0007\u0003\u0002",
    "\u0002\u0002@A\u0007<\u0002\u0002AB\u0007<\u0002\u0002B\t\u0003\u0002",
    "\u0002\u0002CD\u0007<\u0002\u0002D\u000b\u0003\u0002\u0002\u0002EF\u0007",
    "=\u0002\u0002F\r\u0003\u0002\u0002\u0002GH\u0007}\u0002\u0002H\u000f",
    "\u0003\u0002\u0002\u0002IJ\u0007\u007f\u0002\u0002J\u0011\u0003\u0002",
    "\u0002\u0002KL\u0007*\u0002\u0002L\u0013\u0003\u0002\u0002\u0002MN\u0007",
    "+\u0002\u0002N\u0015\u0003\u0002\u0002\u0002OP\u0007/\u0002\u0002P\u0017",
    "\u0003\u0002\u0002\u0002Q[\u0005\u001a\r\u0002R[\u0005\u001c\u000e\u0002",
    "S[\u0005\u001e\u000f\u0002T[\u0005 \u0010\u0002U[\u0005\"\u0011\u0002",
    "V[\u0005$\u0012\u0002W[\u0005&\u0013\u0002X[\u0005(\u0014\u0002Y[\u0005",
    "*\u0015\u0002ZQ\u0003\u0002\u0002\u0002ZR\u0003\u0002\u0002\u0002ZS",
    "\u0003\u0002\u0002\u0002ZT\u0003\u0002\u0002\u0002ZU\u0003\u0002\u0002",
    "\u0002ZV\u0003\u0002\u0002\u0002ZW\u0003\u0002\u0002\u0002ZX\u0003\u0002",
    "\u0002\u0002ZY\u0003\u0002\u0002\u0002[\u0019\u0003\u0002\u0002\u0002",
    "\\]\u0007-\u0002\u0002]^\u0007?\u0002\u0002^\u001b\u0003\u0002\u0002",
    "\u0002_`\u0007/\u0002\u0002`a\u0007?\u0002\u0002a\u001d\u0003\u0002",
    "\u0002\u0002bc\u0007,\u0002\u0002cd\u0007?\u0002\u0002d\u001f\u0003",
    "\u0002\u0002\u0002ef\u00071\u0002\u0002fg\u0007?\u0002\u0002g!\u0003",
    "\u0002\u0002\u0002hi\u0007\'\u0002\u0002ij\u0007?\u0002\u0002j#\u0003",
    "\u0002\u0002\u0002kl\u0007-\u0002\u0002l%\u0003\u0002\u0002\u0002mn",
    "\u0007,\u0002\u0002n\'\u0003\u0002\u0002\u0002op\u00071\u0002\u0002",
    "p)\u0003\u0002\u0002\u0002qr\u0007\'\u0002\u0002r+\u0003\u0002\u0002",
    "\u0002st\u0007u\u0002\u0002tu\u0007e\u0002\u0002uv\u0007t\u0002\u0002",
    "vw\u0007k\u0002\u0002wx\u0007r\u0002\u0002xy\u0007v\u0002\u0002y-\u0003",
    "\u0002\u0002\u0002z|\t\u0002\u0002\u0002{z\u0003\u0002\u0002\u0002|",
    "}\u0003\u0002\u0002\u0002}{\u0003\u0002\u0002\u0002}~\u0003\u0002\u0002",
    "\u0002~/\u0003\u0002\u0002\u0002\u007f\u0081\t\u0003\u0002\u0002\u0080",
    "\u007f\u0003\u0002\u0002\u0002\u0081\u0082\u0003\u0002\u0002\u0002\u0082",
    "\u0080\u0003\u0002\u0002\u0002\u0082\u0083\u0003\u0002\u0002\u0002\u0083",
    "\u0084\u0003\u0002\u0002\u0002\u0084\u0085\b\u0018\u0002\u0002\u0085",
    "1\u0003\u0002\u0002\u0002\u0086\u0087\u00071\u0002\u0002\u0087\u0088",
    "\u00071\u0002\u0002\u0088\u008c\u0003\u0002\u0002\u0002\u0089\u008b",
    "\n\u0004\u0002\u0002\u008a\u0089\u0003\u0002\u0002\u0002\u008b\u008e",
    "\u0003\u0002\u0002\u0002\u008c\u008a\u0003\u0002\u0002\u0002\u008c\u008d",
    "\u0003\u0002\u0002\u0002\u008d\u008f\u0003\u0002\u0002\u0002\u008e\u008c",
    "\u0003\u0002\u0002\u0002\u008f\u0090\b\u0019\u0002\u0002\u00903\u0003",
    "\u0002\u0002\u0002\u0091\u0092\u00071\u0002\u0002\u0092\u0093\u0007",
    ",\u0002\u0002\u0093\u0097\u0003\u0002\u0002\u0002\u0094\u0096\u000b",
    "\u0002\u0002\u0002\u0095\u0094\u0003\u0002\u0002\u0002\u0096\u0099\u0003",
    "\u0002\u0002\u0002\u0097\u0098\u0003\u0002\u0002\u0002\u0097\u0095\u0003",
    "\u0002\u0002\u0002\u0098\u009a\u0003\u0002\u0002\u0002\u0099\u0097\u0003",
    "\u0002\u0002\u0002\u009a\u009b\u0007,\u0002\u0002\u009b\u009c\u0007",
    "1\u0002\u0002\u009c\u009d\u0003\u0002\u0002\u0002\u009d\u009e\b\u001a",
    "\u0002\u0002\u009e5\u0003\u0002\u0002\u0002\u009f\u00a0\u0007$\u0002",
    "\u0002\u00a0\u00a1\u0003\u0002\u0002\u0002\u00a1\u00a2\b\u001b\u0003",
    "\u0002\u00a27\u0003\u0002\u0002\u0002\u00a3\u00a5\n\u0005\u0002\u0002",
    "\u00a4\u00a3\u0003\u0002\u0002\u0002\u00a5\u00a6\u0003\u0002\u0002\u0002",
    "\u00a6\u00a4\u0003\u0002\u0002\u0002\u00a6\u00a7\u0003\u0002\u0002\u0002",
    "\u00a79\u0003\u0002\u0002\u0002\u00a8\u00a9\u0007$\u0002\u0002\u00a9",
    "\u00aa\u0003\u0002\u0002\u0002\u00aa\u00ab\b\u001d\u0004\u0002\u00ab",
    "\u00ac\b\u001d\u0005\u0002\u00ac;\u0003\u0002\u0002\u0002\n\u0002\u0003",
    "Z}\u0082\u008c\u0097\u00a6\u0006\b\u0002\u0002\u0007\u0003\u0002\t\u001c",
    "\u0002\u0006\u0002\u0002"].join("");


var atn = new antlr4.atn.ATNDeserializer().deserialize(serializedATN);

var decisionsToDFA = atn.decisionToState.map( function(ds, index) { return new antlr4.dfa.DFA(ds, index); });

function HercScriptLexer(input) {
	antlr4.Lexer.call(this, input);
    this._interp = new antlr4.atn.LexerATNSimulator(this, atn, decisionsToDFA, new antlr4.PredictionContextCache());
    return this;
}

HercScriptLexer.prototype = Object.create(antlr4.Lexer.prototype);
HercScriptLexer.prototype.constructor = HercScriptLexer;

Object.defineProperty(HercScriptLexer.prototype, "atn", {
        get : function() {
                return atn;
        }
});

HercScriptLexer.EOF = antlr4.Token.EOF;
HercScriptLexer.COMMA = 1;
HercScriptLexer.HASH = 2;
HercScriptLexer.UNIQUE_NAME = 3;
HercScriptLexer.COLON = 4;
HercScriptLexer.SEMI_COL = 5;
HercScriptLexer.OPEN_BRACE = 6;
HercScriptLexer.CLOSE_BRACE = 7;
HercScriptLexer.OPEN_PARENTHESIS = 8;
HercScriptLexer.CLOSE_PARENTHESIS = 9;
HercScriptLexer.MINUS = 10;
HercScriptLexer.OP = 11;
HercScriptLexer.SUMEQ = 12;
HercScriptLexer.MINEQ = 13;
HercScriptLexer.MULEQ = 14;
HercScriptLexer.DIVEQ = 15;
HercScriptLexer.MODEQ = 16;
HercScriptLexer.SUM = 17;
HercScriptLexer.MUL = 18;
HercScriptLexer.DIV = 19;
HercScriptLexer.MOD = 20;
HercScriptLexer.SCRIPT = 21;
HercScriptLexer.ID = 22;
HercScriptLexer.WS = 23;
HercScriptLexer.LINE_COMMENT = 24;
HercScriptLexer.BLOCK_COMMENT = 25;
HercScriptLexer.DQUOTE = 26;
HercScriptLexer.TEXT = 27;

HercScriptLexer.IN_STRING = 1;

HercScriptLexer.prototype.channelNames = [ "DEFAULT_TOKEN_CHANNEL", "HIDDEN" ];

HercScriptLexer.prototype.modeNames = [ "DEFAULT_MODE", "IN_STRING" ];

HercScriptLexer.prototype.literalNames = [ null, "','", "'#'", "'::'", "':'", 
                                           "';'", "'{'", "'}'", "'('", "')'", 
                                           "'-'", null, "'+='", "'-='", 
                                           "'*='", "'/='", "'%='", "'+'", 
                                           "'*'", "'/'", "'%'", "'script'" ];

HercScriptLexer.prototype.symbolicNames = [ null, "COMMA", "HASH", "UNIQUE_NAME", 
                                            "COLON", "SEMI_COL", "OPEN_BRACE", 
                                            "CLOSE_BRACE", "OPEN_PARENTHESIS", 
                                            "CLOSE_PARENTHESIS", "MINUS", 
                                            "OP", "SUMEQ", "MINEQ", "MULEQ", 
                                            "DIVEQ", "MODEQ", "SUM", "MUL", 
                                            "DIV", "MOD", "SCRIPT", "ID", 
                                            "WS", "LINE_COMMENT", "BLOCK_COMMENT", 
                                            "DQUOTE", "TEXT" ];

HercScriptLexer.prototype.ruleNames = [ "COMMA", "HASH", "UNIQUE_NAME", 
                                        "COLON", "SEMI_COL", "OPEN_BRACE", 
                                        "CLOSE_BRACE", "OPEN_PARENTHESIS", 
                                        "CLOSE_PARENTHESIS", "MINUS", "OP", 
                                        "SUMEQ", "MINEQ", "MULEQ", "DIVEQ", 
                                        "MODEQ", "SUM", "MUL", "DIV", "MOD", 
                                        "SCRIPT", "ID", "WS", "LINE_COMMENT", 
                                        "BLOCK_COMMENT", "DQUOTE", "TEXT", 
                                        "DQUOTE_IN_STRING" ];

HercScriptLexer.prototype.grammarFileName = "HercScriptLexer.g4";



exports.HercScriptLexer = HercScriptLexer;

