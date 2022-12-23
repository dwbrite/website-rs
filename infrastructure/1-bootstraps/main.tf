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

# ACME #################################################################################################################

module "cert_manager" {
  source        = "terraform-iaac/cert-manager/kubernetes"

  cluster_issuer_email                   = "dwbrite@gmail.com"
  cluster_issuer_name                    = "letsencrypt-prod"
  cluster_issuer_private_key_secret_name = "letsencrypt-prod-pkey"
}

# Load Balancing  ######################################################################################################
# TODO: add namespace for load balancing?

resource "helm_release" "nginx_ingress_controller" {
  repository = "https://charts.bitnami.com/bitnami"
  name       = "nginx-ingress-controller"
  chart      = "nginx-ingress-controller"

  values = [templatefile("${path.module}/nginx_values.template.yml", {})]
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
