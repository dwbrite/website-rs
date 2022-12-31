variable "linode_token" {
  type = string
}

variable "email" {
  type    = string
}

variable "root_domain" {
  type    = string
}

variable "linode_region" {
  type    = string
}

variable "linode_bucket_region" {
  type    = string
}

variable "container_registry_username" {
  type = string
}

variable "container_registry_password" {
  type = string
}

variable "kubernetes_backend" {
  type = map
}

variable "linode_token_dns" {
  type = string
  description = "linode API token strictly for accessing DNS records, used for the ACME solver"
}
