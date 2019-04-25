import * as parser from "./parser";
//1,9
//1,11
console.log("------------- AT 1,9 -------------");
console.log(parser.getContextOfText("prontera,150,150,4	script	Test	FAKE_NPC,{\n\tgetitem()\n}", { line: 1, character: 9 }));

console.log("------------- AT 1,13 -------------");
console.log(parser.getContextOfText("prontera,150,150,4	script	Test	FAKE_NPC,{\n\tgetitem(501,)\n}", { line: 1, character: 13 }));
