const fs = require('fs');
const path = require('path');

if (!process.argv[2]) {
    console.error("Error: Give a input file (e.g. node script_cmd_2_json.js script_cmd.txt output/");
    process.exit(1);
}
if (!process.argv[3]) {
    console.error("Error: Give an output folder (e.g. node script_cmd_2_json.js script_cmd.txt output/");
    process.exit(1);
}

let file;
try {
    file = fs.readFileSync(process.argv[2], { encoding: "utf8" });
} catch (error) {
    console.error(`Error: Failed to open file. Error: ${error.message}`);
}

const outFolder = process.argv[3];
fs.mkdirSync(outFolder, { recursive: true });

let lines = file.split('\n');
let scriptCmds = [];

let currentSet = [];
let currentDoc = [];
let inCmd = false;
for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    let cmd = null;
    
    if (line[0] === '*' && line[1] !== ' ') {
        inCmd = true;
        cmd = parseCmdLine(line);

        if (cmd !== null) {
            inCmd = true;
            currentSet.push(cmd);
        }
    } else if (inCmd && line !== '---------------------------------------') {
        currentDoc.push(line);
    } else if (inCmd && line === '---------------------------------------') {
        while (currentDoc.length > 0 && currentDoc[0] === '') currentDoc.shift();
        while (currentDoc.length > 0 && currentDoc[currentDoc.length - 1] === '') currentDoc.pop();

        const deprecated = currentDoc.filter(l => l.indexOf("command is deprecated") >= 0).length > 0;
        currentSet.forEach(aCmd => {
            aCmd.doc = currentDoc.map(ln => (ln + "\n"));
            aCmd.return = "unknown";
            aCmd.deprecated = deprecated;
        });

        scriptCmds.push(...currentSet);
        currentSet = [];
        currentDoc = [];
    }
}

if (currentSet.length > 0) {
    while (currentDoc.length > 0 && currentDoc[0] === '') currentDoc.shift();
    while (currentDoc.length > 0 && currentDoc[currentDoc.length - 1] === '') currentDoc.pop();

    const deprecated = currentDoc.filter(l => l.indexOf("command is deprecated")).length > 0;
    currentSet.forEach(aCmd => {
        aCmd.doc = currentDoc.map(ln => (ln + "\n"));
        aCmd.return = "unknown";
        aCmd.deprecated = deprecated;
    });

    scriptCmds.push(...currentSet);
    currentSet = [];
    currentDoc = [];
}

scriptCmds.forEach(cmd => {
    const formattedCmd = {
        [cmd.name]: {
            params: {},
            signatures: [],
            doc: cmd.doc,
            return: cmd.return,
            deprecated: cmd.deprecated,
        }
    };
    cmd.params.forEach(param => {
        formattedCmd[cmd.name].params[param.name] = {
            type: param.type,
            default: param.default,
            doc: param.doc,
        }
    });
    cmd.signatures.forEach(sign => {
        formattedCmd[cmd.name].signatures.push(sign.map(arg => arg.name));
    });
    
    fs.writeFileSync(path.resolve(outFolder, `${cmd.name}.json`), JSON.stringify(formattedCmd, undefined, "\t"));
});

function parseCmdLine(line) {
    const cmdData = {
        name: '',
        params: [], // { name, type, doc, default }
        signatures: [], // [ { name }, { name } ]
    }

    let i = 1;
    while (i < line.length && line[i] !== ' ' && line[i] !== '(') {
        cmdData.name += line[i];
        i++;
    }

    if (i === line.length || line[i] !== '(') { // end, return "smth", for (), etc...
        console.log(`Language specials: ${cmdData.name}`);
        return null;
    }

    // We DO have ()
    let paramName = '';
    let paramType = '';
    let params = []; // { name }
    let isOptionalClose = false;
    let isOptionalCloseWarnEmit = false;
    while (i < line.length && line[i] !== ')') {
        const curChar = line[i];
        i++;
        let lineObj = { i };

        if (curChar === '}') {
            isOptionalClose = true;
        }

        if (isOptionalClose && curChar !== '}' && !isOptionalCloseWarnEmit) {
            console.warn(`${cmdData.name} is in an unsuported format. Fix it manually·`);
            isOptionalCloseWarnEmit = true;
        }

        if (curChar === '{') {
            cmdData.signatures.push([ ...params ]);
        }

        if (curChar === '"') {
            paramType = 'string';
        }

        if (curChar === "<") {
            if (paramType === '') {
                paramType = 'number';
            }

            paramName = getParamName(line, lineObj); // it eats '>'

            if (paramType === 'string') {
                lineObj.i++; // eats '"'
            }

            cmdData.params.push({ name: paramName, type: paramType, default: '*?*', doc: [], });
            params.push({ name: paramName });
            paramName = '';
            paramType = '';
        } 

        i = lineObj.i;
    }

    cmdData.signatures.push([ ...params ]);

    return cmdData;
}

function getParamName(line, lineObj) {
    let name = '';
    let i = lineObj.i;
    while (i < line.length && line[i] !== '>') {
        name += line[i];
        i++;
    }

    i++;
    lineObj.i = i;
    return name;
}
