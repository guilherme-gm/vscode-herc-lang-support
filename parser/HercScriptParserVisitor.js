// Generated from ./parser-gen/HercScriptParser.g4 by ANTLR 4.7.2
// jshint ignore: start
var antlr4 = require('antlr4/index');

// This class defines a complete generic visitor for a parse tree produced by HercScriptParser.

function HercScriptParserVisitor() {
	antlr4.tree.ParseTreeVisitor.call(this);
	return this;
}

HercScriptParserVisitor.prototype = Object.create(antlr4.tree.ParseTreeVisitor.prototype);
HercScriptParserVisitor.prototype.constructor = HercScriptParserVisitor;

// Visit a parse tree produced by HercScriptParser#file.
HercScriptParserVisitor.prototype.visitFile = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#OnMap.
HercScriptParserVisitor.prototype.visitOnMap = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#Floating.
HercScriptParserVisitor.prototype.visitFloating = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#npcName.
HercScriptParserVisitor.prototype.visitNpcName = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#hiddenName.
HercScriptParserVisitor.prototype.visitHiddenName = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#uniqueName.
HercScriptParserVisitor.prototype.visitUniqueName = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#npcSprite.
HercScriptParserVisitor.prototype.visitNpcSprite = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#npcTrigger.
HercScriptParserVisitor.prototype.visitNpcTrigger = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#script.
HercScriptParserVisitor.prototype.visitScript = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#scriptBody.
HercScriptParserVisitor.prototype.visitScriptBody = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#mathExpr.
HercScriptParserVisitor.prototype.visitMathExpr = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#paramExpr.
HercScriptParserVisitor.prototype.visitParamExpr = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#scriptStmt.
HercScriptParserVisitor.prototype.visitScriptStmt = function(ctx) {
  return this.visitChildren(ctx);
};


// Visit a parse tree produced by HercScriptParser#label.
HercScriptParserVisitor.prototype.visitLabel = function(ctx) {
  return this.visitChildren(ctx);
};



exports.HercScriptParserVisitor = HercScriptParserVisitor;