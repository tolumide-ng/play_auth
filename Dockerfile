# https://github.com/zupzup/rust-docker-web/blob/main/debian/Dockerfile
FROM rust:1.59.0 AS builder

RUN USER=root cargo new --bin play_auth
WORKDIR /play_auth
COPY . ./

ENV SQLX_OFFLINE true
RUN cargo build --release -p auth
RUN rm src/*.rs target/release/deps/auth*

RUN cargo build --release -p auth

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=app_user

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

RUN chown -R $APP_USER:$APP_USER /usr/src/app
RUN chown -R $APP_USER:$APP_USER /play_auth

COPY --from=builder /play_auth/target/release/auth ${APP}/auth
COPY configuration ${APP}/configuration

USER $APP_USER
WORKDIR ${APP}
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

ENTRYPOINT [ "./auth" ]
