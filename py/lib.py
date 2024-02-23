import requests

class Language:
    def __init__(self, lang: str, locale: str):
        self.lang = lang
        self.locale = locale

class Body:
    def __init__(self, content: str, encoding: str, other: dict[str, any]