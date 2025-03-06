from app.internal.exception import http_exception_handler, exception_handler
from dotenv import load_dotenv
load_dotenv()
import fastapi
from fastapi import HTTPException
from app.routers import wikipedia
import os

os.makedirs('logs', exist_ok=True)

app = fastapi.FastAPI()

app.include_router(wikipedia.router)

app.add_exception_handler(HTTPException, http_exception_handler)
app.add_exception_handler(Exception, exception_handler)

def start():
    import uvicorn
    from app.internal.logging import logger
    logger.info("Starting server...")
    uvicorn.run(app, host="0.0.0.0", port=11160, log_config=None)
