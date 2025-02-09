# wasm-opt GitHub action

This docker action allows usage of [binaryen's](https://github.com/WebAssembly/binaryen) `wasm-opt` to optimize Wasm files inside your GitHub workflows. Binaryen is available under its [Apache 2.0 License](LICENSE-BINARYEN).

*Currently this action supports Binaryen versions 116-122*

## Usage

```yaml
      # Optimize a single wasm file with options
      - name: Optimize file.wasm
        uses: JohannesDeml/binaryen-wasm-opt@v1
        with:
          file: some/path/to/file.wasm
          output: some/path/to/file.wasm
          options: --enable-bulk-memory --enable-threads
          binaryen_version: 122

      # Optimize all wasm files in the dist folder
      - name: Optimize all .wasm files
        uses: JohannesDeml/binaryen-wasm-opt@v1
        with:
          file: dist/*.wasm
          optimize_all: true
```

* `file` [Required] input name, supports Unix shell like patterns. By default, only the first match is optimized.
* `optimize_all` [Optional] Optimize all files matching the pattern, defaults to false.
* `output` [Optional] output name, defaults to the original file path. This is very useful if you pass a glob as `file`.
* `binaryen_version` [Optional] binaryen version used, defaults to 122.
* `options` [Optional] options passed to wasm-opt, defaults to `-Os`. See also [wasm-opt options](https://github.com/WebAssembly/binaryen/blob/main/src/tools/optimization-options.h)

The input parameters are passed to `wasm-opt` like so: `<input> -o <output> <options>`.

For more examples (e.g. for godot wasm files) see [ci.yaml](.github/workflows/ci.yaml).

## Improvements added in the fork

* Added support for selecting specific Binaryen versions
* Action now fails if wasm-opt returns a non-zero exit code
* Updated library versions (e.g rust) to make docker builds faster
* Added CI pipeline for building and pushing docker images

This is a fork of [wasm-opt-action](https://github.com/NiklasEi/wasm-opt-action)
