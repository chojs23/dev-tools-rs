FROM rust:1.81 AS builder

WORKDIR /app
COPY src src
COPY assets assets
COPY Cargo.toml Cargo.lock ./

RUN apt-get update
RUN apt-get -y install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libfreetype6 libfontconfig1 fontconfig libfontconfig1-dev libxcursor1

RUN --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/git/db \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  set -eux; \
  cargo build --release;\
  cp target/release/dev_tools .

FROM ubuntu:latest AS runtime

WORKDIR /app

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update
# RUN apt-get -y install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libfreetype6 libfontconfig1 fontconfig libfontconfig1-dev libxcursor1 libxrandr2 libxi6 libx11-xcb1 libxkbcommon-x11-0
RUN set -ex \
  apt-get install -y -qq --no-install-recommends \
  apt-utils asciidoc autoconf automake build-essential \
  libargon2-0-dev libbz2-dev libc6-dev libcurl4-openssl-dev \
  libdb-dev libdbd-sqlite3-perl libdbi-perl libdpkg-perl \
  libedit-dev liberror-perl libevent-dev libffi-dev libgeoip-dev \
  libglib2.0-dev libhttp-date-perl libio-pty-perl libjpeg-dev \
  libkrb5-dev liblzma-dev libmagickcore-dev libmagickwand-dev \
  libmysqlclient-dev libncurses5-dev libncursesw5-dev libonig-dev \
  libpq-dev libreadline-dev libserf-1-1 libsodium-dev libsqlite3-dev libssl-dev \
  libsvn1 libsvn-perl libtcl8.6 libtidy-dev libtimedate-perl \
  libtool libwebp-dev libxml2-dev libxml2-utils libxslt1-dev \
  libyaml-dev libyaml-perl llvm locales make mlocate

RUN apt-get -y install x11-apps libxkbcommon-x11-0


COPY --from=builder /app/dev_tools ./

CMD ["./dev_tools"]
# CMD ["xclock"]
