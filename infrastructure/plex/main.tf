variable "root_domain" {
  type = string
}

resource "helm_release" "plex" {
  chart = "plex"
  repository = "https://k8s-at-home.com/charts"
  name  = "plex"

  set {
    name  = "ingress.main.enabled"
    value = "false"
  }
}

module "ingress" {
  source       = "../modules/nginx-ingress"
  port         = 32400
  root_domain  = var.root_domain
  service_name = "${helm_release.plex.name}-${helm_release.plex.chart}"
  subdomain    = "plex"
  ingress_name = "plex-ingress"
}
