# Providers ############################################################################################################

terraform {
  required_providers {
    linode = {
      source  = "linode/linode"
      version = "1.28.0"
    }
  }
}


# Variables ############################################################################################################
variable "linode_bucket_region" { type = string }
variable "root_domain" { type = string }


# Container Registry ###################################################################################################

data "linode_object_storage_cluster" "primary" {
  id = var.linode_bucket_region
}

resource "linode_object_storage_key" "registry_key" {
  label = "container-registry-key"
}

resource "linode_object_storage_bucket" "dewbrite_registry" {
  access_key = linode_object_storage_key.registry_key.access_key
  secret_key = linode_object_storage_key.registry_key.secret_key

  cluster = data.linode_object_storage_cluster.primary.id
  label   = "dewbrite-registry"
}

resource "helm_release" "docker_registry" {
  chart      = "docker-registry"
  repository = "https://charts.helm.sh/stable"
  name       = "docker-registry"

  set {
    name  = "service.type"
    value = "ClusterIP"
  }
}

output "docker_registry" {
  value = helm_release.docker_registry
}

module "ingress" {
  source =  "../modules/nginx-ingress"
  port         = 5000
  root_domain  = var.root_domain
  service_name = helm_release.docker_registry.name
  subdomain    = "registry"
  ingress_name = "container-registry-ingress"
}


