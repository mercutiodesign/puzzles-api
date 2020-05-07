#!/usr/bin/env python

import asyncio

from sanic import Sanic
from sanic.response import text

app = Sanic(name="puzzles")


@app.route("/")
async def home(_request):
    return text("home")


@app.route("/loopy/<w:int>x<h:int>d<difficulty:[entd]>")
async def loopy(request, w, h, difficulty):
    assert w > 2
    assert h > 2
    assert difficulty in "entd"

    args = []
    for var in ["seed", "count"]:
        value = request.args.get(var, False)
        if value:
            args.append(f"--{var}")
            args.append(value[0])

    proc = await asyncio.create_subprocess_exec(
        "loopygenerator",
        f"{w}x{h}d{difficulty}",
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

    if proc.returncode:
        return text((stdout).decode(), status=500)
    return text(stdout.decode())


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8000)
