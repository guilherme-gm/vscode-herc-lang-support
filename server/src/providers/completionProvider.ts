'use strict';

import {
    TextDocumentPositionParams,
    CompletionItem,
    CompletionItemKind,
    MarkupContent,
    MarkupKind
} from "vscode-languageserver";
import * as cmdHelper from "../helpers/commandHelper";

/**
 * Retrieves and returns an array of possible inputs to complete the code
 * where user cursor is currently at. Documentation is only provided later on
 * by getCompletionInfo
 * 
 * @param _textDocumentPosition Information about where the completion is being requested
 */
export function getCompletions(_textDocumentPosition: TextDocumentPositionParams): CompletionItem[] {
    let completeList: Array<CompletionItem> = [];

    // TODO : Improve autocomplete based on position
    Object.keys(cmdHelper.cmd).forEach((cmdName) => {
        const completion: CompletionItem = {
            label: cmdName,
            kind: CompletionItemKind.Function,
            data: cmdName
        };
        completeList.push(completion);
    });

    // TODO : I think it is possible : Could give higher priority to commands that has the return type of the context

    return completeList;
}

/**
 * Returns details about a completion item.
 * 
 * @param item Completion Item to retrieve data of.
 */
export function getCompletionInfo(item: CompletionItem): CompletionItem {
    if (!!cmdHelper.cmd[item.data]) {
        item.detail = cmdHelper.getCommandSignature(item.data, cmdHelper.cmd[item.data]); // !FIXME : I think we can simply get rid of the second param
        item.documentation = {
            kind: MarkupKind.Markdown,
            value: cmdHelper.getCommandDocumentation(cmdHelper.cmd[item.data]) // !FixME : I think we can use cmd name instead
        };
    }

    return item;
}
