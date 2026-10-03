#!/usr/bin/env bash
# Cross-builds the Windows app from Linux for local testing.
#
# Produces target/x86_64-pc-windows-msvc/release/muxduit.exe plus a portable
# zip next to it. Building the NSIS installer additionally needs `makensis`
# (AUR: nsis); with it installed, pass --installer.
#
# Requires: rustup target x86_64-pc-windows-msvc, cargo-xwin, trunk.
set -euo pipefail

TARGET="x86_64-pc-windows-msvc"
FFMPEG_BUILD="ffmpeg-n8.1-latest-win64-gpl-8.1"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$ROOT/target/$TARGET/release"
INSTALLER=0
[[ "${1:-}" == "--installer" ]] && INSTALLER=1

cd "$ROOT"

# FFmpeg ships as a Tauri sidecar on Windows; the names carry the target triple.
if [[ ! -f "src-tauri/binaries/ffmpeg-$TARGET.exe" ]]; then
  echo "==> fetching FFmpeg sidecars"
  mkdir -p src-tauri/binaries
  tmp="$(mktemp -d)"
  curl -sL -o "$tmp/ffmpeg.zip" \
    "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/$FFMPEG_BUILD.zip"
  unzip -q -o "$tmp/ffmpeg.zip" -d "$tmp"
  cp "$tmp/$FFMPEG_BUILD/bin/ffmpeg.exe" "src-tauri/binaries/ffmpeg-$TARGET.exe"
  cp "$tmp/$FFMPEG_BUILD/bin/ffprobe.exe" "src-tauri/binaries/ffprobe-$TARGET.exe"
  rm -rf "$tmp"
fi

echo "==> building frontend"
trunk build

# The static CRT flag has to reach cargo-xwin through the environment: read from
# .cargo/config.toml alone it links libcmt without libucrt and the link fails.
echo "==> cross-compiling $TARGET"
CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS="-C target-feature=+crt-static" \
  cargo xwin build --release --target "$TARGET" -p muxduit-desktop

if [[ $INSTALLER == 1 ]]; then
  echo "==> bundling NSIS installer"
  (cd src-tauri && cargo tauri bundle --bundles nsis --target "$TARGET")
  exit 0
fi

echo "==> packing portable zip"
python3 - "$OUT" "$TARGET" <<'PY'
import pathlib
import sys
import zipfile

out, target = pathlib.Path(sys.argv[1]), sys.argv[2]
binaries = pathlib.Path("src-tauri/binaries")
archive = out / "muxduit-windows-portable.zip"
members = {
    "muxduit.exe": out / "muxduit.exe",
    "ffmpeg.exe": binaries / f"ffmpeg-{target}.exe",
    "ffprobe.exe": binaries / f"ffprobe-{target}.exe",
}
with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as bundle:
    for name, path in members.items():
        bundle.write(path, name)
print(f"==> {archive}")
PY
