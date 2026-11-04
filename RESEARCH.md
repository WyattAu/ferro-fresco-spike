# R9 Spike: Ferro screens on fresco — measured gap list

Status: CURRENT (research round R9, 2026-11-04)
Consumer: ferro desktop + Android clients (estate plan: ferro follows kestrel
as the second fresco consumer once Phase 3 earns it — Phase 3 landed 2026-11-03).

## What was built

`ferro-fresco-spike` (this repo) — the Ferro Connect + Browser screens on
fresco 0.3.5 widget-kit, verifiable headlessly:

- Connect screen: server/user/password TextFields + Connect button.
- Browser screen: header, ScrollPane + TreeList over the user root
  (folders/files with depth), Share/Upload/Delete action row, path bar.
- Interaction proof: focus chain walks to the Connect button, `activate`
  fires action 1 (asserted in `main.rs`).
- Goldens: `goldens/connect.bmp`, `goldens/browser.bmp` (BMP + fnv1a).
- `live-probe` (feature `live`): native reqwest client against a real
  Ferro server — list, ranged video read, upload, with timings.

Verified live against https://ferro.wyattau.com (production):

```
list:  123 ms | 200 (auth-gated listing)
video: 100 ms | 206 video/mp4, bytes 1000000-1000063/18187229
upload: 52 ms | 401 (expected: production is OIDC-only)
```

## Measured gap list (fresco 0.3.5 vs Ferro's needs)

| # | Gap | Severity | Notes |
|---|---|---|---|
| 1 | Video playback | HIGH | media-kit is images-only. Ferro's headline flow is video sharing. Needs either a `video-kit` (ffmpeg-kit on Android, decode-to-premultiplied-frames on desktop feeding paint-kit image blits) or platform-player handoff on Android (Intent) with desktop getting decode first. |
| 2 | Async data loading | HIGH | WidgetTree is sync/retained; the reactive layer is ADR-0001-deferred. Native clients must fetch off-thread and mutate the tree on the main thread — workable today with channels, but the ergonomic pattern (state signals -> widget props) does not exist yet. |
| 3 | Virtualized file list | MEDIUM | TreeList derives visible rows but `compute` walks all items. Hundreds OK; a 10k-file folder needs either windowing in TreeList or server-side pagination (the REST API already paginates via depth/offset). |
| 4 | Android window backend | HIGH (for mobile) | window-kit winit backend targets desktop. ADR-0003 plans NDK; winit supports android-activity — unproven in fresco. Blocking for the native APK replacing the WebView shell. |
| 5 | Mobile IME | MEDIUM | TextField over ui-text-edit is desktop-proven; Android IME (GBoard composition, backspace semantics) unproven until F2. |
| 6 | Thumbnails | LOW | paint-kit image blits + media-kit decode make async thumbnails feasible now; media-kit has bomb-guard. |
| 7 | OAuth/OIDC login | MEDIUM | Production is OIDC: the native client needs the system-browser + redirect-handshake pattern (oauth-toolkit covers primitives; no native helper exists). |
| 8 | API churn risk | MEDIUM | fresco is pre-1.0; widget/model APIs move. The spike's golden hashes + interaction assertions are the upgrade tripwire. |

## Positives measured

- Retained model + focus chain + typed actions all work headlessly (no
  display needed for CI) — the interaction contract is CI-gateable.
- Golden-image rendering is deterministic (stable fnv1a across runs).
- Ranged video reads from a native client: 100 ms through Cloudflare.
- Same-origin proxy pattern (Android AssetServer -> localhost backend)
  composes cleanly with adb reverse for dev.

## Verdict

GO for a ferro-fresco desktop client once fresco lands the reactive layer
(ADR-0001) or with a documented channel-bridge pattern. GO for Android
after the window-kit Android backend (gap 4). Video (gap 1) is the largest
product gap — the WebView shell remains the video-sharing UI until a
`video-kit` exists. Re-evaluate at fresco 0.4/Phase 4 exit.
