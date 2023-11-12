# Advanced
This section will cover additional configurations you can do with the editor and compiler.
Feel free to skip this first and come back once you have gone through the rest of the tutorial.

## Idle Cycles
Events like compile and auto-save are scheduled to run automatically on idle.
Celer uses a smart idle system that progressively makes the idle interval longer if
the app is inactive for a long time, to save resources.

### Web editor workflow
When using the web editor, the idle cycle could get to as long as 120 seconds.
This means if the app is left inactive for long enough, it will auto-save or recompile (if needed)
every 2 minutes.

Any actions, such as typing in the editor or clicking on <FluentIcon name="ArrowSync20Regular" /> `Compile project`
would reset the idle cycle to the shortest.

### External editor workflow
When using an external editor, the app could potentially be left without user interactions
in a long time. The app determines if the user is active by detecting changes in the files.
If the files are left unchanged for a while, the idle cycle could get to as long as 8 seconds.

Clicking on <FluentIcon name="ArrowSync20Regular" /> `Compile project` would reset the idle cycle to the shortest.

## Errors and Warnings
The compiler generates errors and warnings when something goes wrong.
You will see these errors attached to the line in the document where that error was originated from.

You can also click on <FluentIcon name="DocumentError20Regular" /> `View diagnostics`
on the Toolbar to quickly jump to an error.

## Cache Troubleshooting
The compiler caches the configuration by default to speed up compilation.
If you changed something in the configuration (i.e. map, icon, presets...) but
the change doesn't reflect in the document, follow the steps below to disable cache:

1. Click on <FluentIcon name="Settings20Regular"/> `Settings` on the Toolbar.
2. Select the <FluentIcon name="Code20Regular" /> `Editor` category.
3. Under `Compiler`, turn off `Cache Config`.

If this doesn't solve the issue, close the project, refresh the page and reopen the project.
The editor doesn't store data across sessions (other than settings), so refreshing the page
would get rid of the cache completely.
:::warning
If you are loading a file from GitHub, the resource could also be cached in the CDN.
If the file is updated on GitHub, you need to wait around 5 minutes to get the updated file.
:::
