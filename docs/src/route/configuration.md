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
|`tags`|Add tag definition for use in Rich Text. See [Tags](./config/tags.md) for details|
|`presets`|Add preset definition. See [Presets](./config/presets.md) for details|
|`plugins`|Add plugin definition. See [Plugins](../plugin/getting-started.md) for details|
|`map`|Define map properties. See [Map](./config/map.md) for details|
|`includes`|Include other config objects. See [Include](#include) below for details|

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
If you need a config to include other configs, check out the [include](#include) property below.
:::warning
CAUTION: The example above is for what is NOT supported
:::
:::tip
You can still use `use` in the config properties themselves, like
the example above in the `icons` property
:::

## Include
If you are distributing config files, it may be helpful to split up the "source files"
of the presets into smaller, more maintainable chunks. However, it could lead to
situations where users need to `use` a lot of files in their config to get started.
```yaml
# project.yaml
config:
- use: foo/bar/core.yaml
- use: foo/bar/basic.yaml
- use: foo/bar/items.yaml
- use: foo/bar/enenmies.yaml 

# ^ that's a lot of `use`s!
```

This is when the `includes` property can be helpful. It defines an array of config objects
that can nest inside the parent config:
```yaml
# project.yaml
config:
- use: foo/bar/all.yaml

# foo/bar/all.yaml
includes:
- use: ./core.yaml
- use: ./basic.yaml
- use: ./items.yaml
- use: ./enenmies.yaml 
```
In the example above, `all.yaml` is a config file that includes the content
of all the individual config chunks.
When distributing these configs, users can simply include `all.yaml` and not
worry about the internals.
