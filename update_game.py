import requests
import json

with open("config/config.json") as f:
    config = json.load(f)

try:
    r = requests.head("https://ak.hypergryph.com/downloads/android_lastest", timeout=30)
    location = r.headers.get("location")

    if location.endswith(".apk"):  # type: ignore
        with open("game.txt", "w") as f:
            f.write(location)  # type: ignore
except Exception:
    pass
