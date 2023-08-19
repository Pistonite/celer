# Customizing Text
This section is an extension of [Customizing Lines](./customizing-lines.md),
focusing on using [Tags](./config/tags.md) to customize text style in supported places.

## Syntax
The general syntax for tagging part of a text is:
```
.foo(example)
```
Here, the text `example` is tagged with the tag `foo`.
There should be a tag defined with the name `foo`. Otherwise, the compiler will generate a warning.

## White Spaces
All white spaces in Rich Text are significant.

If you write
```
.foo( example )
```
It will be `" example "` tagged with `foo`. The quotes are not part of the text. They are just there to show the white spaces.

## Nesting
Tags cannot be nested.

If you write
```
.outer(hello .inner(world))
```

It will be parsed as:
1. The text `hello .inner(world` tagged with `outer`
2. The text `)`

## Escaping
The Rich Text syntax can be escaped. 
A common scenario is if you want to have a closing parenthesis in the text (`.tag(hello (world))`)

You can use `\)` and `\.` to escape `.` and `)` in the string.
For example, the example above should be `.tag(hello (world\))`

Another example: `\.tag(hello)` will be literally the text `.tag(hello)`, with no tags.
:::tip
Tag names cannot contain spaces. So you usually don't have to escape `.` unless it doesn't do what you want by default
:::

Finally, if you want literally `\)` or `\.`, use `\\` to escape the slash (i.e. `\\)` will be literally `\)`)
:::tip
You don't need to escape every `\`, only if the `\` and the next character forms an escape sequence.
:::
