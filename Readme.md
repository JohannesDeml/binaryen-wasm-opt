# wasm-opt GitHub action

This docker action allows usage of [binaryen's](https://github.com/WebAssembly/binaryen) wasm-opt to optimise Wasm files inside your GitHub workflows. Binaryen is available under its [Apache 2.0 License](LICENSE-BINARYEN).

*Currently, this action uses Binaryen version 116*

## Usage

In one of your GitHub workflow steps:
```yaml
      - name: Optimize Wasm
        uses: JohannesDeml/wasm-opt-action@v2
        with:
          file: some/path/to/file.wasm
          output: some/path/to/file.wasm
```

The parameter `file` is required and supports Unix shell like patterns. By default, only the first match is optimized. If you would like to optimize all files matching the pattern, set `optimize_all` to true.

The parameter `output` is optional and defaults to the original file path. This is very useful if you pass a glob as `file`.

For example, if your wasm files get a hash attached to them at build time, you can optimize all of them like this:
```yaml
      - name: Optimize Wasm
        uses: JohannesDeml/wasm-opt-action@v2
        with:
          file: dist/*.wasm
          optimize_all: true
```
This will replace your wasm files with the optimized ones.

There is another optional parameter `options` with a default value of `-Os`.

The input parameters are passed to `wasm-opt` like so: `<input> -o <output> <options>`.

Take a look at the [wasm-opt options](https://github.com/WebAssembly/binaryen/blob/main/src/tools/optimization-options.h) for more info.

## Build project

```bash
# 1. Build the image
podman build --build-arg BINARYEN_VERSION=122 -t docker.io/johannesdeml/wasm-opt-action:v1.122.0 .

# 2. Login to Docker Hub (if you haven't already)
podman login docker.io

# 3. Now push the image
podman push docker.io/johannesdeml/wasm-opt-action:v1.122.0
```
