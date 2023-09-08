# ![](./assets/celer-3-small.svg) celer
Celer Route Engine

# Dev Setup
need npm, cargo, docker
use [`just`](https://github.com/casey/just) script runner
```
cargo install just
just install
```

## HTTPS
To enable HTTPS for local testing. Follow https://vmsetup.pistonite.org/tool/cert
to create a certificate `.pfx` file. Then use `openssl` to extract the cert and key and put them
in `/cert/cert-key.pem` and `/cert/cert.pem` (relative to repo root).

Once done, vite dev server for web-client should run in https.
