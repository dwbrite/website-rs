variable "root_domain" {
  type = string
}

resource "helm_release" "firefly" {
  chart = "firefly-iii"
  repository = "https://k8s-at-home.com/charts"
  name  = "firefly"
}

module "ingress" {
  source       = "../modules/nginx-ingress"
  port         = 8080
  root_domain  = var.root_domain
  service_name = "${helm_release.firefly.name}-${helm_release.firefly.chart}"
  subdomain    = "firefly"
  ingress_name = "firefly-ingress"
}
