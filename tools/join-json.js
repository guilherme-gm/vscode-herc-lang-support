const util = require('util');
const fs = require('fs');
const path = "./server/src/commands/";
const out = "./server/out/commands.json";

let cmdList = {};
fs.readdirSync(path).forEach(fn => {
    const cmdInfo = JSON.parse(fs.readFileSync(util.format("%s%s", path, fn)));
    Object.assign(cmdList, cmdInfo);
});
fs.writeFileSync(out, JSON.stringify(cmdList));

console.log("Done");
