//! Ensure that we don't import from clsx (use mergeClasses instead)

function checkFile(file, content) {
    const lines = content.split("\n");
    const errors = [];
    lines.forEach((line, index) => {
        if (isImportFromClsx(line)) {
            errors.push(`${index + 1}: ${line}`);
        }
    });

    if (errors.length === 0) {
        return [];
    }

    errors.push(
        "Please use `smartMergeClasses` or `mergeClasses` instead of `clsx`.",
    );
}

function isImportFromClsx(line) {
    line = line.trim();
    if (!line.startsWith("import")) {
        return false;
    }
    return line.includes('"clsx"') || line.includes("'clsx'");
}

module.exports = checkFile;
