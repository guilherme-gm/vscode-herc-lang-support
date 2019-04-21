// Generated from ./parser-gen/HercScript.g4 by ANTLR 4.7.2
// jshint ignore: start
var antlr4 = require('antlr4/index');



var serializedATN = ["\u0003\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964",
    "\u0002\u0005\u001b\b\u0001\u0004\u0002\t\u0002\u0004\u0003\t\u0003\u0004",
    "\u0004\t\u0004\u0003\u0002\u0003\u0002\u0003\u0002\u0003\u0002\u0003",
    "\u0002\u0003\u0002\u0003\u0003\u0006\u0003\u0011\n\u0003\r\u0003\u000e",
    "\u0003\u0012\u0003\u0004\u0006\u0004\u0016\n\u0004\r\u0004\u000e\u0004",
    "\u0017\u0003\u0004\u0003\u0004\u0002\u0002\u0005\u0003\u0003\u0005\u0004",
    "\u0007\u0005\u0003\u0002\u0004\u0003\u0002c|\u0005\u0002\u000b\f\u000f",
    "\u000f\"\"\u0002\u001c\u0002\u0003\u0003\u0002\u0002\u0002\u0002\u0005",
    "\u0003\u0002\u0002\u0002\u0002\u0007\u0003\u0002\u0002\u0002\u0003\t",
    "\u0003\u0002\u0002\u0002\u0005\u0010\u0003\u0002\u0002\u0002\u0007\u0015",
    "\u0003\u0002\u0002\u0002\t\n\u0007j\u0002\u0002\n\u000b\u0007g\u0002",
    "\u0002\u000b\f\u0007n\u0002\u0002\f\r\u0007n\u0002\u0002\r\u000e\u0007",
    "q\u0002\u0002\u000e\u0004\u0003\u0002\u0002\u0002\u000f\u0011\t\u0002",
    "\u0002\u0002\u0010\u000f\u0003\u0002\u0002\u0002\u0011\u0012\u0003\u0002",
    "\u0002\u0002\u0012\u0010\u0003\u0002\u0002\u0002\u0012\u0013\u0003\u0002",
    "\u0002\u0002\u0013\u0006\u0003\u0002\u0002\u0002\u0014\u0016\t\u0003",
    "\u0002\u0002\u0015\u0014\u0003\u0002\u0002\u0002\u0016\u0017\u0003\u0002",
    "\u0002\u0002\u0017\u0015\u0003\u0002\u0002\u0002\u0017\u0018\u0003\u0002",
    "\u0002\u0002\u0018\u0019\u0003\u0002\u0002\u0002\u0019\u001a\b\u0004",
    "\u0002\u0002\u001a\b\u0003\u0002\u0002\u0002\u0005\u0002\u0012\u0017",
    "\u0003\b\u0002\u0002"].join("");


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
HercScriptLexer.T__0 = 1;
HercScriptLexer.ID = 2;
HercScriptLexer.WS = 3;

HercScriptLexer.prototype.channelNames = [ "DEFAULT_TOKEN_CHANNEL", "HIDDEN" ];

HercScriptLexer.prototype.modeNames = [ "DEFAULT_MODE" ];

HercScriptLexer.prototype.literalNames = [ null, "'hello'" ];

HercScriptLexer.prototype.symbolicNames = [ null, null, "ID", "WS" ];

HercScriptLexer.prototype.ruleNames = [ "T__0", "ID", "WS" ];

HercScriptLexer.prototype.grammarFileName = "HercScript.g4";



exports.HercScriptLexer = HercScriptLexer;

