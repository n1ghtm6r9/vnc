# vnc — a Windows client for macOS Screen Sharing

A Rust VNC client for Windows 11. It connects to macOS Screen Sharing (RFB 003.889), logs in as a macOS user and shows the Retina screen of the Mac (for example, 3456×2234) in a Windows window. Scaling runs on the graphics card. The interface speaks English, Russian and Ukrainian.

## Stack

- Rust 2024, cargo-nextest for the tests.
- winit 0.30 for the window and input; wgpu 30 (DirectX 12) for output and scaling; egui 0.37 for the login form, the toolbar and the reconnect window.
- arboard for the clipboard; windows for the Windows key hook, the Credential Manager and the display languages.
- NVDEC (`nvcuvid.dll` from the NVIDIA driver) decodes HEVC 4:4:4 in High Performance mode.
- fdk-aac 0.8 (Fraunhofer FDK AAC) decodes AAC-ELD sound; cpal 0.18 (WASAPI) plays it.
- The RFB protocol, the login and the decoders are written in this repository.

## Structure

| Path | Contents |
|---|---|
| `apps/client` | the `vnc.exe` app: window, login form, toolbar, reconnecting, input, clipboard, settings, interface languages, icon |
| `packages/rfb` | RFB protocol messages: reading and writing |
| `packages/rfb_auth` | login: Apple Remote Desktop (type 30, macOS user name and password) and VNC password (type 2) |
| `packages/rfb_decode` | Raw, CopyRect, RRE, Hextile, Zlib, Tight (with JPEG), ZRLE and Apple MVS decoders, the cursor, screen resizes |
| `packages/rfb_session` | the session in its own thread: connecting, commands, events, statistics, starting High Performance; the `snapshot` example |
| `packages/ard_media` | the Apple media protocol: virtual display, offer and answer, SRTP, HEVC frames assembled from RTP, receiver reports |
| `packages/hp_media` | the High Performance session: UDP ports 5900 and 5901, stream recovery, decoding into picture slices |
| `packages/hp_audio` | High Performance sound: packet order, the AAC-ELD decoder, a 60 ms buffer with speed adjustment, output to a Windows device |
| `packages/nvdec` | the HEVC decoder on an NVIDIA graphics card, YUV planes in pooled buffers |
| `packages/gpu_scaler` | the YUV → RGB and Display P3 → sRGB shader, frame scaling on the graphics card |
| `packages/remote_input` | keys → keysyms, mouse, wheel |
| `packages/platform` | Windows: the Windows key hook, the Credential Manager, the display languages |

Code rules: one type, function or constant per file; no comments; tests live in `test/<sentence>_test.rs`, one `test_<sentence>` function per file; logs carry the tags `[RFB]`, `[AUTH]`, `[DECODE]`, `[SESSION]`, `[MEDIA]`, `[AUDIO]`, `[UI]`, `[GPU]`, `[PLATFORM]`.

## Preparing the Mac

System Settings → General → Sharing → turn on Screen Sharing. Log in with the name and password of a macOS user; a separate VNC password is not needed.

## Running

```
cargo client-release
```

Or build and run `target\release\vnc.exe`:

```
cargo build --release -p app
```

In the form enter the address of the Mac (for example, `192.168.0.220`), port `5900`, the macOS user and the password. After the login the window fits the Mac screen and moves to the centre of the monitor.

## Interface language

- The interface speaks English, Russian and Ukrainian. The ENG / RUS / UKR buttons sit at the top of the login form and on the toolbar. A choice applies at once, error messages included, and is saved.
- Until a language is chosen, the client follows Windows: it takes the first Windows display language it knows, and English when there is none.
- Hovering a button shows the name of the language in that language: English, Русский, Українська.

## Building

- Release (`cargo client-release`, `cargo build --release -p app`) is built for speed: `opt-level = 3`, fat LTO, one codegen unit and code for the processor of this machine. The last one is `target-cpu=native` in `.cargo/config.toml`; on a Ryzen 7 7700 it turns on AVX2 and AVX-512. Such a `vnc.exe` may not start on another processor. A build for another PC (any processor with AVX2): `$env:RUSTFLAGS='-C target-cpu=x86-64-v3'; cargo build --release -p app`, then `Remove-Item Env:RUSTFLAGS`, otherwise the next builds keep the flag.
- `panic = "abort"` is off: the JPEG decoder and the NVDEC callbacks catch panics, so broken data does not bring the client down.
- Debug (`cargo client`) builds fast and runs almost like release. The dependencies and the hot packages (`rfb_decode`, `rfb_session`, `hp_media`, `hp_audio`, `nvdec`, `ard_media`, `gpu_scaler`) build with `opt-level = 2`. The dependencies and `rfb_decode` have no debug assertions and no overflow checks. So wgpu does not turn on the DirectX 12 debug layer, and zlib and JPEG run as fast as in release. In tests `rfb_decode` keeps these checks. The rest of the code is not optimized and keeps line numbers for backtraces. The wgpu debug layer is turned on with `$env:WGPU_VALIDATION=1; $env:WGPU_DEBUG=1; cargo client`.
- The linker is `rust-lld` instead of `link.exe`. A change in the app rebuilds in 1.5 s, a change in `rfb_decode` in 2 s, a release build after a change in the app in 75 s.
- The icon `apps/client/assets/vnc.ico` holds ten sizes, from 16 to 256 pixels. The script `apps/client/build.rs` embeds it into `vnc.exe` with `winresource` and `rc.exe` from the Windows SDK, together with the file description "VNC". The window takes the icon from the exe resources at the scale of the monitor: at 100% that is 16 pixels in the title bar and 32 on the taskbar.

## Speed

- Balanced and Fast ask the Mac for its own Apple MVS encoding (number 1011, lossy compression in 8×8 blocks, like JPEG). A full 3024×1964 frame weighs about 300 KB and decodes in 22 ms; a usual update of a 1–2 Mpx area decodes in 6 ms.
- The next update is requested as soon as the current one starts to arrive, so the Mac prepares a frame and the network carries it while the client decodes the previous one.
- "fps" on the toolbar is how many updates the Mac sent in a second. The Mac sends them only when its screen changes: at rest the number is small, with mouse movement and scrolling it reaches about 60.
- The Mac draws the cursor right into the picture; in VNC mode it sends no separate cursor shape.
- Lossless goes through ZRLE: a full Retina frame takes up to 8 MB and 0.6–0.8 s, so motion stutters visibly.
- `rfb_decode` builds without overflow checks and debug assertions in the debug build too, so `cargo client` decodes as fast as `cargo client-release`. The tests keep these checks.

## High Performance mode

The High Performance checkbox in the login form. It is available in Apple Remote Desktop mode and is off by default. Under it is the Retina checkbox, on by default.

- The Mac creates a virtual display for the window and sends it as an HEVC 4:4:4 video stream at up to 60 fps over UDP with SRTP encryption. An NVIDIA graphics card decodes the stream.
- The first session window takes 90% of the Windows monitor (2304×1296 on 2560×1440). The client remembers the window at the end of a session: its size, whether it was maximized, whether it was full screen. The next session opens the same window and at once asks the Mac for a display of that size, so the stream does not restart when the window maximizes.
- With Retina the Mac draws two pixels per point, as for the Apple client: on 2560×1440 that is 1920×1080 points and a 3840×2160 video. The window scales the picture down with a Catmull-Rom filter, both in a window and in full screen. The text is sharp and the Mac interface is larger.
- Without Retina the display is 2304×1296 points, one pixel per point, and the picture in the window is 1:1. There is more room, but macOS draws the text without Retina, and it is softer.
- The display follows the window. A second after the window size stops changing (dragging an edge, Full screen), the client asks the Mac for a display of the new size and a new stream, without reconnecting, like the Apple client and noVNC-ARD. Until the new stream arrives, the old picture stays on screen.
- Without Retina, full screen on 2560×1440 gives a 2560×1440 display and a 1:1 picture. With Retina a 16:9 display already reaches 4K (1920×1080 points), so full screen does not change it, while other window proportions do.
- While the Mac has not answered the previous size, a new one waits, and only the last one goes out. A size change does not spend the stream recovery attempts. If the Mac does not answer a new size within 5 s, the old stream stays. After a reconnect the display is requested for the current window size.
- While the session runs, the screen of the Mac itself is dark. It turns back on after the client disconnects.
- The video arrives on UDP ports 5900 and 5901. On the first start Windows Firewall asks for permission for `vnc.exe`; it has to be given.
- The Mac does not draw the cursor into the video, so the cursor is drawn by Windows, as in standard mode. The cursor shape comes over TCP. After each shape update the client renews the AutoUpdate subscription, like Apple Screen Sharing, otherwise the cursor shape freezes. The picture comes in pixels and the cursor in points, so with Retina the cursor grows by the display scale.
- A minimized window does not hoard memory: a frame goes to the graphics card at once, without waiting for a redraw.
- The processor never walks over the frame. NVDEC decodes a slice, its YUV planes are copied into a pooled buffer and reach the window without further copies. There a shader converts YUV to RGB right into the picture texture. The graphics card receives 3 bytes per pixel at 4:4:4 and 1.5 at 4:2:0 instead of 4.
- The Mac sends the video in Display P3 colours, while Windows (all the more with HDR on) shows the window as sRGB. The shader converts P3 to sRGB in linear light, so the colours match the Mac screen. Colours outside sRGB are clipped to the nearest ones.
- The sound of the Mac comes with the video: AAC-ELD, 48 kHz, stereo, a packet every 10 ms. The toolbar has the buttons "Sound: Here / On the Mac / Off". Here (the default) plays the sound on the default Windows output device with a delay of about 60 ms. Off means the client plays no sound and releases the device. The choice is saved. Between Here and Off it applies at once, without a reconnect.
- In High Performance the Mac always gives its sound to the client and stays silent itself. Keeping the sound on the Mac in this mode is impossible: the Mac drops the connection on an offer without an audio stream and refuses an empty audio stream without starting it. So On the Mac reconnects the session in standard mode, without High Performance: the sound plays on the Mac itself, but the picture is worse. The next connections go in standard mode too. Here or Off reconnect the session back to High Performance.
- In standard VNC mode no sound is transferred. If High Performance is off in the settings, the toolbar shows On the Mac, and Here and Off are disabled.
- Sound packets are put in order. A missing packet is awaited until the three after it arrive (~30 ms), then it is covered with silence with a smooth fade-out and fade-in. If the sound piles up in the buffer or runs out, the playback speed changes by no more than 0.4%, which the ear does not notice. Anything over 150 ms in the buffer is dropped.
- When the Windows output device changes (headphones, speakers), the sound moves to the new one within a second.
- The decoder is Fraunhofer FDK AAC through the `fdk-aac` crate. Its license allows personal use. Whoever gives `vnc.exe` to others has to include the FDK license text; it grants no AAC patents.
- When nothing changes on the Mac screen, the Mac sends no video while the sound goes on. This is rest, and the stream does not restart.
- The stream is requested again only if no picture was assembled in full for 3 s or all packets were lost for 3 s. A new stream after a restart has to show a picture within 3 s, otherwise it is requested once more. The reason goes to the log as `[MEDIA] ...: last picture ... ms ago, ...`.
- If the Mac does not offer High Performance, the session runs in standard mode. If the stream breaks more than three times in 30 s, the client reconnects without High Performance a second later.
- In the statistics line the encoding reads "High Performance", and frames per second and Mbit/s are counted on the video stream.

## Commands

| Command | What it does |
|---|---|
| `cargo t` | all tests (`cargo nextest run --workspace`) |
| `cargo test-rfb` | tests of the protocol, the login, the decoders and the session |
| `cargo test-client` | tests of the client, scaling, input and `platform` |
| `cargo clippy --workspace --all-targets -- -D warnings` | the linter, warnings count as errors |
| `cargo client` / `cargo client-release` | run the client (debug / release) |
| `cargo snapshot <address> <port> <user> <file.png>` | connect, save one frame as PNG and print the statistics |

The password for `snapshot` comes from the `VNC_PASSWORD` variable:

```powershell
$env:VNC_PASSWORD = "<password>"; cargo snapshot 192.168.0.220 5900 <user> mac.png; Remove-Item Env:VNC_PASSWORD
```

## Tests

The tests run under cargo-nextest; its settings are in `.config/nextest.toml`.

- The protocol, the login and the session are checked against a fake server that the tests start on `127.0.0.1`.
- The decoders get corrupted data: the session has to end with an error, without a panic and without huge memory allocations.
- `gpu_scaler` draws on a real graphics card.
- `platform` writes a temporary password to the Credential Manager and deletes it, and reads the Windows display languages.
- The sound is checked without an output device: the packet order, the decoder, the buffer and the rate conversion run on test data.

## Settings

The file `%APPDATA%\vnc\settings.json` is created on the first connection or the first language choice. A broken file does not stop the client from starting: it takes the defaults.

| Field | Default | Meaning |
|---|---|---|
| `host` | `""` | address of the Mac |
| `port` | `5900` | port |
| `username` | `""` | macOS user |
| `remember_password` | `true` | keep the password in the Windows Credential Manager |
| `scale_mode` | `"fit"` | `"fit"` fits the window, `"actual"` is 1:1 |
| `quality` | `"balanced"` | `"lossless"`, `"balanced"`, `"fast"` |
| `sound` | `"here"` | where the sound of the Mac plays: `"here"` on this PC, `"mac"` on the Mac itself (a session without High Performance), `"off"` nowhere |
| `swap_control_and_command` | `false` | swap Ctrl and Cmd |
| `grab_keyboard` | `true` | the Windows key works as Cmd |
| `clipboard_sync` | `true` | shared clipboard |
| `apple_remote_desktop` | `true` | Apple Remote Desktop mode: the Apple protocol, Windows draws the Mac cursor |
| `high_performance` | `false` | the High Performance video stream; the Mac screen goes dark for the session |
| `retina` | `true` | High Performance with two pixels per point: sharp text, a larger Mac interface |
| `language` | `null` | interface language: `"en"`, `"ru"`, `"uk"`; `null` follows the Windows display language |
| `session_window` | `null` | the window at the end of the last session: `width`, `height`, `maximized`, `fullscreen`. The next session opens the same |

The password is never written to the file. It is kept in the Windows Credential Manager under the name `vnc:<user>@<address>:<port>`: it is saved after a successful login and deleted when Remember the password is unticked.

The log is `%APPDATA%\vnc\vnc.log`; it is overwritten on every start.

## Keyboard and mouse

- Letters and symbols are typed in the Windows layout: the language is switched in Windows (Alt+Shift or the language icon on the taskbar), the Mac layout does not affect typing. The Apple server types exactly the character it receives, so Cyrillic goes out as Unicode characters.
- Shortcuts with Cmd, Ctrl and Option go by the physical US keys: Cmd+C and Cmd+V work in the Russian layout too.
- The Windows key works as Cmd while the client is connected and its window is active: Start does not open, and Win+Space does not switch the Windows layout (on the Mac it is Cmd+Space).
- Alt is Option. The Swap Ctrl and Cmd button on the toolbar swaps Ctrl and Cmd: Ctrl+C and Ctrl+V copy and paste on the Mac, and the Windows key works as Control.
- Ctrl+Alt+Del, Win+L and Alt+Tab stay with Windows; Alt+F4 closes the client.
- When the window loses focus, the held keys and mouse buttons are released on the Mac.
- During a session every key goes to the Mac and the toolbar of the client does not hear them. Tab reaches the Mac, and Space and Enter do not press toolbar buttons.
- The Mac cursor is shown as a Windows cursor and scales with the picture. The Mac sends the cursor in points and reports how many pixels its screen has per point, so on a Retina screen the cursor grows by the same factor. The shape is scaled by area averaging, so its edges and outline stay smooth.

## Clipboard

- Text copied on the Mac lands in the Windows clipboard.
- Text from the Windows clipboard goes to the Mac on connecting and every time the client window becomes active again.
- The VNC protocol carries text in Latin-1, so Cyrillic from Windows reaches the Mac as "?". Images and files are not transferred.

## Toolbar and reconnecting

- The toolbar appears when the mouse reaches the top edge of the window: Fit to window / 1:1, Full screen, Swap Ctrl and Cmd, Quality (Lossless, Balanced, Fast), Sound (Here, On the Mac, Off), ENG / RUS / UKR, Refresh, Disconnect and the statistics (frames per second, Mbit/s, encoding).
- If the link drops after the login, the client reconnects by itself: after 1, 2 and 5 s, then every 10 s, until Cancel is pressed. The last frame stays on screen.
- If on the first connection the Mac accepts the connection but stays silent for more than 10 s, the client tries again at once. The error "The Mac does not answer" appears only if the second attempt gets silence too. A Mac that the connection does not reach at all gets no retry.
- A wrong password is not retried: the client goes back to the form and shows a message.
- If the Mac user ends the session (the Screen Sharing icon in the menu bar of the Mac), the client does not reconnect either and shows "The Mac user ended the session." Otherwise it would take over the Mac again, and in High Performance it would darken its screen again.

## Acknowledgements

The Apple MVS decoder (`packages/rfb_decode/src/mvs`) is ported to Rust from the JavaScript decoder of the [noVNC-ARD](https://github.com/peetinc/noVNC-ARD) project (MPL-2.0 license). The files of this decoder are a derivative work and are distributed under the terms of MPL-2.0.
