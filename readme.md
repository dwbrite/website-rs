# dwbrite.com, but this time in rust

## containerizing

`podman-compose build` to build the images

`podman run dwbrite-com`

``

## architecture

dwbrite.com is a multi-part server with several moving parts.

Terraform sets a kubernetes cluster and the domain, 
then some "bootstrap" pods: `nginx-ingress` and `docker-registry`

In theory deploying is a simple `terraform apply` away after `export TF_VARS_linode_token=...` (and `export LINODE_TOKEN=...`)

DNS propagation can take a long time though so setting up ACME certs can be a "luck" based process...


### requirements:
- `build-essential`
- `pkg-config`
- `openssl +/ libssl-dev (on linux)`
- `rustup / rust nightly`

- `terraform`
- ``


recommended: 

- `nginx`
- `certbot`
- `python3-certbox-nginx`
- `apache-utils`

### nginx + https
```
server {
    server_name dwbrite.com;
    listen 80;
    location / {
        proxy_pass http://127.0.0.1:41234;
    }
}

server {
    server_name media.dwbrite.com;
    listen 80;

    location / {
        proxy_pass http://127.0.0.1:41233;
    }
    
    location /upload {
        auth_basic "o hej me";
        auth_basic_user_file /etc/nginx/.htpasswd;
        
        proxy_pass http://127.0.0.1:41233;
    }
}
```

start with simple nginx routes, then run `certbot --nginx`

## running dwbrite.com

`cargo run --bin media-dwbrite-com` (port 41233)  
`cargo run --bin dwbrite-com` (port 41234)

### with nohup

`killall dwbrite-com`  
`killall media-dwbrite-com`  
`cd media-dwbrite-com & nohup cargo run --bin media-dwbrite-com &`  
wait... then  
`cd dwbrite-com & nohup cargo run --bin dwbrite-com &`  
