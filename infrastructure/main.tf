terraform {
  backend "s3" {}
}

# Provider Configs #####################################################################################################

provider "linode" {
  token = var.linode_token
}

provider "kubernetes" {
  config_path = local_file.kubeconfig.filename
}

provider "helm" {
  kubernetes {
    config_path = local_file.kubeconfig.filename
  }
}

provider "acme" {
  server_url = "https://acme-v02.api.letsencrypt.org/directory"
}


# dewbrite LKE  Cluster ################################################################################################

resource "linode_lke_cluster" "dewbrite_cluster" {
  label       = "dewbrite"
  k8s_version = "1.23"
  region      = var.linode_region
  tags        = []

  pool {
    type  = "g6-standard-1"
    count = 2
  }
}

resource "local_file" "kubeconfig" {
  content_base64 = linode_lke_cluster.dewbrite_cluster.kubeconfig
  filename       = ".kubeconfig"
}


# Domain / DNS Records #################################################################################################

resource "linode_domain" "dewbrite_com" {
  depends_on = [linode_lke_cluster.dewbrite_cluster]

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

resource "tls_private_key" "reg_private_key" {
  algorithm = "RSA"
}

resource "acme_registration" "reg" {
  account_key_pem = tls_private_key.reg_private_key.private_key_pem
  email_address   = var.email
}

resource "tls_private_key" "cert_private_key" {
  algorithm = "RSA"
}

resource "tls_cert_request" "root_and_wildcard" {
  private_key_pem = tls_private_key.cert_private_key.private_key_pem

  subject {
    common_name = linode_domain.dewbrite_com.domain
  }

  dns_names = [linode_domain.dewbrite_com.domain, "*.${linode_domain.dewbrite_com.domain}"]

  key_algorithm = tls_private_key.cert_private_key.algorithm
}

resource "acme_certificate" "certificate" {
  account_key_pem         = acme_registration.reg.account_key_pem
  certificate_request_pem = tls_cert_request.root_and_wildcard.cert_request_pem

  dns_challenge {
    provider = "linode"
  }
}

resource "kubernetes_secret" "domains_private_key" {
  depends_on = [linode_lke_cluster.dewbrite_cluster]

  metadata {
    name = "domains-private-key"
  }

  data = {
    "tls.crt" = "${acme_certificate.certificate.certificate_pem}${acme_certificate.certificate.issuer_pem}}"
    "tls.key" = tls_private_key.cert_private_key.private_key_pem
  }

  type = "kubernetes.io/tls"
}


# Load Balancing  ######################################################################################################
# TODO: add namespace for load balancing?

resource "helm_release" "nginx_ingress_controller" {
  depends_on = [linode_lke_cluster.dewbrite_cluster]

  repository = "https://charts.bitnami.com/bitnami"
  name       = "nginx-ingress-controller"
  chart      = "nginx-ingress-controller"

  values = [templatefile("${path.module}/nginx_values.template.yml", {})]

  set {
    name  = "extraArgs.default-ssl-certificate"
    value = "${kubernetes_secret.domains_private_key.metadata.0.namespace}/${kubernetes_secret.domains_private_key.metadata.0.name}"
  }
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

resource "linode_domain_record" "matrix_srv_record" {
  domain_id = linode_domain.dewbrite_com.id
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


########################################################################################################################
#                                               _       _                                                              #
#                                              | |     | |                                                             #
#                               ____   ___   __| |_   _| | _____  ___                                                  #
#                              |    \ / _ \ / _  | | | | || ___ |/___)                                                 #
#                              | | | | |_| ( (_| | |_| | || ____|___ |                                                 #
#                              |_|_|_|\___/ \____|____/ \_)_____|___/                                                  #
########################################################################################################################

module "container_registry" {
  source = "./container-registry"
  providers = {
    linode = linode
  }

  linode_bucket_region = var.linode_bucket_region
  root_domain          = var.root_domain
}

module "matrix" {
  source      = "./matrix"
  root_domain = var.root_domain
}
