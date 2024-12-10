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

RUN apt-get update
RUN apt-get -y install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libfreetype6 libfontconfig1 fontconfig libfontconfig1-dev libxcursor1 libxrandr2 libxi6 libx11-xcb1 libxkbcommon-x11-0

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get -y install x11-apps libgl1-mesa-dev

COPY --from=builder /app/dev_tools ./

CMD ["./dev_tools"]
# CMD ["xclock"]
