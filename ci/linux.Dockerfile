
FROM rust:latest
RUN apt-get update \
  && apt-get -y install ninja-build clang