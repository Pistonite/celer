//! Script to dump doc and/or code of a member
//!
//! Usage: node dumpSymbol.js <path> <member> [--doc][--sig][--code]
//! Member can be dot separated to search nested members
const { readModuleFromFile, findRepoRoot, getSignatureFromCode, readModule } = require("./util");

const path = process.argv[2];
const member = process.argv[3];

if (!path || !member) {
    console.error("Usage: node dumpSymbol.js <path> <member> [--doc][--sig]");
    process.exit(1);
}

const configs = require("./config.js");
const config = (() => {
    for (const key in configs) {
        if (configs[key].filePatterns.some(p => p.test(path))) {
            return configs[key];
        }
    }
})();

const memberPath = member.split(".");

let theModule = undefined;
let theMember = undefined;
let nestPath = path;
for (let i = 0; i < memberPath.length; i++) {

    if (i === 0) {
        theModule = readModuleFromFile(findRepoRoot(), config, path);
    } else {
        nestPath = nestPath + "." + memberPath[i];
        const [_, nestedCode] = getSignatureFromCode(theMember.code);
        theModule = readModule(config, nestPath, nestedCode);
    }
    const members = theModule.members.filter(m => {
        return m.code.length > 0 && m.code[0].includes(memberPath[0]);
    });
    if (members.length === 0) {
        console.error("Member not found");
        process.exit(1);
    }

    if (members.length > 1) {
        console.error("Multiple members found:");
        members.forEach(m => console.error(m.code[0]));
        process.exit(1);
    }

    theMember = members[0];
}

if (process.argv.includes("--doc")) {
    console.log(theMember.doc.join("\n"));
}

if (process.argv.includes("--sig")) {
    console.log(getSignatureFromCode(theMember.code)[0]);
} else if (process.argv.includes("--code")) {
    console.log(theMember.code.join("\n"));
}
