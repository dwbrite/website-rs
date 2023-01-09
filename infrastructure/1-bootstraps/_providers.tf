terraform {
  required_providers {
    linode = {
      source  = "linode/linode"
      version = "1.28.0"
    }

    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.12.0"
    }

    kubectl = {
      source  = "gavinbunney/kubectl"
      version = "1.14.0"
    }

    helm = {
      source  = "hashicorp/helm"
      version = "2.6.0"
    }
  }
}


# Provider Configs #####################################################################################################

provider "linode" {
  token = var.linode_token
}

data terraform_remote_state "kubernetes" {
  backend = "s3"
  config = var.kubernetes_backend
}

provider "kubernetes" {
  host                   = data.terraform_remote_state.kubernetes.outputs.kubeconfig.host
  cluster_ca_certificate = base64decode(data.terraform_remote_state.kubernetes.outputs.kubeconfig.cluster_ca_certificate)
  token                  = data.terraform_remote_state.kubernetes.outputs.kubeconfig.token
}

provider "kubectl" {
  host                   = data.terraform_remote_state.kubernetes.outputs.kubeconfig.host
  cluster_ca_certificate = base64decode(data.terraform_remote_state.kubernetes.outputs.kubeconfig.cluster_ca_certificate)
  token                  = data.terraform_remote_state.kubernetes.outputs.kubeconfig.token
  load_config_file       = false
}

provider "helm" {
  kubernetes {
    host                   = data.terraform_remote_state.kubernetes.outputs.kubeconfig.host
    cluster_ca_certificate = base64decode(data.terraform_remote_state.kubernetes.outputs.kubeconfig.cluster_ca_certificate)
    token                  = data.terraform_remote_state.kubernetes.outputs.kubeconfig.token
  }
}
