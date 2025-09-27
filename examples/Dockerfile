# RL解析器生成器 Docker镜像
# 多阶段构建，优化镜像大小

# 构建阶段
FROM rust:1.70-slim as builder

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制Cargo文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src/ ./src/
COPY examples/ ./examples/
COPY templates/ ./templates/

# 构建应用
RUN cargo build --release

# 运行阶段
FROM debian:bullseye-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -r -s /bin/false rluser

# 设置工作目录
WORKDIR /app

# 复制构建的二进制文件
COPY --from=builder /app/target/release/rl /usr/local/bin/
COPY --from=builder /app/examples /app/examples
COPY --from=builder /app/templates /app/templates

# 复制配置文件
COPY examples/rl-config.toml /app/config.toml

# 创建必要的目录
RUN mkdir -p /app/logs /app/data /app/knowledge_graphs /app/models

# 设置权限
RUN chown -R rluser:rluser /app
USER rluser

# 暴露端口
EXPOSE 8080 9090

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 启动命令
CMD ["rl", "server", "--config", "/app/config.toml"]
