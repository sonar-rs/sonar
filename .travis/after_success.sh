#!/usr/bin/env sh

set -ex

main() {
    if [ $TARGET = "x86_64-unknown-linux-gnu" ] && [ $TRAVIS_RUST_VERSION = "stable" ] &&
            [ $TRAVIS_BRANCH = "master" ] && [ $TRAVIS_PULL_REQUEST = false ]; then
        cargo doc --package sonar --no-deps
        cargo doc --package sonar-window --no-deps

        echo '<meta http-equiv=refresh content=0;url=sonar/index.html>' > target/doc/index.html

        pip install --user ghp-import
        ghp-import -n target/doc

        git push -qf https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
    fi
}

main
