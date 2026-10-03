# Muxduit

A desktop media transcoder powered by [FFmpeg](https://ffmpeg.org/).
Pick codecs, quality and filters in the UI; encoding runs natively so the full
codec set is available (AV1, HEVC, …).

> Inspired by [**Nmkoder** by n00mkrad](https://github.com/n00mkrad/nmkoder).

Muxduit is a [Tauri](https://tauri.app/) app: a Yew/WASM UI in a native
window, driving the system `ffmpeg`/`ffprobe` through Tauri commands. Files are
added via the native picker or OS drag-and-drop and encoded straight into the
output folder you choose.

## Run

```sh
# Development
cargo tauri-dev

# Release bundle (AppImage/deb/…)
cargo tauri-build

# Flatpak (bundles its own FFmpeg)
flatpak-builder --force-clean --user --install build-dir flatpak/dev.aligator.muxduit.yml
flatpak run dev.aligator.muxduit
```

Requires the Rust toolchain, [Trunk](https://trunkrs.dev/) and `ffmpeg` on
`PATH` (except Flatpak and the Windows installer, which bundle FFmpeg).

FFmpeg is looked up in this order: `MUXDUIT_FFMPEG` / `MUXDUIT_FFPROBE`, then a
binary sitting next to the executable, then `PATH`.

## Windows

The release workflow builds an NSIS installer (`*-setup.exe`) for x86_64 and
arm64, with a static GPL FFmpeg bundled as a Tauri sidecar, so nothing else has
to be installed. To build one locally:

```sh
# Drop ffmpeg.exe/ffprobe.exe named for the target triple next to the app, e.g.
#   src-tauri/binaries/ffmpeg-x86_64-pc-windows-msvc.exe
#   src-tauri/binaries/ffprobe-x86_64-pc-windows-msvc.exe
cargo tauri-build --bundles nsis
```

Bundling a GPL FFmpeg build keeps the distributed installer under the GPL,
which matches this project's license.

## License

Based on the upstream [Nmkoder](https://github.com/n00mkrad/nmkoder) project.
