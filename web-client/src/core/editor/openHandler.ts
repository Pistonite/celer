import { FsErr, FsError, FsResult } from "pure/fs";

import { AlertMgr, consoleEditor as console } from "low/utils";

export function createRetryOpenHandler(alertMgr: AlertMgr) {
    return async (err: FsError): Promise<FsResult<boolean>> => {
        const { code, message } = err;
        console.error(`open failed with code ${code}: ${message}`);
        if (code === FsErr.PermissionDenied) {
            const retry = await alertMgr.show({
                title: "Permission Denied",
                message:
                    "You must given file system access permission to the app to use this feature. Please try again and grant the permission when prompted.",
                okButton: "Grant Permission",
                cancelButton: "Cancel",
            });
            if (retry) {
                console.info("retrying open after permission denied");
            }
            return { val: retry };
        }
        if (code === FsErr.UserAbort) {
            // don't retry if user aborted
            return { val: false };
        }
        if (code === FsErr.NotSupported) {
            // don't retry if not supported
            await alertMgr.show({
                title: "Not Supported",
                message: "Your browser does not support this feature.",
                okButton: "Close",
                learnMoreLink: "/docs/route/editor/web#browser-os-support",
            });
            return { val: false };
        }
        if (code === FsErr.IsFile) {
            await alertMgr.show({
                title: "Error",
                message:
                    "You opened a file. Make sure you are opening the project folder and not individual files.",
                okButton: "Close",
            });
            return { val: false };
        }
        // if it's unknown error, let user know and ask if they want to retry
        const retry = await alertMgr.show({
            title: "Cannot open project",
            message: `File system operation failed with code ${code}: ${message}`,
            okButton: "Retry",
            cancelButton: "Cancel",
        });
        if (retry) {
            console.info("retrying open after error");
        }
        return { val: retry };
    };
}
