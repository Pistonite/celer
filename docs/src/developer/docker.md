# Deploy Celer using the official Docker image
You can host Celer yourself using its official Docker image [ghcr.io/pistonite/celer](https://github.com/Pistonite/celer/pkgs/container/celer).
The container includes everything about Celer: server, client (view and editor), and the docs.

## Environment variables
You must provide the following environment variables to the container for all features to work properly:
| Variable | Description |
| --- | --- |
|`CELERSERVER_SITE_ORIGIN` |The origin of the site, such as `http://example.com`.|

See the server documentation for additional environment variables you can set.

## HTTPS
Note that the official image does not contain any certificate or keys you need to enable HTTPS.
However, the server supports HTTPS through environment variables. Please see the server documentation for more information.