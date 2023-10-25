//! build script
//!
//! abc.css => postcss => clean-css => abc.min.css

const fs = require("fs");
const { spawn, spawnSync } = require("child_process");
const path = require("path");
const autoprefixer = require("autoprefixer");
const postcss = require("postcss");
const preprocessor = postcss([autoprefixer]);
const CleanCSS = require("clean-css");
const minifier = new CleanCSS({});

const srcDir = path.join(__dirname, "src");
const minifiedDistDir = path.join(__dirname, "../web-client/public/themes");
const defDistDir = path.join(__dirname, "../web-client/src/low");
const intermediateDistDir = path.join(__dirname, "dist");

// clean previous build
fs.rmSync(minifiedDistDir, { recursive: true, force: true });
fs.mkdirSync(minifiedDistDir);
fs.rmSync(intermediateDistDir, { recursive: true, force: true });
fs.mkdirSync(intermediateDistDir);

function runTxtpp() {
    const proc = spawnSync("txtpp", ["-r", "src"], {
        stdio: "inherit",
    });
    if (proc.status !== 0) {
        throw new Error("txtpp failed");
    }
}

function writeIntermediateOutput(filePath, result) {
    fs.writeFile(filePath, result.css, () => true);
    if (result.map) {
        fs.writeFile(filePath + ".map", result.map.toString(), () => true);
    }
}

/// Process a single src file
///
/// File name is not the full path (i.e. "abc.g.css"),
/// Base name is the name without the extension (i.e. "abc")
async function processSrcFile(fileName, baseName) {
    const srcFile = path.join(srcDir, fileName);
    const cssInput = await fs.promises.readFile(srcFile, "utf8");
    const interDistFile = path.join(intermediateDistDir, fileName);
    const cssProcessed = await preprocessor.process(cssInput, {
        from: srcFile,
        to: interDistFile,
    });
    // no need to wait for intermediate output to finish
    writeIntermediateOutput(interDistFile, cssProcessed);
    const { styles: cssMinified } = minifier.minify(cssProcessed.css);
    const minDistFile = path.join(minifiedDistDir, baseName + ".min.css");
    await fs.promises.writeFile(minDistFile, cssMinified, "utf8");
    console.log(`${srcFile} => ${minDistFile}`);
}

const EXTS = [".g.css", ".css"];
async function processSrcDir() {
    const files = await fs.promises.readdir(srcDir);
    const baseNames = (
        await Promise.all(
            files.map(async (file) => {
                const ext = EXTS.find((ext) => file.endsWith(ext));
                if (ext) {
                    const base = path.basename(file, ext);
                    await processSrcFile(file, base);
                    return base;
                }
            }),
        )
    ).filter(Boolean);
    console.log();
    console.log(`Intermediate output saved to ${intermediateDistDir}`);
    console.log(`Minified output saved to ${minifiedDistDir}`);

    // generate typescript theme def
    createThemeDef(baseNames);
    console.log(`Theme ids saved to ${defDistDir}`);
}

function createThemeDef(baseNames) {
    const content = `//! GENERATED FILE - DO NOT EDIT
//!
//! See the web-themes project for how to regenerate this

export const ThemeIds = [${baseNames.map((name) => `"${name}"`).join(", ")}];
`;
    fs.writeFileSync(path.join(defDistDir, "themes.g.ts"), content, "utf8");
}

runTxtpp();
processSrcDir();
