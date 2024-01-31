FROM rust:1.73 as builder

WORKDIR  /usr/local/bin

COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release

# 実際のソースコードをコピー
COPY src ./src

# アプリケーションをリリースモードでビルド
RUN touch src/main.rs \
    && cargo build --release

# 実行ステージ
FROM debian:buster-slim

# 必要なパッケージをインストール
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 作業ディレクトリを設定
WORKDIR /usr/local/bin

# ビルドステージからバイナリをコピー
COPY --from=builder /usr/src/batch_worker/target/release/batch_worker .

# コンテナが起動したときに実行されるコマンドを設定
CMD ["./batch_worker"]