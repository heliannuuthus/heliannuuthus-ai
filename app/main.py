from dotenv import load_dotenv

load_dotenv()
import fastapi
from fastapi import HTTPException, Request
from fastapi.responses import JSONResponse
from datetime import datetime
from app.routers import wikipedia
from logging.config import fileConfig
import os

os.makedirs('logs', exist_ok=True)

app = fastapi.FastAPI()

app.include_router(wikipedia.router)


@app.exception_handler(HTTPException)
async def http_exception_handler(request: Request, exc: HTTPException):
    return JSONResponse(status_code=exc.status_code,
                        content={
                            "status": exc.status_code,
                            "message": exc.detail,
                            "timestamp": datetime.now().strftime("%Y-%m-%d %H:%M:%S")
                        })


@app.exception_handler(Exception)
async def exception_handler(request: Request, exc: Exception):
    return JSONResponse(status_code=500,
                        content={
                            "status": 500,
                            "message": str(exc),
                            "timestamp": datetime.now().strftime("%Y-%m-%d %H:%M:%S")
                        })


def start():
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=11160, log_config="logging.json")
