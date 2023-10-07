# Getting Started
:::info
Page under construction
:::
To get started with creating a route in Celer, first create an empty folder
to put all the project files. Then create a file called `project.yaml` with
the following content:
```yaml
# project.yaml
title: My Project
version: 1.0.0
route: 
  use: ./main.yaml
config: []
```
:::tip
The file names are case-sensitive. This file should be named `project.yaml`,
not `Project.yaml` or `project.yml`
:::
Then, create a `main.yaml` file in the same folder with the following content:
```yaml
- hello world!
- Section 1:
  - Step 1
```
