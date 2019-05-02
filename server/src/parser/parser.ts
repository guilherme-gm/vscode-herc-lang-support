import { TextDocumentPositionParams, Position } from "vscode-languageserver";
import * as docMngr from "../helpers/documentsManager";

export type ContextInfo = {
    readonly funcName: string;
    readonly paramNum: number;
}

export type VariableInfo = {
    readonly varName: string;
    definedAt: Position[];
}

export type VarList = {
    list: VariableInfo[];
}

export function getContext(params: TextDocumentPositionParams): ContextInfo {
    return getContextOfText(docMngr.getText(params.textDocument), params.position)
}

export function getVariablesAt(params: TextDocumentPositionParams): VariableInfo[] {
    return getVariablesAtText(docMngr.getText(params.textDocument), params.position);
}

function getParseTree(sourceCode: string) {
    const TreeSitterParser = require('tree-sitter');
    const HercScriptLang = require('../../tree-sitter-hercscript');

    const parser = new TreeSitterParser();
    parser.setLanguage(HercScriptLang);

    return parser.parse(sourceCode);
}

export function getVariablesAtText(sourceCode: string, position: Position): VariableInfo[] {
    const tree = getParseTree(sourceCode);

    const npcNode = getCurrentNpc(tree.rootNode.child(0), position.line, position.character);

    // TODO : Probably it is a good idea to cache them and use the incremental parser to refresh it
    let varList: VarList = { list: [] };
    getNpcVariables(npcNode, position.line, position.character, varList);

    return varList.list;
}

function getCurrentNpc(scriptNode, cursorRow, cursorCol) {
    let i = 0;

    // Skip all headers that ends before desired row
    while (i < scriptNode.childCount && scriptNode.child(i).endPosition.row < cursorRow)
        i++;

    // Now all childs include our current row,
    // Skip all childs that ends before desired col
    while (i < scriptNode.childCount
        && scriptNode.child(i).startPosition.row <= cursorRow // Must not start after our row
        && (scriptNode.child(i).endPosition.row == cursorRow && scriptNode.child(i).endPosition.column <= cursorCol) // ends in the same row, but before cursor
    )
        i++;

    if (i > scriptNode.childCount)
        return null; // No context found (should never happen?)

    const childNode = scriptNode.child(i);
    return childNode;
}

function isNestable(node) {
    return ([
        "block",
        "if_stmt"
    ]).indexOf(node.type) >= 0;
}

function getNpcVariables(node, line: number, col: number, varList: VarList) {
    let i = 0;
    while (i < node.childCount && node.child(i).startPosition.row < line) {
        const childNode = node.child(i);
        
        if (childNode.type === 'assignment_stmt') {
            const varname = childNode.child(0).text; // assignment_stmt.identifier
            let found = false;

            varList.list.forEach((varInfo: VariableInfo) => {
                if (varInfo.varName === varname) {
                    varInfo.definedAt.push({ line: childNode.startPosition.row + 1, character: childNode.startPosition.column });
                    found = true;
                }
            });

            if (!found) {
                varList.list.push({
                    varName: varname,
                    definedAt: [{ line: childNode.startPosition.row + 1, character: childNode.startPosition.column }]
                });
            }
        } else if (isNestable(childNode)) {
            getNpcVariables(childNode, line, col, varList);
        }
        i++;
    }
}

export function getContextOfText(sourceCode: string, position: Position): ContextInfo {
    const tree = getParseTree(sourceCode);

    let res = getMyContext(tree.rootNode.child(0), position.line, position.character);
    //connection.console.log(res);
    let cmdInfo = getCommandCursor(res, position.line, position.character);

    return { funcName: cmdInfo.funcName, paramNum: cmdInfo.paramNum };
}
function getMyContext(node, searchRow, searchCol) {
    let i = 0;

    // Skip all childs that ends before desired row
    while (i < node.childCount && node.child(i).endPosition.row < searchRow)
        i++;

    // Now all childs include our current row,
    // Skip all childs that ends before desired col
    while (i < node.childCount
        && node.child(i).startPosition.row <= searchRow // Must not start after our row
        && (node.child(i).endPosition.row == searchRow && node.child(i).endPosition.column <= searchCol) // ends in the same row, but before cursor
    )
        i++;

    if (i > node.childCount)
        return null; // No context found (should never happen?)

    const childNode = node.child(i);
    if (childNode.type !== 'function_stmt') {
        return getMyContext(childNode, searchRow, searchCol);
    }

    return childNode;
}

function isCompositeExpression(node) {
    return ([
        "plusop",
        "mulop"
    ].indexOf(node.type) >= 0)
}

function getCommandCursor(cmdNode, searchRow, searchCol) {
    // cmdNode:
    // 0 - identifier (name)
    // 1 - parameterList
    // But we never know how much info we have...

    console.log("--------- getCommandCursor ---------");

    let funcName = '';
    if (cmdNode.childCount > 0) {
        funcName = cmdNode.child(0).text;
        if (inRange(
            cmdNode.child(0).startPosition.row,
            cmdNode.child(0).startPosition.column,
            cmdNode.child(0).endPosition.row,
            cmdNode.child(0).endPosition.column,
            searchRow,
            searchCol
        )) {
            return {
                funcName: funcName,
                paramNum: -1
            };
        }
    }

    let paramNum = -1;
    if (cmdNode.childCount > 1) {
        let i = 0;
        while (i < cmdNode.child(1).childCount) {
            const childNode = cmdNode.child(1).child(i);
            console.log(">> Type: " + childNode.type.toString());
            if (childNode.type === '(')
                paramNum = 1;
            else if (childNode.type == ',')
                paramNum++;
            //else if (childNode.type == ')')
            //    paramNum++;

            if (inRange(
                childNode.startPosition.row,
                childNode.startPosition.column,
                childNode.endPosition.row,
                childNode.endPosition.column,
                searchRow,
                searchCol
            )) {
                console.log(childNode.type);
                if (isCompositeExpression(childNode)) {
                    const solvedExp = recursiveSearchCommand(childNode, searchRow, searchCol);
                    if (solvedExp != null) {
                        return solvedExp;
                    }
                } else if (childNode.type === 'function_stmt') {
                    // cursor is in a parameter that is also a function,
                    // recursively expand this parameter to find out what
                    // is the actual function/parameter
                    return getCommandCursor(childNode, searchRow, searchCol);
                } else {
                    console.log(childNode);
                    return {
                        funcName: funcName,
                        paramNum: paramNum
                    };
                }
            }
            i++;
        }
    }

    return {
        funcName: funcName,
        paramNum: paramNum
    };
}

function recursiveSearchCommand(node, searchRow, searchCol) {
    switch (node.type) {
        case "mulop":
        case "plusop":
            let i = 0;
            while (i < node.childCount) {
                const childNode = node.child(i);
                i++;

                if (inRange(
                    childNode.startPosition.row,
                    childNode.startPosition.column,
                    childNode.endPosition.row,
                    childNode.endPosition.column,
                    searchRow,
                    searchCol)
                ) {
                    if (isCompositeExpression(childNode)) {
                        return recursiveSearchCommand(childNode, searchRow, searchCol);
                    } else if (childNode.type === 'function_stmt') {
                        return getCommandCursor(childNode, searchRow, searchCol);
                    } else {
                        return null;
                    }
                }
            }
            break;
    }

    return null;
}

function inRange(startRow, startCol, endRow, endCol, checkRow, checkCol) {
    //console.log(startRow + " ; " + startCol + " ; " + endRow + " ; " + endCol + " ; " + checkRow + " ; " + checkCol);
    //
    //console.log("(startRow < checkRow) => " + (startRow < checkRow));
    //console.log("(startRow == checkRow && startCol < checkCol) => " + (startRow == checkRow && startCol < checkCol));
    //console.log("(endRow > checkRow) => " + (endRow > checkRow));
    //console.log("(endRow == checkRow && endCol > checkCol) => " + (endRow == checkRow && endCol > checkCol));

    return (
        (startRow < checkRow || (startRow == checkRow && startCol < checkCol)) &&
        (endRow > checkRow || (endRow == checkRow && endCol >= checkCol))
    );
}
