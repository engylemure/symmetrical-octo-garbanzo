FROM rust:1.74.0-alpine3.18

ENV HOME=/opt/app

WORKDIR $HOME

# Adding system dependencies
RUN apk --no-cache add libpq libaio libstdc++ libc6-compat  musl musl-dev

COPY ./rust /opt/app

RUN cargo build --release && cargo build

CMD ["sh", "./start.sh"]