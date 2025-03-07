from datetime import datetime
from http.client import HTTPException
from requests import Request
from fastapi.responses import JSONResponse


async def http_exception_handler(request: Request, exc: HTTPException):
    return JSONResponse(status_code=exc.status_code,
                        content={
                            "status": exc.status_code,
                            "message": exc.detail,
                            "timestamp": datetime.now().strftime("%Y-%m-%d %H:%M:%S")
                        })


async def exception_handler(request: Request, exc: Exception):
    return JSONResponse(status_code=500,
                        content={
                            "status": 500,
                            "message": str(exc),
                            "timestamp": datetime.now().strftime("%Y-%m-%d %H:%M:%S")
                        })
