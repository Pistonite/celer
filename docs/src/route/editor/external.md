# External Editor
This workflow is meant for people who wish to edit the files with another program,
and for those whose environment doesn't support the web editor.

## Open a project
A project is a folder on your PC. Follow the steps to open a project.
:::tip
You can choose any folder if you are just testing. Or, skip ahead to [this section](../hello-world.md)
where step-by-step instructions are given for creating a minimal project folder.
:::

1. Click on <FluentIcon name="FolderOpen20Regular" /> `Open project` on the Toolbar and select the project folder.
:::tip
Alternatively, drag and drop the folder into the "Drag and drop a project folder here to open it" box.
:::
:::warning
Some browsers (Firefox) does not support directory scanning if the directory is uploaded from
a dialog (rather than drag-and-dropped). You will see a warning in this case.

You can do one of the following:
- Use drag-and-drop instead to open the project
- Continue opening, but note that if you create a new file in the project, you have to close and reopen the project to load it.
- Switch to Google Chrome or Microsoft Edge
:::
2. If asked for permissions to read from the file system, grant the permission.
3. If you see any error, follow the instruction in the error message.
4. The compiler should start running, and you should see the doc and map after it's done.

## Compiling the route
The browser will watch for changes you made to the file and automatically recompile.
The <FluentIcon name="ArrowSync20Regular" /> icon will spin when the compiler is running. You can also click on it to manually trigger a compilation.

