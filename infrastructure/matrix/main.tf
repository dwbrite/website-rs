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

module "matrix_ingress" {
  source       = "../modules/nginx-ingress"
  port         = 8008
  root_domain  = var.root_domain
  service_name = "${helm_release.matrix_server.name}-matrix-synapse"
  subdomain    = local.subdomain
  ingress_name = "matrix-ingress"
}

#// special ingress for matrix
#resource "kubernetes_ingress_v1" "ingress_rules" {
#  metadata {
#    name = "matrix-special"
#  }
#
#  spec {
#    ingress_class_name = "nginx"
#
#    rule {
#      http {
#        path {
#          path = "/_matrix"
#          backend {
#            service {
#              name = "${helm_release.matrix_server.name}-matrix-synapse"
#              port { number = 8008 }
#            }
#          }
#        }
#      }
#    }
#
#    rule {
#      http {
#        path {
#          path = "/.well-known/matrix/client"
#          backend {
#            service {
#              name = "${helm_release.matrix_server.name}-matrix-synapse"
#              port { number = 8008 }
#            }
#          }
#        }
#      }
#    }
#
#    rule {
#      http {
#        path {
#          path = "/_synapse"
#          backend {
#            service {
#              name = "${helm_release.matrix_server.name}-matrix-synapse"
#              port { number = 8008 }
#            }
#          }
#        }
#      }
#    }
#  }
#}
