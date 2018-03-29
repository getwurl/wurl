FROM rust
WORKDIR /tmp/build
COPY . /tmp/build
RUN ["cargo", "install", "--root", "/usr/local"]

#FROM ubuntu
#RUN apt-get update && apt-get install -y libssl-dev
#COPY --from=0 /usr/local/bin/wsy /usr/local/bin
ENTRYPOINT ["/usr/local/bin/wsy"]
