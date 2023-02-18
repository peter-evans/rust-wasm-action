# Rust-generated WebAssembly GitHub action template
[![CI](https://github.com/peter-evans/rust-wasm-action/workflows/CI/badge.svg)](https://github.com/peter-evans/rust-wasm-action/actions?query=workflow%3ACI)

A template to bootstrap the creation of a Rust-generated WebAssembly GitHub action.

## About

This project is experimental. While it is very much feasible to write a GitHub action in Rust-generated WebAssembly, as this project demonstrates, there are some challenges compared to JavaScript/TypeScript.

The following is a summary of pain points I discovered, and some caveats when using the template.

- It relies on raw bindings to the official [actions toolkit](https://github.com/actions/toolkit) NPM packages. Bindings for this template are defined in [actions-toolkit-bindings](actions-toolkit-bindings) but they are incomplete. I've only defined bindings that are in use by the template. It would be great to have a well maintained set of bindings published as a crate.

- WebAssembly runs in a safe, sandboxed execution environment. As a result there is no access to files on disks, environment variables, etc. This means bindings to JavaScript functions are necessary for some functionality.

- Panics cause the action to fail on exit but could be handled a little better. Perhaps we need a hook similar to the [console_error_panic_hook](https://github.com/rustwasm/console_error_panic_hook) that calls the binding for the [`setFailed`](https://github.com/actions/toolkit/blob/main/packages/core/src/core.ts#L103-L112) function in `@actions/core`.

- When dealing with string input you need to make decisions about whether to leave the string in JavaScript encoded as UTF-16 (`js_sys::JsString`), or to copy the string into Rust encoded as UTF-8. The later is lossy and in some cases could cause the string to be different in Rust than JavaScript. See the wasm-bindgen documentation [here](https://rustwasm.github.io/wasm-bindgen/reference/types/str.html#utf-16-vs-utf-8) for further detail.

## License

[MIT](LICENSE)
