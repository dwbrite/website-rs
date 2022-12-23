resource "linode_lke_cluster" "dewbrite_cluster" {
  label       = var.cluster_name
  k8s_version = "1.24"
  region      = var.linode_region
  tags        = []

  pool {
    type = var.cluster_pool.type
    count = var.cluster_pool.count
  }
}

locals {
  kubeconfig = yamldecode(base64decode(linode_lke_cluster.dewbrite_cluster.kubeconfig))
}

output "kubeconfig" {
  value = {
    host = local.kubeconfig.clusters[0].cluster.server
    cluster_ca_certificate = local.kubeconfig.clusters[0].cluster.certificate-authority-data
    token = local.kubeconfig.users[0].user.token
  }
  sensitive = true
}
