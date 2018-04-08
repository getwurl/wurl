# This script takes care of testing your crate

set -ex

main() {
    cargo fmt --all -- --write-mode=diff
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET --release
    cross run --target $TARGET --release -- --help
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
