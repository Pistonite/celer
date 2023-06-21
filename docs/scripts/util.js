const { readFileSync, readdirSync, statSync } = require('fs');
const { join, resolve, dirname } = require('path');

/* type Config = {
    filePatterns: RegExp[],
    privatePatterns: RegExp[],
    publicPatterns: RegExp[],
    allowInBetweenPatterns: RegExp[]
} */

/* type Module = {
    filePath: string,
    doc: string[],
    members: Member[]
} */

/* type Member = {
    doc: string[],
    isPrivate: boolean,
    code: string[]
} */

const ignores = [
    "node_modules",
    "target",
    "dist",
    "assets",
];

const getMemberType = (config /* :Config */, line /* :string */) /* :number 0=not 1=public 2=private */ => {
    // Check privatePatterns
    for (let j = 0; j < config.privatePatterns.length; j++) {
        if (config.privatePatterns[j].test(line)) {
            return 2;
        }
    }
    // Check publicPatterns
    for (let j = 0; j < config.publicPatterns.length; j++) {
        if (config.publicPatterns[j].test(line)) {
            return 1;
        }
    }
    return 0;
}

const readModulesInDirectory = (
    root /* :string */,
    config /* :Config */,
    filePath /* :string */,
    modules /* :Module[] */
) /* :void */ => {
    readdirSync(join(root, filePath)).forEach((file) => {
        if (ignores.includes(file)) {
            return;
        }
        if (statSync(join(root, filePath, file)).isDirectory()) {
            readModulesInDirectory(root, config, join(filePath, file), modules);
        } else {
            let isMatch = false;
            for (let i = 0; i < config.filePatterns.length; i++) {
                if (config.filePatterns[i].test(file)) {
                    isMatch = true;
                    break;
                }
            }
            if (!isMatch) {
                return;
            }
            modules.push(readModuleFromFile(root, config, join(filePath, file)));
        }
    });
}

const readModuleFromFile = (root /* :string */, config /* :Config */, filePath /* :string */) /* :Module */ => {
    if (config.verbose) {
        console.log(`Reading ${filePath}`);
    }
    const file = readFileSync(join(root, filePath), 'utf-8');

    const lines = file.split('\n');
    return readModule(config, filePath, lines);

}

const readModule = (config /* :Config */, path /* :string */, content /* :string[] */) /* :Module */ => {
    const { allowInBetweenPatterns } = config;

    const moduleDoc = [];
    let i = 0;
    for (; i < lines.length; i++) {
        const line = lines[i];
        if (line.startsWith('//!')) {
            moduleDoc.push(line);
        } else {
            break;
        }
    }

    const members = [];
    let currentMember = null;
    const resetCurrentMember = () => {
        currentMember = {
            doc: [],
            isPrivate: false,
            code: []
        };
    };
    resetCurrentMember();
    let state = 0; //0 = looking for doc or code, 1 = looking for more doc, 2 = looking for more code
    // Transitions, c=add member, m=set member type
    //     doc       member     docInBetween neither
    // 0 , +doc,1    +code,2,m  0            0
    // 1 , +doc,1    +code,2,m  +doc,1       reset,0
    // 2 , +doc,1,c  +code,2,m,c+code,2      +code,2



    // +doc, 1
    const addToDocAndSetState1 = (line) => {
        currentMember.doc.push(line);
        state = 1;
    }

    // +code, 2
    const addToCodeAndSetState2 = (line) => {
        currentMember.code.push(line);
        state = 2;
    }

    for (; i < lines.length; i++) {
        const line = lines[i].trimEnd();
        let isDoc = line.startsWith('///');
        const memberType = getMemberType(config, line);
        const isMember = !isDoc && memberType > 0;
        if (state === 0) {
            if (isDoc) {
                addToDocAndSetState1(line);
            } else if (isMember) {
                addToCodeAndSetState2(line);
                currentMember.isPrivate = memberType === 2;
            }
        } else if (state === 1) {
            if (!isDoc) {
                // Check allowInBetweenPatterns
                for (let j = 0; j < allowInBetweenPatterns.length; j++) {
                    if (allowInBetweenPatterns[j].test(line)) {
                        isDoc = true;
                        break;
                    }
                }
            }
            if (isDoc) {
                addToDocAndSetState1(line);
            } else if (isMember) {
                addToCodeAndSetState2(line);
                currentMember.isPrivate = memberType === 2;
            } else {
                // not member, dangling doc comment
                resetCurrentMember();
                state = 0;
            }
        } else if (state === 2) {
            if (isDoc) {
                members.push(currentMember);
                resetCurrentMember();
                addToDocAndSetState1(line);
            } else if (isMember) {
                members.push(currentMember);
                resetCurrentMember();

                addToCodeAndSetState2(line);
                currentMember.isPrivate = memberType === 2;
            } else {
                addToCodeAndSetState2(line);
            }
        }
    }

    if (currentMember.code.length > 0) {
        members.push(currentMember);
    }

    return {
        filePath: path,
        doc: cleanCode(moduleDoc),
        members: members.map(member => ({
            doc: cleanCode(member.doc),
            isPrivate: member.isPrivate,
            code: cleanCode(member.code)
        }))
    };
}


const findRepoRoot = () /* :string */ => {
    let dir = resolve(".");
    while (true) {
        if (dir.endsWith("celer") || dir.endsWith("celer/") && fs.existsSync(path.join(dir, ".git"))) {
            return dir;
        }
        const parent = dirname(dir);
        if (dir === parent) {
            throw new Error("Could not find repo root");
        }
        dir = parent;
    }
}

const cleanCode = (code /* :string[] */) /* :string[] */ => {
    return code.join("\n").trim().split("\n").map(line => line.trimEnd());
}

const getSignatureFromCode = (code /* :string[] */) /* :[string, string[]] */ => {
    if (code.length === 0) {
        return "";
    }
    // detect the last line starts with }, >, or )
    const charMap = {
        "}": "{",
        ">": "<",
        "]": "[",
    }
    const codeEndChar = code[code.length - 1][0];
    if (!Object.keys(charMap).includes(codeEndChar)) {
        // don't know what to do
        return code.join("\n");
    }
    const codeStartChar = charMap[codeEndChar];
    // find the first line that has no leading whitespace, and also ends with the corresponding start char
    let startLine = code.length - 2;
    for (; startLine > 0; startLine--) {
        // skip if the line has leading whitespace
        if (code[startLine].trimStart() !== code[startLine]) {
            continue;
        }
        if (code[startLine].endsWith(codeStartChar)) {
            break;
        }
    }
    let signature = code.filter((_, i) => i <= startLine).join("\n");
    signature += " ... " + code[code.length - 1];

    const lastBlockContent = code.filter((_, i) => i > startLine && i < code.length - 1);
    let indent = 0;
    for (let i=0;i<lastBlockContent.length;i++) {
        const trimmed = lastBlockContent[i].trimStart();
        if (trimmed !== lastBlockContent[i]) {
            indent = lastBlockContent[i].length - trimmed.length;
            break;
        }
    }
    return [signature, lastBlockContent.map(line => line.substring(indent))];
}

module.exports = {
    readModuleFromFile,
    readModulesInDirectory,
    findRepoRoot,
    getSignatureFromCode,
    readModule,
}
