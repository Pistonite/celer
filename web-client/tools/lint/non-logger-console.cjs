//! Ensure that every console.log uses the log wrapper instead of plain window.console
//! This is because the log wrapper also saves the log for export

const rules = [
    "console.log(",
    "console.info(",
    "console.warn(",
    "console.debug(",
    "console.error(",
];

function checkFile(file, content) {
    const include = getLowUtilsImportLocationFrom(file);
    const errors = [];
    const lines = content.split("\n");
    lines.forEach((line, index) => {
        if (containsConsole(line)) {
            errors.push(`${index + 1}: ${line}`);
        }
    });
    if (errors.length === 0) {
        return [];
    }
    let temp = "";
    for (const line of lines) {
        if (line.startsWith("//")) {
            continue;
        }
        if ((line.startsWith("import") || temp) && !line.endsWith(";")) {
            temp += line;
            continue;
        }
        if (isImportConsoleFromLowUtils(include, temp + line)) {
            return [];
        }
        temp = "";
    }
    errors.push(
        `Please import { console } from "${include}"; or remove the console statement.`,
    );
    return errors;
}

function containsConsole(line) {
    if (line.startsWith("//")) {
        return false;
    }
    const lineReplaced = line.replace("window.console", "consoleignore");
    for (const rule of rules) {
        if (lineReplaced.includes(rule)) {
            return true;
        }
    }
    return false;
}

function isImportConsoleFromLowUtils(include, line) {
    return (
        line.startsWith("import") &&
        line.endsWith(`from "${include}";`) &&
        line.includes("console")
    );
}

function getLowUtilsImportLocationFrom(file) {
    if (!file.replace(/^src\/low\/utils\//, "").includes("/")) {
        return "./logging.ts";
    }
    return file.replace(/^src\/low\//, "").includes("/")
        ? "low/utils"
        : "./utils";
}

module.exports = checkFile;
