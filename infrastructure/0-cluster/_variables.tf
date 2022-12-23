variable "email" {
  type    = string
}

variable "linode_region" {
  type    = string
}

variable "cluster_name" {
  type = string
  description = "Linode LKE Cluster Label"
}

variable "cluster_pool" {
  default = {
    type  = "g6-standard-1"
    count = 1
  }

  description = "todo: make this support autoscaling and multiple pools"
}

variable "linode_token" {
  type = string
  description = "keep secrets in a `secrets.auto.tfvars` file"
}
