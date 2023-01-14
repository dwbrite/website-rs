terraform {
  required_providers {
    kubernetes = {
      source = "hashicorp/kubernetes"
      version = "2.16.1"
    }
  }
}

variable "name" {
  default = "outline"
}

variable "replicas" {
  default = "1"
}

variable "image" {
  default = {
    repository: "outlinewiki/outline",
    tag: "0.66.3"
  }
}

variable "secretKey" {}
variable "utilsSecret" {}

variable "host" {}

locals {
  port = "3000"
  collaborationUrl = ""
}

resource "kubernetes_deployment" "outline" {
  metadata {
    name = var.name
  }
  spec {
    selector {
      match_labels = {
        app = var.name
      }
    }
    replicas = var.replicas
    strategy {
      type = "Recreate"
    }
    template {
      metadata {
        labels = {
          app = var.name
          release = var.name # TODO: different release name?
        }
      }
      spec {
        init_container {
          name = "${var.name}-migrate"
          image = "${var.image.repository}:${var.image.tag}"
          args = ["yarn", "db:migrate", "--env=production-ssl-disabled"]

          env {
            name = "SECRET_KEY"
            value = var.secretKey
          }
          env {
            name = "UTILS_SECRET"
            value = var.secretKey
          }
          env {
            name = "PORT"
            value = local.port
          }
          env {
            name = "COLLABORATION_URL"
            value = local.collaborationUrl
          }
          env {
            name = "URL"
            value = "https://${var.host}"
          }
          env {
            name : "DATABASE_URL"
            value : "postgres://${var.postgresql.postgresqlUsername }:${ var.postgresql.postgresqlPassword }@${ var.name }-postgresql:5432/${ var.postgresql.postgresqlDatabase }"
          }
          env {
            name: "DATABASE_URL_TEST"
            value: "postgres://${ var.postgresql.postgresqlUsername }:${ var.postgresql.postgresqlPassword }@${ var.name }-postgresql:5432/${ var.postgresql.postgresqlDatabase }-test"
          }
          env {
            name: "PGSSLMODE"
            value: "disable"
          }
          env {
            name: "REDIS_URL"
            value: "redis://${ var.name }-redis-master:6379"
          }
          env {
            name: REDIS_URL
            value: "redis://${ var.name }-redis-master:6379"
          }
          env {
            name: AWS_ACCESS_KEY_ID
            value: var.minio.accessKey.password
          }
          env {
            name: AWS_SECRET_ACCESS_KEY
            value: var.minio.secretKey.password
          }
          env {
            name: AWS_REGION
            value: "us-east-1"
          }
          env {
            name: AWS_S3_UPLOAD_BUCKET_URL
            value: "http://${ var.minio.ingress.hostname }"
          }
          env {
            name: AWS_S3_UPLOAD_BUCKET_NAME
            value: var.minio.defaultBuckets
          }
          env {
            name: AWS_S3_UPLOAD_MAX_SIZE
            value: "26214400"
          }
          env {
            name: AWS_S3_FORCE_PATH_STYLE
            value: "true"
          }
          env {
            name: AWS_S3_ACL
            value: "private"
          }

          image_pull_policy = var.image.image_pull_policy
        }
      }
    }
  }
}
