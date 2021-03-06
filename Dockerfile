FROM rust:latest as builder
# 要求 Rust 在 scratch 镜像下运行
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /backend

COPY . ./

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

WORKDIR /backend
COPY --from=builder /backend/target/x86_64-unknown-linux-musl/release/curriculum_board_backend .
COPY --from=builder /backend/static/cedict_ts.u8 ./static/


EXPOSE 11451

ENTRYPOINT ["/backend/curriculum_board_backend"]