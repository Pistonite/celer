# Icon Configuration
The `icons` property in the configuration defines a mapping between icon id to
asset location. You can use the `use` property to include an icon, or just specify a URL string.

Icons defined in all configurations are combined.
If multiple configuration defines the same icon id, later configuration will override previous ones.
Icons IDs are by convention written in `kebab-case`

## Examples
```yaml
config:
- icons:
    shrine: https://icons.pistonite.org/icon/shrine.shrine.none.69a2d5.c1fefe.69a2d5.c1fefe.69a2d5.c1fefe.png
    foo:
      use: ./icons/foo.png
    bar:
      use: someone/repo/bar.png
```

## Support
Check below for which icon formats are supported. The icon format is determined
from the extension, so make sure your icon file name or URL has one of the supported extensions as well.

|Format|Extensions|Support Note|
|-|-|-|
|`image/png`|`.png`|Full Support|
|`image/jpeg`|`.jpg`, `.jpeg`|Full Support|
|`image/gif`|`.gif`|Animated in document. Not animated in map|
|`image/webp`|`.webp`|Animated in document. Not animated in map. Converted to GIF in LiveSplit export|
