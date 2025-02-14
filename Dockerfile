##
## ---------------build frontend-----------------------
##

FROM oven/bun:1 AS base
WORKDIR /usr/src/app

RUN apt update -y && apt upgrade -y && apt install -y unzip

FROM base AS install
RUN mkdir -p /temp/dev
COPY package.json bun.lockb /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

RUN mkdir -p /temp/prod
COPY package.json bun.lockb /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

FROM base AS prerelease
COPY --from=install /temp/dev/node_modules node_modules
COPY . .

ENV NODE_ENV=production
RUN bun test
RUN bun run build-only ## FIXME

##
## -------------build backend-------------------------
##

ENV APP_NAME=backend
ENV SQLX_OFFLINE=true

FROM rust:1.84-alpine AS build
WORKDIR /build


RUN apk update && apk add git alpine-sdk make libffi-dev openssl-dev pkgconfig bash sqlite

COPY backend/Cargo.lock backend/Cargo.toml ./

COPY backend/.sqlx .sqlx
COPY backend/dev_setup.sh dev_setup.sh

RUN ./dev_setup.sh
RUN mkdir /var/zaiko

COPY backend/src src
RUN cargo build --locked --release
RUN cp ./target/release/backend /bin/server
RUN cp ./db.sqlite /var/zaiko/db.sqlite

##
## -------------deploy-----------------------------
##

FROM alpine:3.18 AS final

ENV DATABASE_URL=sqlite://db.sqlite
ENV DATABASE_PATH=/var/zaiko/db.sqlite
ENV OIDC_PROVIDER=https://sso.datasektionen.se/op
ENV OIDC_ID=zaiko
ENV OIDC_SECRET=bccmIyRN3JfZWHog1AuujNEautrmi5Z_hV9qfgEG0pg=
ENV REDIRECT_URL=http://localhost:8080/api/oidc/callback
ENV PLS_URL=https://pls.datasektionen.se/api
ENV APP_URL=0.0.0.0
ENV PORT=8080

ENV APP_SECRET=s70wSM9Qz5oX3EHQdHzihIKe7vYBXW3G8f9JPZC2A0tx7qBuRztOCAKwjGbKTGVW
RUN mkdir /var/zaiko

COPY --from=prerelease /usr/src/app/dist dist

COPY --from=build /bin/server /bin/
COPY --from=build /var/zaiko/db.sqlite /var/zaiko/db.sqlite

EXPOSE 8000

CMD ["/bin/server"]
