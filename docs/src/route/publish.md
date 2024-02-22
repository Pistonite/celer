# Publish the Route
To publish or share your route, you need to upload it to [GitHub](https://github.com). 
Everyone can then view it on Celer.
:::tip
You need a GitHub account for this. If you are already familiar with Git, you can skip to the bottom of the page which tells you how to view the route
once uploaded.
:::

If you aren't familiar with git, don't worry. This page will guide you through all the steps. The general idea is:

1. You create a so-called repository (repo for short) on GitHub that will store the project.
2. You create a folder on your PC that is linked to the repository on GitHub. This process is known as "clone".
3. You move your project files inside the cloned repository.
4. You upload those files to the repository on GitHub. This process is known as "push".
5. Make future updates in the local repository and push again to upload the changes.


## Creating the repository
1. Go to https://github.com/new to start creating a new repository
2. Enter a name under `Repository name`. Note the following:
   - It is the best for your repository name to only contain alphanumeric characters (`a-z`, `A-Z` and `0-9`), `_` and `-`. Special characters like `%` or unicode characters will cause inconvienience.
3. Enter a description (e.g. "My Awesome Celer Project")
3. Make sure the new repository will be `public`. Celer cannot access your private repositories.
4. Check "Add a README file".
5. Click "Create repository"

## Cloning the repository
You can either clone the repository with the `git` CLI tool, or use a GUI tool like GitHub Desktop.
It is recommended to use GitHub Desktop if you don't know how or aren't comfortable running commands in a terminal.

### With GIT Command line tool
:::tip
If you are on windows, you can install `git` [here](https://git-scm.com/download/win)
:::
With `git` installed, run the following command in the directory where you want to store all your repos.
Replace `YOUR_USER_NAME` with your GitHub username and `YOUR_REPO_NAME` with the name you entered in step 2 above.

```bash
git clone git@github.com:YOUR_USER_NAME/YOUR_REPO_NAME
```

### With GitHub Desktop
Install GitHub Desktop from [here](https://desktop.github.com/). Then open it and sign in with your GitHub account.

1. Click on "Clone a repository from the internet"
2. Search for the repository you just created
3. Choose a path on your PC where you want the repo to be cloned.
4. Click "Clone"

## Move your project inside the repository
Once the repo is cloned, you should see a directory `.git` inside it on your PC and a `README.md` file.

You can simply copy and paste the project files you have been editing to the repository. The `project.yaml`
file should be at the root, next to the `README.md` file.

## Push your files
### With GIT Command line tool
First stage your changes
```bash
git add .
```
Then commit them with a message
```bash
git commit -m "example message"
```
Then push the changes
```bash
git push
```

### With GitHub Desktop
1. In GitHub Desktop, it should show the local changes you made in the "Changes" panel on the left.
Select the files you want to upload.
2. At the bottom of the changes panel, enter a short message describing the change. This is known as a commit message.
3. Click "Commit to main"
4. Now the changes panel should say "0 changed files"
5. On the top, you should see something like this:
   ![image of push origin](https://cdn.discordapp.com/attachments/951389021114871819/1209290318076444723/image.png?ex=65e6625f&is=65d3ed5f&hm=dab6cefc2abbd3f7796c8298cdb50bd6299dbf4c39ef30af5f8452e86cc43bba&)
6. Click that, and your commits are now uploaded. You can go to the repository on GitHub to confirm.

## Viewing the route on Celer
To view the route on celer, go to the URL below. Replace the placeholders with your GitHub username and repo name
```
scheme://celer.placeholder.domain/view/YOUR_USER_NAME/YOUR_REPO_NAME
```

### Viewing entry point
If you configured the project as a monorepo with the `entry-points` property, as described [here](./file-structure.md),
the URL above will take you to the `default` entry point. You can add an entry point to the URL to view a particular entry point.

For example, say your root `project.yaml` has:
```yaml
entry-points:
  my-sub-project: /path/to/project.yaml
```
You can view `my-sub-project` as
```
scheme://celer.placeholder.domain/view/YOUR_USER_NAME/YOUR_REPO_NAME/my-sub-project
```

You can also refer to the target `project.yaml` directly:
```
scheme://celer.placeholder.domain/view/YOUR_USER_NAME/YOUR_REPO_NAME/path/to/project.yaml
```

### Viewing specific branch/commit/tag
You can add `:BRANCH` to the end of the URL to view the route at a particular branch, commit, or tag. The default is the `main` branch when you don't specify one.

For example, let's say you created a branch `v1.2`, you can refer to this branch as
```
scheme://celer.placeholder.domain/view/YOUR_USER_NAME/YOUR_REPO_NAME:v1.2
scheme://celer.placeholder.domain/view/YOUR_USER_NAME/YOUR_REPO_NAME/ENTRY_POINT:v1.2
```

This can be useful for versioning your route
