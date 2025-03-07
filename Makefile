# 命令变量定义
UI_BUILTIN := pnpm
SERVER_BUILTIN := poetry
CONTAINERIZE := docker compose

# 目录变量
UI_DIR := ui
SERVER_DIR := server

# UI 命令
UI_INSTALL_CMD := $(UI_BUILTIN) install
UI_DEV_CMD := $(UI_BUILTIN) run tauri dev
UI_BUILD_CMD := $(UI_BUILTIN) run tauri build

# Server 命令
SERVER_DEV_CMD := $(SERVER_BUILTIN) run start

.PHONY: ui-dev ui-build server-dev docker-build docker-up docker-down help

# UI 相关命令
ui-install:
	cd $(UI_DIR) && $(UI_INSTALL_CMD)

ui-dev:
	cd $(UI_DIR) && $(UI_DEV_CMD)

ui-build:
	cd $(UI_DIR) && $(UI_BUILD_CMD) -- $(PLATFORM)

# Server 相关命令
server-dev:
	cd $(SERVER_DIR) && $(SERVER_DEV_CMD)

# Docker 相关命令
docker-build:
	$(CONTAINERIZE) build

docker-up:
	$(CONTAINERIZE) up -d

docker-down:
	$(CONTAINERIZE) down

# Help
help:
	@echo "可用的命令："
	@echo "  ui-install  - 安装 UI 项目依赖"
	@echo "  ui-dev      - 以开发模式运行 UI (Tauri) 项目"
	@echo "  ui-build    - 构建 UI 项目，使用 PLATFORM=target 指定目标平台"
	@echo "  server-dev  - 运行服务器项目"
	@echo "  docker-build- 构建 Docker 镜像"
	@echo "  docker-up   - 启动 Docker 容器"
	@echo "  docker-down - 停止 Docker 容器" 