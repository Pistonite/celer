# Variables
:::tip
The plugin system is currently unstable.
:::
The `variables` plugin adds a variable system. Variables are numbers
that can be manipulated through addition, subtraction, multiplication
and division. Those numbers can then be interpolated into the text
or used as counters.

Add the plugin with
```yaml
config:
- plugins:
  - use: variables
```
See [below](#advanced-configuration) for advanced configurations

## Manipulation
At each step of the route, you can change the variables with the `vars` property
```yaml
route: # The main route array
- Section 1:
  - Change some variables!:
      vars:
        x: 1 # change x to 1
        y: .add(2) # add 2 to y
        z: .sub(y) # subtract y from z
```
### Operations
Variables are manipulated using the [Rich Text](../route/customizing-text.md) system with special tags.
These tags don't change how the text is displayed.

There are 5 operations available to use in the `vars` property.
These operations are *unary*, which means they take 1 argument and apply the change to the variable directly.

For example:
```yaml
- Change x to 4:
    vars:
      x: 4
- Add 1 to x:
    vars:
      x: .add(1) # x is now 5
```
:::tip
When the variable is first referred, it will be initialized to 0
:::

List of available operations
|Operation|Example (numeric)|Example (variable)|
|-|-|-|
|Addition|`x: .add(1)`|`x: .add(y)`|
|Subtraction|`x: .sub(1)`|`x: .sub(y)`|
|Multiplication|`x: .mul(1)`|`x: .mul(y)`|
|Division|`x: .div(1)`|`x: .div(y)`|
|Assignment|`x: 1`|`x: .var(y)`|
:::warning
Division is floating point division instead of integer. `5 / 2 = 2.5`!
:::

### Sequential Execution
Operations in the `vars` property are not sequential. For example, you could do:
```yaml
- Swaps x and y:
    vars:
      x: .var(y)
      y: .var(x)
```
If this were sequential, x will be first assigned the value of y, and the second operation will have no effect.

However, sometimes you do want to have more complex calculations that require sequential execution.
In that case, you can supply an array of operations to `vars` instead of a single mapping:
```yaml
- x = 3x + 5:
    vars:
    - x: .mul(3)
    - x: .add(5)
```
In this example `.mul(3)` will be executed first, then `.add(5)`, making `3x + 5` the new value of `x`

### Temporary
Assume you need to do assign `3x + 5` to `x` like the example above, but
you also want to assign `3x + 7` to `y` at the same time.
You could do this:
```yaml
- x = 3x + 5 then y = x + 2:
    vars:
    - x: .mul(3)
    - x: .add(5)
    - y: .var(x) # new x is 3x + 5
    - y: .add(2) # y is now 3x + 7!
```
This example is simple enough to be rearranged to use no temporary variable. However,
it isn't always possible to do so (or the result maybe REALLY complicated). In this case,
you could use a temporary variable:
```yaml
- x = 3x + 5 and y = 3x + 7:
    vars:
    - _t: .var(x)
    - _t: .mul(3) # _t is 3x
    - x: .var(_t)
      y: .var(_t)
    - x: .add(5)
      y: .add(7)
```
Variable prefixed with `_` are considered temporary (local) to the step. They will be deleted
after the current step is executed.
:::warning
The sequential and temporary system is meant to make the system as flexible as possible.
However, if you are adding really complicated logic, consider using a JavaScript plugin instead.
The JavaScript plugin system is still pending and is tracked by TODO [#24](https://github.com/Pistonite/celer/issues/24)
:::

## Displaying a variable
To display a variable, use the `.var` tag in anywhere that accepts Rich Text:
```yaml
route:
- Example Section:
  # using variable in the primary text
  - I have .var(apple) apples
  - Number of wood:
      comment: .var(wood) # in secondary text
      notes: I really have .var(wood) # in notes
```
See [Customizing Lines](../route/customizing-lines.md) for all the properties that accept Rich Text.

Internally, the `var` tag will be replaced with the `val` tag and the actual value of the variable.
For example, if `wood` is 3, `.var(wood)` will be turned into `.val(3)`.
To customize the style of interpolated variables, use the `val` tag
```yaml
# project.yaml
config:
- tags:
    val:
      bold: true # make all the variables bold
```
:::warning
`val` and `var` are different! `var` is shorthand for `variable` and `val` is shorthand for `value`
:::

### Counter
One special property is `counter`. When using `var` as the counter tag, it will automatically increment the variable,
**and** apply the counter tag instead of the `val` tag
```yaml
config:
- tags:
    counter-korok:
      color: green
route:
- Example:
  - Add a korok:
      counter: .var(counter-korok) 
        # this will add 1 to counter-korok variable
        # and change the counter to .counter-korok(1)
  - Add another korok:
      counter: .var(counter-korok) 
        # turned into .counter-korok(2)
```

### Formatting
Besides the `.var` tag, there are additional tags that can format the variable in different ways:

|Tag|Format|Example|
|-|-|-|
|`.var-roman`|Lowercase Roman numeral|`lxiii`|
|`.var-roman-upper`|Uppercase Roman numeral|`LXIII`|
|`.var-hex`|Lowercase Hexadecimal|`3f`|
|`.var-hex-upper`|Uppercase Hexadecimal|`3F`|
:::warning
The value will be rounded down for the conversion. If the conversion is not possible (say the number is negative),
then it will be treated as `.var`
:::

Additionally, you can pad the result to X characters or get the last X digits:
```yaml
# get the value of my-var (decimal), and pad the `_` character until
# it is 4 characters long
- .var(pad_4:my-var)  # if my-var is "378", the result is "_378"

# get the last 2 digits of my-var in hex
- .var-hex(last2:my-var) # if my-var is "378" (17A in hex), the result is "7A"

# get the last 2 digits of x, then pad it to 3 characters
# note the order of operation here.
- .var(pad03:last2:x) # if x is "7", then result is "007"
                      # if x is "317", then result is "017"
```
:::tip
The number is always rounded down to an integer before formatting with `pad` and `last`
:::
:::warning
If the variable name contains `:`, then you cannot use this to format.
:::

This is how the functions work exactly. `X` is any character and `ZZZZ...` is a non-negative integer
- `padXZZZZ...`: Left pad the result with character `X` until the text is `ZZZZ...` characters long
- `lastZZZZ...`: Get the last `ZZZZ...` characters of the result, or the whole result if it the length is less than `ZZZZ...`

## Advanced Configuration
Advanced configurations are available with extra properties:
```yaml
- plugins:
  - use: variables
    with:
      # give initial values to variables
      init:
        - x: 3
          y: 1
        - y: .add(x)
      # expose the variables to later plugins (default false)
      expose: true
```
### Initialization
The `init` property is basically a `vars` property that will be executed before the first step in the route

If `init` property is not given, all variables will have the value 0 when they are first used.

### Expose
If `expose` is `true`, the plugin will add a `vals` property to each line, containing
a mapping of variable name to variable value. Plugins can rely on this feature to
do extra things based on the variable values. 

<!--One example is the [Assertion Plugin](./assertion.md).-->
