# Server APIs
This is the `v1` server API that can be accessed at
```
scheme://celer.placeholder.domain/api/v1
```
For example:
```
https://celer.pistonite.org/api/v1/version
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

## `GET /compile/{owner}/{repo}/{ref}[/{path}]`
Compiles the document and returns an ExpoContext in JSON
### Parameters
|Name|Description|
|-|-|
|`owner`|Owner of the GitHub repo to pull the route from|
|`repo`|The GitHub repo name (`owner/repo` makes up the repo)|
|`ref`| The branch, tag, or commit for the repo to pull the route from. |
|`path`| (Optional) Either a path in the repo to the directory containing the `project.yaml`, or an alias defined in the `entry-points` of the root `project.yaml`. When omitted, it uses the `default` entry point if defined, or the root `project.yaml` itself.|
### Returns
It should always return status `200 OK`.

If the compilation is successful, it will return the following. `data` contains ExpoContext serialized to JSON.
```json
{
    "type": "success",
    "data": { ... }
}
```
Otherwise, it will return
```json
{
    "type": "failure",
    "data": "error message here"
}
```
