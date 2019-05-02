import 'mocha';
import { expect } from 'chai';
import * as parser from "../parser/parser";
import { Position } from 'vscode-languageserver';

type TestCase = {
    it: string,
    content: string[],
    position: Position,
    return: parser.VariableInfo[]
};

const testLines = [
    //<Header> // L1
    "", // L2
    ".@i = 0;", // L3
    "str$ = \"myStr\";", // L4
    "", // L5
    "if (.@i > 0) {", // L6
    "", // L7
    ".@i = 2;", // L8
    ".@j = 1;", // L9
    "", // L10
    "}" // L11
]

const testCases: TestCase[] = [
    {
        it: "Should find nothing",
        content: testLines,
        position: { line: 2, character: 0 },
        return: []
    },
    {
        it: "Should find .@i and str$ (after definition)",
        content: testLines,
        position: { line: 5, character: 0 },
        return: [
            { varName: ".@i", definedAt: [{ line: 3, character: 0 }] },
            { varName: "str$", definedAt: [{ line: 4, character: 0 }] },
        ]
    },
    {
        it: "Should find .@i and str$ (after definition, inside if)",
        content: testLines,
        position: { line: 7, character: 0 },
        return: [
            { varName: ".@i", definedAt: [{ line: 3, character: 0 }] },
            { varName: "str$", definedAt: [{ line: 4, character: 0 }] },
        ]
    },
    {
        it: "Should find .@i, str$ and .@j",
        content: testLines,
        position: { line: 10, character: 0 },
        return: [
            { varName: ".@i", definedAt: [{ line: 3, character: 0 }, { line: 8, character: 0 },] },
            { varName: "str$", definedAt: [{ line: 4, character: 0 }] },
            { varName: ".@j", definedAt: [{ line: 9, character: 0 }] },
        ]
    },
];

describe('Variable Finder', function () {

    testCases.forEach((tcase: TestCase) => {
        it(tcase.it, () => {
            expect(parser.getVariablesAtText(
                "-\tscript\tTest\tFAKE_NPC,{\n" + tcase.content.join("\n") + "\n}",
                tcase.position
            )).to.be.deep.eq(tcase.return);
        });
    });

});
