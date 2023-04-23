# dwbrite.com, but this time in rust

website-rs consists of a media server, the [dwbrite.com](https://dwbrite.com) frontend server, and the IaC to build and deploy them.
The IaC here also includes services like nginx-ingress, cert-manager, Matrix (chat), Keycloak.

## requirements

```
- rustup + rust (nightly)
- openssl-devel
```

## running locally

`cd dwbrite-com && cargo run` (0.0.0.0:41234)   
`cd media-dwbrite-com && cargo run` (0.0.0.0:41233)

## containerization

`podman-compose build` to build the images  
`podman-compose push` to push the images. By default this will use the `latest` tag.


## infrastructure

In theory deploying is a simple `terraform apply` away after `export TF_VARS_linode_token=...` (and `export LINODE_TOKEN=...`)

DNS propagation can take a long time though so setting up ACME certs can be a "luck" based process...

### requirements:

```
- build-essential
- pkg-config
- terraform
- helm
- podman, podman-compose
```
