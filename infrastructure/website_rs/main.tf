variable "root_domain" { type = string }

resource "kubernetes_deployment" "dwbrite-com" {
  metadata {
    name = "dwbrite-com"
    labels = {
      app = "dwbrite.com"
      revision = "0x02"
    }
  }

  spec {
    replicas = 1

    selector {
      match_labels = {
        app = "dwbrite.com"
      }
    }

    template {
      metadata {
        labels = {
          app = "dwbrite.com"
        }
      }

      spec {
        image_pull_secrets {
          name = "container-registry-creds"
        }


        container {
          name  = "dwbrite-com"
          image = "registry.dwbrite.com/dwbrite/dwbrite-com:latest"
          image_pull_policy = "Always"

          resources {
            limits = {
              cpu    = "0.5"
              memory = "512Mi"
            }
            requests = {
              cpu    = "250m"
              memory = "50Mi"
            }
          }

          liveness_probe {
            http_get {
              path = "/"
              port = 41234

              http_header {
                name  = "X-Custom-Header"
                value = "Awesome"
              }
            }

            initial_delay_seconds = 3
            period_seconds        = 3
          }
        }
      }
    }
  }
}

resource "kubernetes_service_v1" "site_service" {
  metadata {
    name = "dwbrite-com-service"
  }
  spec {
    selector = {
      app = "dwbrite.com"
    }
    session_affinity = "ClientIP"
    port {
      port        = 443
      target_port = 41234
    }

    type = "NodePort"
  }
}

module "ingress" {
  source       = "../modules/nginx-ingress"
  port         = 443
  root_domain  = var.root_domain
  service_name = "dwbrite-com-service"
  subdomain    = ""
  ingress_name = "dwbrite-com-ingress"
}