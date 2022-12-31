variable "root_domain" { type = string }
variable "subdomain" { type = string }
variable "service_name" { type = string }
variable "port" { type = number }
variable "ingress_name" { type = string }
variable "internal" { default = false }

locals {
  host = var.subdomain != "" ? "${var.subdomain}.${var.root_domain}" : var.root_domain
  cert_domain = var.subdomain != "" ? "*.${var.root_domain}" : var.root_domain
  cert_secret = var.subdomain != "" ? "tls-wildcard.${var.root_domain}" : "tls-root.${var.root_domain}"
  ingress_class = var.internal ? "nginx-internal" : "nginx"
}

resource "kubernetes_ingress_v1" "ingress_rules" {
  metadata {
    name = var.ingress_name
    annotations = {
      "nginx.ingress.kubernetes.io/proxy-body-size": 0
      "cert-manager.io/cluster-issuer": "letsencrypt-prod"
      "kubernetes.io/ingress.class": local.ingress_class
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
      hosts = [local.cert_domain]
      secret_name = local.cert_secret
    }
  }
}
