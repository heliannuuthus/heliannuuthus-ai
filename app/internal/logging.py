from datetime import datetime
import logging
from logging.handlers import TimedRotatingFileHandler
from app.internal import logger
import sys
import os

log_dir = "logs"
os.makedirs(log_dir, exist_ok=True)


class TimedRotatingFileNameHandler(TimedRotatingFileHandler):
    def __init__(self, filename, when, interval, backupCount, encoding=None):
        super().__init__(filename, when, interval, backupCount, encoding)
    
    def rotation_filename(self, default_name):
        """自定义日志文件名称"""
        base_filename, ext = os.path.splitext(default_name)
        new_filename = f"{base_filename}-{datetime.now().strftime('%Y-%m-%d')}{ext}"
        return new_filename

# 创建日志格式
log_formatter = logging.Formatter("%(asctime)s %(levelname)s %(pathname)s:%(lineno)d: %(message)s")

# 控制台日志
console_handler = logging.StreamHandler(sys.stderr)
console_handler.setFormatter(log_formatter)
file_handler = TimedRotatingFileNameHandler(
    filename=os.path.join(log_dir, "app.log"),
    when="midnight",
    interval=1,
    backupCount=7,
    encoding="utf-8",
)
file_handler.setFormatter(log_formatter)
logger.setLevel(logging.INFO)
logger.addHandler(console_handler)
logger.addHandler(file_handler)
