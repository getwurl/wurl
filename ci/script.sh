# This script takes care of testing your crate

set -ex

main() {
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET --release
    cross run --target $TARGET --release -- --help

    if [ -z $DOCKER_DEPLOY ]; then
        echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
        docker build -t "$DOCKER_USERNAME/$DOCKER_REPONAME:$TRAVIS_TAG" -t "$DOCKER_USERNAME/$DOCKER_REPONAME:latest" .
        docker push "$DOCKER_USERNAME/$DOCKER_REPONAME"
    fi
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
