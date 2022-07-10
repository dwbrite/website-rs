# Matrix Homeserver ####################################################################################################
variable "root_domain" { type = string }

locals {
  subdomain = "matrix"
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
      }
    )
  ]
}

