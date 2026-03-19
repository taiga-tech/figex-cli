#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'EOF'
Place the release binary into vendor/ using the same path convention as CI.

Usage:
  scripts/place-binary-like-ci.sh [--target <rust-target>] [--build]
  scripts/place-binary-like-ci.sh <rust-target> [--build]

Examples:
  scripts/place-binary-like-ci.sh --build
  scripts/place-binary-like-ci.sh x86_64-apple-darwin --build
  scripts/place-binary-like-ci.sh x86_64-unknown-linux-musl
EOF
}

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

target=""
build=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --target)
            if [[ $# -lt 2 ]]; then
                echo "Missing value for --target" >&2
                usage
                exit 1
            fi
            target="$2"
            shift 2
            ;;
        --build)
            build=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            if [[ -z "$target" ]]; then
                target="$1"
                shift
            else
                echo "Unknown argument: $1" >&2
                usage
                exit 1
            fi
            ;;
    esac
done

if [[ -z "$target" ]]; then
    target="$(rustc -vV | sed -n 's/^host: //p')"
fi

if [[ -z "$target" ]]; then
    echo "Failed to detect target. Pass --target explicitly." >&2
    exit 1
fi

binary_name="sample-cli"
case "$target" in
    aarch64-apple-darwin)
        platform_pkg="cli-darwin-arm64"
        ;;
    x86_64-apple-darwin)
        platform_pkg="cli-darwin-x64"
        ;;
    x86_64-pc-windows-msvc)
        platform_pkg="cli-win32-x64"
        binary_name="sample-cli.exe"
        ;;
    x86_64-unknown-linux-gnu)
        platform_pkg="cli-linux-x64-gnu"
        ;;
    x86_64-unknown-linux-musl)
        platform_pkg="cli-linux-x64-musl"
        ;;
    *)
        echo "Unsupported target: $target" >&2
        exit 1
        ;;
esac

if [[ "$build" -eq 1 ]]; then
    cargo build --release --target "$target"
fi

src="target/$target/release/$binary_name"
dst_dir="packages/$platform_pkg/vendor/$target/sample-cli"
dst="$dst_dir/$binary_name"

if [[ ! -f "$src" ]]; then
    echo "Binary not found: $src" >&2
    echo "Run: cargo build --release --target $target" >&2
    exit 1
fi

mkdir -p "$dst_dir"
cp "$src" "$dst"

echo "Placed binary using CI-compatible path:"
echo "  source:      $src"
echo "  destination: $dst"
