# wasm-opt GitHub action

This docker action allows usage of [binaryen's](https://github.com/WebAssembly/binaryen) wasm-opt to optimize Wasm files inside your GitHub workflows. Binaryen is available under its [Apache 2.0 License](LICENSE-BINARYEN).

*Currently this action supports Binaryen versions 116-122*

## Usage

In one of your GitHub workflow steps:
```yaml

      - name: Optimize Wasm
        uses: JohannesDeml/wasm-opt-action@v1
        with:
          file: some/path/to/file.wasm
          output: some/path/to/file.wasm
          options: --enable-bulk-memory --enable-threads
          binaryen_version: 122
```

* `file` [Required] input name, supports Unix shell like patterns. By default, only the first match is optimized.
* `optimize_all` [Optional] If you would like to optimize all files matching the pattern, set this var to true.
* `output` [Optional] output name, defaults to the original file path. This is very useful if you pass a glob as `file`.
* `binaryen_version` [Optional] binaryen version used, defaults to 122.
* `options` [Optional] options passed to wasm-opt, defaults to `-Os`. See also [wasm-opt options](https://github.com/WebAssembly/binaryen/blob/main/src/tools/optimization-options.h)

For example, if your wasm files get a hash attached to them at build time, you can optimize all of them like this:
```yaml
      - name: Optimize Wasm
        uses: JohannesDeml/wasm-opt-action@v1
        with:
          file: dist/*.wasm
          optimize_all: true
```
This will replace your wasm files with the optimized ones.

The input parameters are passed to `wasm-opt` like so: `<input> -o <output> <options>`.

For more examples (e.g. for godot wasm files) see [ci.yaml](.github/workflows/ci.yaml).

## Improvements added in the fork

* Added support for selecting specific Binaryen versions
* Action now fails if wasm-opt returns a non-zero exit code
* Updated library versions (e.g rust) to make docker builds faster
* Added CI pipeline for building and pushing docker images
