# 使用 Python 3.11 作为基础镜像
FROM python:3.11-slim

# 设置工作目录
WORKDIR /app

# 安装 poetry
RUN pip install poetry

# 复制项目文件
COPY pyproject.toml poetry.lock ./
COPY . .

# 安装依赖
RUN poetry config virtualenvs.create false \
    && poetry install --no-interaction --no-ansi

# 暴露端口
EXPOSE 11160

# 启动应用
CMD ["poetry", "run", "start"] 