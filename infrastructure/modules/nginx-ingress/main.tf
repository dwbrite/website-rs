variable "root_domain" { type = string }
variable "subdomain" { type = string }
variable "service_name" { type = string }
variable "port" { type = number }
variable "ingress_name" { type = string }

locals {
  host = var.subdomain != "" ? "${var.subdomain}.${var.root_domain}" : var.root_domain
}

resource "kubernetes_ingress_v1" "ingress_rules" {
  metadata {
    name = var.ingress_name
    annotations = {
      "nginx.ingress.kubernetes.io/proxy-body-size": 0
      "cert-manager.io/cluster-issuer": "letsencrypt-prod"
      "kubernetes.io/ingress.class": "nginx"
    }
  }

  spec {
    rule {
      host = local.host
      http {
        path {
          path = "/"
          backend {
            service {
              name = var.service_name
              port { number = var.port }
            }
          }
        }
      }
    }
    tls {
      hosts = [local.host]
      secret_name = "tls-${local.host}"
    }
  }
}
