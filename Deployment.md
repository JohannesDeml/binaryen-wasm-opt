# Deployment of this action

## Updating the action

1. Create a new branch with the version number you want to release
2. Update version in the image used in [action.yml](action.yml)
3. Push the branch to the repository
4. Push a tag matching the version (e.g. `v1.0.0`)
5. Check that the CI pipeline passes
6. Create new release in the GitHub UI and add the changelog for the new version
