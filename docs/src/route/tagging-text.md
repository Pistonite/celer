# Tagging Text
In this section, you will see how to use tags to customize the styles of texts in the main column and in the notes.

## Configuring a Tag
Let's add a tag called `red` in the configuration, that changes the text to red
```yaml
config:
- tags:
    red:
      color: red
```
:::tip
See [Tags](./config/tags.md) for full reference on configuring tags.
:::

## Tagging Text
The general syntax for tagging part of a text is:
```
.foo(example)
```
Here, the text `example` is tagged with the tag `foo`.
There should be a tag defined with the name `foo`. Otherwise, the compiler will generate a warning.

Now let's add our `red` tag to some text:
```yaml
route:
- Example Section:
  - I am .red(red):
      comment: .red(hello) world
      notes: this .red(color) is cool
```
![image of example](https://cdn.discordapp.com/attachments/951389021114871819/1180394934998683679/image.png?ex=657d4373&is=656ace73&hm=6ff820e7d348437325904f8133b6e1dba8f1d965731e99e63d63e4faac2eecab&)

## White Spaces
All white spaces in tags are significant.

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
