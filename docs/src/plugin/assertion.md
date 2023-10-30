# Assertion
:::info
The plugin system is currently unstable.
:::
The `assertion` plugin uses the output of the `variables` plugin,
and can check if variables meet certain conditions. It will generate
a warning or error with a customizable message.

The `variables` plugin is required and must have `expose: true`. Add the plugin with
```yaml
config:
- plugins:
  - use: variables # required and must be before assertion
    with:
      expose: true # required, otherwise we don't know the values
  - use: assertion 
    with:
      ... # see below on how to configure the conditions
```

## Configuration
The `with` property should be a mapping with an `assert` property. `assert` should be
an array of checks to run. For example:
```yaml
- use: assertion
  with:
    assert:
    - check:
      - x: .gt(50) # checks if x > 50
      - x: .lt(70) # checks if x < 70
      type: warning # could be `warning` or `error`
      message: Assertion failed! X must be between 50 and 70 (exclusive)!
```
The `check` property has a structure similar to `vars` in the [Variables Plugin](./variables.md).
It can be a single mapping, or an array of mappings. Use an array if you need to check multiple conditions
on the same variable, like the example above.

## Conditions
The conditions are also configured similar to the `vars` property.
However, the operations are different. The example above showed the `gt` (greater than)
and the `lt` (less than) comparators. There are 6 comparators in total:

|Comparator Tag|Example (numeric)|Example (variable)|
|-|-|-|
|`eq` Equal| `x: .eq(70)`|`y: .eq(x)`|
|`ne` Not Equal| `x: .ne(70)`|`y: .ne(x)`|
|`gt` Greater Than| `x: .gt(70)`|`y: .gt(x)`|
|`lt` Less Than| `x: .lt(70)`|`y: .lt(x)`|
|`ge` Greater Than or Equal To| `x: .ge(70)`|`y: .ge(x)`|
|`ge` Less Than or Equal To| `x: .le(70)`|`y: .le(x)`|
:::tip
The variable system (and computers in general, in most applications) does not represent decimals exactly. This may result in
inaccuracy with decimal comparison for `eq` and `ne`.
Therefore, `eq` will compare if 2 numbers are very close instead of exactly equal.
And `ne` will compare if 2 numbers are not `eq`.
:::

## Message Display
When the assertion fails on a step, it will display a message below the line when the assertion first fails.

If one assertion fails in consecutive lines, the message will only be displayed for the first line, until the assertion passes and fails again later,
in which case another message will be displayed.
