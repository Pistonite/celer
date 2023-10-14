//! Ensure that every console.log uses the log wrapper instead of plain window.console
//! This is because the log wrapper also saves the log for export

const rules = [
    "console.log(",
    "console.info(",
    "console.warn(",
    "console.debug(",
    "console.error(",
]

function containsConsole(line) {
    const lineReplaced = line.replace("window.console", "consoleignore");
    for (const rule of rules) {
        if (lineReplaced.includes(rule)) {
            return true;
        }
    }
    return false;
}

function isImportConsoleFromLowUtils(line) {
    return line.startsWith("import") && line.endsWith("from \"low/utils\";") && line.includes("console");
}

function checkFile(file, content) {
    if (file === "src/low/utils/Logger.ts") {
        return [];
    }
    const errors = [];
    const lines = content.split("\n");
    lines.forEach((line, index) => {
        if (containsConsole(line)) {
            errors.push(`${index}: ${line}`);
        }
    });
    if (errors.length === 0) {
        return [];
    }
    for (const line of lines) {
        if (isImportConsoleFromLowUtils(line)) {
            return [];
        }
    }
    errors.push("Please import { console } from \"low/utils\";");
    return errors;
}

module.exports = checkFile;

