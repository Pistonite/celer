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

## Extension and Overriding
When defining a tag, if a tag with the same name is already defined, the previous one will be deleted.
```yaml
config:
- tags:
    my-tag:
      color: red
- tags:
    my-tag:
      bold: true # my-tag will be bold, but not red!
```
:::tip
Note that the example had to use 2 config objects, since YAML doesn't allow duplicate keys in a mapping
:::

If you want to extend the previous tag definition, use the `includes` property
```yaml
config:
- tags:
    my-tag:
      color: red
    my-tag2:
      includes: my-tag # you can also include multiple tags by specifying an array here
      bold: true # my-tag2 will be bold and red
```
Note that the example above uses a different name `my-tag2`. You can also use the same name to replace the tag definition earlier with the extended tag:
```yaml
config:
- tags:
    my-tag:
      color: red
- tags:
    my-tag:
      includes: my-tag
      bold: true # my-tag will be bold and red from now on
```
:::tip
This is possible because `includes` are resolved immediately when processed, instead of recursively resolved when used.
:::
This can be useful if you need to add-on to a tag and keep its other properties
      
## Accessibility
The `color` and `background` property let you change the color of the text. However, this introduces an issue with Celer's theme system. Each theme could set different backgrounds (light or dark) for different parts of the document. If you specify a color like `cyan`, it will look fine with dark backgrounds, but it will be barely visible in lighter backgrounds.

To address this problem, Celer lets you define separately what the color of the Rich Text tag should be under light and dark backgrounds.
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
The themes will then pick the appropriate color, depending on where the text is displayed.

You can still use the string shorthand from the previous example (i.e. `color: red`). This will set the color to `red` for both light and dark backgrounds. This could be helpful if:

1. The color looks fine in both light and dark backgrounds, or
2. The tag is for a `counter` block. Counter backgrounds cover the entire block. However, you can still define different colors for counters under different backgrounds.
