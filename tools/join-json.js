const util = require('util');
const fs = require('fs');
const path = "./src/commands/";
const out = "./out/commands.json";

let cmdList = {};
fs.readdirSync("./src/commands/").forEach(fn => {
    const cmdInfo = JSON.parse(fs.readFileSync(util.format("%s%s", path, fn)));
    Object.assign(cmdList, cmdInfo);
});
fs.writeFileSync(out, JSON.stringify(cmdList));

console.log("Done");
