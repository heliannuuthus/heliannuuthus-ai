# 构建阶段
FROM node:20-slim as builder

# 安装 Rust 和其他必要的依赖
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libwebkit2gtk-4.0-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    && rm -rf /var/lib/apt/lists/*

# 安装 Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

# 复制项目文件
COPY . .

# 安装依赖并构建
RUN npm install
RUN npm run tauri build

# 运行阶段
FROM debian:bookworm-slim

RUN apt-get update && sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 从构建阶段复制构建好的应用
COPY --from=builder /app/src-tauri/target/release/bundle /app/

CMD ["./ui"] 