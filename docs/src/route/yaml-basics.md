# YAML Basics
:::tip
This is for people who have no prior experience with YAML. If you already know
how YAML works, you can skip this section
:::
Celer files are in the `yaml` language, which is a language that can describe
data in a human-readable format. For example, a `yaml` file can look like:
```yaml
name: John Doe
age: 30
location: New York
email: johndoe@example.com
is-student: false
```

This section will help you understand the basics of YAML syntax. Feel free
to come back here when you don't understand code snippets in later sections.

## Mapping (Dictionary/Object)
The example above is an example of a **mapping**. A mapping maps property
names to values. In the example above, the property `name` has value `Johe Doe`,
the property `age` has value `30`, etc.

You may also know mapping as dictionary or object from other programming language concepts.

## Sequence (List/Array)
Another common data structure you need when developing Celer routes is **sequence**.
A sequence is an ordered list of things, for example:
```yaml
- apple
- banana
- orange
- mango
- kiwi
```
This would be a list of 5 fruits.

You can also specify sequences with `[` and `]`, for example `[1, 2, 3]`.
This is commonly used to specify coordinates in Celer.

## Other Data Types
Besides mappings and sequences, there are other data types: `boolean`, `number` and `string`.
### Boolean
The `boolean` type is either `true` or `false`. You can also use `yes` or `no`.
For example, the `is-student` property in the example at the top can be written as
```yaml
is-student: no
```

### Number
If you put a valid number like `30` or `0.5` or `-100`, it will be interpreted as a number.
For example, the `age` property in the example at the top is a number.

### String
`string` is the programming term for "text". For example, `John Doe`, `New York` and 
`johndoe@example.com` are all strings. You can also put quotes around text to force it
to be a string:
```yaml
foo: "30"
```
Without the quotes, `foo` would be a number.

## Nested Data
YAML use indentation to signify nesting, for example:
```yaml
person:
  name: Alice
  age: 28
  address:
    street: 123 Main St
    city: Wonderland
    country: Fantasyland
  hobbies:
  - reading
  - hiking
  - painting
  friends:
  - name: Bob
    age: 30
  - name: Carol
    age: 29
```
Here, if the properties are the same indentation (same number of spaces from beginning of line),
then they are considered to be in the same mapping.

On the root level, there's one property `person`, and the value is a mapping with 5 properties:
- `name` - a string `Alice`
- `age` - a number `28`
- `address` - a mapping with 3 properties: `street`, `city`, and `country`
- `hobbies` - a sequence with 3 values
- `friends` - a sequence where each value is a mapping with properties `name` and `age`
:::tip
When writing YAML, You cannot mix spaces and tabs for indentation. Commonly people
use 2 spaces for indentation. Make sure your editor is not inserting tabs for you.
:::

## Comments
You can add comments after the `#` sign in a line. Comments are notes to yourself
and will not alter the data
```yaml
# Simple YAML example with comments
name: John       # Person's name
age: 25          # Person's age
city: New York   # City of residence
interests: [music, sports]  # List of interests
employed: true   # Employment status
```

Because of this, when you need to put a string that starts with `#`, you need to quote it.
For example, when specifying a color by hex string:
```yaml
color: "#ff8800"
```
