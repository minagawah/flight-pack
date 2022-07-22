# flight-pack

Flight simulation using wasm-pack

## 1. About

## 2. What I Did

#### 2-1. Directory Structure

__(BEFORE)__

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

__(AFTER)__  
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

#### 2-2. Rust

##### `Cargo.toml`

__(BEFORE)__

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

__(AFTER)__

```diff
[dependencies]
+ js-sys = "0.3.58"
+ serde = { version = "1.0.140", features = ["derive"] }
- wasm-bindgen = "0.2.45"
+ wasm-bindgen = { version = "0.2.81", features = ["serde-serialize"] }
- wee_alloc = { version = "0.4.2", optional = true }
+ wee_alloc = { version = "0.4.5", optional = true }

[dependencies.web-sys]
- version = "0.3.22"
- features = ["console"]
+ version = "0.3.58"
+ features = [
+   'console',
+   'CanvasRenderingContext2d',
+   'Document',
+   'Element',
+   'HtmlCanvasElement',
+   'HtmlElement',
+   'Node',
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

#### 2-3. JavaScript

##### `package.json`

__Initial `devDependencies`__

```json
  "devDependencies": {
    "@wasm-tool/wasm-pack-plugin": "^1.1.0",
    "copy-webpack-plugin": "^5.0.3",
    "rimraf": "^3.0.0",
    "webpack": "^4.42.0",
    "webpack-cli": "^3.3.3",
    "webpack-dev-server": "^3.7.1"
  }
```

##### NPM Installation

```bash
yarn add --dev prettier
```


## 3. License

Dual-licensed under either of the followings.  
Choose at your option.

- The UNLICENSE ([LICENSE.UNLICENSE](LICENSE.UNLICENSE))
- MIT license ([LICENSE.MIT](LICENSE.MIT))
