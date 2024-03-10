# Export LiveSplit
The `export-livesplit` plugin lets you export the route to a split file (.lss) for LiveSplit.

This plugin comes pre-configured in the web app:

1. Click on <FluentIcon name="Settings20Regular" /> `Settings`.
2. Select the <FluentIcon name="Wrench20Regular" /> `Plugins` category.
3. Under `App Plugins`, make sure `Export split files` is checked.

Alternatively, you can add it to the route configuration:
```yaml
config:
- plugins:
  - use: export-livesplit
```

## Extra Options
The plugin provides extra configuration when exporting.

### Icons
Setting `icons: true` will export the splits with the icons.
The icon from the document (i.e. `icon-doc`) is used, if the icon from the
document is different from the one from the map.

Currently, Webp icons are not supported by LiveSplit. The plugin will give
and error if webp icons are used in splits if icons are enabled.
You can set `webp-compat: skip` in the export configuration to skip those icons.
If you want to use a webp icon, you could convert it to a PNG or GIF with image
editing software or online tools.

### Subsplits
Setting `subsplits: true` in the option will divide the splits into subsplits
based on sections in the route. The subsplit group name will be the section name.

### Split Types
The recommended way to configure which split types are exported is through [Split Settings](../doc#splits)

By having `split-types: null`, Celer will automatically add the split settings for you.
You can also override it by putting an array of split types. For example:
```yaml
split-types:
- Lightroots
- Shrines
- Tears
```
:::tip
The split type names should match exactly with the checkbox labels in Split Settings. Case matters.
:::
