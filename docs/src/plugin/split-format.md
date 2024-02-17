# Split Format
The `split-format` plugin automatically sets the [`split-name`](../route/counter-and-splits#format-split-names) property for lines
based on the split type of that route (set by the `counter` property), with
a configurable format.

Add the plugin with
```yaml
config:
- plugins:
  - use: split-format
    with:
      # configure format here
```

## Configuration
### Use the correct name for split types
You can configure one format per split type based on the [display name of the type
configured on the counter tag](../route/counter-and-splits#splitting). 

For example, with this setup:
```yaml
config:
- tags:
    my-counter:
      split-type: My Counters

route:
- Example Section:
  - Example Line:
      counter: .my-counter()
```
The name to use is "My Counters", not "my-counter".
:::tip
The split-type is also what's shown in the settings dialog. Check there if you don't know!
:::

### Configure formats
Use the split type names as keys and the split format as the value in the configuration. 

For example:
```yaml
config:
- plugins:
  - use: split-format
    with:
      My Counters: Counter
      Shrines: "[.var(pad03:counter-shrine)] .prop(text)"
```
The `prop` tag can be used to access properties of the line:
- `.prop(text)` becomes the primary text 
- `.prop(comment)` becomes the secondary text
- `.prop(counter)` becomes the counter text

Note that tags cannot be nested - `.var(.prop(text))` is not supported!

:::warning
If `.var` is used inside the split formats, the `split-format` plugin needs to
be BEFORE the `variables` plugin in the `plugins` list!
:::
