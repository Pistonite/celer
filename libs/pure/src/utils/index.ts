
/// Try converting an error to a string
export function errstr(e: unknown): string {
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
