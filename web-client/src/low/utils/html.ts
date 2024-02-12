//! Basic utilities for working in browser environments.

import { mergeClasses } from "@fluentui/react-components";

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

/// Append "px" to a number
export const px = (n: number) => `${n}px`;

/// Accessor for DOM element of type E identified by an id
export class DOMId<N extends string, E extends HTMLElement = HTMLElement> {
    readonly id: N;
    constructor(id: N) {
        this.id = id;
    }

    public as<E2 extends E>(): DOMId<N, E2> {
        return this as unknown as DOMId<N, E2>;
    }

    public get(): E | undefined {
        const element = document.getElementById(this.id) || undefined;
        return element as E | undefined;
    }

    public style(style: Record<string, unknown>) {
        const map = {};
        addCssObjectToMap(`#${this.id}`, style, map);
        const css = Object.entries(map)
            .map(([selector, group]) => {
                return `${selector}{${group}}`;
            })
            .join("");
        injectDOMStyle(`${this.id}-styles-byid`, css);
    }
}

/// Inject a css string into a style tag identified by the id
export function injectDOMStyle(id: string, style: string) {
    const styleTags = document.querySelectorAll(`style[data-inject="${id}"`);
    if (styleTags.length !== 1) {
        const styleTag = document.createElement("style");
        styleTag.setAttribute("data-inject", id);
        styleTag.innerText = style;
        document.head.appendChild(styleTag);
        setTimeout(() => {
            styleTags.forEach((tag) => tag.remove());
        }, 0);
    } else {
        const e = styleTags[0] as HTMLStyleElement;
        if (e.innerText !== style) {
            e.innerText = style;
        }
    }
}

/// Wrap CSS inside a query
export function cssQuery(selector: string, css: string): string {
    return `${selector}{${css}}`;
}

/// Get the media query for the given theme
export function prefersColorScheme(theme: "light" | "dark"): string {
    return `@media(prefers-color-scheme: ${theme})`
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
export class DOMClass<N extends string, E extends HTMLElement = HTMLElement> {
    readonly className: N;
    constructor(className: N) {
        this.className = className;
    }

    /// Inject a raw css map into the head that targets this class
    public style(style: Record<string, unknown>, query?: string) {
        const map = {};
        addCssObjectToMap(`.${this.className}`, style, map);
        let css = Object.entries(map)
            .map(([selector, group]) => {
                return `${selector}{${group}}`;
            })
            .join("");
        if (query) {
            css = cssQuery(query, css);
        }
        injectDOMStyle(`c-${this.className}-styles`, css);
    }

    public query(selector?: string): E | undefined {
        const element =
            document.querySelector(`.${this.className}${selector || ""}`) ||
            undefined;
        return element as E | undefined;
    }

    public queryAll(selector?: string): NodeListOf<E> {
        return document.querySelectorAll(`.${this.className}${selector || ""}`);
    }

    public queryAllIn(element: HTMLElement, selector?: string): NodeListOf<E> {
        return element.querySelectorAll(`.${this.className}${selector || ""}`);
    }

    public addTo(element: HTMLElement) {
        element.classList.add(this.className);
    }

    public removeFrom(element: HTMLElement) {
        element.classList.remove(this.className);
    }
}

/// Merge a set of DOMClasses and raw class names into a single class name.
export function smartMergeClasses<N extends string>(
    style: Record<N, string>,
    ...classes: (DOMClass<N> | string | false | undefined | null | 0)[]
): string {
    const inputs = [];
    for (let i = 0; i < classes.length; i++) {
        const c = classes[i];
        if (!c) {
            continue;
        }
        if (typeof c === "string") {
            inputs.push(c);
        } else {
            inputs.push(style[c.className]);
            inputs.push(c.className);
        }
    }
    return mergeClasses(...inputs);
}

export function concatClassName<TBase extends string, TName extends string>(
    base: TBase,
    name: TName,
): `${TBase}-${TName}` {
    return `${base}-${name}`;
}

/// Wrapper for a CSS variable
export class CSSVariable<N extends `--${string}`> {
    readonly name: N;
    constructor(name: N) {
        this.name = name;
    }

    /// Get the var(name) expression
    public varExpr() {
        return `var(${this.name})` as const;
    }

    /// Get the var(name, fallback) expression
    public fallback<S>(value: S) {
        return `var(${this.name}, ${value})` as const;
    }
}
