import { HercScriptParserVisitor } from "../../antlr-parser/HercScriptParserVisitor";
import { TextDocumentPositionParams } from "vscode-languageserver";
import * as documentsManager from "../helpers/documentsManager";

export type ContextInfo = {
    readonly funcName: string;
    readonly paramNum: number;
}

export function getContext(params: TextDocumentPositionParams): ContextInfo {

    var antlr4 = require('antlr4');
    var HercScriptLexer = require('../../antlr-parser/HercScriptLexer').HercScriptLexer;
    var HercScriptParser = require('../../antlr-parser/HercScriptParser').HercScriptParser;
    var HercScriptListener = require('../../antlr-parser/HercScriptListener').HercScriptListener;

    var input = documentsManager.getText(params.textDocument);
    var chars = new antlr4.InputStream(input);
    var lexer = new HercScriptLexer(chars);
    var tokens = new antlr4.CommonTokenStream(lexer);
    var parser = new HercScriptParser(tokens);
    parser.buildParseTrees = true;
    var tree = parser.file();

    // let visitor = function() {
    //     require('../parser/HercScriptParserVisitor').HercScriptParserVisitor.call(this); // inherit default listener
    //     return this;
    // };

    // continue inheriting default listener
    // KeyPrinter.prototype = Object.create(MyGrammarListener.prototype);
    // KeyPrinter.prototype.constructor = KeyPrinter;

    // // override default listener behavior
    // KeyPrinter.prototype.exitKey = function(ctx) {
    //     console.log("Oh, a key!");
    // };

    let visitor = new Visitor(params.position.line, params.position.character);
    tree.accept(visitor);

    if (!!visitor.contextInfo && !!visitor.contextInfo.funcName) {
        return visitor.contextInfo;
    } else {
        return null;
    }
}

class Visitor extends HercScriptParserVisitor {
    searchLine: number;
    searchChar: number;
    found: boolean;
    contextInfo: ContextInfo;

    constructor(searchLine: number, searchChar: number) {
        //require('../parser/HercScriptParserVisitor').HercScriptParserVisitor.call(this); // inherit default listener
        super();
        this.searchLine = searchLine + 1; // ANTLR starts on line 1, while code on line 0
        this.searchChar = searchChar;
        this.found = false;
    }

    // visitChildren(ctx) {
    //     if (!ctx) {
    //         return;
    //     }

    //     if (ctx.children) {
    //         return ctx.children.map(child => {
    //             if (child.children && child.children.length != 0) {
    //                 return child.accept(this);
    //             } else {
    //                 return child.getText();
    //             }
    //         });
    //     }
    // }

    visitScriptStmt(ctx) {
        if (!ctx || this.found) {
            return;
        }

        // FIXME : Maybe this could miss things when we have more than 1 ctx
        //         in the same line

        let openParen = ctx.OPEN_PARENTHESIS().getSymbol();
        let closeParen = ctx.CLOSE_PARENTHESIS().getSymbol();
        if (openParen.line <= this.searchLine && openParen.column <= this.searchChar
            && closeParen.line >= this.searchLine && closeParen.column > this.searchChar) {
            this.searchContext(ctx.ID().getText(), ctx.scriptExpr()); // Note : assumes it is ScriptExpr
            return;
        }

        if (ctx.children) {
            return ctx.children.map(child => {
                if (child.children && child.children.length != 0) {
                    return child.accept(this);
                } else {
                    return child.getText();
                }
            });
        }
    }

    // ctx is ScriptExpr
    searchContext(funcName: string, ctx) {
        if (!ctx.children) {
            return;
        }

        let paramNum = 1;
        // usar while aqui pra procurar até achar...
        let expr = ctx;
        do {
            if (typeof expr.COMMA !== "function") {
                expr = null;
            } else {
                // FIXME : Current grammar allows it to take the mathExpression
                //         rule on the last parameter, where COMMA doesn't exists.
                const comma = expr.COMMA();

                if (comma != null && comma.getSymbol().column <= this.searchChar) {
                    paramNum++;
                    expr = expr.scriptExpr();
                } else {
                    this.found = true;
                }
            }
        } while (!this.found && expr != null);

        this.contextInfo = { funcName: funcName, paramNum: paramNum };
    }
}