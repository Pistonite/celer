# Getting Started
:::tip
The plugin system is currently unstable.
:::
The principle of the plugin system is to separate core Celer functionalities from additional (and mostly optional) functionalities.

A plugin in Celer is a piece of program that runs as part of the compiler. The process goes as the following:

1. The compiler parses the income files and celer-specific syntax like presets
2. The compiler hands the compiled document to a plugin
3. The plugin is free to make any changes to the document. Then it hands the document to the next plugin in line.
4. After the last plugin is done modifying the document, it hands the document back to the compiler.


## Configuration
To add a plugin to the compiler, use the `plugins` property in your `config`.

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
:::

## Built-in Plugins
Here is a list of all built-in plugins. The `ID` column is what you put after `use` in the config.
|Name|ID|Description|
|-|-|-|
|[Link](./link.md)|`link`|Turns `link` tags into clickable links|