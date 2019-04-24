import {
    TextDocumentPositionParams,
    SignatureHelp,
    SignatureInformation,
    MarkedString,
    MarkupKind
} from "vscode-languageserver";
import * as parser from "../parser/parser";
import * as cmdHelp from "../helpers/commandHelper";

export function getSignature(params: TextDocumentPositionParams): SignatureHelp {
    const cursorContext: parser.ContextInfo = parser.getContext(params);

    if (cursorContext == null || cmdHelp.cmd[cursorContext.funcName] == null) {
        return null; // Not a command
    }

    const cmdInfo = cmdHelp.cmd[cursorContext.funcName];

    // https://code.visualstudio.com/api/references/vscode-api#SignatureHelp
    // TODO : Multiple Signatures support

    return {
        activeParameter: cursorContext.paramNum - 1,
        activeSignature: 0,
        signatures: generateSignatureInfos(cursorContext.funcName, cmdInfo)
    };
}

function generateSignatureInfos(cmdName: string, cmdInfo): SignatureInformation[] {
    let signatures: SignatureInformation[] = [];

    cmdInfo.params.forEach(signGroup => {
        const signatureInfo: SignatureInformation = {
            label: cmdHelp.getCommandSignature(cmdName, cmdInfo, signGroup),
            documentation: cmdHelp.getCommandDocumentation(cmdInfo)
        };

        signatureInfo.parameters = [];

        // FIXME : Can we move that to commandHelper somehow?
        Object.keys(signGroup).forEach(parName => {
            //console.log(format("%s: %s", parName, signGroup[parName]));
            signatureInfo.parameters.push(
                {
                    label: cmdHelp.formatParam(parName, signGroup[parName]),
                    documentation: {
                        kind: MarkupKind.Markdown,
                        value: cmdHelp.getParameterDocumentation(cmdInfo, parName)
                    }
                }
            );
        });

        signatures.push(signatureInfo);
    });

    return signatures;
}
