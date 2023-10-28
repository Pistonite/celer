# Customizing Lines
You can add extra properties to the line by specifying it in the following format:
```yaml
- Get the weapon:
    comment: In a chest
    icon: chest
    notes: Under the rock
```
Here, `Get the weapon` is the primary text of the line. The mapping under it
contains the customization for `comment`, `icon` and `notes`.

The rest of this page lists all available properties.

## Primary and Secondary Text
The following properties are used for text customization:
|Property|Type|Description|
|-|-|-|
|`comment`|[Rich Text](./customizing-text.md)|Secondary text. This text will appear below the primary text in a smaller font|
|`text`|[Rich Text](./customizing-text.md)|Override the primary text. Presets specified here will NOT be expanded|

Example:
```yaml
- Do the thing:
    comment: .!!(be fast)
```

## Notes
The `notes` property can be used for adding notes that appear on the side.
|Property|Type|Description|
|-|-|-|
|`notes`|[Rich Text](./customizing-text.md)|Set a text note to appear on the side|

You can also specify an array instead of a single note.

Example:
```yaml
- One Note:
    notes: bala bala
- Many Notes:
    notes:
    - if you can't do it you can do the backup
    - if you fail the backup you can do the other backup
```

## Counter
This property can be used to customize the counter block.
|Property|Type|Description|
|-|-|-|
|`counter`|[Rich Text](./customizing-text.md)|Set the text and style for the counter block on the left of the main column|
:::warning
The Rich Text style is applied to the whole block. This means you can only have one tag like `.tag(hello)` or `hello` (no tag)
:::
:::tip
This is the replacement for the `split-type` property in the older Celer format
:::

## Icon
You can configure the icon by using the `icon` property, which takes 3 (optional) sub-properties:
|Property|Type|Description|
|-|-|-|
|`doc`|`string`|Icon ID of the icon on the document|
|`map`|`string`|Icon ID of the icon on the map|
|`priority`|0, 1 or 2|Set the priority/level of the icon.|

When setting priority, `0` is primary, `1` is secondary, and other values are "other".
If not set, the default priority is `2` and can be configured using global configuration

Example:
```yaml
- Only icon on the map:
    icon: 
      map: shrine
- icon on the map with priority:
    icon:
      map: temple
      priority: 1
- icon only on the doc:
    icon:
      doc: chest
- different icons on doc/map:
    icon:
      doc: chest
      map: shrine
```
There's also a shorthand that will probably be more commonly used:
```yaml
- specify both doc and map with default priority:
    icon: shrine 
# above and below are the same
- specify both doc and map with default priority:
    icon:
      doc: shrine 
      map: shrine 
```
:::tip
If you want to hide the icon on the map, but keep the icon already specified by a preset,
check out the [Compatibility Plugin](../plugin/compat.md) which brings back the `hide-icon-on-map` property
from old Celer.
:::

## Color
Use the `color` property to change the line color, both on the map and on the document.
The current line will have the new color, and lines afterward will keep this color until
changed again.

The color is interpreted as a [CSS color](https://www.w3schools.com/cssref/css_colors.php).
|Property|Type|Description|
|-|-|-|
|`color`|`string`|Change the line color|
:::tip
You can temporarily override the color on the map by using the `color` property under `movements` property
:::

Examples:
```yaml
- Change to red:
    color: red
- Change to orange:
    color: "#ff8800"
```

## Markers
Use the `markers` property to add circular markers.

|Property|Type|Description|
|-|-|-|
|`markers`|sequence|A list of markers. Each marker should have the `at` property and optionally a `color` property.|

The `at` property is required, and should be a valid [Route Coord](./config/map#coordinate-concepts) with 2 or 3 axes specified.

The `color` property is optional, and the default is the same color as the line

Examples:
```yaml
- Markers example:
    markers:
    - at: [0, 0]
    - at: [1, 2, 3]
      color: red
```

## Movements
Movements are more complicated than other properties, so it has its own dedicated section.
See [Customizing Movements](./customizing-movements)

## Split Name
Use this property to set the split name.
|Property|Type|Description|
|-|-|-|
|`split-name`|[Rich Text](./customizing-text.md)|Name to be used when exporting to compatible software, such as livesplit|
:::tip
You can use Rich Text for the split name so that plugins can alter it if necessary.
When exporting, the style provided by the Rich Text is usually not kept.
:::

If not specified, the primary text will be the split name.

## Presets
The `presets` property allow you to define additional presets for the line.
See [Using Presets](./using-presets.md) for details.

## Other Properties
If you are using [Plugins](../plugin/index.md). You may be able to specify additional properties. Please refer to the
documentation for the plugin you are using.

If you specify a property that is not recognized by Celer or any plugin, you will see a warning message
saying that property is unused.
