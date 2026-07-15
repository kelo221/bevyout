#!/usr/bin/env python3
"""Produce a bounded inventory of Bevy performance and parallelism candidates."""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from dataclasses import asdict, dataclass
from pathlib import Path


SKIP_DIRS = {
    ".bevyout",
    ".git",
    ".claude",
    "BevyCheatSheet",
    "BevyDocs",
    "bevy_markdown_docs",
    "node_modules",
    "target",
    "third_party",
}

RULES = (
    ("exclusive-world-access", r"(?:&mut\s+World|ExclusiveSystemParam)", "parallelism"),
    (
        "schedule-order",
        r"\.(?:chain|chain_ignore_deferred)\s*\(\s*\)|\.(?:before|after)\s*\(",
        "parallelism",
    ),
    ("deferred-barrier", r"\bapply_deferred\b", "parallelism"),
    ("broad-mutable-resource", r"\bResMut\s*<", "parallelism"),
    ("non-send", r"\bNonSend(?:Mut)?\s*<", "parallelism"),
    ("task-pool", r"\b(?:ComputeTaskPool|AsyncComputeTaskPool|IoTaskPool)\b", "parallelism"),
    ("parallel-iterator", r"\b(?:par_iter|par_iter_mut|into_par_iter)\s*\(", "parallelism"),
    ("sync-wait", r"\b(?:block_on|thread::sleep|park|park_timeout)\s*\(|\.recv(?:_timeout)?\s*\(", "blocking"),
    ("shared-lock", r"\b(?:Mutex|RwLock)\s*<|\.lock\s*\(", "blocking"),
    ("filesystem-io", r"\b(?:std::fs|fs::(?:read|write|read_to_string|File))\b", "io"),
    ("asset-load", r"\b(?:AssetServer|asset_server)\b.*\.load(?:_with_settings)?\s*\(", "assets"),
    ("entity-churn", r"\.(?:spawn|spawn_batch|despawn|despawn_related)\s*\(", "ecs"),
    (
        "full-query-scan",
        r"\.(?:iter|iter_mut)\s*\(\s*&?(?:world|query)|"
        r"\bfor\s+.+\s+in\s+&(?:mut\s+)?(?:query[A-Za-z0-9_]*|"
        r"[A-Za-z_][A-Za-z0-9_]*query[A-Za-z0-9_]*)\b",
        "ecs",
    ),
    (
        "collection-allocation",
        r"\b(?:Vec|HashMap|HashSet|BTreeMap)(?:\s*::<[^;=()]*>)?::(?:new|with_capacity)\s*\(|"
        r"\.collect\s*::<\s*(?:Vec|HashMap|HashSet|BTreeMap)",
        "allocation",
    ),
    ("render-specialization", r"\b(?:SpecializedMeshPipeline|specialize|RenderAsset|RenderCommand)\b", "render"),
)

COMPILED_RULES = tuple(
    (name, re.compile(pattern), concern) for name, pattern, concern in RULES
)


@dataclass(frozen=True)
class Hit:
    category: str
    concern: str
    path: str
    line: int
    text: str


def source_files(root: Path, include_tests: bool):
    for path in sorted(root.rglob("*.rs")):
        parts = path.relative_to(root).parts
        if any(part in SKIP_DIRS for part in parts):
            continue
        if not include_tests and "tests" in parts:
            continue
        yield path


def mask_comments_and_literals(text: str) -> str:
    """Replace Rust comments and string literals with spaces, preserving lines."""
    output = list(text)
    index = 0
    block_depth = 0
    state = "code"
    raw_hashes = 0

    def mask(position: int) -> None:
        if output[position] != "\n":
            output[position] = " "

    while index < len(text):
        if state == "line-comment":
            if text[index] == "\n":
                state = "code"
            else:
                mask(index)
            index += 1
            continue

        if state == "block-comment":
            if text.startswith("/*", index):
                mask(index)
                if index + 1 < len(text):
                    mask(index + 1)
                block_depth += 1
                index += 2
            elif text.startswith("*/", index):
                mask(index)
                if index + 1 < len(text):
                    mask(index + 1)
                block_depth -= 1
                index += 2
                if block_depth == 0:
                    state = "code"
            else:
                mask(index)
                index += 1
            continue

        if state == "string":
            mask(index)
            if text[index] == "\\" and index + 1 < len(text):
                mask(index + 1)
                index += 2
            elif text[index] == '"':
                state = "code"
                index += 1
            else:
                index += 1
            continue

        if state == "raw-string":
            terminator = '"' + ("#" * raw_hashes)
            if text.startswith(terminator, index):
                for offset in range(len(terminator)):
                    mask(index + offset)
                index += len(terminator)
                state = "code"
            else:
                mask(index)
                index += 1
            continue

        if text.startswith("//", index):
            mask(index)
            mask(index + 1)
            index += 2
            state = "line-comment"
            continue
        if text.startswith("/*", index):
            mask(index)
            mask(index + 1)
            index += 2
            block_depth = 1
            state = "block-comment"
            continue

        raw_match = re.match(r"(?:br|cr|r)(?P<hashes>#{0,255})\"", text[index:])
        if raw_match:
            token_length = raw_match.end()
            raw_hashes = len(raw_match.group("hashes"))
            for offset in range(token_length):
                mask(index + offset)
            index += token_length
            state = "raw-string"
            continue

        if text[index] == '"' or (
            text[index] in {"b", "c"}
            and index + 1 < len(text)
            and text[index + 1] == '"'
        ):
            if text[index] != '"':
                mask(index)
                index += 1
            mask(index)
            index += 1
            state = "string"
            continue
        index += 1

    return "".join(output)


def scan_text(path: str, text: str) -> list[Hit]:
    hits: list[Hit] = []
    source_lines = text.splitlines()
    code_lines = mask_comments_and_literals(text).splitlines()
    if len(source_lines) != len(code_lines):
        raise AssertionError("lexical masking changed the source line count")
    for line_number, (source_line, code_line) in enumerate(
        zip(source_lines, code_lines), 1
    ):
        stripped = source_line.strip()
        if not code_line.strip():
            continue
        for category, pattern, concern in COMPILED_RULES:
            if pattern.search(code_line):
                hits.append(Hit(category, concern, path, line_number, stripped[:240]))
    return hits


def scan(root: Path, include_tests: bool = False) -> list[Hit]:
    hits: list[Hit] = []
    for path in source_files(root, include_tests):
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        hits.extend(scan_text(path.relative_to(root).as_posix(), text))
    return hits


def print_markdown(root: Path, hits: list[Hit], maximum: int) -> None:
    grouped: dict[str, list[Hit]] = defaultdict(list)
    for hit in hits:
        grouped[hit.category].append(hit)
    print(f"# Bevy static candidate inventory\n\nRoot: `{root}`")
    print("\nThese are review leads, not confirmed bottlenecks.")
    for category in sorted(grouped):
        category_hits = grouped[category]
        concern = category_hits[0].concern
        print(f"\n## {category} ({concern}, {len(category_hits)} hits)")
        for hit in category_hits[:maximum]:
            print(f"- `{hit.path}:{hit.line}` — `{hit.text}`")
        if len(category_hits) > maximum:
            print(f"- … {len(category_hits) - maximum} more (raise --max-per-category)")


def bounded_hits(hits: list[Hit], maximum: int) -> list[Hit]:
    category_counts: dict[str, int] = defaultdict(int)
    selected = []
    for hit in hits:
        if category_counts[hit.category] < maximum:
            selected.append(hit)
            category_counts[hit.category] += 1
    return selected


def self_test() -> None:
    sample = """
fn exclusive(world: &mut World) { std::thread::sleep(duration); }
fn regular(mut state: ResMut<State>) { values.par_iter().for_each(work); }
app.add_systems(Update, (a, b).chain());
fn scan(query: Query<&Transform>) { for transform in &query { consume(transform); } }
// fn commented(world: &mut World) { block_on(work); }
const EXAMPLE: &str = "mut fake: ResMut<State>";
/* nested /* .chain() */ AssetServer.load("fake") */
"""
    categories = {hit.category for hit in scan_text("src/sample.rs", sample)}
    expected = {
        "exclusive-world-access",
        "sync-wait",
        "broad-mutable-resource",
        "parallel-iterator",
        "schedule-order",
        "full-query-scan",
    }
    if not expected.issubset(categories):
        raise AssertionError(f"missing categories: {sorted(expected - categories)}")
    unexpected = {"asset-load"}
    if categories & unexpected:
        raise AssertionError(f"unexpected categories: {sorted(categories & unexpected)}")
    if len([hit for hit in scan_text("sample.rs", sample) if hit.category == "broad-mutable-resource"]) != 1:
        raise AssertionError("comments or strings produced a broad-mutable-resource hit")
    print("self-test passed")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", default=".")
    parser.add_argument("--json", action="store_true", dest="as_json")
    parser.add_argument("--max-per-category", type=int, default=80)
    parser.add_argument("--include-tests", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return 0
    if args.max_per_category < 1:
        parser.error("--max-per-category must be at least 1")
    root = Path(args.root).resolve()
    if not root.is_dir():
        parser.error(f"root is not a directory: {root}")
    hits = scan(root, args.include_tests)
    if args.as_json:
        selected = bounded_hits(hits, args.max_per_category)
        category_counts: dict[str, int] = defaultdict(int)
        for hit in hits:
            category_counts[hit.category] += 1
        json.dump(
            {
                "root": str(root),
                "candidate_count": len(hits),
                "reported_candidate_count": len(selected),
                "truncated": len(selected) < len(hits),
                "category_counts": dict(sorted(category_counts.items())),
                "candidates": [asdict(hit) for hit in selected],
            },
            sys.stdout,
            indent=2,
        )
        print()
    else:
        print_markdown(root, hits, args.max_per_category)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
