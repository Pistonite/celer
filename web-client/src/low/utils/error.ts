//! Error utilities

export function errorToString(e: unknown): string {
    if (typeof e === "string") {
        return e;
    }
    if (e && typeof e === "object" && "message" in e) {
        if (typeof e.message === "string") {
            return e.message;
        }
    }
    return `${e}`;
}
