#!/usr/bin/env python3
"""BloxPM: create, inspect, verify and safely install .blox packages."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
import re
from pathlib import Path, PurePosixPath
import sys
import tempfile
import zipfile

FORMAT_NAME = "Blox Package"
FORMAT_VERSION = 1
MANIFEST_NAME = "manifest.json"
MAX_FILES = 10000
MAX_UNCOMPRESSED = 512 * 1024 * 1024


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def safe_member(name: str) -> PurePosixPath:
    path = PurePosixPath(name)
    if path.is_absolute() or not path.parts or any(part in ("", ".", "..") for part in path.parts):
        raise ValueError(f"Unsafe package path: {name}")
    if "\\" in name or "\x00" in name:
        raise ValueError(f"Invalid package path: {name}")
    return path


def read_package(archive: zipfile.ZipFile) -> tuple[dict, dict[str, zipfile.ZipInfo]]:
    infos = archive.infolist()
    if len(infos) > MAX_FILES:
        raise ValueError("Package contains too many entries")
    total = 0
    by_name: dict[str, zipfile.ZipInfo] = {}
    for info in infos:
        if info.is_dir():
            continue
        safe_member(info.filename)
        if info.filename in by_name:
            raise ValueError(f"Duplicate package entry: {info.filename}")
        total += info.file_size
        by_name[info.filename] = info
    if total > MAX_UNCOMPRESSED:
        raise ValueError("Package exceeds the 512 MiB unpacked size limit")
    if MANIFEST_NAME not in by_name:
        raise ValueError("manifest.json is missing")
    manifest = json.loads(archive.read(MANIFEST_NAME))
    if manifest.get("format") != FORMAT_NAME or manifest.get("format_version") != FORMAT_VERSION:
        raise ValueError("Unsupported Blox Package format")
    for required in ("id", "name", "version", "files"):
        if required not in manifest:
            raise ValueError(f"Manifest field is missing: {required}")
    if not re.fullmatch(r"[a-z0-9][a-z0-9._-]{0,127}", manifest["id"]):
        raise ValueError("Invalid package id; use lowercase letters, digits, dot, underscore or hyphen")
    if not isinstance(manifest["files"], list):
        raise ValueError("manifest.files must be an array")
    if len(manifest["files"]) > MAX_FILES:
        raise ValueError("Manifest contains too many files")
    return manifest, by_name


def verify(archive_path: Path) -> dict:
    with zipfile.ZipFile(archive_path, "r") as archive:
        manifest, entries = read_package(archive)
        expected = set()
        for item in manifest["files"]:
            if not isinstance(item, dict):
                raise ValueError("Each manifest file entry must be an object")
            name = item.get("path", "")
            safe_member(name)
            archive_name = f"payload/{name}"
            if archive_name in expected:
                raise ValueError(f"Duplicate manifest path: {name}")
            expected.add(archive_name)
            if archive_name not in entries:
                raise ValueError(f"Package file is missing: {name}")
            if not re.fullmatch(r"[0-9a-f]{64}", str(item.get("sha256", ""))):
                raise ValueError(f"Invalid SHA-256 value: {name}")
            digest_obj = hashlib.sha256()
            actual_size = 0
            with archive.open(archive_name) as stream:
                for chunk in iter(lambda: stream.read(1024 * 1024), b""):
                    digest_obj.update(chunk)
                    actual_size += len(chunk)
            if digest_obj.hexdigest() != item["sha256"]:
                raise ValueError(f"Checksum mismatch: {name}")
            if item.get("size") is not None and actual_size != item["size"]:
                raise ValueError(f"File size mismatch: {name}")
        actual = {name for name in entries if name.startswith("payload/")}
        if actual != expected:
            raise ValueError("Package payload and manifest file lists do not match")
        return manifest


def create(source: Path, output: Path) -> None:
    config_path = source / "blox.json"
    if not config_path.is_file():
        raise ValueError("Source directory must contain blox.json")
    config = json.loads(config_path.read_text(encoding="utf-8"))
    for key in ("id", "name", "version"):
        if not isinstance(config.get(key), str) or not config[key].strip():
            raise ValueError(f"blox.json requires a non-empty '{key}'")
    if not re.fullmatch(r"[a-z0-9][a-z0-9._-]{0,127}", config["id"]):
        raise ValueError("Invalid package id; use lowercase letters, digits, dot, underscore or hyphen")
    if output.suffix.lower() != ".blox":
        raise ValueError("Output package filename must end with .blox")
    all_files = [p for p in source.rglob("*") if p.is_file() and p.name != "blox.json"]
    if any(p.is_symlink() for p in all_files):
        raise ValueError("Symbolic links are not allowed in package sources")
    payload_files = sorted(all_files)
    if not payload_files:
        raise ValueError("No application files found")
    if len(payload_files) > MAX_FILES:
        raise ValueError("Too many application files")
    manifest = {
        "format": FORMAT_NAME,
        "format_version": FORMAT_VERSION,
        "id": config["id"],
        "name": config["name"],
        "version": config["version"],
        "publisher": config.get("publisher", "Unknown"),
        "architecture": config.get("architecture", "any"),
        "entrypoint": config.get("entrypoint"),
        "description": config.get("description", ""),
        "files": [],
    }
    for path in payload_files:
        relative = path.relative_to(source).as_posix()
        safe_member(relative)
        manifest["files"].append({"path": relative, "sha256": sha256(path), "size": path.stat().st_size})
    output.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        archive.writestr(MANIFEST_NAME, json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
        for path in payload_files:
            archive.write(path, f"payload/{path.relative_to(source).as_posix()}")
    print(f"Created {output} ({len(payload_files)} files)")


def install(archive_path: Path, destination: Path) -> None:
    manifest = verify(archive_path)
    app_root = destination / manifest["id"]
    if app_root.exists():
        raise ValueError(f"Application already exists: {app_root}")
    app_root.parent.mkdir(parents=True, exist_ok=True)
    staging = Path(tempfile.mkdtemp(prefix=".bloxpm-", dir=app_root.parent))
    try:
        with zipfile.ZipFile(archive_path, "r") as archive:
            for item in manifest["files"]:
                relative = safe_member(item["path"])
                target = staging.joinpath(*relative.parts)
                target.parent.mkdir(parents=True, exist_ok=True)
                with archive.open(f"payload/{item['path']}") as src, target.open("wb") as dst:
                    while True:
                        chunk = src.read(1024 * 1024)
                        if not chunk:
                            break
                        dst.write(chunk)
        os.replace(staging, app_root)
    except Exception:
        import shutil
        shutil.rmtree(staging, ignore_errors=True)
        raise
    print(f"Installed {manifest['name']} {manifest['version']} to {app_root}")
    print("Note: BloxPM installs files only; it does not execute package scripts.")


def main() -> int:
    parser = argparse.ArgumentParser(prog="bloxpm", description="BloxOS package manager")
    commands = parser.add_subparsers(dest="command", required=True)
    p_create = commands.add_parser("create", help="Build a .blox package")
    p_create.add_argument("source", type=Path)
    p_create.add_argument("output", type=Path)
    p_verify = commands.add_parser("verify", help="Verify package structure and SHA-256 hashes")
    p_verify.add_argument("package", type=Path)
    p_info = commands.add_parser("info", help="Display package metadata")
    p_info.add_argument("package", type=Path)
    p_install = commands.add_parser("install", help="Safely extract a package into an application directory")
    p_install.add_argument("package", type=Path)
    p_install.add_argument("--root", type=Path, default=Path("./apps"))
    args = parser.parse_args()
    try:
        if args.command == "create":
            create(args.source.resolve(), args.output.resolve())
        elif args.command == "verify":
            manifest = verify(args.package)
            print(f"Verified: {manifest['name']} {manifest['version']} ({len(manifest['files'])} files)")
        elif args.command == "info":
            manifest = verify(args.package)
            print(json.dumps(manifest, ensure_ascii=False, indent=2))
        elif args.command == "install":
            install(args.package.resolve(), args.root.resolve())
        return 0
    except (OSError, ValueError, zipfile.BadZipFile, json.JSONDecodeError) as exc:
        print(f"bloxpm: error: {exc}", file=sys.stderr)
        return 1

if __name__ == "__main__":
    raise SystemExit(main())
