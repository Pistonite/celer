
# Export mist
The `export-mist` plugin lets you export the route to a split file (.msf) for mist.

This plugin comes pre-configured in the web app:

1. Click on <FluentIcon name="Settings20Regular" /> `Settings`.
2. Select the <FluentIcon name="Wrench20Regular" /> `Plugins` category.
3. Under `App Plugins`, make sure `Export split files` is checked.

Alternatively, you can add it to the route configuration:
```yaml
config:
- plugins:
  - use: export-mist
```

## Extra Options
This plugin provides the same split type configuration as [the LiveSplit plugin](/plugin/export-livesplit#split-types).
