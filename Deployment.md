# Deployment of this action

## Updating Binaryen

Check https://github.com/WebAssembly/binaryen/tags for the latest version and update all occurrences of the version number in the [Dockerfile](Dockerfile) and the readme.

## Updating the action

1. Build a new docker image and push it to dockerhub
   * add tags with the ver version and the binaryen version you want to use with the pattern `major.binaryen-version.patch` (e.g. `1.122.0`)
2. Update version in [action.yml](action.yml) (see comment in file)
3. Create new release in the GitHub UI and add the changelog for the new version
