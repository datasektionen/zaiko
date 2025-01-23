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

ARG RUST_VERSION=1.77.2
ENV APP_NAME=backend

FROM rust:1.77.2-alpine AS build
ARG APP_NAME
WORKDIR /build

ARG DATABASE_URL="sqlite:/db.sqlite"
ENV DATABASE_URL=$DATABASE_URL

ENV SQLX_OFFLINE=true

RUN apk update && apk add git alpine-sdk make libffi-dev openssl-dev pkgconfig bash sqlite

COPY backend/Cargo.lock backend/Cargo.toml ./

COPY backend/.sqlx .sqlx
COPY backend/dev_setup.sh dev_setup.sh

RUN ./dev_setup.sh

COPY backend/src src
RUN cargo build --locked --release
RUN cp ./target/release/backend /bin/server
RUN cp ./db.sqlite /bin/db.sqlite

##
## -------------deploy-----------------------------
##

FROM alpine:3.18 AS final

COPY --from=prerelease /usr/src/app/dist dist

COPY --from=build /bin/server /bin/
COPY --from=build /bin/db.sqlite /bin/db.sqlite

EXPOSE 8000

CMD ["/bin/server"]
