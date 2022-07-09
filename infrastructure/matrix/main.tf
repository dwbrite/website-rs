# Matrix Homeserver ####################################################################################################
variable "root_domain" { type = string }

locals {
  subdomain = "matrix"
}

resource "helm_release" "matrix_server" {
  chart = "matrix-synapse"
  name  = "matrix-dwbrite"
  repository = "https://ananace.gitlab.io/charts"

  values = [
    templatefile(
      "${path.module}/values.template.yml", {
        root_domain = var.root_domain
        subdomain = local.subdomain
        full_domain = "${local.subdomain}.${var.root_domain}"
      }
    )
  ]
}

module "ingress" {
  source =  "../modules/nginx-ingress"
  port         = 8008
  root_domain  = "dwbrite.com"
  service_name = "${helm_release.matrix_server.name}-matrix-synapse"
  subdomain    = "matrix"
  ingress_name = "matrix-ingress"
}
