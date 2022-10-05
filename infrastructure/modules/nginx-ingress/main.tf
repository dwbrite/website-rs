variable "root_domain" { type = string }
variable "subdomain" { type = string }
variable "service_name" { type = string }
variable "port" { type = number }
variable "ingress_name" { type = string }

resource "kubernetes_ingress_v1" "ingress_rules" {
  metadata {
    name = var.ingress_name
    annotations = {
      "nginx.ingress.kubernetes.io/proxy-body-size": 0
    }
  }

  spec {
    ingress_class_name = "nginx"

    rule {
      host = "${var.subdomain}.${var.root_domain}"
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
  }
}
