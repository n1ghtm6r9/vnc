# VNC Client Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A Windows desktop VNC viewer in Rust that logs into a macOS Screen Sharing server with the macOS account (Apple Remote Desktop auth, security type 30) and shows the Retina screen scaled to the window with high quality.

**Architecture:** A Cargo workspace laid out like `C:\Projects\price-percent-trading`: `apps/client` (the GUI binary `vnc`) on top of small layered crates in `packages/`. The protocol runs on two plain threads per session (blocking reader that decodes into a CPU framebuffer, writer that serializes input), and the UI thread uploads changed regions to a GPU texture that a two-pass Catmull-Rom resampler draws into the window; egui draws the connect form and the toolbar on top.

**Tech Stack:** Rust 1.99 (edition 2024), winit 0.30.13, wgpu 30, egui / egui-winit / egui-wgpu 0.37, flate2, zune-jpeg 0.5, num-bigint 0.5, aes 0.9, md-5 0.11, des 0.9, getrandom 0.4, windows 0.62, arboard 3, serde / serde_json, log + tracing-subscriber, cargo-nextest.

**Spec:** the "Requirements" section of this document (the owner skipped the design interview and said "делай").

## Requirements

- Runs on Windows 11 (the owner's PC, monitor 2560x1440, RTX 5070); the server is a MacBook Pro 16 at `192.168.0.220:5900` running macOS Screen Sharing.
- The server answers `RFB 003.889` and offers security types `30,33,36,31,32,35` only (probed 2026-10-21). VNC password (type 2) is off. Login is the macOS user name and password via type 30 (ARD Diffie-Hellman + AES-128-ECB). The client answers with version `RFB 003.008`.
- The Mac sends its framebuffer at full Retina resolution (about 3456x2234) and ignores resize requests. The client must scale it to the window (owner: "это клиент должен управлять масштабом"). Default mode is fit-to-window with aspect kept; a 1:1 mode exists.
- "Good client": crisp downscaling, low input latency, local cursor, full keyboard incl. the Windows key as Command, clipboard text both ways, reconnect after a network drop, remembered connection and password.
- UI text is Russian.

## Global Constraints

- One item per file: each type, function, constant, and each inherent method (`<type>_<method>.rs`, an `impl` block holding only that method) lives in its own file named in snake_case after the item. `Display`, `Error`, `Default`, `Drop`, `From` impls stay in the type's file.
- Every module is a directory with `mod.rs`; inside a module use `constants/`, `services/`, `types/`, `test/` subdirectories (omit empty ones). `mod.rs` order: private `mod x;` lines, then `pub use x::*;`, then `#[cfg(test)] mod test;`. Siblings import with `use super::X`; the rest with `use crate::X`.
- No comments in code (no `//`, `///`, `//!`), except a short note for a non-obvious workaround.
- Tests live in `src/**/test/<sentence_in_snake_case>_test.rs`, one test function per file named `test_<same sentence>`; assert messages are prose. No inline `mod tests {}`.
- Errors are hand-written enums: type + `impl fmt::Display` (lowercase prose sentence) + `impl std::error::Error` + `From` conversions in one file. No `thiserror`/`anyhow`.
- Log with `log::{error,warn,info,debug}!` and a bracketed tag: `[RFB]`, `[AUTH]`, `[DECODE]`, `[SESSION]`, `[GPU]`, `[UI]`, `[INPUT]`, `[PLATFORM]`.
- Multi-argument functions (more than 4 parameters) take an `...Options` struct.
- Dependencies come from `[workspace.dependencies]` with `{ workspace = true }`; every lib sets `[lib] doctest = false`. Do not edit the root `Cargo.toml`, `Cargo.lock`, or another crate's files unless your task says so.
- Imports at the top of the file. Edition 2024.
- Build/test commands for a crate: `cargo nextest run -p <crate>` and `cargo clippy -p <crate> --all-targets -- -D warnings`. When several agents work at once, each uses its own `CARGO_TARGET_DIR=target/agent-<crate>`.
- Commit style `type(scope): lowercase prose`. Do not push.

## Review Focus

- A server that sends a rect outside the framebuffer, an unknown subencoding, an oversized cursor or a truncated stream must end the session with an error, never a panic or a huge allocation. Tests: Task 3.
- Minimizing the window (0x0 client area) or resizing it while frames arrive must not crash or reconfigure the surface with a zero size. Test: Task 6.
- Losing focus while Shift/Ctrl/Cmd are held must release them on the Mac, or the Mac keeps them stuck. Tests: Task 5 and Task 9.
- Wrong credentials, a refused connection and a dropped network must be told apart: wrong password shows a Russian message and is not retried; a drop after a successful login reconnects. Tests: Task 7 and Task 10.
- A Cyrillic password and a user name longer than 63 bytes must reach the Mac as UTF-8, cut at a character boundary. Tests: Task 2.

## Workspace

```
vnc/
├── Cargo.toml                 workspace, profiles, [workspace.dependencies]
├── .cargo/config.toml         aliases: t / nt / test-all = nextest run --workspace, client, client-release, snapshot
├── .config/nextest.toml       default + ci profiles
├── apps/client/               package "app", lib "app", bin "vnc"
└── packages/
    ├── rfb/                   wire protocol: types, constants, read/write of every message (std::io)
    ├── rfb_auth/              VNC auth (DES) and Apple auth (DH + AES); feature test_server
    ├── rfb_decode/            Framebuffer, PixelConverter, decoders for every encoding, cursor
    ├── rfb_session/           connect + handshake + reader/writer threads, events and commands
    ├── gpu_scaler/            wgpu two-pass resampler of an RGBA texture into a viewport
    ├── remote_input/          winit keys/pointer/wheel -> RFB keysyms, button masks, points
    └── platform/              Windows: credential manager, low-level keyboard grab
```

Dependency direction: `rfb` <- `rfb_auth`, `rfb_decode` <- `rfb_session` <- `app`; `gpu_scaler`, `remote_input`, `platform` <- `app`.

## Waves

- Wave 0 (lead): Task 1.
- Wave 1 (parallel): Tasks 2, 3, 4, 5. Lead does Task 6 at the same time.
- Wave 2 (parallel): Tasks 7, 8. Lead does Task 9.
- Wave 3 (lead + reviewers): Tasks 10, 11.

---

### Task 1: Workspace and the `rfb` wire crate

**Files:**
- Create: `Cargo.toml`, `.cargo/config.toml`, `.config/nextest.toml`, `.gitignore`, skeleton `Cargo.toml` + `src/lib.rs` for every crate above.
- Create: `packages/rfb/src/{wire,geometry,pixel_format,encoding,security,version,init,client_message,server_message,error}/...`

**Interfaces (Produces, all re-exported at the `rfb` crate root):**
- `read_u8(r: &mut impl Read) -> io::Result<u8>`, `read_u16` (BE), `read_u32` (BE), `read_i32` (BE), `read_bytes(r, len: usize) -> io::Result<Vec<u8>>`, `read_text(r) -> io::Result<String>` (u32 BE length + bytes, lossy UTF-8, length capped at 1 MiB -> `InvalidData`), `skip_bytes(r, len: usize) -> io::Result<()>`.
- `struct Rect { x: u16, y: u16, width: u16, height: u16 }` (Copy, Eq, Hash, Default, Debug); `Rect::new(x, y, width, height)`, `rect.fits_within(width: u16, height: u16) -> bool`, `rect.union(other: Rect) -> Rect`, `rect.area() -> usize`, `rect.is_empty() -> bool`.
- `struct PixelFormat { bits_per_pixel: u8, depth: u8, big_endian: bool, true_colour: bool, red_max: u16, green_max: u16, blue_max: u16, red_shift: u8, green_shift: u8, blue_shift: u8 }`; `PixelFormat::rgba32()` (32, 24, little endian, true colour, 255/255/255, shifts 0/8/16 -> bytes in memory are R,G,B,X); `PixelFormat::read(r) -> io::Result<PixelFormat>`; `pf.write(w) -> io::Result<()>` (16 bytes incl. 3 padding); `pf.bytes_per_pixel() -> usize`.
- Encoding constants (`i32`): `ENCODING_RAW = 0`, `ENCODING_COPY_RECT = 1`, `ENCODING_RRE = 2`, `ENCODING_HEXTILE = 5`, `ENCODING_ZLIB = 6`, `ENCODING_TIGHT = 7`, `ENCODING_ZRLE = 16`, `ENCODING_CURSOR = -239`, `ENCODING_DESKTOP_SIZE = -223`, `ENCODING_LAST_RECT = -224`, `ENCODING_EXTENDED_DESKTOP_SIZE = -308`, `ENCODING_JPEG_QUALITY_LEVEL_0 = -32` (level n = -32 + n), `ENCODING_COMPRESS_LEVEL_0 = -256` (level n = -256 + n).
- Security constants (`u8`): `SECURITY_NONE = 1`, `SECURITY_VNC_AUTH = 2`, `SECURITY_APPLE_REMOTE_DESKTOP = 30`.
- `enum ProtocolVersion { V3_3, V3_7, V3_8 }`; `ProtocolVersion::read_server(r) -> Result<ProtocolVersion, ProtocolError>` (12-byte banner `RFB xxx.yyy\n`; major 3 minor < 7 -> V3_3, minor 7 -> V3_7, minor >= 8 (incl. 889) or major > 3 -> V3_8; anything else -> `UnsupportedVersion(banner)`); `version.write_client(w) -> io::Result<()>`.
- `read_security_types(r, version) -> Result<Vec<u8>, ProtocolError>` (3.7+: count + list, count 0 -> reason -> `ConnectionRefused`; 3.3: u32 chosen by server, 0 -> reason -> `ConnectionRefused`); `write_security_choice(w, security_type: u8) -> io::Result<()>`; `read_security_result(r, version) -> Result<(), ProtocolError>` (0 ok; else 3.8 reads a reason, -> `AuthenticationFailed(reason)`).
- `write_client_init(w, shared: bool)`; `struct ServerInit { width: u16, height: u16, pixel_format: PixelFormat, name: String }`; `ServerInit::read(r) -> Result<ServerInit, ProtocolError>`.
- Client messages: `write_set_pixel_format(w, &PixelFormat)`, `write_set_encodings(w, &[i32])`, `write_framebuffer_update_request(w, incremental: bool, rect: Rect)`, `write_key_event(w, down: bool, keysym: u32)`, `write_pointer_event(w, buttons: u8, x: u16, y: u16)`, `write_client_cut_text(w, text: &[u8])`; all `-> io::Result<()>`.
- Server messages: `enum ServerMessage { FramebufferUpdate { rect_count: u16 }, SetColourMapEntries { first_colour: u16, colours: Vec<[u16; 3]> }, Bell, ServerCutText { text: Vec<u8> } }`; `ServerMessage::read(r) -> Result<ServerMessage, ProtocolError>` (unknown type -> `UnknownMessage(t)`; cut text capped at 16 MiB); `struct RectHeader { rect: Rect, encoding: i32 }`; `RectHeader::read(r) -> io::Result<RectHeader>`.
- Server-side helpers (used by fake servers in tests): `enum ClientMessage { SetPixelFormat(PixelFormat), SetEncodings(Vec<i32>), FramebufferUpdateRequest { incremental: bool, rect: Rect }, KeyEvent { down: bool, keysym: u32 }, PointerEvent { buttons: u8, x: u16, y: u16 }, ClientCutText { text: Vec<u8> } }` + `ClientMessage::read(r) -> Result<ClientMessage, ProtocolError>`; `server_init.write(w)`; `rect_header.write(w)`; `write_framebuffer_update_header(w, rect_count: u16)`; `write_bell(w)`; `write_server_cut_text(w, text: &[u8])`; `write_security_types(w, &[u8])` (3.7+ form); `write_security_result(w, failure_reason: Option<&str>)` (3.8 form); `read_client_init(r) -> io::Result<bool>`; `write_text(w, &[u8])`; constants `MAX_TEXT_LENGTH: u32 = 1 << 20`, `MAX_CUT_TEXT_LENGTH: u32 = 16 << 20`, `CLIENT_*` and `SERVER_*` message type constants (`u8`).
- `enum ProtocolError { Io(io::Error), UnsupportedVersion(String), ConnectionRefused(String), AuthenticationFailed(String), UnknownMessage(u8) }` + Display + Error + `From<io::Error>`.

- [x] Write the workspace files and crate skeletons (all dependencies of every crate declared up front so `Cargo.lock` is settled before wave 1).
- [x] Write failing tests for banner parsing (`RFB 003.889\n` -> V3_8, `RFB 003.003\n` -> V3_3, garbage -> UnsupportedVersion), security list/refusal/result, ServerInit, every client message byte layout, every server message, pixel format round trip.
- [x] Implement, run `cargo nextest run -p rfb` -> all pass, clippy clean.

### Task 2: `rfb_auth` — VNC auth and Apple auth

**Files:** `packages/rfb_auth/src/{credentials,vnc_auth,apple_auth,crypto,error,test_server}/...`

**Interfaces:**
- Consumes: `rfb::{read_u16, read_bytes}`.
- Produces:
  - `struct Credentials { username: String, password: String }` with a manual `Debug` that prints the user name and `"***"` for the password; `Credentials::new(username: impl Into<String>, password: impl Into<String>)`.
  - `authenticate_vnc(stream: &mut (impl Read + Write), password: &str) -> Result<(), AuthError>` — reads the 16-byte challenge, writes the 16-byte response. Does not read SecurityResult.
  - `authenticate_apple(stream: &mut (impl Read + Write), credentials: &Credentials) -> Result<(), AuthError>` — the ARD exchange below. Does not read SecurityResult.
  - Pure helpers, each its own file: `vnc_auth_response(challenge: &[u8; 16], password: &str) -> [u8; 16]`; `dh_public_key(generator: &[u8], prime: &[u8], private_key: &[u8]) -> Vec<u8>`; `dh_shared_secret(peer_public: &[u8], private_key: &[u8], prime: &[u8]) -> Vec<u8>` (both big-endian, left-padded with zeros to `prime.len()`); `apple_credentials_block(username: &str, password: &str, filler: &[u8; 128]) -> [u8; 128]`; `aes128_ecb_encrypt(key: &[u8; 16], block: &[u8; 128]) -> [u8; 128]`; `aes128_ecb_decrypt(key: &[u8; 16], block: &[u8; 128]) -> [u8; 128]`; `md5_digest(data: &[u8]) -> [u8; 16]`; `random_bytes(len: usize) -> Vec<u8>` (getrandom).
  - `enum AuthError { Io(io::Error), InvalidServerKey(String), Random(String) }` + Display + Error + `From<io::Error>`.
  - Feature `test_server` (off by default): `serve_apple_auth(stream: &mut (impl Read + Write), options: AppleServerOptions) -> Result<Credentials, AuthError>` with `struct AppleServerOptions { generator: Vec<u8>, prime: Vec<u8>, private_key: Vec<u8> }` — sends generator/length/prime/public key, reads the 128-byte ciphertext and the client key, returns the decrypted credentials. Also `RFC2409_GROUP2_PRIME: [u8; 128]` (Oakley group 2, 1024-bit MODP prime, generator 2).

**Apple (type 30) exchange:**
```
server -> generator: [u8; 2] (big-endian), key_length: u16 BE, prime: [u8; key_length], server_public: [u8; key_length]
key_length must be 1..=1024, prime must not be zero, else InvalidServerKey
private = random_bytes(key_length); client_public = dh_public_key(generator, prime, private)
shared = dh_shared_secret(server_public, private, prime)          // key_length bytes
aes_key = md5_digest(shared)
block = apple_credentials_block(username, password, random 128 bytes)
   username UTF-8 at [0..64): at most 63 bytes, cut at a char boundary, NUL after it
   password UTF-8 at [64..128): same rule; the remaining bytes keep the random filler
client -> aes128_ecb_encrypt(aes_key, block) (128 bytes), then client_public (key_length bytes), one flush
```
**VNC (type 2):** key = first 8 password bytes zero-padded, each byte bit-reversed; DES-ECB encrypt challenge[0..8] and challenge[8..16].

- [ ] Tests (one file each): `test_a_cyrillic_password_reaches_the_server_as_utf8` (round trip through `serve_apple_auth` with password `"пароль"`), `test_a_long_username_is_cut_at_a_character_boundary` (70 Cyrillic bytes -> 62 bytes kept + NUL), `test_the_apple_exchange_works_with_a_2048_bit_group`, `test_a_zero_key_length_is_refused`, `test_dh_keys_are_left_padded_to_the_prime_length`, `test_the_vnc_response_decrypts_back_to_the_challenge` (DES decrypt with the bit-reversed key), `test_a_password_byte_is_bit_reversed_into_the_des_key` (0x01 -> 0x80), `test_debug_never_prints_the_password`, `test_aes_ecb_round_trips`.
- [ ] Run each failing, implement, run `cargo nextest run -p rfb_auth --features test_server` -> pass; clippy clean.

### Task 3: `rfb_decode` — framebuffer and decoders

**Files:** `packages/rfb_decode/src/{framebuffer,pixel_converter,decoder_state,raw,copy_rect,rre,hextile,zlib,zrle,tight,cursor,desktop_size,decode_rect,error}/...`

**Interfaces:**
- Consumes: `rfb::{Rect, PixelFormat, RectHeader, ENCODING_*, read_*}`.
- Produces:
  - `struct Framebuffer` (private fields `width: u16, height: u16, pixels: Vec<u32>`; a pixel is `u32::from_le_bytes([r, g, b, 255])`); `Framebuffer::new(width, height)` (opaque black), `width()`, `height()`, `pixels() -> &[u32]`, `resize(width, height)`, `fill_rect(rect, rgba)`, `put_pixels(rect, &[u32])`, `copy_rect(src_x: u16, src_y: u16, dst: Rect)` (overlap-safe), `read_rect(rect) -> Vec<u32>`.
  - `struct PixelConverter`; `PixelConverter::new(&PixelFormat)`, `bytes_per_pixel()`, `compact_bytes_per_pixel()` (ZRLE CPIXEL size), `tight_bytes_per_pixel()` (TPIXEL size), `rgba(&[u8]) -> u32`, `compact_rgba(&[u8]) -> u32`, `tight_rgba(&[u8]) -> u32`, `set_colour_map(first: u16, colours: &[[u16; 3]])`. Fast path when the format equals `PixelFormat::rgba32()`.
  - `struct DecoderState` (one zlib stream for Zlib, one for ZRLE, four for Tight, scratch buffers); `DecoderState::new()`.
  - `struct CursorShape { width: u16, height: u16, hotspot_x: u16, hotspot_y: u16, rgba: Vec<u8> }` (alpha 255 where the mask bit is set, else all four bytes 0; 0x0 = hidden cursor).
  - `enum RectOutcome { Pixels(Rect), Cursor(CursorShape), DesktopSize { width: u16, height: u16 }, LastRect }`.
  - `struct DecodeTarget<'a> { converter: &'a PixelConverter, state: &'a mut DecoderState, framebuffer: &'a mut Framebuffer }`.
  - `decode_rect(reader: &mut impl Read, header: &RectHeader, target: DecodeTarget) -> Result<RectOutcome, DecodeError>` dispatching to one pub fn per encoding: `decode_raw`, `decode_copy_rect`, `decode_rre`, `decode_hextile`, `decode_zlib`, `decode_zrle`, `decode_tight`, `decode_cursor`, `decode_extended_desktop_size`. DesktopSize / ExtendedDesktopSize resize the framebuffer and return `DesktopSize`.
  - `enum DecodeError { Io(io::Error), UnsupportedEncoding(i32), RectOutOfBounds(Rect), Corrupt(String) }` + Display + Error + `From<io::Error>`.

**Rules the tests pin (from rfbproto.rst — fetch https://github.com/rfbproto/rfbproto/blob/master/rfbproto.rst for detail):**
- Every pixel rect must satisfy `rect.fits_within(fb.width(), fb.height())` before any byte is read beyond its header-defined fixed part; else `RectOutOfBounds`.
- CPIXEL (ZRLE) is 3 bytes when bpp 32, depth <= 24, true colour, and all colour bits fit in the least or the most significant 3 bytes; the 3 bytes are those bytes of the pixel value in the format's byte order. TPIXEL (Tight) is 3 bytes R,G,B (always this order) when bpp 32, depth 24, true colour, all max 255.
- Hextile: 16x16 tiles; bits Raw=1, BackgroundSpecified=2, ForegroundSpecified=4, AnySubrects=8, SubrectsColoured=16; background/foreground persist across tiles of one rect; subrects must fit the tile.
- ZRLE: u32 length + data of one persistent zlib stream; 64x64 tiles; subencodings 0 raw, 1 solid, 2..=16 packed palette (1/2/4 bits per index, rows byte-aligned, MSB first), 128 plain RLE, 130..=255 palette RLE; 17..=127 and 129 -> `Corrupt`; run lengths are 1 + sum of bytes while byte == 255, must not overrun the tile.
- Tight: control byte low nibble = reset streams 0..3; high nibble n in 0..=7 basic (stream = n & 3; when n & 4 is set a filter byte follows: 0 copy, 1 palette (n+1 colours; 2 colours -> 1 bit per pixel rows byte-aligned; else 1 byte per pixel), 2 gradient (24-bit only)), 8 fill (one TPIXEL), 9 JPEG (compact length + JPEG via zune-jpeg, must decode to the rect size), others -> `Corrupt`. Data shorter than 12 bytes is sent raw; otherwise compact length (1-3 bytes, 7 bits each, high bit = more) + zlib data inflated to exactly the expected size.
- Cursor: w*h PIXELs + mask `((w+7)/8)*h` bytes MSB first; cursors larger than 512x512 -> `Corrupt`.
- Inflated output is never allowed to exceed the size the rect implies (zip-bomb guard).

- [ ] Tests (one file each; build compressed fixtures with `flate2::write::ZlibEncoder`, keep one encoder alive across two rects to prove stream persistence): raw in rgba32 and in a 16-bit big-endian format; copy_rect overlapping down-right; rre; hextile with persisting background and coloured subrects; zlib two rects one stream; zrle each subencoding (raw, solid, packed palette 2/3/16 colours, plain RLE, palette RLE); tight fill, copy filter, palette 2 colours (1 bit), palette 5 colours, gradient, jpeg (encode a fixture with the `jpeg-encoder` dev-dependency or a checked-in tiny JPEG), stream reset bit; cursor mask; desktop size resizes; and the Review Focus set: `test_a_rect_outside_the_framebuffer_is_refused`, `test_a_zrle_tile_with_an_unused_subencoding_is_refused`, `test_a_cursor_larger_than_the_cap_is_refused`, `test_a_truncated_raw_rect_reports_the_io_error`, `test_a_zrle_run_longer_than_the_tile_is_refused`.
- [ ] Implement, `cargo nextest run -p rfb_decode` -> pass; clippy clean.

### Task 4: `gpu_scaler` — two-pass resampler

**Files:** `packages/gpu_scaler/src/{scaler,shader,viewport,filter,test_support}/...`, WGSL in `src/shader/constants/resample_wgsl.rs` (`pub const RESAMPLE_WGSL: &str = include_str!(...)` of `src/shader/resample.wgsl`).

**Interfaces (Produces):**
- `struct Viewport { x: u32, y: u32, width: u32, height: u32 }` (target pixels).
- `enum ScaleFilter { Smooth, Nearest }`.
- `struct GpuScaler`; `GpuScaler::new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> GpuScaler`; `scaler.resize_source(device, width: u32, height: u32)` (Rgba8UnormSrgb source, cleared black); `scaler.upload(queue, rect: SourceRect, rgba: &[u8])` with `struct SourceRect { x: u32, y: u32, width: u32, height: u32 }` (rows tightly packed, `width*4` bytes); `scaler.render(frame: ScalerFrame)` with `struct ScalerFrame<'a> { device: &'a wgpu::Device, queue: &'a wgpu::Queue, encoder: &'a mut wgpu::CommandEncoder, target: &'a wgpu::TextureView, target_width: u32, target_height: u32, viewport: Viewport, filter: ScaleFilter, background: wgpu::Color }` — clears the whole target to `background` and draws the image into `viewport`; `scaler.source_size() -> Option<(u32, u32)>`.

**Algorithm:** Smooth = pass 1 horizontal source(W x H) -> intermediate Rgba16Float (viewport.width x H), pass 2 vertical -> target viewport. Per output coordinate `o` with source size `S` and output size `V`: `scale = V / S`, `f = max(1/scale, 1)`, `c = (o + 0.5) / scale - 0.5`, taps `i` from `floor(c - 2f) + 1` to `floor(c + 2f)`, weight `catmull_rom((i - c) / f)` with `catmull_rom(x) = |x|<1: 1.5|x|³ - 2.5|x|² + 1; |x|<2: -0.5|x|³ + 2.5|x|² - 4|x| + 2; else 0`, texel via `textureLoad` with clamped coordinates, result `sum / wsum`. Filtering happens in linear light (sRGB source texture decodes on load). If the target format is not sRGB the last pass applies the sRGB transfer function itself. Nearest = one pass, `textureLoad` at `floor((o + 0.5) * S / V)`. Intermediate texture re-created only when its size changes. Look up the wgpu 30 API (context7 `/gfx-rs/wgpu` or docs.rs) — it differs from older versions.

- [ ] Tests (headless device via `pollster`; when no adapter exists the test prints "no gpu adapter" and returns): `test_one_to_one_smooth_scaling_reproduces_the_source` (random pattern, every byte within 1), `test_halving_a_uniform_colour_keeps_the_colour`, `test_halving_a_black_and_white_checkerboard_gives_linear_grey` (sRGB 188 ± 2), `test_pixels_outside_the_viewport_get_the_background`, `test_nearest_doubling_repeats_each_pixel`, `test_a_non_srgb_target_matches_an_srgb_target`, `test_uploading_a_sub_rect_changes_only_that_rect`.
- [ ] Implement, `cargo nextest run -p gpu_scaler` -> pass; clippy clean.

### Task 5: `remote_input` — keys, pointer, wheel

**Files:** `packages/remote_input/src/{keysym,key_mapping,pressed_keys,pointer,wheel,view_rect}/...`

**Interfaces (Produces):**
- `keysym_for_key_code(code: winit::keyboard::KeyCode) -> Option<u32>` — US-layout unshifted keysyms, physical position (so the Mac's own input source decides the character): letters `0x61..=0x7a`, digits `0x30..=0x39`, `Minus 0x2d`, `Equal 0x3d`, `BracketLeft 0x5b`, `BracketRight 0x5d`, `Backslash 0x5c`, `Semicolon 0x3b`, `Quote 0x27`, `Backquote 0x60`, `Comma 0x2c`, `Period 0x2e`, `Slash 0x2f`, `IntlBackslash 0x3c`, `Space 0x20`, `Enter 0xff0d`, `Tab 0xff09`, `Backspace 0xff08`, `Escape 0xff1b`, `Delete 0xffff`, `Insert 0xff63`, `Home 0xff50`, `End 0xff57`, `PageUp 0xff55`, `PageDown 0xff56`, arrows `0xff51..=0xff54` (left, up, right, down), `F1..F24 = 0xffbe..`, `ShiftLeft 0xffe1`, `ShiftRight 0xffe2`, `ControlLeft 0xffe3`, `ControlRight 0xffe4`, `AltLeft 0xffe9`, `AltRight 0xffea`, `SuperLeft 0xffeb`, `SuperRight 0xffec`, `CapsLock 0xffe5`, `NumLock 0xff7f`, `ScrollLock 0xff14`, `PrintScreen 0xff61`, `Pause 0xff13`, `ContextMenu 0xff67`, `Numpad0..9 = 0xffb0..`, `NumpadDecimal 0xffae`, `NumpadDivide 0xffaf`, `NumpadMultiply 0xffaa`, `NumpadSubtract 0xffad`, `NumpadAdd 0xffab`, `NumpadEnter 0xff8d`, `NumpadEqual 0xffbd`.
- `struct KeyMapping { swap_control_and_command: bool, command_keysym: CommandKeysym }`, `enum CommandKeysym { Super, Meta }` (Super = 0xffeb/0xffec, Meta = 0xffe7/0xffe8), `Default` = no swap, Super.
- `keysym_for_physical_key(key: winit::keyboard::PhysicalKey, mapping: KeyMapping) -> Option<u32>` — applies the command keysym choice to SuperLeft/Right and swaps ControlLeft<->SuperLeft, ControlRight<->SuperRight when asked.
- `struct PressedKeys`; `press(keysym) -> bool` (true when newly pressed), `release(keysym) -> bool` (true when it was held), `drain() -> Vec<u32>` (every held keysym once, set emptied).
- `struct ViewRect { x: f64, y: f64, width: f64, height: f64 }`; `remote_point(window_x: f64, window_y: f64, view: ViewRect, remote_width: u16, remote_height: u16) -> (u16, u16)` (clamped to `0..remote-1`); `view.contains(x: f64, y: f64) -> bool`.
- `button_bit(button: winit::event::MouseButton) -> Option<u8>` (Left 1, Middle 2, Right 4, Back 128).
- `enum WheelStep { Up, Down, Left, Right }` with `wheel_step.button_bit() -> u8` (8, 16, 32, 64); `struct WheelAccumulator`; `accumulator.push(delta: winit::event::MouseScrollDelta) -> Vec<WheelStep>` (LineDelta: one step per whole line, positive y = Up, positive x = Left; PixelDelta: one step per 50 pixels; remainders carried; at most 20 steps per push).

- [ ] Tests (one file each): every letter/digit/punctuation/named mapping row above (table-driven), `test_swapping_moves_control_to_command_and_back`, `test_meta_mode_sends_meta_for_the_windows_key`, `test_draining_pressed_keys_returns_every_held_key_once`, `test_a_repeated_press_is_not_newly_pressed`, `test_a_point_in_the_letterbox_is_clamped_to_the_edge`, `test_the_view_centre_maps_to_the_remote_centre`, `test_three_line_scroll_gives_three_up_steps`, `test_small_pixel_deltas_add_up_to_one_step`, `test_a_huge_delta_is_capped`.
- [ ] Implement, `cargo nextest run -p remote_input` -> pass; clippy clean.

### Task 6: client shell (lead)

**Files:** `apps/client/src/{main.rs,lib.rs,app,gpu,ui,layout,settings,logger,text}/...`

**Produces:** `app::run() -> ExitCode`; `enum AppEvent`; `struct Settings` (serde, `%APPDATA%\vnc\settings.json`: host, port, username, remember_password, scale_mode, quality, swap_control_and_command, grab_keyboard, clipboard_sync); `enum ScaleMode { Fit, Actual }`; layout `fit_view(window_w, window_h, remote_w, remote_h) -> Option<ViewRect>` (None for a zero-size window), `actual_view(window, remote, pointer) -> Option<ViewRect>` (pan follows the pointer), `initial_window_size(monitor_w, monitor_h, remote_w, remote_h) -> (u32, u32)` (fits 90% of the monitor, aspect kept); winit `ApplicationHandler<AppEvent>` with wgpu surface + egui; the connect screen.

- [ ] Tests: `test_a_minimized_window_has_no_viewport`, `test_fit_keeps_the_aspect_and_centres`, `test_actual_size_pans_with_the_pointer`, `test_the_first_window_fits_ninety_percent_of_the_monitor`, `test_settings_survive_a_round_trip`, `test_missing_settings_give_defaults`.
- [ ] `cargo nextest run -p app`, `cargo run -p app` shows the connect form.

### Task 7: `rfb_session` — connection engine

**Files:** `packages/rfb_session/src/{options,event,command,handle,start,handshake,reader,writer,patches,quality,stats,error,test_support}/...`, `packages/rfb_session/examples/snapshot.rs`.

**Interfaces:**
- Consumes: Tasks 1-3 (`rfb::*`, `rfb_auth::{Credentials, authenticate_vnc, authenticate_apple}`, `rfb_auth` feature `test_server` in dev-dependencies, `rfb_decode::*`).
- Produces:
  - `pub use rfb_auth::Credentials; pub use rfb_decode::CursorShape; pub use rfb::Rect;`
  - `struct ConnectOptions { host: String, port: u16, credentials: Credentials, shared: bool, encodings: Vec<i32>, connect_timeout: Duration }`.
  - `enum Quality { Lossless, Balanced, Fast }`; `encodings_for_quality(quality: Quality) -> Vec<i32>`: Lossless `[COPY_RECT, ZRLE, HEXTILE, ZLIB, RRE, RAW]`, Balanced `[COPY_RECT, TIGHT, ZRLE, HEXTILE, ZLIB, RRE, RAW, JPEG_QUALITY_LEVEL_0 + 8, COMPRESS_LEVEL_0 + 6]`, Fast `[COPY_RECT, TIGHT, ZRLE, HEXTILE, ZLIB, RRE, RAW, JPEG_QUALITY_LEVEL_0 + 5, COMPRESS_LEVEL_0 + 2]`; each followed by `[CURSOR, DESKTOP_SIZE, EXTENDED_DESKTOP_SIZE, LAST_RECT]`.
  - `struct Patch { rect: Rect, pixels: Vec<u32> }`.
  - `struct SessionStats { bytes_per_second: u64, updates_per_second: f32, last_encoding: i32 }`.
  - `enum SessionEvent { Connected { width: u16, height: u16, name: String }, FrameUpdated { patches: Vec<Patch> }, Resized { width: u16, height: u16 }, CursorChanged(CursorShape), Bell, ClipboardText(String), Stats(SessionStats), Disconnected { error: Option<SessionError> } }`.
  - `enum ClientCommand { Pointer { x: u16, y: u16, buttons: u8 }, Key { keysym: u32, down: bool }, ClipboardText(String), RefreshScreen, SetEncodings(Vec<i32>) }`.
  - `struct SessionHandle`; `start_session(options: ConnectOptions, on_event: Box<dyn FnMut(SessionEvent) + Send>) -> SessionHandle` (returns at once; a thread connects); `handle.send(command: ClientCommand)` (silently dropped after the end); `handle.disconnect()` (works during connect too; ends with `Disconnected { error: None }`).
  - `enum SessionError { Connect(io::Error), Protocol(ProtocolError), Auth(AuthError), Decode(DecodeError), NoSupportedSecurity(Vec<u8>), MissingCredentials }` + Display + Error + From impls; `error.is_authentication_failure() -> bool` (Protocol(AuthenticationFailed) or MissingCredentials).

**Behaviour:**
- Security choice: 30 when offered and the user name is not empty; else 2 when offered and the password is not empty; else 1 when offered; 30 offered with an empty user name -> `MissingCredentials`; else `NoSupportedSecurity(offered)`. For V3_8 read SecurityResult after every type incl. None; for V3_3/V3_7 only after VNC auth (and after type 30 too).
- After ServerInit: SetPixelFormat(`PixelFormat::rgba32()`), SetEncodings(options.encodings), FramebufferUpdateRequest(non-incremental, full), emit `Connected`.
- Reader thread: decode each FramebufferUpdate into its Framebuffer, then emit one `FrameUpdated` whose patches cover every changed pixel (overlapping rects merged; more than 64 rects -> one bounding patch), then queue an incremental request. Cursor -> `CursorChanged`; DesktopSize -> `Resized` + non-incremental request; SetColourMapEntries -> converter; Bell; ServerCutText (Latin-1 -> String) -> `ClipboardText`. `Stats` at most once per second, counted with a byte-counting reader.
- Writer thread: `BufWriter` over a cloned `TcpStream`; drains the channel, keeps only the last of consecutive pointer moves with equal buttons, flushes per batch; `ClipboardText` is sent as Latin-1 (chars above U+00FF become `?`).
- Exactly one `Disconnected` per session, always the last event.

- [ ] Tests with an in-process fake server on `127.0.0.1:0` (`src/test_support/`, scripted with `rfb` writers and `rfb_auth::serve_apple_auth`): `test_apple_login_reaches_connected_with_the_server_size`, `test_the_first_frame_arrives_as_one_full_patch`, `test_a_wrong_password_ends_with_an_authentication_failure`, `test_a_refused_connection_reports_the_reason`, `test_a_server_that_hangs_up_mid_update_ends_with_an_error`, `test_disconnect_during_connect_ends_without_error`, `test_keys_and_pointer_reach_the_server_in_order`, `test_consecutive_pointer_moves_are_coalesced`, `test_a_desktop_resize_is_reported_and_refetched`, `test_clipboard_text_goes_out_as_latin1`, `test_missing_user_name_with_apple_only_server_is_reported`.
- [ ] `examples/snapshot.rs`: `cargo run -p rfb_session --example snapshot -- <host> <port> <user> <out.png>`, password from env `VNC_PASSWORD`; prints server name, size, encodings seen and time to first frame; writes the PNG.
- [ ] `cargo nextest run -p rfb_session` -> pass; clippy clean.

### Task 8: `platform` — Windows integration

**Files:** `packages/platform/src/{credentials,keyboard_grab,error}/...`

**Interfaces (Produces):**
- `save_password(target: &str, username: &str, password: &str) -> Result<(), PlatformError>`, `load_password(target: &str) -> Result<Option<String>, PlatformError>`, `delete_password(target: &str) -> Result<(), PlatformError>` — Windows Credential Manager generic credentials (`CredWriteW`/`CredReadW`/`CredDeleteW`, UTF-16 blob); on other OSes return `Ok(None)` / `Ok(())`.
- `struct GrabbedKey { virtual_key: u32, down: bool }`; `struct KeyboardGrab` (uninstalls on `Drop`); `KeyboardGrab::install(window: isize, on_key: Box<dyn Fn(GrabbedKey) + Send + Sync>) -> Result<KeyboardGrab, PlatformError>`; `grab.set_active(active: bool)`. A `WH_KEYBOARD_LL` hook swallows `VK_LWIN`/`VK_RWIN` (0x5B/0x5C) only while active and while `window` is the foreground window, and reports them through `on_key`; everything else passes on. Pure decision `should_grab(virtual_key: u32, active: bool, foreground_is_ours: bool) -> bool` in its own file.
- `enum PlatformError { Os(String) }` + Display + Error.

- [ ] Tests: `test_a_saved_password_loads_back_and_deletes` (unique target per run), `test_a_missing_target_loads_as_none`, `test_only_windows_keys_are_grabbed`, `test_nothing_is_grabbed_when_inactive_or_unfocused`.
- [ ] `cargo nextest run -p platform` -> pass; clippy clean.

### Task 9: client integration (lead)

- [ ] Render the remote screen: on `Connected` resize the window with `initial_window_size`, `resize_source`; `FrameUpdated` -> `upload` per patch + `request_redraw`; render = scaler pass then egui pass; skip rendering for a zero-size surface.
- [ ] Input: winit keyboard -> `keysym_for_physical_key` + `PressedKeys` (repeats forwarded as extra key-downs), pointer -> `remote_point` + `button_bit`, wheel -> `WheelAccumulator` (press+release per step); `Focused(false)` -> release `PressedKeys::drain()` and buttons. Test `test_focus_loss_releases_every_held_key` on the input router.
- [ ] Remote cursor: `CursorChanged` -> winit `CustomCursor` scaled by the current view scale; hidden when 0x0.

### Task 10: client polish (lead)

- [ ] Toolbar (egui, appears when the pointer touches the top edge): scale Fit/1:1, fullscreen, Ctrl<->Cmd, quality, refresh, disconnect, stats (fps, Mbit/s, encoding name).
- [ ] Reconnect after a drop that followed a successful login: backoff 1, 2, 5, 10, 10… s with a cancel button; authentication failures are shown and not retried (test `test_authentication_failures_are_not_reconnected`, `test_reconnect_delays_grow_and_cap`).
- [ ] Clipboard: `ClipboardText` -> arboard; on focus gain send local text if changed since the last sync (no echo).
- [ ] Credentials: remember password via `platform` (target `vnc:<user>@<host>:<port>`); keyboard grab active only while connected and focused.

### Task 11: verification and docs (lead + reviewers)

- [ ] `cargo nextest run --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --release -p app`.
- [ ] Real Mac: snapshot example, then the GUI driven with windows-mcp (login, fit scaling, typing, Cmd+Space, clipboard, reconnect by toggling the network).
- [ ] Code review of the whole branch; fix findings.
- [ ] README (Tech Stack, Project Structure, Quick Start, Commands, Tests, Configuration, Keyboard).
