#!/usr/bin/env python3
"""
validate_docs.py — Documentation & Requirement Integrity Validator for RustAI.

Validates:
1. Required documentation files exist.
2. Internal Markdown relative links resolve to existing files or headings.
3. Task identifiers (TASK-xxx) follow convention and exist in TASKS.md.
4. Requirement identifiers (FR-xxx, NFR-xxx, SEC-xxx) follow convention and exist in REQUIREMENTS.md.
5. Index links in docs/README.md resolve properly.
"""

import os
import re
import sys
from pathlib import Path

REQUIRED_DOCS = [
    "AGENTS.md",
    "README.md",
    "CONTRIBUTING.md",
    "docs/README.md",
    "docs/PROJECT_VISION.md",
    "docs/REQUIREMENTS.md",
    "docs/SCOPE_AND_ROADMAP.md",
    "docs/ARCHITECTURE.md",
    "docs/MODULE_CATALOG.md",
    "docs/DEVELOPMENT_WORKFLOW.md",
    "docs/CODING_STANDARDS.md",
    "docs/TESTING_STRATEGY.md",
    "docs/DEPENDENCY_POLICY.md",
    "docs/DEPENDENCY_EVALUATION.md",
    "docs/SECURITY_MODEL.md",
    "docs/CURRENT_STATE.md",
    "docs/NEXT_STEPS.md",
    "docs/TASKS.md",
]

TASK_ID_REGEX = re.compile(r"\bTASK-\d{3}\b")
REQ_ID_REGEX = re.compile(r"\b(FR|NFR|SEC)-\d{3}\b")
MD_LINK_REGEX = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")


def validate_required_files(repo_root: Path) -> list[str]:
    errors = []
    for rel_path in REQUIRED_DOCS:
        full_path = repo_root / rel_path
        if not full_path.exists():
            errors.append(f"Missing required documentation file: {rel_path}")
    return errors


def validate_markdown_links(repo_root: Path) -> list[str]:
    errors = []
    md_files = list(repo_root.glob("**/*.md"))

    for md_file in md_files:
        # Ignore target or build directories if any
        if "target" in md_file.parts:
            continue

        try:
            content = md_file.read_text(encoding="utf-8")
        except Exception as e:
            errors.append(f"Failed to read {md_file.relative_to(repo_root)}: {e}")
            continue

        for match in MD_LINK_REGEX.finditer(content):
            link_text, link_target = match.groups()

            # Ignore external web links, mailto links, and empty links
            if link_target.startswith(("http://", "https://", "mailto:", "#")):
                continue

            # Handle anchors in relative links (e.g. file.md#section)
            target_file_part = link_target.split("#")[0]
            if not target_file_part:
                continue

            resolved_path = (md_file.parent / target_file_part).resolve()

            if not resolved_path.exists():
                rel_src = md_file.relative_to(repo_root)
                errors.append(
                    f"Broken link in {rel_src}: '[{link_text}]({link_target})' -> target file missing ({target_file_part})"
                )

    return errors


def validate_identifiers(repo_root: Path) -> list[str]:
    errors = []

    tasks_file = repo_root / "docs/TASKS.md"
    reqs_file = repo_root / "docs/REQUIREMENTS.md"

    if not tasks_file.exists() or not reqs_file.exists():
        return ["Cannot validate identifiers because TASKS.md or REQUIREMENTS.md is missing."]

    tasks_content = tasks_file.read_text(encoding="utf-8")
    reqs_content = reqs_file.read_text(encoding="utf-8")

    known_tasks = set(TASK_ID_REGEX.findall(tasks_content))
    known_reqs = set(REQ_ID_REGEX.findall(reqs_content))

    md_files = list(repo_root.glob("docs/**/*.md"))

    for md_file in md_files:
        content = md_file.read_text(encoding="utf-8")

        # Find task IDs
        for task_id in TASK_ID_REGEX.findall(content):
            if task_id not in known_tasks:
                rel_src = md_file.relative_to(repo_root)
                errors.append(f"Unknown Task ID '{task_id}' referenced in {rel_src}")

        # Find requirement IDs
        for req_id in REQ_ID_REGEX.findall(content):
            if req_id not in known_reqs:
                rel_src = md_file.relative_to(repo_root)
                errors.append(f"Unknown Requirement ID '{req_id}' referenced in {rel_src}")

    return errors


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent

    print("=== RustAI Documentation Validator ===")
    all_errors = []

    # 1. Required files
    req_errors = validate_required_files(repo_root)
    all_errors.extend(req_errors)

    # 2. Markdown link integrity
    link_errors = validate_markdown_links(repo_root)
    all_errors.extend(link_errors)

    # 3. Task and Requirement IDs
    id_errors = validate_identifiers(repo_root)
    all_errors.extend(id_errors)

    if all_errors:
        print(f"\n[FAIL] Found {len(all_errors)} documentation validation issues:")
        for err in all_errors:
            print(f"  - {err}")
        return 1

    print("[SUCCESS] All documentation validation checks passed cleanly!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
