/**
 * Loads command.json with the data of all script commands.
 */

import { format, isString } from "util";

export const cmd = require("./commands.json");

export function getCommandSignature(cmdName: string, cmdInfo, signGroup = null) {
    let returnType = "(void)";
    let params = "";
    let paramGroup = null;

    if (signGroup !== null) {
        paramGroup = signGroup;
    } else if (cmdInfo.params != null) {
        paramGroup = cmdInfo.params[0];
    }

    if (paramGroup != null) {
        let paramList = [];
        
        Object.keys(paramGroup).forEach(paramName => {
            paramList.push(formatParam(paramName, paramGroup[paramName]));
        })

        params = paramList.join(", ");
    }
    
    if (cmd.return != null) {
        returnType = format("(%s)", cmd.return);
    }

    return format("%s %s(%s)", returnType, cmdName, params);
}

export function getCommandDocumentation(cmdInfo) : string {
    if (cmdInfo.doc == null) {
        return "Documentation unavailable.";
    }

    cmdInfo.doc.join('');
}

export function getParameterDocumentation(cmdInfo, paramName:string): string {
    if (cmdInfo.paramDoc == null || cmdInfo.paramDoc[paramName] == null) {
        return "Documentation unavailable";
    }

    return cmdInfo.paramDoc[paramName];
}

export function formatParam(paramName, paramInfo) {
    let type = "";
    let defVal = "";

    if (!isString(paramInfo)) {
        type = paramInfo['type'];
        
        if (paramInfo['default'] !== null) {
            defVal = paramInfo['default']
            if (type === "string") {
                defVal = format("\"%s\"", defVal);
            }
            defVal = format(" = %s", defVal);
        }
    } else {
        type = paramInfo;
    }
    
    return format("%s: %s%s", paramName, type, defVal);
}