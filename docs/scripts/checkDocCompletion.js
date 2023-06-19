//! Scripts that checks documentation completeness
//!
//! This assumes that the file follows the Rust doc syntax (//! for module and /// for members)

const { readModulesInDirectory, findRepoRoot } = require("./util.js");
const { statSync } = require("fs");
const { join } = require("path");

const path = process.argv[2];

if (!path) {
    console.error("Usage: node checkDocCompletion.js <path> [--verbose]");
    process.exit(1);
}

const configs = require("./config.js");

const root = findRepoRoot();
if (!statSync(join(root, path)).isDirectory()) {
    console.error("Invalid path! Path needs to be directory from repo root.");
    console.error("Usage: node checkDocCompletion.js <path> [--verbose]");
    process.exit(1);
}
const modules = [];
readModulesInDirectory(root, configs.js, path, modules);
readModulesInDirectory(root, configs.rs, path, modules);

// module that have documentation on the module
let documentedModule = 0;
// module that have self and all public members documented
let documentedPublicModule = 0;
// module that have self and all members documented
let documentedFullModule = 0;
const verbose = process.argv.includes("--verbose");

const isValidDoc = (doc /*: string[] */) => {
    if (doc.length === 0) {
        return false;
    }
    const docString = doc.map(line => line.length <= 3 ? "" : line.substring(3).trim()).join("\n").trim();
    if(!docString) {
        return false;
    }
    return true;
}

modules.forEach(module => {
    if (module.doc.length > 0) {
        documentedModule++;
        if (module.members.every(member => isValidDoc(member.doc))) {
            documentedFullModule++;
            documentedPublicModule++;
        } else if (module.members.filter(m => !m.isPrivate).every(member => isValidDoc(member.doc))) {
            documentedPublicModule++;
        } else {
            if (verbose) {
                console.log("Missing Public Member Doc: " + module.filePath);
            }
        }
    } else {
        if (verbose) {
            console.log("Missing Module Doc: " + module.filePath);
        }
    }
});
console.log();
console.log(`Fully Documented: ${documentedFullModule}/${modules.length} (${Math.round(documentedFullModule/modules.length*100)}%)`);
console.log(`Public Documented: ${documentedPublicModule}/${modules.length} (${Math.round(documentedPublicModule/modules.length*100)}%)`);
console.log(`Module Documented: ${documentedModule}/${modules.length} (${Math.round(documentedModule/modules.length*100)}%)`);
console.log(`Undocumented: ${modules.length - documentedModule}/${modules.length} (${Math.round((modules.length - documentedModule)/modules.length*100)}%)`);