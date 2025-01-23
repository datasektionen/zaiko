# zaiko

prototype of an inventory management system for the computer science chapter at KTH university

# API

| Endpoint   | Description        |
|------------|--------------------|
| item       |                    |
|------------|--------------------|
|        get | get all items      |
|------------|--------------------|
|       post | add new item       |
|------------|--------------------|
|      patch | update item        |
|------------|--------------------|
| supplier   |                    |
|------------|--------------------|
|        get | get supplier by id |
|------------|--------------------|
|       post | add new supplier   |
|------------|--------------------|
|      patch | update supplier    |
|------------|--------------------|
| log        |                    |
|------------|--------------------|
|        get | get all logs       |
|------------|--------------------|
| shortage   |                    |
|------------|--------------------|
|        get | get shortage       |
|------------|--------------------|
| take_stock |                    |
|------------|--------------------|
|       post | take stock         |
|------------|--------------------|

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) to make the TypeScript language service aware of `.vue` types.

## Customize configuration

See [Vite Configuration Reference](https://vite.dev/config/).

## Project Setup

```sh
bun install
```

### Compile and Hot-Reload for Development

```sh
bun dev
```

### Type-Check, Compile and Minify for Production

```sh
bun build
```

### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
bun test:unit
```

### Run End-to-End Tests with [Cypress](https://www.cypress.io/)

```sh
bun test:e2e:dev
```

This runs the end-to-end tests against the Vite development server.
It is much faster than the production build.

But it's still recommended to test the production build with `test:e2e` before deploying (e.g. in CI environments):

```sh
bun build
bun test:e2e
```

### Lint with [ESLint](https://eslint.org/)

```sh
bun lint
```
