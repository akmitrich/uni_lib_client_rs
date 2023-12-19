# UniMRCP library for client software in Rust
## Build
For succesful building you must have UniMRCP software installed on your machine including APR library with unimrcp modifications.
`build.rs` expects that 
- `UNIMRCP_PATH` variable is the path to unimrcp library including `lib/` and `include/` subfolders. By default it is `/usr/local/unimrcp` if you installed UniMRCP from source.
- `APR_LIB_PATH` variable is the path to `lib/` subfolder of the APR with unimrcp modifications. By default it is `/usr/local/apr` if you installed UniMRCP from source.
- `APR_INCLUDE_PATH` variable is the path to `include/` subfolder of the APR with unimrcp modifications. By default it is `/usr/local/apr` if you installed UniMRCP from source.

```
cargo build --release
```
will take care of everything else:)

## Maybe this crate should be named
`unimrcpclient-sys`
but it is what it is;)
