'use strict';

import * as vscode from 'vscode';
import * as parser from "./parser";
import * as cmdHelp from "./commandHelper";
import { format } from 'util';

export function register(context: vscode.ExtensionContext) {

    context.subscriptions.concat(
        vscode.languages.registerSignatureHelpProvider('hercscript', {
            provideSignatureHelp(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken): vscode.ProviderResult<vscode.SignatureHelp> {
                const cursorContext: parser.ContextInfo = parser.getContext(document, position);

                if (cursorContext === null || cmdHelp.cmd[cursorContext.funcName] === null) {
                    return null; // Not a command
                }

                const cmdInfo = cmdHelp.cmd[cursorContext.funcName];

                // https://code.visualstudio.com/api/references/vscode-api#SignatureHelp
                let signature: vscode.SignatureHelp = new vscode.SignatureHelp();
                signature.activeParameter = cursorContext.paramNum - 1;
                signature.activeSignature = 0; // TODO : Multiple signatures support
                signature.signatures = generateSignatureInfos(cursorContext.funcName, cmdInfo);

                return signature;
            }
        }, "(", ",")
    );
}

function generateSignatureInfos(cmdName: string, cmdInfo): vscode.SignatureInformation[] {
    let signatures: vscode.SignatureInformation[] = [];

    cmdInfo.params.forEach(signGroup => {
        const signatureInfo = new vscode.SignatureInformation(
            cmdHelp.getCommandSignature(cmdName, cmdInfo, signGroup),
            cmdHelp.getCommandDocumentation(cmdInfo)
        );

        signatureInfo.parameters = [];

        // FIXME : Can we move that to commandHelper somehow?
        Object.keys(signGroup).forEach(parName => {
            console.log(format("%s: %s", parName, signGroup[parName]));
            signatureInfo.parameters.push(
                new vscode.ParameterInformation(
                    cmdHelp.formatParam(parName, signGroup[parName]),
                    new vscode.MarkdownString(cmdHelp.getParameterDocumentation(cmdInfo, parName))
                )
            )
        });

        signatures.push(signatureInfo);
    });

    return signatures;
}
