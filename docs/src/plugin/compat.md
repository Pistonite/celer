# Compatibility Plugin
:::tip
The plugin system is currently unstable.
:::
The `compat` plugin gives additional short-hands for properties. Some of them are for compatibility with old Celer syntax.

Add the plugin with
```yaml
config:
- plugins:
  - use: compat
```

## Hide icons on map or doc
The old Celer has a `hide-icon-on-map` property to make the icon only appear on the map.
The new Celer has a similar functionality, but does require to specify the icon again, if the icon is specified in a preset:

```yaml
config:
- presets:
    # icon is set in the preset
    Example:
      icon: my-icon

route:
- Example Section:
  - _Example:
      hide-icon-on-map: true # in old celer you could just do this
  - _Example:
      icon:
        map: my-icon # in new celer, you have to specify the icon too
```
This is not ideal, since if the icon ID in the preset changes, the route won't automatically get updated.

This plugin brings back the `hide-icon-on-map` property and a new `hide-icon-on-doc` property.
You can use them just like the old Celer syntax.
```yaml
config:
- presets:
    # icon is set in the preset
    Example:
      icon: my-icon
  plugins:
  - use: compat

route:
- Example Section:
  - _Example:
      hide-icon-on-map: true # works in new celer with the `compat` plugin
  - _Example:
      hide-icon-on-doc: true # works too
```
