#!/usr/bin/env python3
import os
import signal
import subprocess
import sys
import threading
import time

ROOT = os.path.dirname(os.path.abspath(__file__))
COLORS = {"[db]": "\033[36m", "[api]": "\033[33m", "[web]": "\033[32m"}
RESET = "\033[0m"
procs = []


def spawn(cmd, cwd, tag):
    p = subprocess.Popen(
        cmd,
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
    )
    threading.Thread(
        target=lambda: [
            sys.stdout.write(f"{COLORS[tag]}{tag}{RESET} {l}") or sys.stdout.flush()
            for l in p.stdout
        ],
        daemon=True,
    ).start()
    return p


def stop(sig=0, frame=None, code=0):
    for p in procs:
        if p.poll() is None:
            p.terminate()
    for p in procs:
        try:
            p.wait(timeout=5)
        except subprocess.TimeoutExpired:
            p.kill()
    sys.exit(code)


def main():
    signal.signal(signal.SIGINT, stop)
    signal.signal(signal.SIGTERM, stop)
    m = spawn(["cargo", "run"], os.path.join(ROOT, "crud_api", "migration"), "[db]")
    if m.wait() != 0:
        sys.exit("[db] falló")
    procs.append(spawn(["cargo", "run"], os.path.join(ROOT, "crud_api"), "[api]"))
    procs.append(spawn(["pnpm", "dev"], os.path.join(ROOT, "frontend"), "[web]"))
    while True:
        for p in procs:
            if p.poll() is not None:
                stop(code=1)
        time.sleep(0.2)


if __name__ == "__main__":
    main()
