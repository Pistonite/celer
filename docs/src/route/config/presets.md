# Preset Configuration
Presets are property templates for lines. You can define common properties
for multiple lines, or even multiple routes (by including the same configuration file in those route projects).

In a line, you can reference a preset by using `_` followed by the preset
```
_Example::Preset<Text>
```

For more on how to use presets in routing, see [Using Presets](../using-presets.md).
This section focuses on how to define presets in config.

Presets defined in all configurations are combined.
Presets with the same full name in a later configuration override the previous one.
Preset names are by convention written in `PascalCase`.

## Simple Example
Define a preset called `ItemBox` that defines an icon and text
```yaml
config:
- presets:
    ItemBox:
      icon: example-icon      
      text: Item Box
```
You can use this preset as `_ItemBox`:
```yaml
- _ItemBox:
    comment: example
```
This is equivalent to
```yaml
- Item Box:
    icon: example-icon
    comment: example
```

## Namespace
Presets can be grouped into namespaces, like `_Boss::Talus::Rare`. 
You can define a preset namespace by `_` followed by the namespace:
```yaml
config:
- presets:
    _Boss: # This defines the `Boss` namespace
      _Talus: # This defines the `Boss::Talus` namespace
        Rare:
          text: Rare Talus
          icon: talus-rare
        # You can have multiple presets in the same namespace
        # Below will be `Boss::Talus::Frost`
        Frost:
          text: Frost Talus 
```
You can also define a preset with the same name as the namespace
```yaml
config:
- presets:
    _Chest:
      Special: # `Chest::Special`
        icon: chest-special
    Chest: # `Chest`
      icon: chest
```
:::warning
Because of this, you cannot define a namespace with no presets inside it.
:::

## Properties
The properties you can define inside a preset is the same as a line.
See [Customizing Lines](../customizing-lines.md) for details.
This includes specifying other presets with the `presets` property.
```yaml
config:
- presets:
    ExampleA:
      text: An example
    ExampleB:
      presets:
      - _ExampleA
      comment: Using ExampleB will have ExampleA's text
```

## Variables
Presets can also take variables like `_Example<Hello, World>`.
In this case, the variables used in the presets are `"Hello"` and `" World"`. 
Quotes are not part of the variable, just to show that the spaces are significant (note the space before World)

To reference a variable in the preset definition, use `$(X)` where X is the 0-based index of the variable.
For example, `$(0)` is the first variable, `$(1)` is the second, etc.
```yaml
config:
- presets:
    WarpToCoord:
      text: Warp $(0)
      movements:
      - to: [$(1), $(2)]
        warp: true
```
You can use the preset like:
```yaml
- _WarpToCoord<Home,100,200>
```
Which is equivalent to:
```yaml
- Warp Home:
    movements:
    - to: ["100", "200"]
```
:::tip
Note that the coordinates are strings. All variables will be strings.
Most properties allow for coercion to the correct type.
:::
:::warning
String properties maybe reinterpreted. For example, you can take `_Preset` as
a variable, and if the variable is put in `presets` or `movements`, it will be
parsed as a preset. However, you cannot pass data structures like mapping or sequence.
`_Preset<[test\,1\,2\,3]>` will only treat `[test,1,2,3]` as a string, not an array.
:::

If the preset references a variable not provided (like `$(2)` in `_Example<A,B>`),
an empty string will be put in the place.

### Substituting keys

Keys in mappings can also be substituted with variables with the same syntax
```yaml
config:
- presets:
    AddFive:
      vars:
        $(0): .add(5)

route:
- Example:
  - _AddFive<hello>:
      notes: Add 5 to `hello` variable
```
:::tip
The `vars` property and the variable system requires the [`variables`](../../plugin/variables.md) plugin.
:::
The preset in the example above is expanded to
```yaml
- _AddFive<hello>:
    vars:
      hello: .add(5)
    notes: Add 5 to `hello` variable
```
If the template key conflicts with a static key, the result is undefined.
```yaml
config:
- presets:
    What:
      text: "hello"
      $(0): "world"

route:
- Example:
  - _What<text> 
  # text is mostly likely "hello" because of how the compiler is implemented
  # however there's no guarantee it will stay this way, even across compilations!
```

### Escaping
If you need to put literally the symbol `$`, you can escape the `$` with `$$`, if it conflicts with the syntax.
```yaml
config:
- presets:
    EscapeExample:
      text: $$(0)
```
`_EscapeExample<Hi>` will be literally `$(0)`, not `Hi`.

In most cases, however, the `$` doesn't conflict with the syntax, and there's no need to escape it:
```yaml
config:
- presets:
    NoNeedEscapeExample:
      text: I have a lot of $ money
```
