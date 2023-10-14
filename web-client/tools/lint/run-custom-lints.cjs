//! Runs custom lints
const fs = require("fs");
const path = require("path");

const lints = Object.entries({
    "react-hooks": require("./react-hooks.cjs"),
    "non-logger-console": require("./non-logger-console.cjs"),
});

const rootDir = path.resolve(__dirname, "../../src");
const ok = checkPath(rootDir, "src");
if (!ok) {
    console.log();
    console.log(
        "Please fix issues above."
    );
    process.exit(1);
}

function checkPath(filePath, displayPath) {
    let ok = true;
    if (fs.statSync(filePath).isDirectory()) {
        fs.readdirSync(filePath).forEach((file) => {
            if (
                !checkPath(
                    path.join(filePath, file),
                    path.join(displayPath, file),
                )
            ) {
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
    lints.forEach(([lint, run]) => {
        runLint(lint, file, content, run);
    });
}

function runLint(lint, file, content, run) {
    const errors = run(file, content);
    if (errors.length > 0) {
        console.log(`[${lint}] ${file}:`)
        for (const error of errors) {
            console.log(`  ${error}`);
        }
        console.log()
    }
    return errors.length === 0;
}
