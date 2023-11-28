# File Structure

All your files in a Celer project should be put in one folder (or subfolders).
Files in the project can reference other files, or an external file on GitHub,
by using the `use` property in many places, examples include:

- Loading an external configuration
- Separating the route into multiple files
- Loading an icon 

Check the corresponding documentation page for more details.

## The `use` property

The `use` property takes a file path as a string. You can specify the path as a relative, absolute, or GitHub reference.

### Relative Path

Relative path should start with `.` or `..`, and is resolved relative to the (directory of the) current file.

Examples:
```yaml
# File is foo/bar/a.yaml
- use: ./b.yaml # resolves to foo/bar/b.yaml
- use: ../c.yaml # resolves to foo/c.yaml
```
:::tip
Windows style (`\`) separator works, but it's recommended to not use it because you need to escape it.
:::
:::warning
Trying to access the parent of the "root" of the project will result in an error. See [Absolute Path](#absolute-path)
below for what "root" is.
:::

### Absolute Path

Absolute paths are resolved relative to the "root" of the project:

- If you are using the web editor, the root is the folder that's loaded in the editor
- In a GitHub context, the root is the repository root.

For example, consider the following directory structure:
``` 
.
├── parts/
│   ├── a.yaml
│   └── subfolder/
│       └── b.yaml
└── project.yaml
```

Suppose `b.yaml` want to include `a.yaml`, it could use either of the two options:
```yaml
# relative
use: ../a.yaml
# absolute
use: /parts/a.yaml
```

### GitHub Reference

If the path does not start with `.`, `..`, or `/`, it will be considered a GitHub reference, in which case it should be formatted as:
```
{owner}/{repo}/{path/to/file}:{ref}
```
The `:{ref}` can be a branch, tag, or commit hash. 
It is optional and when omitted, it will resolve to the `main` branch.

Examples:
```yaml
# The following resolves to https://raw.githubusercontent.com/Pistonite/celer/main/README.md
# The path to view on GitHub is https://github.com/Pistonite/celer/README.md
- use: Pistonite/celer/README.md
# View giz/file.yaml in the foo/bar repo on the `test` branch
- use: foo/bar/giz/file.yaml:test
```
:::tip
When a file loaded from a GitHub reference uses a relative or absolute reference,
it will resolve to the same branch/reference as that file.
:::

## Multiple projects in the same repo
Celer also supports putting multiple projects in the same repository/directory (commonly referred to as a monorepo).
You may want to do this if you want to have shared configurations across projects.

### Example
Here is one way to organize files in the monorepo:
```
.
├── common/
│   └── config.yaml
├── example1/
│   ├── project.yaml
│   └── main.yaml
├── example2/
│   ├── project.yaml
│   └── main.yaml
└── project.yaml
```
```yaml
# /example1/project.yaml
title: example 1
version: 1.0
route:
  use: ./main.yaml
config:
- use: /common/config.yaml

# /example2/project.yaml
title: example 2
version: 1.0
route:
  use: ./main.yaml
config:
- use: /common/config.yaml

#/project.yaml
entry-points:
  example1: /example1/project.yaml
  example2: /example2/project.yaml
```

### `entry-points`
To configure a monorepo, the root `project.yaml` needs to define a `entry-points` property that maps entry points
to the `project.yaml` of the project. The path must be an absolute path
```yaml
# /project.yaml
entry-points:
  example1: /example1/project.yaml
  example2: /example2/project.yaml
```
:::tip
The root `project.yaml` doesn't have to contain other properties like `title` and `version` that usually sit inside
`project.yaml`. These properties are ignored if they do exist.
:::
The entry point for each project doesn't have to be named `project.yaml`. For example, you can also set up the directory like this:
```
.
├── common/
│   └── config.yaml
├── example1.yaml
├── example2.yaml
└── project.yaml
```
```yaml
#/project.yaml
entry-points:
  example1: /example1.yaml
  example2: /example2.yaml

#/example1.yaml
title: Example 1
... # detail not shown

#/example2.yaml
title: Example 2
... # detail not shown
```

### Aliasing
You can also specify another entry point in the place of the project path. This allows you to have multiple names (aliases) for the same project, which is useful for versioning and other things.
```yaml
#/project.yaml
entry-points:
  hundo-latest: hundo-v4
  hundo-v4: hundo-v4.1
  hundo-v4.1: /hundo-v4.1.yaml
```
In the example above, all the entry points point to the same project, but the router can distribute an URL with the 
"latest" version, and update the route later without redistribution.
:::warning
There is a maximum depth of aliasing to prevent infinite recursion.
:::

### Default
You can also specify a special entry point named `default`. The compiler will be redirected
to that entry point if it is given the root project as entry path. `default` can be either an alias
or a path like other entry points.
```yaml
#/project.yaml
entry-points:
  default: my-project
  my-project: /path/to/project.yaml
```

### Choose entry point
To switch between entry points in the web editor:

1. Load the monorepo directory (not subdirectories) in the web editor
2. Click on `Settings` in the toolbar, then go to the `Editor` category.
3. Under the `Compiler` section, find `Entry point` settings and change the entry point.
4. Close the settings dialog, and the document should reload to use the new entry point.
