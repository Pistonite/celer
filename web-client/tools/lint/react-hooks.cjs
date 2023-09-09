//! Ensure react hooks are used consistently
//!
//! Error if used like React.useXXX

const fs = require("fs");
const path = require("path");

const rules = [
    /React\.use.*/,
    /React\.forwardRef/,
    /React\.createContext/,
    /React\.memo/,
];

const rootDir = path.resolve(__dirname, "../../src");
const ok = checkPath(rootDir, "src");
if (!ok) {
    console.log();
    console.log("Removed the `React.` prefix from the function usage in lines above.");
    process.exit(1);
}

function checkPath(filePath, displayPath) {
    let ok = true;
    if(fs.statSync(filePath).isDirectory()) {
        fs.readdirSync(filePath).forEach((file) => {
            if (!checkPath(path.join(filePath, file), path.join(displayPath, file))) {
                ok = false;
            }
        });
    } else {
        const content = fs.readFileSync(filePath, "utf-8");
        ok = checkFile(displayPath, content);
    }
    return ok;
}

function checkFile(file, content) {
    let ok = true;
    const lines = content.split("\n");
    for (const line of lines) {
        if (!checkLine(line)) {
            ok = false;
            console.log(`${file}: ${line}`);
        }
    }
    return ok;
}

function checkLine(line) {
    for (const rule of rules) {
        if (rule.test(line)) {
            return false;
        }
    }
    return true;
}

