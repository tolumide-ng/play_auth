# https://github.com/zupzup/rust-docker-web/blob/main/debian/Dockerfile
FROM rust:1.59.0 AS base
ENV SQLX_OFFLINE true
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# -------------------------------------
FROM base AS dev
RUN cargo install cargo-watch
WORKDIR /usr/src/app
COPY . .


# -------------------------------------
FROM base AS builder
ADD . /play_auth
WORKDIR /play_auth
RUN cargo build --release -p auth


FROM debian:buster-slim as debian
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

FROM debian AS prod
WORKDIR /usr/src/app
COPY --from=builder /play_auth/target/release/auth ${APP}/auth
COPY --from=builder /play_auth/configuration ${APP}/configuration
RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

