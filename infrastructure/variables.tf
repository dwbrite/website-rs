variable "linode_token" {
  type = string
}

variable "email" {
  type    = string
  default = "dwbrite@gmail.com"
}

variable "root_domain" {
  type    = string
  default = "dwbrite.com"
}

variable "linode_region" {
  type    = string
  default = "us-east"
}

variable "linode_bucket_region" {
  type    = string
  default = "us-east-1"
}
