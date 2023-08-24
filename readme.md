# dwbrite.com, but this time in rust

website-rs consists of a media server and a frontend server, used for hosting [dwbrite.com](https://dwbrite.com).

### requirements:

```
- rustup + rust (nightly)
- openssl-devel (no longer needed?)

- podman*
- podman-qemu-something*?
```

## running locally

`cd dwbrite-com && cargo run` (0.0.0.0:41234)   
`cd media-dwbrite-com && cargo run` (0.0.0.0:41233)

## containerization

at project root

`podman build -f <subproject>/Dockerfile .` to build the images  
`podman push` to push the images. By default this will use the `latest` tag.
