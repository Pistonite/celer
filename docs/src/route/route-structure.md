# Route Structure
A route is basically a sequence of instructions grouped into sections.
The route structure in Celer mimics this pattern.

On a high level, a route is a sequence of Sections, and each Section is a sequence of Lines.

## Entry Point
Your entire route is loaded from the `route` property in `project.yaml`.

It is strongly recommended to put a single `use` there,
that references another file as the entry point to the route.
For example:
```yaml
# project.yaml
route: 
  use: ./main.yaml
```
This is because Celer sometimes only loads `project.yaml` to grab necessary
metadata for the route, such as the name and version. Having a small
`project.yaml` can make the loading faster those scenarios. The compiler also
utilizes `project.yaml` for caching.

Examples below will assume that the route is loaded from `./main.yaml`

## Sections
The `route` property is a sequence of Section objects. For example:
```yaml
# main.yaml
- Section 1:
  - do this
  - do that
- Section 2:
  - do these
  - do those
```
:::warning
You cannot have steps (lines) outside a section. It was possible with
the old Celer but not anymore. However, you can have a preface
before the first route section. See [Preface](#preface) below.
:::

You can use the `use` property for one or more sections to load part
of your route from another file:
```yaml
# main.yaml
- Section 1:
    ... # defail not shown
- use: ./section-2.yaml
- Section 3:
    ... # detail not shown
- use: ./section-4-5.yaml

# section-2.yaml
- Section 2:
  - do this
  - do that

# section-4-5.yaml
- Section 4:
    ... # detail not shown
- Section 5:
    ... # detail not shown
```
:::warning
The file you load should define a sequence of sections, even
if it only contains 1 section, like `Section 2` in the example.
:::

## Preface
The route can include a preface section before the first section. This is 
a replacement of the Banner feature in the old Celer format.
```yaml
# main.yaml
- "This is the route for XXX, you may want to follow this video: .link(https://youtube.com/XXX)"
- Be careful of XXX, YYY and ZZZ when you do the run
- Section 1:
    ...
```
Each element (i.e. text after `-`) specified before the first section will be rendered as one paragraph.

Rich text formatting is supported here and the example above uses the [Link plugin](../plugin/link.md) to display a clickable link.
See [Customizing Text](./customizing-text.md) for more details on the Rich text system.
:::tip
Note that the first paragraph is surrounded by quotes. This is because there is a `:` in the
text, and YAML will treat it as a mapping without the quotes.
:::

You can load preface text with the `use` property as well, similar to regular sections:
```yaml
# main.yaml
- use: ./preface.yaml
- Section 1:
    ...

# preface.yaml
- just an example
- of a preface with 2 paragraphs
```

## Lines
Each section in the route is a sequence of line objects. The simplest form of a line is just plain text:
```yaml
# main.yaml
- Example Section:
  - this is a line
  - this is another line
```

Just like sections, you can load one or more lines from another file with the `use` property:
```yaml
# main.yaml
- Example Section:
  - this is a line
  - use: ./some-lines.yaml
  - this is the last line

# some-lines.yaml
- Here are
- 3
- lines
```
The resulting `Example Section` will have 5 lines, with the 3 lines from `some-lines.yaml`
placed in the middle, where the `use` is at.

A line can also be customized with extra properties, like icons, notes, and custom styles:
```yaml

# main.yaml
- Example Section:
  - line with custom icon:
      icon: example-icon
```
See [Customizing Lines](./customizing-lines.md) for details.

