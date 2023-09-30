//! Post build script to process index.html

const VIEWER_TAG = "<!-- VIEWER -->";
const EDITOR_TAG = "<!-- EDITOR -->";
const TAGS = [VIEWER_TAG, EDITOR_TAG];

const fs = require("fs");
const path = require("path");

const indexHtmlPath = path.join(__dirname, "../dist/index.html");
const viewerHtmlPath = path.join(__dirname, "../dist/view.html");
const editorHtmlPath = path.join(__dirname, "../dist/edit.html");

const indexHtml = fs
    .readFileSync(indexHtmlPath, "utf8")
    .split("\n")
    .map((line) => line.trim());
function processHtml(htmlLines, removeTag) {
    let including = true;
    const excludeTags = TAGS.filter((tag) => tag !== removeTag);
    const output = [];
    htmlLines.forEach((line) => {
        if (line === removeTag) {
            return;
        }
        if (excludeTags.includes(line)) {
            including = !including;
        } else if (including) {
            output.push(line);
        }
    });

    return output;
}

const viewerHtml = processHtml(indexHtml, VIEWER_TAG)
    .filter((l) => {
        // remove monaco code completely from viewer
        return !l.includes("monaco");
    })
    .join("\n");
const editorHtml = processHtml(indexHtml, EDITOR_TAG).join("");

fs.writeFileSync(viewerHtmlPath, viewerHtml);
fs.writeFileSync(editorHtmlPath, editorHtml);
