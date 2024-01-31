# ビルドステージ
FROM rust:1.73 as builder

# 作業ディレクトリを設定
WORKDIR /usr/src/batch_worker

COPY Cargo.toml Cargo.lock ./

# 実際のソースコードをコピー
COPY src ./src

# アプリケーションをリリースモードでビルド
RUN touch src/main.rs \
    && cargo build --release

# 実行ステージ
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
