# ![](./web-client/public/static/celer-3-small.svg) celer
Celer Route Engine

# Dev Setup
need npm, cargo,
use [`task`](https://taskfile.dev) for command runner

## HTTPS
To enable HTTPS for local testing. Follow https://vmsetup.pistonite.org/tool/cert
to create a certificate `.pfx` file. Then use `openssl` to extract the cert and key and put them
in `/cert/cert-key.pem` and `/cert/cert.pem` (relative to repo root).

Once done, vite dev server for web-client should run in https.

## Local build
If there are issues running `task build`, try:
```
cargo clean
task build:server
task build
```
(Typically need to do this after rustup update)
