FROM rust:1.52.1

# setting of cargo output dir
ENV CARGO_TARGET_DIR=/tmp/target \
  DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
  && apt-get upgrade -y
RUN apt-get install -y -q \
  ca-certificates \
  locales \
  apt-transport-https\
  libssl-dev \
  libpq-dev \
  pkg-config \
  curl \
  build-essential \
  libdbus-1-dev \
  libsqlite3-dev \
  mariadb-client \
  git \
  wget
RUN locale-gen
RUN echo "install rust tools"
RUN rustup default nightly && rustup update
RUN rustup component add rustfmt
RUN cargo install cargo-watch cargo-make

WORKDIR /work/ipython_diff

COPY . .

