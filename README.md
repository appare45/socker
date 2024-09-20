# Socker

A tiny container runtime currentry implementating the OCI runtime-spec in Rust, similar to runc.

## How to try

1. Build socker by yourself
2. Prepare rootfs
```bash
docker export $(docker create ubuntu) | tar -C rootfs -xvf -
```
3. Run socker
```bash
socker config.json
```

## How to build

```bash
git clone https://github.com/appare45/socker.git
cd socker
cargo build --release
```
