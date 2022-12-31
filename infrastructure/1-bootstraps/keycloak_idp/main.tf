## Providers ############################################################################################################
#
#terraform {
#  required_providers {
#    helm = {
#      source = "hashicorp/helm"
#      version = "2.8.0"
#    }
#  }
#}

# Variables ############################################################################################################

variable "root_domain" { type = string }

locals {
  name = "keycloak"
  subdomain = local.name
  host = "${local.subdomain}.${var.root_domain}"
}

# Keycloak #############################################################################################################

data "kubernetes_secret" "keycloak_admin" {
  metadata {
    name      = "keycloak-admin"
  }
}

#auth.adminUser	Keycloak administrator user	user
#auth.adminPassword	Keycloak administrator password for the new user	""
#auth.existingSecret	Existing secret containing Keycloak admin password	""
#auth.passwordSecretKey	Key where the Keycloak admin password is being stored inside the existing secret.

resource "helm_release" "keycloak" {
  chart      = "keycloak"
  repository = "https://charts.bitnami.com/bitnami"
  name       = local.name
  version = "13.0.0"

  values = [
    <<EOT
    auth:
      adminUser: "${data.kubernetes_secret.keycloak_admin.data.username}"
      existingSecret: "${data.kubernetes_secret.keycloak_admin.metadata[0].name}"
      passwordSecretKey: "password"
    EOT
  ]


}

module "ingress" {
  source       = "../../modules/nginx-ingress"
  port         = 80
  root_domain  = var.root_domain
  service_name = helm_release.keycloak.name
  subdomain    = local.subdomain
  ingress_name = "keycloak-ingress"
}




