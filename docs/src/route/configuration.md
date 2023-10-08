# Configuration
The `config` property in `project.yaml` is an array of configuration objects.
You can load configuration from another file using the `use` property (See [File Structure](./file-structure.md)).
Or make your own configuration.

## Properties
Most of the time, you can use config presets provided by Celer or a 3rd party.
In case you want to make your own, here are the available properties:

|Property|Description|
|-|-|
|`icons`|Add icon definition. See [Icons](./config/icons.md) for detail|
|`tags`|Add tag definition for use in Rich Text. See [Tags](./config/tags.md) for detail|
|`presets`|Add preset definition. See [Presets](./config/presets.md) for detail|
|`plugins`|Add plugin definition. See [Plugins](../plugin/getting-started.md) for detail|
|`map`|Define map properties. See [Map](./config/map.md) for detail|

Configurations are meant to be composed and reused with other configurations.
So most properties in all configurations are combined. An exception to this is `map`.
The compiler will give an error if multiple configurations define the map.

## Example
```yaml
# project.yaml
config:
- use: Pistonite/celer/presets/botw/map.yaml
- use: someone/someones-preset/awesome.yaml
- use: ./path/to/local.yaml
- icons:
    example-icon:
      use: hello/world/example.png
  tags:
    colorful:
      color: blue

# path/to/local.yaml
presets:
  HelloWorld:
    text: Hello, world!
```

## Configuration Files
The configuration files that are loaded by `use` should
be a YAML file that defines a mapping on the root level. For example:
```yaml
# something.yaml
icons:
  foo:
    use: bar/biz/boo.png
tags:
  colorful:
    color: blue
```
:::warning
Note that properties in `something.yaml` (`icons` and `tags`) don't have a 
`-` in front because the file needs to contain a mapping, not an array at the root level.
:::

Be careful that top-level `use` is not permitted. The following config file
is invalid for others to include with `use`:
```yaml
# something.yaml
use: another/file/something.yaml
```
:::warning
CAUTION: The example above is for what is NOT supported
:::
:::tip
You can still use `use` in the config properties themselves, like
the example above in the `icons` property
:::

## Grouping
Celer does NOT support complex grouping structures for dependency. For example, 
you CANNOT make a config file A which includes 3 other config files, and include
the 3 files by including config file A.
```yaml
# This maybe desired in some cases, but Celer does not support this

# project.yaml
config:
- use: ./A.yaml

# A.yaml
- use: ./1.yaml
- use: ./2.yaml
- use: ./3.yaml
```
:::warning
CAUTION: The example above is for what is NOT supported
:::

The reason for not supporting this is that Celer discourages complex dependency structure.
Complex dependencies make it tricker to debug when a configuration is not functioning and
make it hard to display meaningful error messages.

There are workarounds to this if you really want this behavior, particularly when distributing the config files.
For example, you can generate `A.yaml` from the 3 other files ahead of time. Or you could use CI like GitHub Actions to do that automatically.
