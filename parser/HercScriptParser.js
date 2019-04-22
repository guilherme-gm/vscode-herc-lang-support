// Generated from ./parser-gen/HercScriptParser.g4 by ANTLR 4.7.2
// jshint ignore: start
var antlr4 = require('antlr4/index');
var HercScriptParserListener = require('./HercScriptParserListener').HercScriptParserListener;
var HercScriptParserVisitor = require('./HercScriptParserVisitor').HercScriptParserVisitor;

var grammarFileName = "HercScriptParser.g4";


var serializedATN = ["\u0003\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964",
    "\u0003\u001d\u0080\u0004\u0002\t\u0002\u0004\u0003\t\u0003\u0004\u0004",
    "\t\u0004\u0004\u0005\t\u0005\u0004\u0006\t\u0006\u0004\u0007\t\u0007",
    "\u0004\b\t\b\u0004\t\t\t\u0004\n\t\n\u0004\u000b\t\u000b\u0004\f\t\f",
    "\u0004\r\t\r\u0003\u0002\u0007\u0002\u001c\n\u0002\f\u0002\u000e\u0002",
    "\u001f\u000b\u0002\u0003\u0003\u0003\u0003\u0003\u0003\u0003\u0003\u0003",
    "\u0003\u0003\u0003\u0003\u0003\u0003\u0003\u0005\u0003)\n\u0003\u0003",
    "\u0004\u0006\u0004,\n\u0004\r\u0004\u000e\u0004-\u0003\u0004\u0005\u0004",
    "1\n\u0004\u0003\u0004\u0005\u00044\n\u0004\u0003\u0004\u0003\u0004\u0005",
    "\u00048\n\u0004\u0003\u0004\u0005\u0004;\n\u0004\u0003\u0005\u0003\u0005",
    "\u0006\u0005?\n\u0005\r\u0005\u000e\u0005@\u0003\u0006\u0003\u0006\u0006",
    "\u0006E\n\u0006\r\u0006\u000e\u0006F\u0003\u0007\u0003\u0007\u0003\b",
    "\u0003\b\u0003\b\u0003\b\u0003\b\u0003\t\u0003\t\u0003\t\u0003\t\u0003",
    "\t\u0005\tU\n\t\u0003\t\u0003\t\u0003\t\u0007\tZ\n\t\f\t\u000e\t]\u000b",
    "\t\u0003\t\u0003\t\u0003\n\u0003\n\u0005\nc\n\n\u0003\n\u0003\n\u0005",
    "\ng\n\n\u0003\n\u0005\nj\n\n\u0003\u000b\u0003\u000b\u0003\u000b\u0005",
    "\u000bo\n\u000b\u0003\u000b\u0003\u000b\u0003\u000b\u0005\u000bt\n\u000b",
    "\u0005\u000bv\n\u000b\u0003\f\u0003\f\u0003\f\u0003\f\u0003\f\u0003",
    "\r\u0003\r\u0003\r\u0003\r\u0002\u0002\u000e\u0002\u0004\u0006\b\n\f",
    "\u000e\u0010\u0012\u0014\u0016\u0018\u0002\u0003\u0003\u0002\f\r\u0002",
    "\u0086\u0002\u001d\u0003\u0002\u0002\u0002\u0004(\u0003\u0002\u0002",
    "\u0002\u0006:\u0003\u0002\u0002\u0002\b<\u0003\u0002\u0002\u0002\nB",
    "\u0003\u0002\u0002\u0002\fH\u0003\u0002\u0002\u0002\u000eJ\u0003\u0002",
    "\u0002\u0002\u0010O\u0003\u0002\u0002\u0002\u0012i\u0003\u0002\u0002",
    "\u0002\u0014u\u0003\u0002\u0002\u0002\u0016w\u0003\u0002\u0002\u0002",
    "\u0018|\u0003\u0002\u0002\u0002\u001a\u001c\u0005\u0010\t\u0002\u001b",
    "\u001a\u0003\u0002\u0002\u0002\u001c\u001f\u0003\u0002\u0002\u0002\u001d",
    "\u001b\u0003\u0002\u0002\u0002\u001d\u001e\u0003\u0002\u0002\u0002\u001e",
    "\u0003\u0003\u0002\u0002\u0002\u001f\u001d\u0003\u0002\u0002\u0002 ",
    "!\u0007\u0018\u0002\u0002!\"\u0007\u0003\u0002\u0002\"#\u0007\u0018",
    "\u0002\u0002#$\u0007\u0003\u0002\u0002$%\u0007\u0018\u0002\u0002%&\u0007",
    "\u0003\u0002\u0002&)\u0007\u0018\u0002\u0002\')\u0007\f\u0002\u0002",
    "( \u0003\u0002\u0002\u0002(\'\u0003\u0002\u0002\u0002)\u0005\u0003\u0002",
    "\u0002\u0002*,\u0007\u0018\u0002\u0002+*\u0003\u0002\u0002\u0002,-\u0003",
    "\u0002\u0002\u0002-+\u0003\u0002\u0002\u0002-.\u0003\u0002\u0002\u0002",
    ".0\u0003\u0002\u0002\u0002/1\u0005\b\u0005\u00020/\u0003\u0002\u0002",
    "\u000201\u0003\u0002\u0002\u000213\u0003\u0002\u0002\u000224\u0005\n",
    "\u0006\u000232\u0003\u0002\u0002\u000234\u0003\u0002\u0002\u00024;\u0003",
    "\u0002\u0002\u000257\u0005\b\u0005\u000268\u0005\n\u0006\u000276\u0003",
    "\u0002\u0002\u000278\u0003\u0002\u0002\u00028;\u0003\u0002\u0002\u0002",
    "9;\u0005\n\u0006\u0002:+\u0003\u0002\u0002\u0002:5\u0003\u0002\u0002",
    "\u0002:9\u0003\u0002\u0002\u0002;\u0007\u0003\u0002\u0002\u0002<>\u0007",
    "\u0004\u0002\u0002=?\u0007\u0018\u0002\u0002>=\u0003\u0002\u0002\u0002",
    "?@\u0003\u0002\u0002\u0002@>\u0003\u0002\u0002\u0002@A\u0003\u0002\u0002",
    "\u0002A\t\u0003\u0002\u0002\u0002BD\u0007\u0005\u0002\u0002CE\u0007",
    "\u0018\u0002\u0002DC\u0003\u0002\u0002\u0002EF\u0003\u0002\u0002\u0002",
    "FD\u0003\u0002\u0002\u0002FG\u0003\u0002\u0002\u0002G\u000b\u0003\u0002",
    "\u0002\u0002HI\u0007\u0018\u0002\u0002I\r\u0003\u0002\u0002\u0002JK",
    "\u0007\u0003\u0002\u0002KL\u0007\u0018\u0002\u0002LM\u0007\u0003\u0002",
    "\u0002MN\u0007\u0018\u0002\u0002N\u000f\u0003\u0002\u0002\u0002OP\u0005",
    "\u0004\u0003\u0002PQ\u0007\u0017\u0002\u0002QR\u0005\u0006\u0004\u0002",
    "RT\u0005\f\u0007\u0002SU\u0005\u000e\b\u0002TS\u0003\u0002\u0002\u0002",
    "TU\u0003\u0002\u0002\u0002UV\u0003\u0002\u0002\u0002VW\u0007\u0003\u0002",
    "\u0002W[\u0007\b\u0002\u0002XZ\u0005\u0012\n\u0002YX\u0003\u0002\u0002",
    "\u0002Z]\u0003\u0002\u0002\u0002[Y\u0003\u0002\u0002\u0002[\\\u0003",
    "\u0002\u0002\u0002\\^\u0003\u0002\u0002\u0002][\u0003\u0002\u0002\u0002",
    "^_\u0007\t\u0002\u0002_\u0011\u0003\u0002\u0002\u0002`b\u0005\u0014",
    "\u000b\u0002ac\u0007\u0007\u0002\u0002ba\u0003\u0002\u0002\u0002bc\u0003",
    "\u0002\u0002\u0002cj\u0003\u0002\u0002\u0002df\u0005\u0016\f\u0002e",
    "g\u0007\u0007\u0002\u0002fe\u0003\u0002\u0002\u0002fg\u0003\u0002\u0002",
    "\u0002gj\u0003\u0002\u0002\u0002hj\u0005\u0018\r\u0002i`\u0003\u0002",
    "\u0002\u0002id\u0003\u0002\u0002\u0002ih\u0003\u0002\u0002\u0002j\u0013",
    "\u0003\u0002\u0002\u0002kn\u0007\u0018\u0002\u0002lm\t\u0002\u0002\u0002",
    "mo\u0005\u0014\u000b\u0002nl\u0003\u0002\u0002\u0002no\u0003\u0002\u0002",
    "\u0002ov\u0003\u0002\u0002\u0002ps\u0007\u0018\u0002\u0002qr\u0007\u0003",
    "\u0002\u0002rt\u0005\u0014\u000b\u0002sq\u0003\u0002\u0002\u0002st\u0003",
    "\u0002\u0002\u0002tv\u0003\u0002\u0002\u0002uk\u0003\u0002\u0002\u0002",
    "up\u0003\u0002\u0002\u0002v\u0015\u0003\u0002\u0002\u0002wx\u0007\u0018",
    "\u0002\u0002xy\u0007\n\u0002\u0002yz\u0005\u0014\u000b\u0002z{\u0007",
    "\u000b\u0002\u0002{\u0017\u0003\u0002\u0002\u0002|}\u0007\u0018\u0002",
    "\u0002}~\u0007\u0006\u0002\u0002~\u0019\u0003\u0002\u0002\u0002\u0013",
    "\u001d(-037:@FT[bfinsu"].join("");


var atn = new antlr4.atn.ATNDeserializer().deserialize(serializedATN);

var decisionsToDFA = atn.decisionToState.map( function(ds, index) { return new antlr4.dfa.DFA(ds, index); });

var sharedContextCache = new antlr4.PredictionContextCache();

var literalNames = [ null, "','", "'#'", "'::'", "':'", "';'", "'{'", "'}'", 
                     "'('", "')'", "'-'", null, "'+='", "'-='", "'*='", 
                     "'/='", "'%='", "'+'", "'*'", "'/'", "'%'", "'script'" ];

var symbolicNames = [ null, "COMMA", "HASH", "UNIQUE_NAME", "COLON", "SEMI_COL", 
                      "OPEN_BRACE", "CLOSE_BRACE", "OPEN_PARENTHESIS", "CLOSE_PARENTHESIS", 
                      "MINUS", "OP", "SUMEQ", "MINEQ", "MULEQ", "DIVEQ", 
                      "MODEQ", "SUM", "MUL", "DIV", "MOD", "SCRIPT", "ID", 
                      "WS", "LINE_COMMENT", "BLOCK_COMMENT", "DQUOTE", "TEXT" ];

var ruleNames =  [ "file", "npcPos", "npcName", "hiddenName", "uniqueName", 
                   "npcSprite", "npcTrigger", "script", "scriptBody", "scriptExpr", 
                   "scriptStmt", "label" ];

function HercScriptParser (input) {
	antlr4.Parser.call(this, input);
    this._interp = new antlr4.atn.ParserATNSimulator(this, atn, decisionsToDFA, sharedContextCache);
    this.ruleNames = ruleNames;
    this.literalNames = literalNames;
    this.symbolicNames = symbolicNames;
    return this;
}

HercScriptParser.prototype = Object.create(antlr4.Parser.prototype);
HercScriptParser.prototype.constructor = HercScriptParser;

Object.defineProperty(HercScriptParser.prototype, "atn", {
	get : function() {
		return atn;
	}
});

HercScriptParser.EOF = antlr4.Token.EOF;
HercScriptParser.COMMA = 1;
HercScriptParser.HASH = 2;
HercScriptParser.UNIQUE_NAME = 3;
HercScriptParser.COLON = 4;
HercScriptParser.SEMI_COL = 5;
HercScriptParser.OPEN_BRACE = 6;
HercScriptParser.CLOSE_BRACE = 7;
HercScriptParser.OPEN_PARENTHESIS = 8;
HercScriptParser.CLOSE_PARENTHESIS = 9;
HercScriptParser.MINUS = 10;
HercScriptParser.OP = 11;
HercScriptParser.SUMEQ = 12;
HercScriptParser.MINEQ = 13;
HercScriptParser.MULEQ = 14;
HercScriptParser.DIVEQ = 15;
HercScriptParser.MODEQ = 16;
HercScriptParser.SUM = 17;
HercScriptParser.MUL = 18;
HercScriptParser.DIV = 19;
HercScriptParser.MOD = 20;
HercScriptParser.SCRIPT = 21;
HercScriptParser.ID = 22;
HercScriptParser.WS = 23;
HercScriptParser.LINE_COMMENT = 24;
HercScriptParser.BLOCK_COMMENT = 25;
HercScriptParser.DQUOTE = 26;
HercScriptParser.TEXT = 27;

HercScriptParser.RULE_file = 0;
HercScriptParser.RULE_npcPos = 1;
HercScriptParser.RULE_npcName = 2;
HercScriptParser.RULE_hiddenName = 3;
HercScriptParser.RULE_uniqueName = 4;
HercScriptParser.RULE_npcSprite = 5;
HercScriptParser.RULE_npcTrigger = 6;
HercScriptParser.RULE_script = 7;
HercScriptParser.RULE_scriptBody = 8;
HercScriptParser.RULE_scriptExpr = 9;
HercScriptParser.RULE_scriptStmt = 10;
HercScriptParser.RULE_label = 11;


function FileContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_file;
    return this;
}

FileContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
FileContext.prototype.constructor = FileContext;

FileContext.prototype.script = function(i) {
    if(i===undefined) {
        i = null;
    }
    if(i===null) {
        return this.getTypedRuleContexts(ScriptContext);
    } else {
        return this.getTypedRuleContext(ScriptContext,i);
    }
};

FileContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterFile(this);
	}
};

FileContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitFile(this);
	}
};

FileContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitFile(this);
    } else {
        return visitor.visitChildren(this);
    }
};




HercScriptParser.FileContext = FileContext;

HercScriptParser.prototype.file = function() {

    var localctx = new FileContext(this, this._ctx, this.state);
    this.enterRule(localctx, 0, HercScriptParser.RULE_file);
    var _la = 0; // Token type
    try {
        this.enterOuterAlt(localctx, 1);
        this.state = 27;
        this._errHandler.sync(this);
        _la = this._input.LA(1);
        while(_la===HercScriptParser.MINUS || _la===HercScriptParser.ID) {
            this.state = 24;
            this.script();
            this.state = 29;
            this._errHandler.sync(this);
            _la = this._input.LA(1);
        }
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function NpcPosContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_npcPos;
    return this;
}

NpcPosContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
NpcPosContext.prototype.constructor = NpcPosContext;


 
NpcPosContext.prototype.copyFrom = function(ctx) {
    antlr4.ParserRuleContext.prototype.copyFrom.call(this, ctx);
};


function OnMapContext(parser, ctx) {
	NpcPosContext.call(this, parser);
    NpcPosContext.prototype.copyFrom.call(this, ctx);
    return this;
}

OnMapContext.prototype = Object.create(NpcPosContext.prototype);
OnMapContext.prototype.constructor = OnMapContext;

HercScriptParser.OnMapContext = OnMapContext;

OnMapContext.prototype.ID = function(i) {
	if(i===undefined) {
		i = null;
	}
    if(i===null) {
        return this.getTokens(HercScriptParser.ID);
    } else {
        return this.getToken(HercScriptParser.ID, i);
    }
};


OnMapContext.prototype.COMMA = function(i) {
	if(i===undefined) {
		i = null;
	}
    if(i===null) {
        return this.getTokens(HercScriptParser.COMMA);
    } else {
        return this.getToken(HercScriptParser.COMMA, i);
    }
};

OnMapContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterOnMap(this);
	}
};

OnMapContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitOnMap(this);
	}
};

OnMapContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitOnMap(this);
    } else {
        return visitor.visitChildren(this);
    }
};


function FloatingContext(parser, ctx) {
	NpcPosContext.call(this, parser);
    NpcPosContext.prototype.copyFrom.call(this, ctx);
    return this;
}

FloatingContext.prototype = Object.create(NpcPosContext.prototype);
FloatingContext.prototype.constructor = FloatingContext;

HercScriptParser.FloatingContext = FloatingContext;

FloatingContext.prototype.MINUS = function() {
    return this.getToken(HercScriptParser.MINUS, 0);
};
FloatingContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterFloating(this);
	}
};

FloatingContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitFloating(this);
	}
};

FloatingContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitFloating(this);
    } else {
        return visitor.visitChildren(this);
    }
};



HercScriptParser.NpcPosContext = NpcPosContext;

HercScriptParser.prototype.npcPos = function() {

    var localctx = new NpcPosContext(this, this._ctx, this.state);
    this.enterRule(localctx, 2, HercScriptParser.RULE_npcPos);
    try {
        this.state = 38;
        this._errHandler.sync(this);
        switch(this._input.LA(1)) {
        case HercScriptParser.ID:
            localctx = new OnMapContext(this, localctx);
            this.enterOuterAlt(localctx, 1);
            this.state = 30;
            this.match(HercScriptParser.ID);
            this.state = 31;
            this.match(HercScriptParser.COMMA);
            this.state = 32;
            this.match(HercScriptParser.ID);
            this.state = 33;
            this.match(HercScriptParser.COMMA);
            this.state = 34;
            this.match(HercScriptParser.ID);
            this.state = 35;
            this.match(HercScriptParser.COMMA);
            this.state = 36;
            this.match(HercScriptParser.ID);
            break;
        case HercScriptParser.MINUS:
            localctx = new FloatingContext(this, localctx);
            this.enterOuterAlt(localctx, 2);
            this.state = 37;
            this.match(HercScriptParser.MINUS);
            break;
        default:
            throw new antlr4.error.NoViableAltException(this);
        }
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function NpcNameContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_npcName;
    return this;
}

NpcNameContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
NpcNameContext.prototype.constructor = NpcNameContext;

NpcNameContext.prototype.ID = function(i) {
	if(i===undefined) {
		i = null;
	}
    if(i===null) {
        return this.getTokens(HercScriptParser.ID);
    } else {
        return this.getToken(HercScriptParser.ID, i);
    }
};


NpcNameContext.prototype.hiddenName = function() {
    return this.getTypedRuleContext(HiddenNameContext,0);
};

NpcNameContext.prototype.uniqueName = function() {
    return this.getTypedRuleContext(UniqueNameContext,0);
};

NpcNameContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterNpcName(this);
	}
};

NpcNameContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitNpcName(this);
	}
};

NpcNameContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitNpcName(this);
    } else {
        return visitor.visitChildren(this);
    }
};




HercScriptParser.NpcNameContext = NpcNameContext;

HercScriptParser.prototype.npcName = function() {

    var localctx = new NpcNameContext(this, this._ctx, this.state);
    this.enterRule(localctx, 4, HercScriptParser.RULE_npcName);
    var _la = 0; // Token type
    try {
        this.state = 56;
        this._errHandler.sync(this);
        switch(this._input.LA(1)) {
        case HercScriptParser.ID:
            this.enterOuterAlt(localctx, 1);
            this.state = 41; 
            this._errHandler.sync(this);
            var _alt = 1;
            do {
            	switch (_alt) {
            	case 1:
            		this.state = 40;
            		this.match(HercScriptParser.ID);
            		break;
            	default:
            		throw new antlr4.error.NoViableAltException(this);
            	}
            	this.state = 43; 
            	this._errHandler.sync(this);
            	_alt = this._interp.adaptivePredict(this._input,2, this._ctx);
            } while ( _alt!=2 && _alt!=antlr4.atn.ATN.INVALID_ALT_NUMBER );
            this.state = 46;
            this._errHandler.sync(this);
            _la = this._input.LA(1);
            if(_la===HercScriptParser.HASH) {
                this.state = 45;
                this.hiddenName();
            }

            this.state = 49;
            this._errHandler.sync(this);
            _la = this._input.LA(1);
            if(_la===HercScriptParser.UNIQUE_NAME) {
                this.state = 48;
                this.uniqueName();
            }

            break;
        case HercScriptParser.HASH:
            this.enterOuterAlt(localctx, 2);
            this.state = 51;
            this.hiddenName();
            this.state = 53;
            this._errHandler.sync(this);
            _la = this._input.LA(1);
            if(_la===HercScriptParser.UNIQUE_NAME) {
                this.state = 52;
                this.uniqueName();
            }

            break;
        case HercScriptParser.UNIQUE_NAME:
            this.enterOuterAlt(localctx, 3);
            this.state = 55;
            this.uniqueName();
            break;
        default:
            throw new antlr4.error.NoViableAltException(this);
        }
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function HiddenNameContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_hiddenName;
    return this;
}

HiddenNameContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
HiddenNameContext.prototype.constructor = HiddenNameContext;

HiddenNameContext.prototype.HASH = function() {
    return this.getToken(HercScriptParser.HASH, 0);
};

HiddenNameContext.prototype.ID = function(i) {
	if(i===undefined) {
		i = null;
	}
    if(i===null) {
        return this.getTokens(HercScriptParser.ID);
    } else {
        return this.getToken(HercScriptParser.ID, i);
    }
};


HiddenNameContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterHiddenName(this);
	}
};

HiddenNameContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitHiddenName(this);
	}
};

HiddenNameContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitHiddenName(this);
    } else {
        return visitor.visitChildren(this);
    }
};




HercScriptParser.HiddenNameContext = HiddenNameContext;

HercScriptParser.prototype.hiddenName = function() {

    var localctx = new HiddenNameContext(this, this._ctx, this.state);
    this.enterRule(localctx, 6, HercScriptParser.RULE_hiddenName);
    try {
        this.enterOuterAlt(localctx, 1);
        this.state = 58;
        this.match(HercScriptParser.HASH);
        this.state = 60; 
        this._errHandler.sync(this);
        var _alt = 1;
        do {
        	switch (_alt) {
        	case 1:
        		this.state = 59;
        		this.match(HercScriptParser.ID);
        		break;
        	default:
        		throw new antlr4.error.NoViableAltException(this);
        	}
        	this.state = 62; 
        	this._errHandler.sync(this);
        	_alt = this._interp.adaptivePredict(this._input,7, this._ctx);
        } while ( _alt!=2 && _alt!=antlr4.atn.ATN.INVALID_ALT_NUMBER );
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function UniqueNameContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_uniqueName;
    return this;
}

UniqueNameContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
UniqueNameContext.prototype.constructor = UniqueNameContext;

UniqueNameContext.prototype.UNIQUE_NAME = function() {
    return this.getToken(HercScriptParser.UNIQUE_NAME, 0);
};

UniqueNameContext.prototype.ID = function(i) {
	if(i===undefined) {
		i = null;
	}
    if(i===null) {
        return this.getTokens(HercScriptParser.ID);
    } else {
        return this.getToken(HercScriptParser.ID, i);
    }
};


UniqueNameContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterUniqueName(this);
	}
};

UniqueNameContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitUniqueName(this);
	}
};

UniqueNameContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitUniqueName(this);
    } else {
        return visitor.visitChildren(this);
    }
};




HercScriptParser.UniqueNameContext = UniqueNameContext;

HercScriptParser.prototype.uniqueName = function() {

    var localctx = new UniqueNameContext(this, this._ctx, this.state);
    this.enterRule(localctx, 8, HercScriptParser.RULE_uniqueName);
    try {
        this.enterOuterAlt(localctx, 1);
        this.state = 64;
        this.match(HercScriptParser.UNIQUE_NAME);
        this.state = 66; 
        this._errHandler.sync(this);
        var _alt = 1;
        do {
        	switch (_alt) {
        	case 1:
        		this.state = 65;
        		this.match(HercScriptParser.ID);
        		break;
        	default:
        		throw new antlr4.error.NoViableAltException(this);
        	}
        	this.state = 68; 
        	this._errHandler.sync(this);
        	_alt = this._interp.adaptivePredict(this._input,8, this._ctx);
        } while ( _alt!=2 && _alt!=antlr4.atn.ATN.INVALID_ALT_NUMBER );
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function NpcSpriteContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_npcSprite;
    return this;
}

NpcSpriteContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
NpcSpriteContext.prototype.constructor = NpcSpriteContext;

NpcSpriteContext.prototype.ID = function() {
    return this.getToken(HercScriptParser.ID, 0);
};

NpcSpriteContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterNpcSprite(this);
	}
};

NpcSpriteContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitNpcSprite(this);
	}
};

NpcSpriteContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitNpcSprite(this);
    } else {
        return visitor.visitChildren(this);
    }
};




HercScriptParser.NpcSpriteContext = NpcSpriteContext;

HercScriptParser.prototype.npcSprite = function() {

    var localctx = new NpcSpriteContext(this, this._ctx, this.state);
    this.enterRule(localctx, 10, HercScriptParser.RULE_npcSprite);
    try {
        this.enterOuterAlt(localctx, 1);
        this.state = 70;
        this.match(HercScriptParser.ID);
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function NpcTriggerContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_npcTrigger;
    return this;
}

NpcTriggerContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
NpcTriggerContext.prototype.constructor = NpcTriggerContext;

NpcTriggerContext.prototype.COMMA = function(i) {
	if(i===undefined) {
		i = null;
	}
    if(i===null) {
        return this.getTokens(HercScriptParser.COMMA);
    } else {
        return this.getToken(HercScriptParser.COMMA, i);
    }
};


NpcTriggerContext.prototype.ID = function(i) {
	if(i===undefined) {
		i = null;
	}
    if(i===null) {
        return this.getTokens(HercScriptParser.ID);
    } else {
        return this.getToken(HercScriptParser.ID, i);
    }
};


NpcTriggerContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterNpcTrigger(this);
	}
};

NpcTriggerContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitNpcTrigger(this);
	}
};

NpcTriggerContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitNpcTrigger(this);
    } else {
        return visitor.visitChildren(this);
    }
};




HercScriptParser.NpcTriggerContext = NpcTriggerContext;

HercScriptParser.prototype.npcTrigger = function() {

    var localctx = new NpcTriggerContext(this, this._ctx, this.state);
    this.enterRule(localctx, 12, HercScriptParser.RULE_npcTrigger);
    try {
        this.enterOuterAlt(localctx, 1);
        this.state = 72;
        this.match(HercScriptParser.COMMA);
        this.state = 73;
        this.match(HercScriptParser.ID);
        this.state = 74;
        this.match(HercScriptParser.COMMA);
        this.state = 75;
        this.match(HercScriptParser.ID);
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function ScriptContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_script;
    return this;
}

ScriptContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
ScriptContext.prototype.constructor = ScriptContext;

ScriptContext.prototype.npcPos = function() {
    return this.getTypedRuleContext(NpcPosContext,0);
};

ScriptContext.prototype.SCRIPT = function() {
    return this.getToken(HercScriptParser.SCRIPT, 0);
};

ScriptContext.prototype.npcName = function() {
    return this.getTypedRuleContext(NpcNameContext,0);
};

ScriptContext.prototype.npcSprite = function() {
    return this.getTypedRuleContext(NpcSpriteContext,0);
};

ScriptContext.prototype.COMMA = function() {
    return this.getToken(HercScriptParser.COMMA, 0);
};

ScriptContext.prototype.OPEN_BRACE = function() {
    return this.getToken(HercScriptParser.OPEN_BRACE, 0);
};

ScriptContext.prototype.CLOSE_BRACE = function() {
    return this.getToken(HercScriptParser.CLOSE_BRACE, 0);
};

ScriptContext.prototype.npcTrigger = function() {
    return this.getTypedRuleContext(NpcTriggerContext,0);
};

ScriptContext.prototype.scriptBody = function(i) {
    if(i===undefined) {
        i = null;
    }
    if(i===null) {
        return this.getTypedRuleContexts(ScriptBodyContext);
    } else {
        return this.getTypedRuleContext(ScriptBodyContext,i);
    }
};

ScriptContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterScript(this);
	}
};

ScriptContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitScript(this);
	}
};

ScriptContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitScript(this);
    } else {
        return visitor.visitChildren(this);
    }
};




HercScriptParser.ScriptContext = ScriptContext;

HercScriptParser.prototype.script = function() {

    var localctx = new ScriptContext(this, this._ctx, this.state);
    this.enterRule(localctx, 14, HercScriptParser.RULE_script);
    var _la = 0; // Token type
    try {
        this.enterOuterAlt(localctx, 1);
        this.state = 77;
        this.npcPos();
        this.state = 78;
        this.match(HercScriptParser.SCRIPT);
        this.state = 79;
        this.npcName();
        this.state = 80;
        this.npcSprite();
        this.state = 82;
        this._errHandler.sync(this);
        var la_ = this._interp.adaptivePredict(this._input,9,this._ctx);
        if(la_===1) {
            this.state = 81;
            this.npcTrigger();

        }
        this.state = 84;
        this.match(HercScriptParser.COMMA);
        this.state = 85;
        this.match(HercScriptParser.OPEN_BRACE);
        this.state = 89;
        this._errHandler.sync(this);
        _la = this._input.LA(1);
        while(_la===HercScriptParser.ID) {
            this.state = 86;
            this.scriptBody();
            this.state = 91;
            this._errHandler.sync(this);
            _la = this._input.LA(1);
        }
        this.state = 92;
        this.match(HercScriptParser.CLOSE_BRACE);
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function ScriptBodyContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_scriptBody;
    return this;
}

ScriptBodyContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
ScriptBodyContext.prototype.constructor = ScriptBodyContext;

ScriptBodyContext.prototype.scriptExpr = function() {
    return this.getTypedRuleContext(ScriptExprContext,0);
};

ScriptBodyContext.prototype.SEMI_COL = function() {
    return this.getToken(HercScriptParser.SEMI_COL, 0);
};

ScriptBodyContext.prototype.scriptStmt = function() {
    return this.getTypedRuleContext(ScriptStmtContext,0);
};

ScriptBodyContext.prototype.label = function() {
    return this.getTypedRuleContext(LabelContext,0);
};

ScriptBodyContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterScriptBody(this);
	}
};

ScriptBodyContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitScriptBody(this);
	}
};

ScriptBodyContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitScriptBody(this);
    } else {
        return visitor.visitChildren(this);
    }
};




HercScriptParser.ScriptBodyContext = ScriptBodyContext;

HercScriptParser.prototype.scriptBody = function() {

    var localctx = new ScriptBodyContext(this, this._ctx, this.state);
    this.enterRule(localctx, 16, HercScriptParser.RULE_scriptBody);
    var _la = 0; // Token type
    try {
        this.state = 103;
        this._errHandler.sync(this);
        var la_ = this._interp.adaptivePredict(this._input,13,this._ctx);
        switch(la_) {
        case 1:
            this.enterOuterAlt(localctx, 1);
            this.state = 94;
            this.scriptExpr();
            this.state = 96;
            this._errHandler.sync(this);
            _la = this._input.LA(1);
            if(_la===HercScriptParser.SEMI_COL) {
                this.state = 95;
                this.match(HercScriptParser.SEMI_COL);
            }

            break;

        case 2:
            this.enterOuterAlt(localctx, 2);
            this.state = 98;
            this.scriptStmt();
            this.state = 100;
            this._errHandler.sync(this);
            _la = this._input.LA(1);
            if(_la===HercScriptParser.SEMI_COL) {
                this.state = 99;
                this.match(HercScriptParser.SEMI_COL);
            }

            break;

        case 3:
            this.enterOuterAlt(localctx, 3);
            this.state = 102;
            this.label();
            break;

        }
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function ScriptExprContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_scriptExpr;
    return this;
}

ScriptExprContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
ScriptExprContext.prototype.constructor = ScriptExprContext;


 
ScriptExprContext.prototype.copyFrom = function(ctx) {
    antlr4.ParserRuleContext.prototype.copyFrom.call(this, ctx);
};


function ParamExprContext(parser, ctx) {
	ScriptExprContext.call(this, parser);
    ScriptExprContext.prototype.copyFrom.call(this, ctx);
    return this;
}

ParamExprContext.prototype = Object.create(ScriptExprContext.prototype);
ParamExprContext.prototype.constructor = ParamExprContext;

HercScriptParser.ParamExprContext = ParamExprContext;

ParamExprContext.prototype.ID = function() {
    return this.getToken(HercScriptParser.ID, 0);
};

ParamExprContext.prototype.COMMA = function() {
    return this.getToken(HercScriptParser.COMMA, 0);
};

ParamExprContext.prototype.scriptExpr = function() {
    return this.getTypedRuleContext(ScriptExprContext,0);
};
ParamExprContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterParamExpr(this);
	}
};

ParamExprContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitParamExpr(this);
	}
};

ParamExprContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitParamExpr(this);
    } else {
        return visitor.visitChildren(this);
    }
};


function MathExprContext(parser, ctx) {
	ScriptExprContext.call(this, parser);
    ScriptExprContext.prototype.copyFrom.call(this, ctx);
    return this;
}

MathExprContext.prototype = Object.create(ScriptExprContext.prototype);
MathExprContext.prototype.constructor = MathExprContext;

HercScriptParser.MathExprContext = MathExprContext;

MathExprContext.prototype.ID = function() {
    return this.getToken(HercScriptParser.ID, 0);
};

MathExprContext.prototype.scriptExpr = function() {
    return this.getTypedRuleContext(ScriptExprContext,0);
};

MathExprContext.prototype.OP = function() {
    return this.getToken(HercScriptParser.OP, 0);
};

MathExprContext.prototype.MINUS = function() {
    return this.getToken(HercScriptParser.MINUS, 0);
};
MathExprContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterMathExpr(this);
	}
};

MathExprContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitMathExpr(this);
	}
};

MathExprContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitMathExpr(this);
    } else {
        return visitor.visitChildren(this);
    }
};



HercScriptParser.ScriptExprContext = ScriptExprContext;

HercScriptParser.prototype.scriptExpr = function() {

    var localctx = new ScriptExprContext(this, this._ctx, this.state);
    this.enterRule(localctx, 18, HercScriptParser.RULE_scriptExpr);
    var _la = 0; // Token type
    try {
        this.state = 115;
        this._errHandler.sync(this);
        var la_ = this._interp.adaptivePredict(this._input,16,this._ctx);
        switch(la_) {
        case 1:
            localctx = new MathExprContext(this, localctx);
            this.enterOuterAlt(localctx, 1);
            this.state = 105;
            this.match(HercScriptParser.ID);
            this.state = 108;
            this._errHandler.sync(this);
            _la = this._input.LA(1);
            if(_la===HercScriptParser.MINUS || _la===HercScriptParser.OP) {
                this.state = 106;
                _la = this._input.LA(1);
                if(!(_la===HercScriptParser.MINUS || _la===HercScriptParser.OP)) {
                this._errHandler.recoverInline(this);
                }
                else {
                	this._errHandler.reportMatch(this);
                    this.consume();
                }
                this.state = 107;
                this.scriptExpr();
            }

            break;

        case 2:
            localctx = new ParamExprContext(this, localctx);
            this.enterOuterAlt(localctx, 2);
            this.state = 110;
            this.match(HercScriptParser.ID);
            this.state = 113;
            this._errHandler.sync(this);
            _la = this._input.LA(1);
            if(_la===HercScriptParser.COMMA) {
                this.state = 111;
                this.match(HercScriptParser.COMMA);
                this.state = 112;
                this.scriptExpr();
            }

            break;

        }
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function ScriptStmtContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_scriptStmt;
    return this;
}

ScriptStmtContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
ScriptStmtContext.prototype.constructor = ScriptStmtContext;

ScriptStmtContext.prototype.ID = function() {
    return this.getToken(HercScriptParser.ID, 0);
};

ScriptStmtContext.prototype.OPEN_PARENTHESIS = function() {
    return this.getToken(HercScriptParser.OPEN_PARENTHESIS, 0);
};

ScriptStmtContext.prototype.scriptExpr = function() {
    return this.getTypedRuleContext(ScriptExprContext,0);
};

ScriptStmtContext.prototype.CLOSE_PARENTHESIS = function() {
    return this.getToken(HercScriptParser.CLOSE_PARENTHESIS, 0);
};

ScriptStmtContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterScriptStmt(this);
	}
};

ScriptStmtContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitScriptStmt(this);
	}
};

ScriptStmtContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitScriptStmt(this);
    } else {
        return visitor.visitChildren(this);
    }
};




HercScriptParser.ScriptStmtContext = ScriptStmtContext;

HercScriptParser.prototype.scriptStmt = function() {

    var localctx = new ScriptStmtContext(this, this._ctx, this.state);
    this.enterRule(localctx, 20, HercScriptParser.RULE_scriptStmt);
    try {
        this.enterOuterAlt(localctx, 1);
        this.state = 117;
        this.match(HercScriptParser.ID);
        this.state = 118;
        this.match(HercScriptParser.OPEN_PARENTHESIS);
        this.state = 119;
        this.scriptExpr();
        this.state = 120;
        this.match(HercScriptParser.CLOSE_PARENTHESIS);
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


function LabelContext(parser, parent, invokingState) {
	if(parent===undefined) {
	    parent = null;
	}
	if(invokingState===undefined || invokingState===null) {
		invokingState = -1;
	}
	antlr4.ParserRuleContext.call(this, parent, invokingState);
    this.parser = parser;
    this.ruleIndex = HercScriptParser.RULE_label;
    return this;
}

LabelContext.prototype = Object.create(antlr4.ParserRuleContext.prototype);
LabelContext.prototype.constructor = LabelContext;

LabelContext.prototype.ID = function() {
    return this.getToken(HercScriptParser.ID, 0);
};

LabelContext.prototype.COLON = function() {
    return this.getToken(HercScriptParser.COLON, 0);
};

LabelContext.prototype.enterRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.enterLabel(this);
	}
};

LabelContext.prototype.exitRule = function(listener) {
    if(listener instanceof HercScriptParserListener ) {
        listener.exitLabel(this);
	}
};

LabelContext.prototype.accept = function(visitor) {
    if ( visitor instanceof HercScriptParserVisitor ) {
        return visitor.visitLabel(this);
    } else {
        return visitor.visitChildren(this);
    }
};




HercScriptParser.LabelContext = LabelContext;

HercScriptParser.prototype.label = function() {

    var localctx = new LabelContext(this, this._ctx, this.state);
    this.enterRule(localctx, 22, HercScriptParser.RULE_label);
    try {
        this.enterOuterAlt(localctx, 1);
        this.state = 122;
        this.match(HercScriptParser.ID);
        this.state = 123;
        this.match(HercScriptParser.COLON);
    } catch (re) {
    	if(re instanceof antlr4.error.RecognitionException) {
	        localctx.exception = re;
	        this._errHandler.reportError(this, re);
	        this._errHandler.recover(this, re);
	    } else {
	    	throw re;
	    }
    } finally {
        this.exitRule();
    }
    return localctx;
};


exports.HercScriptParser = HercScriptParser;
