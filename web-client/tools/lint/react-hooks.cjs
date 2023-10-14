//! Ensure react hooks are used consistently
//!
//! Error if used like React.useXXX

const rules = [
    /React\.use.*/,
    /React\.forwardRef/,
    /React\.createContext/,
    /React\.memo/,
];

function checkFile(_file, content) {
    const errors = [];
    const lines = content.split("\n");
    lines.forEach((line, index) => {
        if (!checkLine(line)) {
            errors.push(`${index}: ${line}`);
        }
    });
    if (errors.length > 0) {
        errors.push(
            "Removed the `React.` prefix from the function usage in lines above.",
        );
    }
    return errors;
}

function checkLine(line) {
    for (const rule of rules) {
        if (rule.test(line)) {
            return false;
        }
    }
    return true;
}

module.exports = checkFile;
