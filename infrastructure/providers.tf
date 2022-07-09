terraform {
  required_providers {
    linode = {
      source  = "linode/linode"
      version = "1.28.0"
    }

    tls = {
      source  = "hashicorp/tls"
      version = "3.4.0"
    }

    acme = {
      source  = "vancluever/acme"
      version = "2.9.0"
    }

    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.12.0"
    }

    helm = {
      source  = "hashicorp/helm"
      version = "2.6.0"
    }
  }
}
