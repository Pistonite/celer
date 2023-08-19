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
|`color`|`string`|Color of the text as a [CSS Color](https://www.w3schools.com/cssref/css_colors.php)|
|`background`|`string`|Background color of the text as a [CSS Color](https://www.w3schools.com/cssref/css_colors.php)|
      
