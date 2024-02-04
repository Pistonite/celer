# Plugin Settings
Plugin is a feature meant for everyone, not just routers. If you are viewing
someone else's route and want to tweak it slightly, plugins can also do that for you.

## Remove Plugins
If you want to disable a plugin that was applied to the route by the author(s) of that route, follow these steps:

1. Open the document you want to tweak. Make sure it has finished loading.
2. Click on <FluentIcon name="Settings20Regular" /> `Settings` on the toolbar.
3. Select the <FluentIcon name="Wrench20Regular" /> `Plugins` category.
4. Under `App Plugins` or `Route Plugins`, uncheck the plugins that you don't want to apply.
5. Celer should start re-compiling the route with the changes.
:::tip
Unchecking a plugin only disable the plugin for the current document you are viewing.
:::

## Add Plugins
Before adding new plugins, make sure you familiarize yourself with how
they are configured [here](./getting-started.md#route-configuration). The syntax
will be similar.

First follow these steps to get to the user plugin settings:
1. Open the document you want to tweak.
2. Click on <FluentIcon name="Settings20Regular" /> `Settings` on the toolbar.
3. Select the <FluentIcon name="Wrench20Regular" /> `Plugins` category.
4. Under `User Plugins`, click the `Edit Config` button.
5. You should see a dialog with a text area popped up.

The syntax to configure user plugins is:
```yaml
"Document Title":
- use: my/extra/plugin1.js
- use: my/extra/plugin2.js
...
```
:::tip
The settings here use the YAML format. See [here](../route/yaml-basics.md) for a quick guide
on YAML if you are not familiar.
:::
Replace:
- `"Document Title"` with the exact title of the document you want to add the plugin to, surrounded by quotes.
  There is a little hint above the text area that tells you what the title of the current document is. You can also use the wildcard`"*"` to 
  add the plugin to all documents.
- `my/extra/plugin1.js` with the path of the plugin. The plugin should be a file on GitHub,
  and you can reference it by `<user>/<repo>/path/to/file.js`. See [here](../route/file-structure.md)
  for more about the `use` property. However note that you cannot use a path to file on your computer.

The same options from [here](./getting-started.md) all work here.

If you want to add plugins to a different document, start a new section with the other document's title.

