# Server APIs
This is the `v1` server API that can be accessed at
```
scheme://celer.placeholder.domain/api/v1
```
For example:
```
scheme://celer.placeholder.domain/api/v1/version
```

## `GET /version`
Gets the server build version
### Returns
|Status|When|Response|
|-|-|-|
|`200`|OK| Version string, such as `0.0.4-alpha fccf5909de655bc2c81ccd8ed8001111a753bbf5`. The hash is the GitHub commit that the image is built from.|

### Example
Request
```
GET scheme://celer.placeholder.domain/api/v1/version
```
Response
```
0.0.4-alpha fccf5909de655bc2c81ccd8ed8001111a753bbf5
```

## `GET /compile/{owner}/{repo}[/{entry_point}][:{ref}]`
Compiles the document and returns an ExecDoc in JSON
### Parameters
|Name|Description|
|-|-|
|`owner`|Owner of the GitHub repo to pull the route from|
|`repo`|The GitHub repo name (`owner/repo` makes up the repo)|
|`entry_point`| (Optional) An entry point defined in the root `project.yaml`. When omitted, it uses the `default` entry point if defined, or the root `project.yaml` itself.|
|`ref`| (Optional) The branch, tag, or commit for the repo to pull the route from. When omitted, defaults to `main`|
### Returns
|Status|When|Response|
|-|-|-|
|`200`|OK|Route compiles without internal errors (errors during compilation will show in the document)|
|`404`|Project path is invalid|Empty|
