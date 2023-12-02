//! Basic utilities for working in browser environments.

export const isInDarkMode = () =>
    !!(
        window.matchMedia &&
        window.matchMedia("(prefers-color-scheme: dark)").matches
    );

/// Sleep for the given number of milliseconds
///
/// Example: await sleep(1000);
export const sleep = (ms: number) =>
    new Promise((resolve) => setTimeout(resolve, ms));

/// Accessor for DOM element of type E identified by an id
export class DOMId<E extends HTMLElement> {
    readonly id: string;
    constructor(id: string) {
        this.id = id;
    }

    public get(): E | undefined {
        return document.getElementById(this.id) as E | undefined;
    }
}

/// Accessor for runtime injected style tag
export class DOMStyleInject {
    readonly id: string;
    constructor(id: string) {
        this.id = id;
    }

    public setStyle(style: string) {
        let styleTag = document.querySelector(`style[data-inject="${this.id}"`);
        if (!styleTag) {
            styleTag = document.createElement("style");
            styleTag.setAttribute("data-inject", this.id);
            const head = document.querySelector("head");
            if (!head) {
                return;
            } else {
                head.appendChild(styleTag);
            }
        }
        (styleTag as HTMLStyleElement).innerText = style;
    }
}

function addCssObjectToMap(
    selector: string,
    obj: Record<string, unknown>,
    map: Record<string, string>,
) {
    let group = map[selector] || "";
    Object.entries(obj).forEach(([key, value]) => {
        if (value === undefined && value === null) {
            return;
        }
        if (typeof value === "object") {
            addCssObjectToMap(
                `${selector}${key}`,
                value as Record<string, unknown>,
                map,
            );
            return;
        }
        group += `${key}:${value};`;
    });
    map[selector] = group;
}

/// Accessor for DOM Class
export class DOMClass {
    readonly className: string;
    constructor(className: string) {
        this.className = className;
    }

    public style(style: Record<string, unknown>) {
        const map = {};
        addCssObjectToMap(`.${this.className}`, style, map);
        const css = Object.entries(map)
            .map(([selector, group]) => {
                return `${selector}{${group}}`;
            })
            .join("");
        new DOMStyleInject(`${this.className}-styles`).setStyle(css);
    }

    public query(selector?: string) {
        return document.querySelector(`.${this.className}${selector || ""}`);
    }

    public queryAll(selector?: string) {
        return document.querySelectorAll(`.${this.className}${selector || ""}`);
    }
}
