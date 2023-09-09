import { FsResult, FsResultCodes } from "./FsResult";

export const decodeFile = async (file: File): Promise<FsResult<string>> => {
    const contentBuffer = await file.arrayBuffer();
    try {
        const text = new TextDecoder("utf-8", {fatal: true}).decode(contentBuffer);
        return {
            code: FsResultCodes.Ok,
            value: text,
        };
    } catch (e) {
        console.error(e);
        return {
            code: FsResultCodes.InvalidEncoding,
        };
    }
}
