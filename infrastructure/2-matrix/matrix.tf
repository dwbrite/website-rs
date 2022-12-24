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

variable "bootstraps_backend" {}

data "terraform_remote_state" "bootstraps" {
  backend = "s3"
  config = var.bootstraps_backend
}

resource "linode_domain_record" "matrix_srv_record" {
  domain_id = data.terraform_remote_state.bootstraps.outputs.domain.id
  record_type = "SRV"
  target = "dwbrite.com"
  service = "matrix"
  protocol = "tcp"
  name = "dwbrite.com"
  ttl_sec = 300
  priority = 0
  weight = 100
  port = 443
}
