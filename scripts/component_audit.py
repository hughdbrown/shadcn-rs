#!/usr/bin/env python3
"""Audit component coverage and lightweight test inventory for shadcn-rs."""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import asdict, dataclass
from pathlib import Path


@dataclass
class ComponentRecord:
    name: str
    path: str
    function_components: int
    tests: int
    lines: int


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def parse_component_records(root: Path) -> list[ComponentRecord]:
    component_dir = root / "shadcn-rs" / "src" / "components"
    records: list[ComponentRecord] = []

    for path in sorted(component_dir.glob("*.rs")):
        if path.name == "mod.rs":
            continue
        text = path.read_text()
        records.append(
            ComponentRecord(
                name=path.stem,
                path=str(path.relative_to(root)),
                function_components=len(re.findall(r"#\[function_component\(", text)),
                tests=len(re.findall(r"#\[test\]", text)),
                lines=text.count("\n") + (0 if text.endswith("\n") else 1),
            )
        )

    return records


def exported_modules(root: Path) -> list[str]:
    mod_rs = (root / "shadcn-rs" / "src" / "components" / "mod.rs").read_text()
    return sorted(set(re.findall(r"^pub mod ([a-z_]+);", mod_rs, re.MULTILINE)))


def showcase_pages(root: Path) -> list[str]:
    pages_dir = root / "shadcn-showcase" / "src" / "pages" / "components"
    return sorted(path.stem.replace("_page", "") for path in pages_dir.glob("*_page.rs"))


def build_report(root: Path) -> dict:
    records = parse_component_records(root)
    modules = exported_modules(root)
    pages = showcase_pages(root)
    missing_pages = sorted(set(modules) - set(pages) - {"direction"})
    extra_pages = sorted(set(pages) - set(modules))

    return {
        "component_count": len(records),
        "components": [asdict(record) for record in records],
        "showcase": {
            "module_count": len(modules),
            "page_count": len(pages),
            "missing_pages": missing_pages,
            "extra_pages": extra_pages,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--json",
        action="store_true",
        help="emit the full report as JSON",
    )
    args = parser.parse_args()

    root = repo_root()
    report = build_report(root)

    if args.json:
        print(json.dumps(report, indent=2))
        return 0

    print(f"components: {report['component_count']}")
    showcase = report["showcase"]
    print(f"showcase modules: {showcase['module_count']}")
    print(f"showcase pages: {showcase['page_count']}")
    print(f"missing showcase pages: {', '.join(showcase['missing_pages']) or 'none'}")
    print(f"extra showcase pages: {', '.join(showcase['extra_pages']) or 'none'}")

    print("\ncomponents by size:")
    for record in sorted(report["components"], key=lambda item: (-item["lines"], item["name"])):
        print(
            f"{record['lines']:4d} lines  {record['tests']:2d} tests  "
            f"{record['function_components']:2d} fc  {record['name']}"
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
