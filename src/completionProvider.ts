'use strict';

import * as vscode from 'vscode';
import cmdList = require("./commands.json");
import { format } from 'util';

export function registerCompletionProviders(context: vscode.ExtensionContext) {

    context.subscriptions.concat(
        vscode.languages.registerCompletionItemProvider('hercscript', {
            provideCompletionItems(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken, context: vscode.CompletionContext) {
                let completeList: Array<vscode.CompletionItem> = [];

                cmdList.commands.forEach((cmd) => {
                    // https://code.visualstudio.com/api/references/vscode-api#CompletionItem
                    const completion = new vscode.CompletionItem(cmd.name);
                    //completion.insertText = new vscode.SnippetString(cmd.name+"(${1})");
                    let returnType = "(void)";
                    let params = "";

                    if (cmd.params != null) {
                        let paramList = [];
                        cmd.params.forEach(param => {
                            paramList.push(param.name + ": " + param.type);
                        })
                        params = paramList.join(",");
                    }
                    if (cmd.return != null) {
                        returnType = format("(%s)", cmd.return);
                    }

                    completion.detail = format("%s %s(%s)", returnType, cmd.name, params);
                    completion.documentation = "Documentation";
                    completion.kind = vscode.CompletionItemKind.Function;
                    completeList.push(completion);
                });
                
                return completeList;
            }
        })
    );

}
