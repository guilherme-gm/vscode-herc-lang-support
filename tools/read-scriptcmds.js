const util = require('util');
const fs = require('fs');
const path = process.argv[2];
const out = "script_out/";

fs.mkdirSync(out);
console.log(path);
const lines = fs.readFileSync(path).toString().split("\n");

let inCmd = false;
let affectedCmds = [];
let doc = [];
lines.forEach(line => {
    console.log(line);
    if (line.substr(0, 1) === '*' && line.substr(1, 1) !== ' ') {
        if (inCmd === false) {
            affectedCmds = [];
            doc = [];
            inCmd = true;
        }

        affectedCmds.push(line.substr(1));
    } else if (line === "---------------------------------------") {
        inCmd = false;
        saveCmds(affectedCmds, doc);
        inCmd = false; // it will be cleaned when it finds the next cmd.
    } else if (inCmd) {
        const trimmed = line.trim();
        if (trimmed.length > 0) {
            doc.push(trimmed);
        }
    }
});

console.log("Done.");

function saveCmds(affectedCmds, doc) {
    affectedCmds.forEach(cmd => {
        let cmdInfo = parseCommand(cmd);
        let cmdData = cmdInfo[1];
        console.log(cmdData);
        cmdData[cmdInfo[0]].doc = [];
        doc.forEach(line => { cmdData[cmdInfo[0]].doc.push(line) });

        fs.writeFileSync(out + cmdInfo[0] + ".json", JSON.stringify(cmdData));
    });
}

function parseCommand(cmd) {
    let cmdName = '';
    let params = {};

    const letters = cmd.split('');
    let i = 0;
    while (i < letters.length && letters[i] !== '(') {
        cmdName += letters[i];
        i++;
    }

    i++; // skip (

    while (i < letters.length && (letters[i] !== ')' && letters[i] !== '}')) {

        let isOptional = false;
        let isStr = false;
        let paramName = "";

        // param loop

        while (i < letters.length && letters[i] === ' ') i++;

        if (letters[i] === "{") {
            isOptional = true;
            i++;
        }

        while (i < letters.length && letters[i] === ' ') i++;
        
        if (letters[i] === '"') {
            isStr = true;
            i++;
        }

        while (i < letters.length && (letters[i] !== ',' && letters[i] !== ')' && letters[i] != '}')) {
            paramName += letters[i];
            i++;
        }

        if (isStr) {
            paramName.trim();
            if (paramName.substring(paramName.length - 1) === '"') {
                paramName = paramName.substring(0, paramName.length - 1);
            }
        }

        paramName = paramName.replace(">", "").replace("<", "");

        if (paramName !== "") {
            if (isOptional)
                params[paramName] = { type: (isStr ? "string" : "int"), default: "??" };
            else
                params[paramName] = (isStr ? "string" : "int");
        }

        i++;
    }

    let cmdObj = { }
    cmdObj[cmdName] = {
        params: [params],
        paramDoc: {}
    };

    Object.keys(params).forEach(p => {
        cmdObj[cmdName].paramDoc[p] = "Missing Documentation"
    });

    return [cmdName, cmdObj];
}