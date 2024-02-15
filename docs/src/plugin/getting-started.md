# Getting Started with Plugins
:::info
The plugin system is currently unstable.
:::
The principle of the plugin system is to separate core Celer functionalities from additional (and mostly optional) functionalities.
This way, the Celer Compiler can stay as stable as possible, while allowing the communities to add features as they wish.

Plugins are meant to be easy to use by both routers and viewers of the routes.
Routers define plugins in the routes to augment the compiler, and viewers
can use plugins to personalize their view of the document (See [here](./settings.md) for more info)

## Concept
Celer has 3 types of plugins:
1. **Compiler Plugins**: These run as part of the compiler and can change the route programmatically.
2. **Exporter Plugins**: These run after the compilation is done. They take the route as the input and produce some other file as the output.
3. **Custom Widgets**: These are custom UI that can be used to display extra information in the document.

Compiler and Exporter plugins are loaded into the route. Both route makers and route viewers can do this.
Custom widgets can only be added by viewers.

## Compiler/Exporter Configuration
To add a plugin to the route, use the `plugins` property in your `config`.

The example below adds the built-in [Link Plugin](./link.md) to the compiler, which transforms `link` tags into clickable links.
```yaml
config:
- plugins:
  - use: link
```
:::tip
Note that the `use` property takes `"link"`, which is not any of the syntax mentioned in
[File structure](../route/file-structure.md). This signals Celer that you want to use a built-in
plugin. Built-in plugins are implemented in Rust and has higher performance.

See [here](./built-in.md) for a full list of built-in plugins
:::

To specify an external plugin, use the `<user>/<repo>/<path>` syntax similar
to how route works explained [here](../route/file-structure.md).
```yaml
config:
- plugins:
  - use: foo/bar/path/to/plugin.js
```

### Additional Settings
Some plugins take additional settings through the `with` property
```yaml
config:
- plugins:
  - use: variables
    with:
      init:
        x: 3
```
Please refer to the documentation for the plugin you are using on what settings are available.


### Allow Duplicates
By default, celer will give an error if you specify the same plugin multiple times.
Most of the time it's due to a mistake, but if you actually want to, you can use the `allow-duplicate` property:
```yaml
config:
- plugins:
  - use: link
  - use: link
    allow-duplicate: true # without this, the compiler will not run
```

External plugins are considered duplicates if they point to the same path or url.

## Custom Widgets Configuration
Coming Soon
