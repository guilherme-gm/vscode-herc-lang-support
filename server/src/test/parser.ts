import 'mocha';
import { expect } from 'chai';
import * as parser from "../parser/parser";
import { Position } from 'vscode-languageserver';

type TestCase = {
    it: string,
    content: string[],
    position: Position,
    return: parser.ContextInfo
};

const testCases : TestCase[] = [
    {
        it: "Should find cmd1, param1",
        content: [ "cmd();" ],
        position: { line: 1, character: 4 },
        return: { funcName: "cmd", paramNum: 1 }
    },
    {
        it: "Should find cmd2, param1",
        content: [ "cmd(cmd2());" ],
        position: { line: 1, character: 9 },
        return: { funcName: "cmd2", paramNum: 1 }
    },
    {
        it: "Should find cmd2, param1 -- Appended command",
        content: [ "cmd(\"str\" + cmd2());" ],
        position: { line: 1, character: 17 },
        return: { funcName: "cmd2", paramNum: 1 }
    },
    {
        it: "Should find cmd2, param1 -- Appended command",
        content: [ "cmd(\"str1\" + \"str2\" + cmd2());" ],
        position: { line: 1, character: 27 },
        return: { funcName: "cmd2", paramNum: 1 }
    },
    {
        it: "Should find cmd2, param1 -- Appended command",
        content: [ "cmd(\"str1\" + \"str2\" + cmd2(\"\"));" ],
        position: { line: 1, character: 27 },
        return: { funcName: "cmd2", paramNum: 1 }
    }
];

describe('Signature Helper', function () {

    testCases.forEach((tcase : TestCase) => {
        it(tcase.it, () => {
            expect(parser.getContextOfText(
                "-\tscript\tTest\tFAKE_NPC,{\n"+tcase.content.join("\n")+"\n}",
                tcase.position
            )).to.be.deep.eq(tcase.return);
        });
    });

});
