# pure/fs

High level browser to file system integration library.

This library integrates the `File`, `FileEntry` and `FileSystemAccess` API
to provide different levels of integration with file system in web apps.

Basically, user can select a directory as a mount point, and browser can access
read and sometimes write in the directory.

## Support
Use `fsGetSupportStatus()` to inspect which implementation will be used.

```typescript
import { fsGetSupportStatus } from "pure/fs";

const { implementation, isSecureContext } = fsGetSupportStatus();
```

`implementation` can be 3 values:
1. `FileSystemAccess`: This is used for Google Chrome and Edge, and possibly other browsers, under secure context.
2. `FileEntry`: This is used for Firefox when the FS is mounted through a drag-and-drop interface.
3. `File`: This is used for Firefox when the FS is mounted by a directory picker dialog

The implementation is also chosen in this order and the first supported one is selected. If you are on Chrome/Edge and `FileSystemAccess` is not used, you can use `isSecureContext` to narrow down the reason.

If you are wondering why Safari is not mentioned, it's because Apple made it so I have to buy a Mac to test, which I didn't.

After you get an instance of `FsFileSystem`, you can use `capabilities` to inspect
what is and is not supported.

See `FsCapabilities` for more info. This is the support matrix:
|Implementation|`write`?|`live`?|
|--------------|--------|-------|
|`FileSystemAccess`|Yes*|Yes    |
|`FileEntry`   |No      |Yes    |
|`File`        |No      |No     |

* - Need to request permission from user.


## Usage
First you need to get an instance of `FsFileSystem`. You can:
1. Call `fsOpenRead()` or `fsOpenReadWrite()` to show a directory picker,
2. Call `fsOpenReadFrom` or `fsOpenReadWriteFrom()` and pass in a `DataTransferItem` from a drag-and-drop interface.

NOTE: `fsOpenReadWrite` does not guarantee the implementation supports writing. You should check
with `capabilities` afterward.

This is an example drop zone implementation in TypeScript
```typescript
import { fsOpenReadWriteFrom } from "pure/fs";

const div = document.createElement("div");

div.addEventListener("dragover", (e) => {
    if (e.dataTransfer) {
        // setting this will allow dropping
        e.dataTransfer.dropEffect = "link";
    }
});

div.addEventListener("drop", async (e) => {
    const item = e.dataTransfer?.items[0];
    if (!item) {
        console.error("no item");
        return;
    }

    const result = await fsOpenReadWriteFrom(item);
    if (result.err) {
        console.error(result.err);
        return;
    }

    const fs = result.val;
    const { write, live } = fs.capabilities;
    // check capabilities and use fs
    // ...
});
```

## Retry open
You can pass in a retry handler and return true to retry, when opening fails.
The handler is async so you can ask user.

```typescript
import { FsError, FsResult } from "pure/fs";

async function shouldRetry(error: FsError, attempt: number): Promise<FsResult<boolean>> {
    if (attempt < 10 && error === FsError.PermissionDenied) {
        alert("you must give permission to use this feature!");
        return { val: true };
    }
    return { val: false };
}

const result = await fsOpenReadWrite(shouldRetry);
```



