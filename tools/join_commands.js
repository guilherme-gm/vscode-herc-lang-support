const util = require('util');
const fs = require('fs');
const path = "./script_commands/";
const out = "./client/out/commands.json";

let cmdList = {};
fs.readdirSync(path).forEach(fn => {
    const cmdInfo = JSON.parse(fs.readFileSync(util.format("%s%s", path, fn)));
    const newCmd = {};
    Object.keys(cmdInfo).forEach(cmd => {
        const info = cmdInfo[cmd];
        const newParams = [];
        info.params.forEach(variation => {
            const newVariation = [];
            if (variation) {
                variation.forEach(param => {
                    const name = Object.keys(param)[0];
                    const type = param[name];
                    if (type.type) {
                        newVariation.push({
                            name,
                            param_type: type.type,
                            default: type.default
                        });
                    } else {
                        newVariation.push({
                            name,
                            param_type: type,
                            default: null
                        });
                    }
                });
            }
            newParams.push(newVariation);
        });
        
        newCmd[cmd] = {};
        newCmd[cmd].params = newParams;
        newCmd[cmd].paramDoc = info.paramDoc;
        newCmd[cmd].doc = info.doc;
    });
    Object.assign(cmdList, newCmd);
});
fs.writeFileSync(out, JSON.stringify(cmdList, null, 4));

console.log("Done");