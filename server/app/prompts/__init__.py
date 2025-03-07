import os
import yaml
from typing import Dict, Optional


class Prompt:
    name: str
    content: str

    def __init__(self, **kwargs):
        self.__dict__.update(kwargs)


__PROMPTS = {}

for file in os.listdir(os.path.dirname(__file__)):
    name, ext = os.path.splitext(file)
    if ext in ['.yml', '.yaml']:
        with open(os.path.join(os.path.dirname(__file__), file), 'r', encoding='utf-8') as f:
            if name not in __PROMPTS:
                __PROMPTS[name] = {}
            __PROMPTS[name].update({item["name"]: Prompt(**item) for item in yaml.safe_load(f)})


def global_prompt() -> Dict[str, Dict[str, Prompt]]:
    return __PROMPTS


def get_prompt(chunk_name: str, prompt_name: str) -> Optional[str]:
    if prompt := __PROMPTS.get(chunk_name, {}).get(prompt_name):
        return prompt.content
    return None
