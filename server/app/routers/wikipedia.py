import asyncio
from typing import AsyncGenerator, Any, Coroutine
from fastapi import APIRouter, HTTPException
from fastapi.responses import StreamingResponse
from pydantic import BaseModel
from app.prompts import get_prompt
from openai import AsyncOpenAI
from openai.resources.chat.completions.completions import AsyncStream, ChatCompletionChunk
from app.routers import API_KEY, API_ENDPOINT, MODEL
from app.internal import logger

router = APIRouter()

client = AsyncOpenAI(api_key=API_KEY, base_url=API_ENDPOINT, max_retries=3, timeout=300)


class WikipediaRequest(BaseModel):
    question: str

    def __str__(self) -> str:
        return f"question: {self.question[:20]}..."


@router.post("/wikipedia/glossary")
async def wikipedia(request: WikipediaRequest) -> StreamingResponse:
    logger.info(f"wikipedia request: {request}")
    prompt = get_prompt("wikipedia", "glossary")
    if not prompt:
        raise HTTPException(status_code=404, detail="prompt not found")
    response = await client.chat.completions.create(
        model=MODEL,
        messages=[{
            "role": "system",
            "content": prompt
        }, {
            "role": "user",
            "content": request.question
        }],
        temperature=0.7,
        max_tokens=2048,
        stream=True,
    )

    return StreamingResponse(parse_response(response), media_type="text/event-stream")


async def parse_response(response: AsyncStream[ChatCompletionChunk]) -> AsyncGenerator[str, None]:
    thinking = True
    async for chunk in response:
        if chunk.choices:
            if "reasoning_content" in chunk.choices[0].delta:
                content = chunk.choices[0].delta.reasoning_content
            else:
                content = chunk.choices[0].delta.content
            yield f"data: {content}\n\n"
            await asyncio.sleep(0.01)
