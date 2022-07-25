# 2. What I Did

By looking at _BEFORE_ and _AFTER_
for when the app was initially created using
[wasm-pack-plugin](https://github.com/wasm-tool/wasm-pack-plugin)
compared to when the app was finished implementation,
it should add more specificity to your understanding.

## 2-1. BEFORE + AFTER: Directory Structure

**(BEFORE)**

```bash
$ npm init rust-webpack flight-pack
$ cd flight-pack
$ tree .

├── Cargo.toml
├── js
│   └── index.js
├── package.json
├── README.md
├── src
│   └── lib.rs
├── static
│   └── index.html
├── tests
│   └── app.rs
├── webpack.config.js
└── yarn.lock
```

**(AFTER)**  
After building WASM (with `npm` replaced with `yarn`)

```bash
$ yarn build:wasm
$ tree .

├── Cargo.lock
├── Cargo.toml
├── dist
│   ├── 1.js
│   ├── 40815612b85747d7ec33.module.wasm
│   ├── index.html
│   └── index.js
├── js
│   └── index.js
├── package.json
├── pkg
│   ├── index_bg.js
│   ├── index_bg.wasm
│   ├── index_bg.wasm.d.ts
│   ├── index.d.ts
│   ├── index.js
│   ├── package.json
│   └── README.md
├── README.md
├── src
│   └── lib.rs
├── static
│   └── index.html
├── target
│   ├── CACHEDIR.TAG
│   ├── release
│   └── wasm32-unknown-unknown
├── tests
│   └── app.rs
├── webpack.config.js
└── yarn.lock
```

## 2-2. BEFORE + AFTER: Rust

### `Cargo.toml`

**(BEFORE)**

```toml
# You must change these to your own details.
[package]
name = "rust-webpack-template"
description = "My super awesome Rust, WebAssembly, and Webpack project!"
version = "0.1.0"
authors = ["You <you@example.com>"]
categories = ["wasm"]
readme = "README.md"
edition = "2018"

[lib]
crate-type = ["cdylib"]

[profile.release]
# This makes the compiled code faster and smaller, but it makes compiling slower,
# so it's only enabled in release mode.
lto = true

[features]
# If you uncomment this line, it will enable `wee_alloc`:
#default = ["wee_alloc"]

[dependencies]
# The `wasm-bindgen` crate provides the bare minimum functionality needed
# to interact with JavaScript.
wasm-bindgen = "0.2.45"

# `wee_alloc` is a tiny allocator for wasm that is only ~1K in code size
# compared to the default allocator's ~10K. However, it is slower than the default
# allocator, so it's not enabled by default.
wee_alloc = { version = "0.4.2", optional = true }

# The `web-sys` crate allows you to interact with the various browser APIs,
# like the DOM.
[dependencies.web-sys]
version = "0.3.22"
features = ["console"]

# The `console_error_panic_hook` crate provides better debugging of panics by
# logging them with `console.error`. This is great for development, but requires
# all the `std::fmt` and `std::panicking` infrastructure, so it's only enabled
# in debug mode.
[target."cfg(debug_assertions)".dependencies]
console_error_panic_hook = "0.1.5"

# These crates are used for running unit tests.
[dev-dependencies]
wasm-bindgen-test = "0.2.45"
futures = "0.1.27"
js-sys = "0.3.22"
wasm-bindgen-futures = "0.3.22"
```

**(AFTER)**

```diff
[dependencies]
+ callback-future = "0.1.0"
+ chrono = { version = "0.4.20", features = ["serde"] }
+ futures = "0.3.23"
+ geoutils = "0.5.1"
+ js-sys = "0.3.58"
+ lazy_static = "1.4.0"
+ load-dotenv = "0.1.2"
+ num = "0.4.0"
+ serde = { version = "1.0.140", features = ["derive"] }
+ serde_json = "1.0.83"
- wasm-bindgen = "0.2.45"
+ wasm-bindgen = { version = "0.2.82", features = ["serde-serialize"] }
+ wasm-bindgen-futures = "0.4.32"
- wee_alloc = { version = "0.4.2", optional = true }
+ wee_alloc = { version = "0.4.5", optional = true }

+ [dependencies.rand]
+ version = "0.7.3"
+ features = ["wasm-bindgen"]

[dependencies.web-sys]
- version = "0.3.22"
- features = ["console"]
+ version = "0.3.58"
+ features = [
+   'console',
+   'CssStyleDeclaration',
+   'CanvasRenderingContext2d',
+   'Document',
+   'DomRect',
+   'Element',
+   'HtmlCanvasElement',
+   'HtmlElement',
+   'Node',
+   'Request',
+   'RequestInit',
+   'RequestMode',
+   'Response',
+   'TextMetrics',
+   'Window',
+ ]

[target."cfg(debug_assertions)".dependencies]
- console_error_panic_hook = "0.1.5"
+ console_error_panic_hook = "0.1.7"

[dev-dependencies]
- js-sys = "0.3.22"
+ js-sys = "0.3.58"

[profile.release]
+ opt-level = "s"
lto = true
```

## 2-3. BEFORE + AFTER: JavaScript

### A. Webpack Config

```diff
- const path = require("path");
- const CopyPlugin = require("copy-webpack-plugin");
- const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
+ const path = require('path');
+ const HtmlWebpackPlugin = require('html-webpack-plugin');
+ const { CleanWebpackPlugin } = require('clean-webpack-plugin');
+ const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
+ const Dotenv = require('dotenv-webpack');
+ const TerserPlugin = require('terser-webpack-plugin');

- const dist = path.resolve(__dirname, "dist");
+ const dist = path.resolve(__dirname, 'dist');
+ 
+ const ENV_NAME = process.env.NODE_ENV;
+ const PROD = ENV_NAME === 'production';

module.exports = {
- mode: "production",
+ mode: 'production',
  entry: {
-   index: "./js/index.js"
+   index: './js/index.js',
  },
  output: {
    path: dist,
-   filename: "[name].js"
+   filename: '[name].[fullhash].js',
  },
+ resolve: {
+   extensions: ['.js'],
+ },
+ experiments: {
+   asyncWebAssembly: true,
+ },
+ performance: {
+   hints: false,
+   maxAssetSize: 512000, // 500 bytes
+ },
+ optimization: {
+   ...(PROD && {
+     minimize: true,
+     minimizer: [
+       new TerserPlugin({
+         terserOptions: {
+           mangle: true,
+         },
+       }),
+     ],
+   }),
+ },
+ devtool: PROD ? 'cheap-source-map' : 'inline-source-map',
  devServer: {
-   contentBase: dist,
+   // contentBase: dist,
+   static: {
+     directory: dist,
+   },
+   port: 8080,
+   devMiddleware: {
+     writeToDisk: true,
+   },
  },
+ module: {
+   rules: [
+     {
+       test: /\.m?js?$/,
+       exclude: /node_modules/,
+       use: {
+         loader: 'babel-loader',
+       },
+     },
+     {
+       test: /\.css$/,
+       use: ['style-loader', 'css-loader', 'postcss-loader'],
+     },
+   ],
+ },
  plugins: [
-   new CopyPlugin([
-     path.resolve(__dirname, "static")
-   ]),
+   new Dotenv(),
+   new CleanWebpackPlugin(),
+   new HtmlWebpackPlugin({
+     template: './js/index.html',
+     filename: 'index.html',
+     minify: { collapseWhitespace: false },
+   }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
+     forceMode: PROD ? 'production' : 'development',
    }),
- ]
+ ],
};
```

### B. NPM Packages

#### (1) Pre-Installed (for `wasm-pack` template)

```json
  "devDependencies": {
    "@wasm-tool/wasm-pack-plugin": "^1.1.0",
    "copy-webpack-plugin": "^5.0.3",
    "prettier": "^2.7.1",
    "rimraf": "^3.0.0",
    "webpack": "^4.42.0",
    "webpack-cli": "^3.3.3",
    "webpack-dev-server": "^3.7.1"
  }
```

#### (2) Custom Installations

__# Babel__

- @babel/core
- @babel/cli
- @babel/preset-env
- core-js@3
- @babel/runtime-corejs3
- babel-loader

__# Webpack__

- webpack
- webpack-cli
- webpack-dev-server
- file-loader
- css-loader
- style-loader
- postcss-loader
- copy-webpack-plugin
- clean-webpack-plugin
- html-webpack-plugin
- @wasm-tool/wasm-pack-plugin
- autoprefixer
- tailwindcss

__# Others__

- rimraf
- prettier
- dotenv-webpack
- @googlemaps/js-api-loader

```bash
# devDependencies
yarn add --dev @babel/cli @babel/core @babel/preset-env @babel/runtime-corejs3 @wasm-tool/wasm-pack-plugin autoprefixer babel-loader clean-webpack-plugin copy-webpack-plugin core-js css-loader dotenv-webpack file-loader html-webpack-plugin postcss-loader prettier rimraf style-loader tailwindcss webpack webpack-cli webpack-dev-server

# dependencies
yarn add @googlemaps/js-api-loader
```
