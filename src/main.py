#!/usr/bin/env python

import asyncio
import re

from sanic import Sanic
from sanic.response import empty, text

# size: either {WIDTH}x{HEIGHT} or just one number for a square board
# difficulty: easy (default), normal, tricky, hard
PARAMS = re.compile(r"(\d+)(?:x(\d+))?(?:d[enth])?")
app = Sanic(name="puzzles")


@app.route("/")
async def home(_):
    return text("home")


@app.route("/favicon.ico")
async def ignore(_):
    return empty()


@app.route("/loopy/<params:str>")
async def loopy(request, params: str):
    m = PARAMS.fullmatch(params)
    if not m:
        return text(
            f"invalid parameter, expected something like '4x7de', got {params}",
            status=400,
        )

    w, h = m.groups()
    if int(w) <= 2 or (h and int(h) <= 2):
        return text(
            f"invalid width / height, require at least 3x3 not {w}x{h}", status=400
        )

    args = []
    for var in ["seed", "count"]:
        value = request.args.get(var, False)
        if value:
            args.append(f"--{var}")
            args.append(value)

    proc = await asyncio.create_subprocess_exec(
        "loopygenerator",
        params,
        *args,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.STDOUT,
    )

    try:
        stdout, _ = await asyncio.wait_for(proc.communicate(), timeout=2.0)
    except asyncio.TimeoutError:
        try:
            # try to give the process time to terminate
            proc.terminate()
            await asyncio.wait_for(proc.wait(), timeout=0.5)
        except asyncio.TimeoutError:
            # didn't respond in time: kill it
            await proc.kill()
        return text("Timeout", status=400)

    status = 500 if proc.returncode else 200
    return text(stdout.decode(), status=status)


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8000)
