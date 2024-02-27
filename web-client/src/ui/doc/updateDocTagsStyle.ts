import { DocTag, DocTagColor } from "low/celerc";
import { injectDOMStyle, consoleDoc as console } from "low/utils";

import { RichTextVariables, getTagClassName } from "./components";

/// Update the styles/classes for rich tags
export function updateDocTagsStyle(tags: Readonly<Record<string, DocTag>>) {
    const tagKeys = Object.keys(tags);
    // sort the tags to ensure consistent order across updates
    tagKeys.sort();
    const colorSelectors = new Map<string, string[]>();
    const boldClasses: string[] = [];
    const italicClasses: string[] = [];
    const underlineClasses: string[] = [];
    const strikethroughClasses: string[] = [];
    const underlineStrikethroughClasses: string[] = [];
    tagKeys.forEach((tag) => {
        const data = tags[tag];
        const tagClass = "." + getTagClassName(tag);
        if (data.bold) {
            boldClasses.push(tagClass);
        }
        if (data.italic) {
            italicClasses.push(tagClass);
        }
        if (data.underline && data.strikethrough) {
            underlineStrikethroughClasses.push(tagClass);
        } else if (data.underline) {
            underlineClasses.push(tagClass);
        } else if (data.strikethrough) {
            strikethroughClasses.push(tagClass);
        }
        if (data.color) {
            addColor(data.color, "fg", colorSelectors, tagClass);
        }
        if (data.background) {
            addColor(data.background, "bg", colorSelectors, tagClass);
        }
    });
    let colorCss = "";
    if (colorSelectors.size > 0) {
        colorSelectors.forEach((classes, rule) => {
            colorCss += `${classes.join(",")}{${rule}}`;
        });
    }
    injectDOMStyle("rich-text-color", colorCss);
    let boldCss = "";
    if (boldClasses.length > 0) {
        boldCss = `${boldClasses.join(",")}{font-weight:bold;}`;
    }
    injectDOMStyle("rich-text-b", boldCss);
    let italicCss = "";
    if (italicClasses.length > 0) {
        italicCss = `${italicClasses.join(",")}{font-style:italic;}`;
    }
    injectDOMStyle("rich-text-i", italicCss);
    let underlineCss = "";
    if (underlineClasses.length > 0) {
        underlineCss = `${underlineClasses.join(
            ",",
        )}{text-decoration:underline;}`;
    }
    injectDOMStyle("rich-text-u", underlineCss);
    let strikethroughCss = "";
    if (strikethroughClasses.length > 0) {
        strikethroughCss = `${strikethroughClasses.join(
            ",",
        )}{text-decoration:line-through;}`;
    }
    injectDOMStyle("rich-text-s", strikethroughCss);
    let underlineStrikethroughCss = "";
    if (underlineStrikethroughClasses.length > 0) {
        underlineStrikethroughCss = `${underlineStrikethroughClasses.join(
            ",",
        )}{text-decoration:underline line-through;}`;
    }
    injectDOMStyle("rich-text-us", underlineStrikethroughCss);

    console.info("rich text css updated.");
}

function addColor(
    color: DocTagColor,
    type: "fg" | "bg",
    map: Map<string, string[]>,
    tagClass: string,
) {
    const lightVar = RichTextVariables[type].light.name;
    const darkVar = RichTextVariables[type].dark.name;
    if (typeof color === "string") {
        addClassToRule(`${lightVar}:${color};`, tagClass, map);
        addClassToRule(`${darkVar}:${color};`, tagClass, map);
        return;
    }
    if (color.light) {
        addClassToRule(`${lightVar}:${color.light};`, tagClass, map);
    }
    if (color.dark) {
        addClassToRule(`${darkVar}:${color.dark};`, tagClass, map);
    }
}

function addClassToRule(
    rule: string,
    tagClass: string,
    map: Map<string, string[]>,
) {
    const existing = map.get(rule);
    if (existing) {
        existing.push(tagClass);
    } else {
        map.set(rule, [tagClass]);
    }
}
