#!/usr/bin/env python3
"""Exercise every showcase route and key interactions in an isolated headless Chrome session.

Requires Chrome and a matching ChromeDriver on PATH (or CHROMEDRIVER).
Start `just showcase-preview` first. Artifacts go to target/showcase-smoke/.
"""

import argparse
import base64
import json
import os
from pathlib import Path
import re
import shutil
import socket
import subprocess
import time
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen


def request(url, data=None, method=None):
    req = Request(
        url,
        data=None if data is None else json.dumps(data).encode(),
        headers={"Content-Type": "application/json"},
        method=method,
    )
    try:
        with urlopen(req, timeout=30) as response:
            result = json.load(response).get("value")
    except HTTPError as exc:
        raise RuntimeError(exc.read().decode()) from exc
    if isinstance(result, dict) and "error" in result:
        raise RuntimeError(result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--url", default="http://127.0.0.1:8181")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    artifacts = root / "target/showcase-smoke"
    artifacts.mkdir(parents=True, exist_ok=True)
    driver = os.environ.get("CHROMEDRIVER") or shutil.which("chromedriver")
    if not driver:
        parser.error("Install ChromeDriver matching Chrome, or set CHROMEDRIVER to its path.")
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        port = sock.getsockname()[1]
    endpoint = f"http://127.0.0.1:{port}"
    report = {"url": args.url, "routes": [], "interactions": [], "console_errors": []}
    session = None
    with (artifacts / "chromedriver.log").open("w") as log:
        process = subprocess.Popen([driver, f"--port={port}"], stdout=log, stderr=log)
        try:
            for _ in range(100):
                try:
                    request(endpoint + "/status")
                    break
                except (URLError, OSError):
                    time.sleep(0.1)
            session = request(endpoint + "/session", {"capabilities": {"alwaysMatch": {
                "browserName": "chrome",
                "goog:chromeOptions": {"args": ["--headless=new", "--window-size=1440,1000"]},
                "goog:loggingPrefs": {"browser": "ALL"},
            }}})["sessionId"]
            base = endpoint + "/session/" + session

            def js(code, *arguments):
                return request(base + "/execute/sync", {"script": code, "args": list(arguments)})

            def wait(code):
                deadline = time.monotonic() + 20
                while time.monotonic() < deadline:
                    if js("return " + code):
                        return
                    time.sleep(0.05)
                raise AssertionError(f"Timed out: {code}")

            def go(path):
                request(base + "/url", {"url": args.url.rstrip("/") + path})
                wait("!!document.querySelector('.app-content')")

            def click(selector):
                # Use WebDriver's element click to exercise hit testing and default actions.
                element = request(base + "/element", {"using": "css selector", "value": selector})
                identifier = element["element-6066-11e4-a52e-4f735466cecf"]
                request(base + f"/element/{identifier}/click", {})

            def type_text(selector, value):
                element = request(base + "/element", {"using": "css selector", "value": selector})
                identifier = element["element-6066-11e4-a52e-4f735466cecf"]
                request(base + f"/element/{identifier}/value", {"text": value})

            def screenshot(name):
                (artifacts / name).write_bytes(base64.b64decode(request(base + "/screenshot")))

            go("/")
            wait("document.querySelectorAll('.catalog-link').length === 66")
            screenshot("catalog.png")
            type_text("#component-filter", "message")
            wait("document.querySelectorAll('.catalog-link').length === 2")
            report["interactions"].append("catalog search")

            paths = re.findall(r'#\[at\("(/components/[^\"]+)"\)\]', (root / "shadcn-showcase/src/routes.rs").read_text())
            modules = set(re.findall(r'^pub mod (\w+);', (root / "shadcn-rs/src/components/mod.rs").read_text(), re.M))
            assert {path.rsplit("/", 1)[-1].replace("-", "_") for path in paths} == modules
            for path in paths:
                go(path)
                wait("!!document.querySelector('.component-title')")
                assert js("return document.querySelectorAll('.example-demo').length") > 0, path
                assert js("return document.styleSheets.length") >= 5, path
                report["routes"].append({"path": path, "title": js("return document.querySelector('.component-title').textContent")})
            print(f"Loaded all {len(paths)} component routes.", flush=True)

            go("/components/message")
            type_text('input[aria-label="Message text"]', "Hello from the production build")
            click('.example-demo button[type="submit"]')
            wait("document.querySelector('.example-demo').textContent.includes('Hello from the production build')")
            wait("document.querySelector('input[aria-label=\"Message text\"]').value === ''")
            screenshot("conversation.png")
            report["interactions"].append("send local message")

            go("/components/attachment")
            click('button[aria-label="Remove attachment"]')
            wait("!document.querySelector('.attachment')")
            click('button[aria-label="Attach sample file"]')
            wait("!!document.querySelector('.attachment')")
            click('.example-demo .btn')
            wait("!!document.querySelector('.attachment-status-uploading')")
            report["interactions"].append("remove, restore, and advance attachment")

            go("/components/bubble")
            click('.bubble-reactions button')
            wait("document.querySelector('.bubble-reactions button').getAttribute('aria-pressed') === 'true'")
            report["interactions"].append("toggle bubble reaction")

            go("/components/marker")
            click('.example-demo .btn')
            wait("!!document.querySelector('.marker-shimmer')")
            report["interactions"].append("toggle processing marker")

            go("/components/direction")
            click('.example-demo .btn')
            wait("!!document.querySelector('.example-demo [dir=rtl]')")
            report["interactions"].append("switch text direction")

            go("/components/questionnaire")
            click('.questionnaire-choice')
            click('.questionnaire-next')
            type_text('.questionnaire-input', "Production demo")
            click('.questionnaire-submit')
            wait("document.querySelector('.example-demo [role=status]').textContent.includes('(1 submission)')")
            screenshot("questionnaire.png")
            report["interactions"].append("complete questionnaire exactly once")

            go("/components/message-scroller")
            wait("document.querySelector('#conversation-viewport').scrollTop > 0")
            js("document.querySelector('#conversation-viewport').scrollTop = 0")
            wait("document.querySelector('#conversation-viewport').scrollTop === 0")
            click('.message-scroller-button')
            wait("document.querySelector('#conversation-viewport').scrollTop > 0")
            report["interactions"].append("jump to latest message")

            go("/")
            click('.theme-toggle')
            wait("document.documentElement.getAttribute('data-theme') === 'dark'")
            screenshot("catalog-dark.png")
            request(base + "/window/rect", {"width": 390, "height": 844})
            wait("window.innerWidth < 768")
            click('.mobile-menu-btn')
            wait("!!document.querySelector('.sidebar-open')")
            wait("document.querySelector('.sidebar-open').getBoundingClientRect().left >= -1")
            click('.sidebar a[href$=\"/components/message\"]')
            wait("!!document.querySelector('.component-title') && !document.querySelector('.sidebar-open')")
            wait("document.querySelector('.sidebar').getBoundingClientRect().right <= 1")
            screenshot("mobile.png")
            report["interactions"].append("theme and mobile navigation")

            entries = request(base + "/log", {"type": "browser"})
            report["console_errors"] = [entry for entry in entries if entry["level"] == "SEVERE"]
            assert not report["console_errors"], report["console_errors"]
            report["passed"] = True
            print(f"Passed {len(report['interactions'])} interaction checks; no browser console errors.")
        except Exception:
            if session:
                (artifacts / "failure.png").write_bytes(base64.b64decode(request(base + "/screenshot")))
            raise
        finally:
            (artifacts / "report.json").write_text(json.dumps(report, indent=2) + "\n")
            if session:
                request(endpoint + "/session/" + session, method="DELETE")
            process.terminate()
            process.wait(timeout=10)


if __name__ == "__main__":
    main()
