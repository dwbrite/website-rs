# Matrix Homeserver ####################################################################################################
variable "root_domain" { type = string }

locals {
  subdomain = "matrix"
}

resource "random_password" "matrix_shared_secret" {
  length = 24
  special = false
}


resource "kubernetes_secret" "matrix_shared_secret" {
  metadata {
    name = "matrix-shared-secret"
  }

  data = {
    shared_secret = random_password.matrix_shared_secret.result
  }

  immutable = true
}

resource "helm_release" "matrix_server" {
  chart      = "matrix-synapse"
  name       = "matrix-dwbrite"
  repository = "https://ananace.gitlab.io/charts"

  values = [
    templatefile(
      "${path.module}/values.template.yml", {
        root_domain = var.root_domain
        subdomain   = local.subdomain
        full_domain = "${local.subdomain}.${var.root_domain}"
        shared_secret = kubernetes_secret.matrix_shared_secret.data.shared_secret
      }
    )
  ]
}

module "matrix_ingress" {
  source       = "../modules/nginx-ingress"
  port         = 8008
  root_domain  = var.root_domain
  service_name = "${helm_release.matrix_server.name}-${helm_release.matrix_server.chart}"
  subdomain    = local.subdomain
  ingress_name = "matrix-ingress"
}
