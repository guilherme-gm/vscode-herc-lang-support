'use strict';

import * as vscode from 'vscode';
import * as cmdHelper from "./commandHelper";

export function registerCompletionProviders(context: vscode.ExtensionContext) {

    context.subscriptions.concat(
        vscode.languages.registerCompletionItemProvider('hercscript', {
            provideCompletionItems(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken, context: vscode.CompletionContext) {
                let completeList: Array<vscode.CompletionItem> = [];
                
                Object.keys(cmdHelper.cmd).forEach((cmdName) => {
                    // https://code.visualstudio.com/api/references/vscode-api#CompletionItem
                    const completion = new vscode.CompletionItem(cmdName, vscode.CompletionItemKind.Function);
                    completion.detail = cmdHelper.getCommandSignature(cmdName, cmdHelper.cmd[cmdName]); // FIXME : I think we can simply get rid of the second param
                    completion.documentation = new vscode.MarkdownString(cmdHelper.getCommandDocumentation(cmdHelper.cmd[cmdName])); // FixME : I think we can use cmd name instead
                    completeList.push(completion);
                    // TODO : Could give higher priority to commands that has the return type of the context
                });
                
                return completeList;
            }
        })
    );

}
