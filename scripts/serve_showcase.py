#!/usr/bin/env python3
"""Serve the production showcase locally with SPA route fallback."""

import argparse
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlsplit


class ShowcaseHandler(SimpleHTTPRequestHandler):
    def do_GET(self):
        path = Path(self.translate_path(self.path))
        if not path.exists() and not Path(urlsplit(self.path).path).suffix:
            self.path = "/index.html"
        super().do_GET()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--port", type=int, default=8181)
    args = parser.parse_args()
    dist = Path(__file__).resolve().parents[1] / "shadcn-showcase" / "dist-release"
    if not (dist / "index.html").is_file():
        parser.error("No production build found. Run `just showcase-build` first.")
    server = ThreadingHTTPServer(
        ("127.0.0.1", args.port), partial(ShowcaseHandler, directory=str(dist))
    )
    print(f"Showcase: http://127.0.0.1:{args.port}", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
