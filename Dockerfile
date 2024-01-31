# Build stage
FROM rust:1.73 as builder

WORKDIR /usr/src/batch_worker

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

# Run stage
FROM debian:bookworm-slim as run

# 必要なパッケージをインストール
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates openssl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# 作業ディレクトリを設定
WORKDIR /usr/local/bin

# ビルドステージからバイナリをコピー
COPY --from=builder /usr/src/batch_worker/target/release/batch_worker /usr/local/bin

# コンテナが起動したときに実行されるコマンドを設定
ENTRYPOINT ["/usr/local/bin/batch_worker"]