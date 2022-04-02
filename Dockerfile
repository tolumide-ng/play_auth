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
RUN USER=root cargo new --bin play_auth
WORKDIR /play_auth
COPY . ./

RUN cargo build --release -p auth \
    && rm src/*.rs target/release/deps/auth*

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=app_user

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}


# -------------------------------------
FROM debian AS prod
COPY --from=builder /play_auth/target/release/auth auth
COPY configuration configuration


# USER $APP_USER
# WORKDIR ${APP}

CMD [ "./auth" ]

