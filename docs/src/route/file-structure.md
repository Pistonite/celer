# File Structure
All your files in a Celer project should be put in one folder (or subfolders).
Files in the project can reference other files, or an external file on GitHub,
by using the `use` property in many places, examples include:

- Loading an external configuration
- Separating the route into multiple files
- Loading an icon 

Check the corresponding documentation page for more details.

## The `use` property
The `use` property takes a file path as a string. You can specify the path as a relative, absolute, or github reference.

### Relative Path
Relative path should start with `.` or `..`, and is resolved relative to the (directory of the) current file.

Examples:
```yaml
# File is foo/bar/a.yaml
- use: ./b.yaml # resolves to foo/bar/b.yaml
- use: ../c.yaml # resolves to foo/c.yaml
```
:::tip
Do not use Windows style (`\`) separator
:::

### Absolute Path
Absolute path can only be used in a route context (i.e. where `project.yaml` exists).
It is resolved relative to the directory of `project.yaml`.

For example, if `a.yaml` includes a `b.yaml` from GitHub, if `b.yaml` references an absolute path, it will be an error, because `b.yaml` is not in a route context.
However, if `a.yaml` includes a `c.yaml` from the same project, `c.yaml` can reference an absolute path in the same project.

```yaml
# project.yaml is foo/project.yaml
# File is foo/bar/a.yaml
use: /biz/d.yaml # resolves to foo/biz/d.yaml
```

### GitHub Reference
If the path does not start with `.`, `..`, or `/`, it will be considered a GitHub reference, in which case it should be formatted as:
```
{owner}/{repo}/{path/to/file}:{ref}
```
The `:{ref}` can be a branch, tag, or commit hash. It is optional and when omitted, it will resolve to the default branch of that repo.

Examples:
```yaml
# The following resolves to https://raw.githubusercontent.com/Pistonite/celer/main/README.md
# The path to view on GitHub is https://github.com/Pistonite/celer/README.md
- use: Pistonite/celer/README.md
# View giz/file.yaml in the foo/bar repo on the `test` branch
- use: foo/bar/giz/file.yaml:test

