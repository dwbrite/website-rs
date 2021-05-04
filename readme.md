# dwbrite.com, but this time in rust

### requirements:
- `build-essential`
- `pkg-config`
- `openssl +/ libssl-dev (on linux)`
- `rustup / rust nightly`

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
`nohup cargo run --bin media-dwbrite-com &`  
wait... then  
`nohup cargo run --bin dwbrite-com &`  
