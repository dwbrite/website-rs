# Infrastructure-as-code

infrastructure is an onion, with layers from 0 to ?

TL;DR: cluster -> bootstraps (certs, load-balancer/ingress, container_registry) -> applications

gitignored `secrets.auto.tfvars` is used to store sensitive vars. I'll probably throw those in keepass.

gitignored `xyz.tfbackend` is used to store sensitive backend, since Linode isn't automagical like AWS (uwu)

you'll need to run `terraform init -backend-config=xyz.tfbackend` before applying a root module

