job "zaiko-dev" {
  type = "service"

  group "zaiko-dev" {
    network {
      port "http" { }
    }

    service {
      name     = "zaiko-dev"
      port     = "http"
      provider = "nomad"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.zaiko-dev.rule=Host(`zaiko.betasektionen.se`)",
        "traefik.http.routers.zaiko-dev.tls.certresolver=default",
      ]
    }

    task "zaiko-dev" {
      driver = "docker"

      config {
        image = var.image_tag
        ports = ["http"]
      }

      template {
        data        = <<ENV
{{ with nomadVar "nomad/jobs/zaiko-dev" }}
APP_SECRET={{ .app_secret }}
OIDC_SECRET={{ .oidc_secret }}
HIVE_SECRET={{ .hive_api_key }}
DATABASE_URL=postgresql://zaiko:{{ .database_password }}@postgres.dsekt.internal:5432/zaiko-dev
{{ end }}
PORT={{ env "NOMAD_PORT_http" }}
OIDC_PROVIDER=https://sso.datasektionen.se/op
OIDC_ID=zaiko
REDIRECT_URL=https://zaiko.betasektionen.se/auth/oidc/callback
HIVE_URL=https://hive.datasektionen.se/api/v1
APP_URL=0.0.0.0
APP_ENV=production
APP_DEBUG=false
RUST_LOG=info
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
