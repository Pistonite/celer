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
        new DOMStyleInject(`${this.id}-styles-byid`).setStyle(css);
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
export class DOMClass<N extends string, E extends HTMLElement = HTMLElement> {
    private readonly _className: N;
    constructor(className: N) {
        this._className = className;
    }

    public get className(): N {
        return this._className;
    }

    // /// Get the combined class name from the given Griffel style map
    // public styledClassName(style: Record<N, string>) {
    //     // put stable class as last to override styles
    //     return mergeClasses(style[this.className], this.className);
    // }

    /// Inject a raw css map into the head that targets this class
    public injectStyle(style: Record<string, string>) {
        const map = {};
        addCssObjectToMap(`.${this.className}`, style, map);
        const css = Object.entries(map)
            .map(([selector, group]) => {
                return `${selector}{${group}}`;
            })
            .join("");
        new DOMStyleInject(`c-${this.className}-styles`).setStyle(css);
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
}

/// Merge a set of DOMClasses and raw class names into a single class name.
/// The stable classnames from DOMClasses are put at the end
export function smartMergeClasses<N extends string>(
    style: Record<N, string>,
    ...classes: (DOMClass<N> | string | false | undefined | null | 0)[]
): string {
    const inputs = [];
    const stables = [];
    for (let i = 0; i < classes.length; i++) {
        const c = classes[i];
        if (!c) {
            continue;
        }
        if (typeof c === "string") {
            stables.push(c);
        } else {
            inputs.push(style[c.className]);
            stables.push(c.className);
        }
    }
    return mergeClasses(...inputs, ...stables);
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
