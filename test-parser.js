#!/usr/bin/env node
const fs = require('fs');

let file = fs.readFileSync('adult-link.txt').toString();
file = file.replace(/\/\/[^\n]*\n/g, '\n');
file = file.replace(/\n[\s]*\n/g, '\n');

let dict = /DICTIONARY\n(.*?)\nEND/gs.exec(file);
let object = /OBJECT(?: POOL=(0x[0-9a-fA-F]+),(0x[0-9a-fA-F]+))?\n(.*?)\nEND/gs.exec(file);
let repoint = /REPOINT\n(.*?)\nEND/gs.exec(file);

let dict_contents = dict[1].split('\n');
/** @type {RegExpExecArray[]} */
let dict_definitions = [];
for (let i = 0; i < dict_contents.length; i++) {
    let matches = /^\s+([^\s]+)\s+([^\s]+)\s*(?:"([^"]+)")?\s*$/.exec(dict_contents[i]);
    if (!matches) continue;
    matches.shift()
    dict_definitions.push(matches);
}

// "Matrix" | "CallMatrix" | "CallList" | "PopMatrix" | "HexString"

/** @type {string[]} */
let object_contents = [...object[3].matchAll(/[^\s]+:(?:\s*(?:Matrix|CallMatrix|CallList|PopMatrix|HexString)\([^)]+\);)+/gs)].map(x => x[0]);

/** @type {[{label: string, commands: [{command: "Matrix" | "CallMatrix" | "CallList" | "PopMatrix" | "HexString", args: string[]}]}]} */
let object_commands = [];

for (let i = 0; i < object_contents.length; i++) {
    // console.log(object_contents[i]);
    let obj = object_contents[i].split('\n');
    let label = /\s*([^:]+):/.exec(obj.shift())[1];
    /** @type {[{command: "Matrix" | "CallMatrix" | "CallList" | "PopMatrix" | "HexString", args: string[]}]} */
    let commands = [];
    for (let j = 0; j < obj.length; j++) {
        let matches = /(Matrix|CallMatrix|CallList|PopMatrix|HexString)\(([^)]+)\);/.exec(obj[j]);
        if (!matches) continue;
        commands.push({
            command: matches[1],
            args: matches[2].split(',').map(x => x.trim())
        });
    }
    object_commands.push({
        label: label,
        commands: commands
    });
}

// "GoTo" | "SetAdvance" | "Write32" | "Write24" | "Write16" | "Write8" | "Hi32" | "Lo32" | "Float" | "HexString"

/** @type {string[]} */
let repoint_commands_raw = [...repoint[1].matchAll(/(?:GoTo|SetAdvance|Write32|Write24|Write16|Write8|Hi32|Lo32|Float|HexString)\([^)]+\);/gs)].map(x => x[0]);

/** @type {[{command: "GoTo" | "SetAdvance" | "Write32" | "Write24" | "Write16" | "Write8" | "Hi32" | "Lo32" | "Float" | "HexString", arg: string}]} */
let repoint_commands = [];

for (let i = 0; i < repoint_commands_raw.length; i++) {
    let matches = /(GoTo|SetAdvance|Write32|Write24|Write16|Write8|Hi32|Lo32|Float|HexString)\(([^)]+)\);/.exec(repoint_commands_raw[i]);
    if (!matches) continue;
    repoint_commands.push({
        command: matches[1],
        arg: matches[2].trim()
    });
}