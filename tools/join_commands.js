const util = require('util');
const fs = require('fs');
const path = "./script_commands/";
const out = "./client/out/commands.json";

let cmdList = {};
fs.readdirSync(path).forEach(fn => {
    const cmdInfo = JSON.parse(fs.readFileSync(util.format("%s%s", path, fn)));
    Object.assign(cmdList, cmdInfo);
});
fs.writeFileSync(out, JSON.stringify(cmdList, null, 4));

console.log("Done");