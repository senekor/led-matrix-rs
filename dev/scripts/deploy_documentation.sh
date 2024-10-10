#!/bin/bash
set -euo pipefail

if git diff-files --quiet || git diff-index HEAD --quiet
then
  echo "Dirty worktree. commit / stash / whatever and try again."
  exit 1
fi

git checkout gh-pages
git reset --hard main

rm -rf docs
echo "Building documentation..."
cargo doc --quiet
mv target/doc docs

git add --all
git commit --quiet --message "deploy documentation"

git push --force-with-lease

git checkout main
