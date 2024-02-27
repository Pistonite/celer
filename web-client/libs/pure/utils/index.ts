export { RwLock } from "./RwLock.ts";

/// Try converting an error to a string
export function errstr(e: unknown): string {
    if (typeof e === "string") {
        return e;
    }
    if (e) {
        if (typeof e === "object" && "message" in e) {
            if (typeof e.message === "string") {
                return e.message;
            }
        }
        if (typeof e === "object" && "toString" in e) {
            return e.toString();
        }
    }
    return `${e}`;
}
