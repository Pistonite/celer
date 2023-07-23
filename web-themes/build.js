//! build script
//! 
//! abc.css => postcss => clean-css => abc.min.css

const fs = require("fs");
const path = require("path");
const autoprefixer = require('autoprefixer')
const postcss = require('postcss')
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

/// Read a file async 
function readFileAsync(filePath) {
    return new Promise((resolve, reject) => {
        fs.readFile(filePath, "utf8", (err, data) => {
            if (err)
                reject(err);
            else
                resolve(data);
        });
    });
}

/// Write a file async
function writeFileAsync(filePath, data) {
    return new Promise((resolve, reject) => {
        fs.writeFile(filePath, data, "utf8", (err) => {
            if (err) { 
                reject(err);
            } else {
                resolve();
            }
        });
    });
}

function writeIntermediateOutput(filePath, result) {
    fs.writeFile(filePath, result.css, () => true)
    if ( result.map ) {
        fs.writeFile(filePath + ".map", result.map.toString(), () => true)
    }
}

/// Process a single src file
///
/// File name is not the full path (i.e. "abc.css")
async function processSrcFile(fileName) {
    const baseName = path.basename(fileName, ".css");
    const srcFile = path.join(srcDir, fileName);
    const cssInput = await fs.promises.readFile(srcFile, "utf8");
    const interDistFile = path.join(intermediateDistDir, fileName);
    const cssProcessed = await preprocessor.process(
        cssInput, 
        { from: srcFile, to: interDistFile }
    );
    // no need to wait for intermediate output to finish
    writeIntermediateOutput(interDistFile, cssProcessed);
    const { styles: cssMinified } = minifier.minify(cssProcessed.css);
    const minDistFile = path.join(minifiedDistDir, baseName + ".min.css");
    await fs.promises.writeFile(minDistFile, cssMinified, "utf8");
    console.log(`${srcFile} => ${minDistFile}`);
}

async function processSrcDir() {
    const files = await fs.promises.readdir(srcDir);
    const baseNames = (await Promise.all(files.map(async (file) => {
        if (path.extname(file) === ".css") {
            await processSrcFile(file);
            return path.basename(file, ".css");
        }
    }))).filter(Boolean);
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

export const ThemeIds = [${baseNames.map(name => `"${name}"`).join(", ")}];
`
    fs.writeFileSync(path.join(defDistDir, "themes.g.ts"), content, "utf8");

}

processSrcDir();
