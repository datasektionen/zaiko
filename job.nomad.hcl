job "zaiko" {
  type = "service"

  group "zaiko" {
    network {
      port "http" { }
    }

    service {
      name     = "zaiko"
      port     = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.zaiko.rule=Host(`zaiko.datasektionen.se`)",
        "traefik.http.routers.zaiko.tls.certresolver=default",
      ]
    }

    task "zaiko" {
      driver = "docker"

      config {
        image = var.image_tag
        ports = ["http"]
      }

      template {
        data        = <<ENV
{{ with nomadVar "nomad/jobs/zaiko" }}
APP_SECRET={{ .app_secret }}
OIDC_SECRET={{ .oidc_secret }}
{{ end }}
PORT={{ env "NOMAD_PORT_http" }}
OIDC_PROVIDER=https://sso.datasektionen.se/op
OIDC_ID=zaiko
REDIRECT_URL=https://zaiko.datasektionen.se/api/oidc/callback
PLS_URL=https://pls.datasektionen.se/api
DATABASE_URL=sqlite://db.sqlite
DATABASE_PATH=/var/zaiko/db.sqlite
APP_URL=https://zaiko.datasektionen.se
APP_ENV=production
APP_DEBUG=false
ENV
        destination = "local/.env"
        env         = true
      }

      resources {
        memory = 120
      }
    }
  }
}

variable "image_tag" {
  type = string
  default = "ghcr.io/datasektionen/zaiko:latest"
}
