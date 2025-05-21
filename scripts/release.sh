#!/bin/bash
set -e

# Constants - These should match with the constants in src/main.rs
REPO_OWNER=${REPO_OWNER:-"your-org"}
REPO_NAME=${REPO_NAME:-"mcdp-binaries"}
BINARY_NAME=${BINARY_NAME:-"mcdp-tool"}
REPO="$REPO_OWNER/$REPO_NAME"

# Check if version is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 0.1.0"
  exit 1
fi

VERSION="$1"
TAG="v$VERSION"

# Check if we're on the main branch
# CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
# if [ "$CURRENT_BRANCH" != "main" ] && [ "$CURRENT_BRANCH" != "master" ]; then
#   echo "You must be on main/master branch to release. Current branch: $CURRENT_BRANCH"
#   exit 1
# fi

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
  echo "You have uncommitted changes. Please commit or stash them before releasing."
  exit 1
fi

# Update version in Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Commit the version change
git add Cargo.toml
git commit -m "Bump version to $VERSION"

# Create and push the tag
git tag "$TAG"
git push origin "$TAG"
git push

echo "Release $VERSION initiated!"
echo "GitHub Actions will now build and publish the binaries."
echo "Check the status at: https://github.com/$REPO/actions" 