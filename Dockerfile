FROM rust:1.73 as builder

WORKDIR /usr/src/batch_worker

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

# 必要なパッケージをインストール
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# 作業ディレクトリを設定
WORKDIR /usr/local/bin

# ビルドステージからバイナリをコピー
COPY --from=builder /usr/src/batch_worker/target/release/batch_worker .

# コンテナが起動したときに実行されるコマンドを設定
CMD ["./batch_worker"]
