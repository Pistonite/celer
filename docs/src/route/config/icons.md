# Icon Configuration
The `icons` property in the configuration defines a mapping between icon id to
asset location. You can use the `use` property to include an icon, or just specify a URL string.

If multiple configuration defines the same icon id, later configuration will override previous ones.

## Examples
```yaml
config:
  icons:
    shrine: https://icons.pistonite.org/icon/shrine.shrine.none.69a2d5.c1fefe.69a2d5.c1fefe.69a2d5.c1fefe.png
    foo: use: ./icons/foo.png
    bar: use: someone/repo/bar.png
```

## Support
Check below for which icon formats are supported:

|Format|Render in Document|Render On Map|As Split Icon|
|-|-|-|-|
|`png`|Yes|Yes|Yes|
|`jpg`|Yes|Yes|Yes|
|`gif`|Yes|Unknown|No|
|`svg`|Yes|Unknown|No|
|`webp`|Yes|Unknown|No|
