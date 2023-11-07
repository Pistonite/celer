# Web Editor
The Web Editor is a great way for anyone to get started routing without having
to install any software (if your browser supports it.. check below!)

## Browser/OS support
Unfortunately, the support for browsers to access files on your PC has been stuck for decades.
It also depends on your Operating System.

**Google Chrome** and **Microsoft Edge** are the known browsers that work.
Firefox is known to not work.

If you use something else, continue below to see if it's supported.

## Open a project
A project is a folder on your PC. Follow the steps to open a project.
:::tip
You can choose any folder if you are just testing. Or, skip ahead to [this section](../hello-world.md)
where step-by-step instructions are given for creating a minimal project folder.
:::

1. Find the project folder in the file manager of your operating system
:::warning
Due to limitation of the browser API used to file system access, if your OS does not have a graphical file manager, you cannot use the web editor workflow.
:::
2. Drag and drop the folder into the "Drag and drop a project folder here to open it" box in Celer.
3. If asked for permissions to read and write to the file system, grant the permission.
4. If you see any error, follow the instruction in the error message.
5. You should be able to see the content of the folder listed if everything worked.

:::tip
If you didn't open an actual Celer project, you will see the route fails to compile. This is expected and it's OK!
:::

## File Tree
The file listing on the right of the editor is the File Tree. You can click on a folder to expand/collapse it.
Clicking on a file will let you edit that file.

You can hide the file tree in the settings (if it's taking too much space or some other reason):
1. Click on <FluentIcon name="Settings20Regular"/> `Settings` on the Toolbar.
2. Select the <FluentIcon name="Code20Regular" /> `Editor` category.
3. Under `Web Editor`. Use the `Show file tree` switch to toggle the feature.

## Saving to file system
If you have unsaved changes, you will see a "*" next to the file name. Click on <FluentIcon name="Save20Regular"/> `Save` on the Toolbar
to save the changes, or press `Ctrl + S`.

By default, changes are automatically saved the changes when you stop typing.
You can enable/disable it in the settings:
1. Click on <FluentIcon name="Settings20Regular"/> `Settings` on the Toolbar.
2. Select the <FluentIcon name="Code20Regular" /> `Editor` category.
3. Under `Web Editor`. Use the `Auto-save` switch to toggle the feature.

:::warning
If the file is also edited outside of the web editor, those changes may be lost when saving!
:::

## Loading from file system
If you made changes in the files outside of the web editor, you can load the changes
by clicking on the <FluentIcon name="FolderArrowUp20Regular" /> `Load from file system` button on the Toolbar.

One example is creating a new file or folder from the operating system (since the web editor doesn't support creating new files).

:::tip
Changes will only be loaded if the same file is not also edited in the web editor.
:::

## Unsupported operations
If you need to do any of the following:
1. Moving files between folders
2. Deleting a file or folder

The recommendation is to close the project (by clicking on <FluentIcon name="Dismiss20Regular" /> `Close project` or refresh the browser)
, make the change from your operating system, then reopen the project.

## Compiling the route
Whenever you stop typing, the route should automatically recompile. The <FluentIcon name="ArrowSync20Regular" /> icon will
spin when the compiler is running. You can also click on it to manually trigger a compilation.

