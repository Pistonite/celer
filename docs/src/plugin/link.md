# Link Plugin
:::tip
The plugin system is currently unstable.
:::
The `link` plugin transforms `link` tags in texts into clickable link.

Add the plugin with
```yaml
config:
- plugins:
  - use: link
```

## Examples
Add clickable links in primary, secondary, or note section of a line
```yaml
config:
- plugins:
  - use: link
route:
- Example Section:
  - Link in the notes:
      notes: .link(https://example.com)
  - Link in the comment:
      comment: .link(https://example.com)
  - .link(https://example.com)
```
Override the link text by putting it in `[` and `]`
```yaml
config:
- plugins:
  - use: link
route:
- Example Section:
  - Example Line:
      # Text will show "click here to see the notes", with "here" clickable
      notes: Click .link([here]https://example.com) to see the notes
```
Override the link style by defining the `link` tag in config

```yaml
config:
- plugins:
  - use: link
  tags:
    link:
      italic: true # show links in italic
route:
- Example Section:
  - Example Line:
      # "here" will be clickable and will be italic
      notes: Click .link([here]https://example.com) to see the notes
```
:::tip
The link already inherits styles from the Celer UI (with a different color and underline).
Only override the style if needed!
:::
:::warning
Specifying `underline: false` will probably have no effect because Celer UI renders
links with underlines.
:::
