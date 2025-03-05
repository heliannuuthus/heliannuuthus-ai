from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from app.prompts import get_prompt
from openai import AsyncOpenAI
from app.routers import API_KEY, API_ENDPOINT
from logging import getLogger

logger = getLogger(__name__)
router = APIRouter()

client = AsyncOpenAI(api_key=API_KEY, base_url=API_ENDPOINT, max_retries=3, timeout=300)


class WikipediaRequest(BaseModel):
    keyword: str


@router.post("/wikipedia/glossary")
async def wikipedia(request: WikipediaRequest):
    logger.info(f"wikipedia request: {request}")
    prompt = get_prompt("wikipedia", "glossary")
    if not prompt:
        raise HTTPException(status_code=404, detail="prompt not found")
    keyword = request.keyword
    prompt = prompt.replace("{{keyword}}", keyword)
    response = await client.chat.completions.create(
        model="deepseek-chat",
        messages=[{
            "role": "user",
            "content": prompt
        }],
        temperature=1.3,
        max_tokens=2048,
        stream=True,
    )

    data = ""
    async for chunk in response:
        data += chunk.choices[0].delta.content

    return {"status": "success", "data": data}


@router.get("/test")
async def test():
    logger.info("test request")
    return {"status": "success", "data": "test"}
