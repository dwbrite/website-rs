terraform {
  required_providers {
    linode = {
      source  = "linode/linode"
      version = "1.28.0"
    }
  }
}

provider "linode" {
  token = var.linode_token
}
