// Generated from ./parser-gen/HercScript.g4 by ANTLR 4.7.2
// jshint ignore: start
var antlr4 = require('antlr4/index');

// This class defines a complete generic visitor for a parse tree produced by HercScriptParser.

function HercScriptVisitor() {
	antlr4.tree.ParseTreeVisitor.call(this);
	return this;
}

HercScriptVisitor.prototype = Object.create(antlr4.tree.ParseTreeVisitor.prototype);
HercScriptVisitor.prototype.constructor = HercScriptVisitor;

// Visit a parse tree produced by HercScriptParser#r.
HercScriptVisitor.prototype.visitR = function(ctx) {
  return this.visitChildren(ctx);
};



exports.HercScriptVisitor = HercScriptVisitor;