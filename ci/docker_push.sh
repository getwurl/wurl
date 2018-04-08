#!/bin/bash
echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
docker build -t "$DOCKER_USERNAME/$DOCKER_REPONAME:$TRAVIS_TAG" -t "$DOCKER_USERNAME/$DOCKER_REPONAME:latest" .
docker push "$DOCKER_USERNAME/$DOCKER_REPONAME"
