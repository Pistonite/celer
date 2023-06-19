//! Script to dump doc and/or code of a member

//! Usage: node dumpSymbol.js <path> <member> [--doc][--sig][--code]
const { readModuleFromFile, findRepoRoot, getSignatureFromCode } = require("./util");

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

const theModule = readModuleFromFile(findRepoRoot(), config, path);
const members = theModule.members.filter(m => {
   return  m.code.length > 0 && m.code[0].includes(member);
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

const m = members[0];

if (process.argv.includes("--doc")) {
    console.log(m.doc.join("\n"));
}

if (process.argv.includes("--sig")) {
    console.log(getSignatureFromCode(m.code));
} else if (process.argv.includes("--code")) {
    console.log(m.code.join("\n"));
}
