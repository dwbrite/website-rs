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

resource "helm_release" "container_registry" {
  chart      = "docker-registry"
  repository = "https://charts.helm.sh/stable"
  name       = "container-registry"

  values = [
    templatefile(
      "${path.module}/values.template.yml", {
        bucket          = linode_object_storage_bucket.dewbrite_registry.label
        region          = linode_object_storage_bucket.dewbrite_registry.cluster
        region_endpoint = data.linode_object_storage_cluster.primary.domain # does this need subdomain?
        secret_key      = linode_object_storage_bucket.dewbrite_registry.secret_key
        access_key      = linode_object_storage_bucket.dewbrite_registry.access_key
      }
    )
  ]

  set {
    name  = "service.type"
    value = "ClusterIP"
  }
}

module "ingress" {
  source       = "../modules/nginx-ingress"
  port         = 5000
  root_domain  = var.root_domain
  service_name = "${helm_release.container_registry.name}-${helm_release.container_registry.chart}"
  subdomain    = "registry"
  ingress_name = "container-registry-ingress"
}


