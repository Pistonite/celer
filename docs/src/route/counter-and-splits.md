# Counter and Splits
The counter is a small block in front of a line in the document that can display
text (usually counter for an objective). It is also used to set up splits for speedrunning.

## Configuring a Counter
To configure the style of the counter block, define a new tag:
```yaml
config:
- tags:
    my-counter:
      color: black
      background: orange
```
Then, use the `counter` property and this tag to add a counter:
```yaml
route:
- Example Section:
  - Line with counter:
      counter: .my-counter(woooo)
```
![image of example](https://cdn.discordapp.com/attachments/951389021114871819/1180399432722821171/image.png?ex=657d47a3&is=656ad2a3&hm=9f05ea0f94c21925c9b5d461b5b890f983d3106d229bb46bdd9bef8ef5e322c7&)
:::tip
Counters can also have real functionality - automatically increment and display the count of something.
Check out the [Variable Plugin](../plugin/variables.md) for how to do that.
:::

## Splitting
Counters are also used for configuring split types for speedruns. Let's turn the
`my-counter` tag into a split type!

First, add a description for the split type in the tag definition. This will tell
Celer that you want to use this tag for splitting.
```yaml
config:
- tags:
    my-counter:
      color: black
      background: orange
      split-type: My Counters # <- add this
```

You can see what split types are available in the document in the settings:
1. Click on <FluentIcon name="Settings20Regular" /> `Settings` in the Toolbar.
2. Choose the <FluentIcon name="Document20Regular" /> `Document` category.
3. Scroll to the `Splits` section.
![image of split settings](https://cdn.discordapp.com/attachments/951389021114871819/1180401223254413352/image.png?ex=657d494e&is=656ad44e&hm=5d7fadf0a7d41a3141d310dc207f79e4ff41465064e5cfb7fd036e108561e0b9&)

Checking the `My Counters` checkbox will enable splitting on counters tagged with the `my-counter`
tag.
:::tip
You can give the same `split-type` to multiple tags. Checking the box will enable splitting on all tags with that name.
:::

## Default Split Types
The split settings allow people who are viewing your route decide themselves where
they want to split. As the route maker, you can also define a set of splits that should be
on by default using the `splits` config:

```yaml
config:
- splits:
  - my-counter
```

Now, go to the split settings and click on `Reset`. You should see the `My Counters` type getting checked by default.

## Export Split Files
See [Export](../export.md)

## Format Split Names
:::tip
This section explains how split names are set manually with the `split-name` property.
Most of the time, it's easier to use the [`split-format`](../plugin/split-format.md) plugin
to set this property automatically based on the split type.
:::
By default, the split names in the exported split files are the same as the primary text of the line.
You can change it with the `split-name` property.

```yaml
route:
- Example Section:
  # This split will have name "Split 1"
  - Split 1: 
      counter: my-counter
  # This split will have name "Split 2"
  - Some text:
      counter: my-counter
      split-name: Split 2
```

The [`variables`](../plugin/variables.md) plugin can be used to interpolate variables into the split name

```yaml
config:
- plugins:
  - use: variables

route:
- Example Section:
  # This split will have name "Split 1"
  - Split 1: 
      counter: my-counter
      vars:
        x: 1
  # This split will have name "x is 1"
  - Some text:
      counter: my-counter
      split-name: x is .var(x)
```
