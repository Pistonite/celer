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
:::info
Coming Soon. Currently it will give an error if `icons` is `true`
:::

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
