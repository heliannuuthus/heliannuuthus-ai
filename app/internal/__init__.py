import logging
from app.internal.exception import exception_handler, http_exception_handler


logger = logging.getLogger(__name__)

for logger_name in ["uvicorn", "uvicorn.access", "uvicorn.error", "fastapi"]:
    logging.getLogger(logger_name).handlers = logger.handlers
    
