/// <reference types="vite/client" />

declare module "is-equal" {
    export default function (a: unknown, b: unknown): boolean;
}

// suppress the error from loading prismjs languages
declare module "prismjs/components/*" {
}
