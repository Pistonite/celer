# Tags Configuration
The tag system lets you customize text styles such as changing colors and
making text bold. Some plugins also use tags to enable additional functionality.
Text with tags are called **Rich Text** in the docs. 

For example:
```
hello .dir(world)
```
The text `world` is tagged with the `dir` tag.

For more on Rich Text and how to use them, see [Customizing Text](../customizing-text.md).
This section focuses on how the tags used in Rich Texts are configured.

Similar to icons, tags defined in all configurations are combined.
Tags with the same name in a later configuration can override a previous one.
Tag names are by convention written in `kebab-case`

## Examples
Let's define a tag called `wonderful`, which changes the text to be bold and red.
```yaml
config:
- tags:
    wonderful:
      bold: true
      color: red
```
:::warning
Tag names must not contain spaces
:::

It can be used like `.wonderful(example text here)`

## Properties
The following properties are available for each tag. All properties are optional

|Property|Type|Description|
|-|-|-|
|`bold`|`boolean`|Show text as bold|
|`italic`|`boolean`|Show text as italic|
|`underline`|`boolean`|Underline the text|
|`strikethrough`|`boolean`|Strike through the text|
|`color`|`string` or [see below](#accessibility)|Color of the text as a [CSS Color](https://www.w3schools.com/cssref/css_colors.php)|
|`background`|`string` or [see below](#accessibility)|Background color of the text as a [CSS Color](https://www.w3schools.com/cssref/css_colors.php)|
      
## Accessibility
The `color` and `background` property let you change the color of the text. However, this introduces an issue with Celer's theme system. Each theme could set different backgrounds (light or dark) for different parts of the document. If you specify a color like `cyan`, it will look fine with dark backgrounds, but it will be barely visible in lighter backgrounds.

To address this problem, Celer let's you define separately what the color of the Rich Text tag should be under light and dark backgrounds.
```yaml
config:
- tags:
    wonderful:
      bold: true
      color:
        # display wonderful tag with dark blue color
        # if it's on a light background
        light: darkblue
        # ... or cyan if it's on a dark background
        dark: cyan
    wonderful-background:
      # works for both color and background
      color:
        light: black
        dark: white
      background:
        light: white
        dark: black
```

You can still use the string shorthand from the previous example (i.e. `color: red`). This will set the color to `red` for both light and dark backgrounds. This could be helpful if:

1. The color looks fine in both light and dark backgrounds, or
2. The tag is for a `counter` block. Counter backgrounds cover the entire block. However you can still define different colors for counters under different backgrounds.