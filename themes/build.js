//! build script
//! Takes .css inside ./src and minifies them into ./dist ending with .min.css

const fs = require("fs");
const path = require("path");
const CleanCSS = require("clean-css");
const minifier = new CleanCSS({});

const srcDir = path.join(__dirname, "src");
const distDir = path.join(__dirname, "../web-client/public/themes");

// clean previous build
fs.rmSync(distDir, { recursive: true, force: true });
fs.mkdirSync(distDir);

// Iterate through src dir
fs.readdirSync(srcDir).forEach(file => {
    // If file is css
    if (path.extname(file) === ".css") {
    const srcFile = path.join(srcDir, file);
    const distFile = path.join(distDir, path.basename(file, ".css") + ".min.css");
        // Read file
        const css = fs.readFileSync(srcFile, "utf8");
        // Minify file
        const { styles } = minifier.minify(css);
        // Write file
        fs.writeFileSync(distFile, styles, "utf8");
        console.log(`${srcFile} => ${distFile}`);
    }
});
