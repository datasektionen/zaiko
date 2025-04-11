##
## ---------------build frontend-----------------------
##

FROM oven/bun:1 AS base
WORKDIR /usr/src/app

ENV VITE_HOST=https://zaiko.datasektionen.se
ENV VITE_HOST_FRONTEND=https://zaiko.datasektionen.se

RUN apt update -y && apt upgrade -y && apt install -y unzip

FROM base AS install
RUN mkdir -p /temp/dev
COPY package.json bun.lock /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

RUN mkdir -p /temp/prod
COPY package.json bun.lock /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

FROM base AS prerelease
COPY --from=install /temp/dev/node_modules node_modules
COPY package.json bun.lock cypress.config.ts index.html tsconfig.app.json tsconfig.node.json vite.config.ts env.d.ts eslint.config.js tsconfig.json tsconfig.vitest.json vitest.config.ts ./
COPY public  public
COPY cypress  cypress
COPY src src

ENV NODE_ENV=production
RUN bun test
RUN bun run build-only # FIXME

##
## -------------build backend-------------------------
##

ENV SQLX_OFFLINE=true
ENV RUST_LOG=info

FROM rust:1.84-alpine AS build
WORKDIR /build

RUN apk update && apk add git alpine-sdk make libffi-dev openssl-dev pkgconfig bash postgresql

COPY backend/Cargo.lock backend/Cargo.toml ./
COPY backend/.sqlx .sqlx

RUN mkdir src
RUN echo "pub fn test() {}" > src/lib.rs

RUN cargo build --locked --release

RUN rm -r src

COPY backend/src src
RUN cargo build --locked --release
RUN cp ./target/release/backend /bin/server

##
## -------------deploy-----------------------------
##

FROM alpine:3.18 AS final

RUN mkdir /var/zaiko

COPY --from=prerelease /usr/src/app/dist dist

COPY --from=build /bin/server /bin/

EXPOSE 8000

CMD ["/bin/server"]
