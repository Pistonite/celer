# Using Presets
Presets are customized line templates that makes it easier to configure the properties of a line.

A preset must be defined in the config to be used.
Usually, you will load presets from configs other people made.
You can also define your own presets by following [the reference](./config/presets.md)

## Syntax
A reference to a preset is `_` followed by the name of the preset, like:
```
_Preset
```
A preset name may have `::` in the middle:
```
_Boss::Pig::Big
```
This is referred to as preset namespaces, which are used to group presets.
In this case, `Boss`
is the outer-most namespace, and `Pig` is a sub-namespace inside `Boss`.
Finally, `Big` is the preset inside the `Pig` namespace.

## Variables
Some presets can also take in variables:
```
_Preset<Hello World>
```
Here, the string `Hello World` is passed into the preset as a variable.

Some presets may require more than one variable, in which case they are
separated with commas (`,`):
```
_Teleport<1345,8769>
```
:::warning
White spaces in variables are significant. `_Teleport<1345, 8769>`
is different from the example above, because there is a space
in front of `8769`. The second variable will have the value `" 8769"`.
Sometimes it's ok, depending on how the preset uses the variable.
:::

## Escaping
If the variable value contains `,` or `>`, you can escape it by prepending a `\`
The example below specifies a single variable with value `Hello, World!`:
```
_Preset<Hello\, World!>
```
The example below specifies a variable with value `<foo>`:
```
_Preset<<foo\>>
```
:::tip
Escape `\` with `\\` to escape an escape sequence. `_Preset<Hello\\,World\\>` will take 2 variables `Hello\` and `World\`.
Note that this only works to escape `\,` and `\>`.
`_Preset<f\\oo,b\ar>` will still take the variables as `f\\oo` and `b\ar`
:::

## Simple Usage
To apply a preset for a line, use the preset syntax as the text for the line.
You can also customize the properties after applying the preset, just
like a regular line:
```yaml
- _Shrine::ShigmaFoo
- _Landmark::Mountain:
    comment: woo hoo
```
However, preset syntax in the `text` property will not trigger:
```yaml
- placeholder:
    text: _Preset # this will be the text "_Preset" literally
```

Usually, the preset will define a `text` property. However, if it does not,
the preset syntax string will be kept as the text of the line. Usually this means
whoever made the preset is expecting whoever uses the preset to override the text.

If the text starts with `_`, but is not a valid preset, the text will also be kept and no error will be generated.
```yaml
- _I just want this line to start with an underscore # this is valid
```

## Multiple Presets
Multiple presets can be applied to a single line with the `presets` property:
```yaml
- _PresetA:
    text: i have 3 presets applied
    presets:
    - _PresetB
    - _PresetC
```
:::tip
You can also do `presets: _FooBar` if you are only adding one extra preset
:::
:::tip
Unlike the simple usage, `presets` property will produce errors if 
any preset syntax is invalid, or the preset is not found in the config.
:::
The `presets` property is applied first, and then the preset
defined from the text. The order the presets are applied are:
- PresetB
- PresetC
- PresetA

Later presets will override the same property defined by a previous preset.
For example, if both `PresetB` and `PresetA` defines the `comment` property,
it will end up with the `comment` from `PresetA`.

Finally, the extra properties defined in the line are applied last, and will override any preset

## Movement Presets
Presets can also be used within the `movements` property.
See [Customizing Movements](./customizing-movements#presets) for more details.
