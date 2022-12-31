# Domain / DNS Records #################################################################################################

resource "linode_domain" "dewbrite_com" {
  domain    = var.root_domain
  type      = "master"
  soa_email = var.email
}

resource "linode_domain_record" "dewbrite_cname" {
  domain_id   = linode_domain.dewbrite_com.id
  name        = "*"
  record_type = "CNAME"
  target      = var.root_domain
}

output "domain" {
  value = {
    id          = linode_domain.dewbrite_com.id
    root_domain = var.root_domain
    soa_email   = var.email
  }
}

# ACME #################################################################################################################

# linode credentials for the DNS solver - should only need to access "Domains"
resource "kubernetes_secret" "linode_credentials" {
  metadata {
    name = "linode-credentials" # secret name used by cert-manager-webhook-linode
    namespace = "cert-manager"
  }
  data = {
    token = var.linode_token_dns
  }
}

resource "helm_release" "linode_webhook_dns01_solver" {
  chart = "cert-manager-webhook-linode-chart"
  name  = "cert-manager-webhook-linode"
  namespace = "cert-manager"
}

module "cert_manager" {
  source        = "terraform-iaac/cert-manager/kubernetes"

  cluster_issuer_email                   = "dwbrite@gmail.com"
  cluster_issuer_name                    = "letsencrypt-prod"
  cluster_issuer_private_key_secret_name = "letsencrypt-prod-pkey"

  solvers = [{
    dns01 = {
      webhook = {
        solverName = "linode"
        groupName = "acme.cluster.local"
        config = {
          apiKey = var.linode_token_dns
        }
      }
    }
  }]
}

# Load Balancing  ######################################################################################################
# TODO: add namespace for load balancing?

resource "helm_release" "nginx_ingress_controller" {
  repository = "https://charts.bitnami.com/bitnami"
  name       = "nginx-ingress-controller"
  chart      = "nginx-ingress-controller"

  values = [ <<-EOT
    service:
      type: "LoadBalancer"
      extraPorts: [ { port: 8008, name: "matrix" }, { port: 8448, name: "matrix-ssl" } ]

    defaultBackend:
      enabled: false
    EOT
  ]
}

resource "helm_release" "nginx_ingress_controller_internal" {
  repository = "https://charts.bitnami.com/bitnami"
  name       = "nginx-ingress-controller-internal"
  chart      = "nginx-ingress-controller"

  values = [<<-EOT
    service:
      type: "ClusterIP"
    ingressClassResource:
      name: "nginx-internal"
      controllerClass: "k8s.io/ingress-nginx-internal"

    defaultBackend:
      enabled: false
    EOT
  ]
}

data "kubernetes_service" "nginx_ingress_data" {
  depends_on = [helm_release.nginx_ingress_controller]
  metadata {
    name = "nginx-ingress-controller"
  }
}

resource "linode_domain_record" "balancer_ingress_a_record" {
  domain_id   = linode_domain.dewbrite_com.id
  record_type = "A"
  target      = data.kubernetes_service.nginx_ingress_data.status.0.load_balancer.0.ingress.0.ip
}

# Container Registry  ##################################################################################################

module "container_registry" {
  source = "./container_registry"
  providers = {
    linode = linode
  }

  linode_bucket_region = var.linode_bucket_region
  root_domain          = var.root_domain
  registry_pass        = var.container_registry_password
  registry_user        = var.container_registry_username
}

# Keycloak  ##################################################################################################

module "keycloak" {
  source      = "./keycloak_idp"
  root_domain = var.root_domain
}
