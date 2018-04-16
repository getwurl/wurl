# This script takes care of testing your crate

set -ex

main() {
    if [ ! -z $STATIC ]; then
      echo Enabling static build
      export OPENSSL_STATIC=true
      export OPENSSL_LIB_DIR=/usr/lib
      export OPENSSL_INCLUDE_DIR=/usr/include/openssl
      export LIBZ_SYS_STATIC=1
      export PKG_CONFIG_ALLOW_CROSS=true
      export PKG_CONFIG_ALL_STATIC=true
    fi

    cargo fmt --all -- --write-mode=diff
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET --release
    cross run --target $TARGET --release -- --help

    if [ ! -z $STATIC ]; then
      ldd target/$TARGET/release/$CRATE_NAME || true
    fi
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
