terraform {
  required_providers {
    linode = {
      source  = "linode/linode"
      version = "1.28.0"
    }

    helm = {
      source  = "hashicorp/helm"
      version = "2.6.0"
    }
  }
}


# Provider Configs #####################################################################################################

# TODO: get linode token from 0-cluster
variable "linode_token" {
  type = string
}

provider "linode" {
  token = var.linode_token
}

variable "kubernetes_backend" {
  type = map
}

data terraform_remote_state "kubernetes" {
  backend = "s3"
  config = var.kubernetes_backend
}

provider "helm" {
  kubernetes {
    host                   = data.terraform_remote_state.kubernetes.outputs.kubeconfig.host
    cluster_ca_certificate = base64decode(data.terraform_remote_state.kubernetes.outputs.kubeconfig.cluster_ca_certificate)
    token                  = data.terraform_remote_state.kubernetes.outputs.kubeconfig.token
  }
}
