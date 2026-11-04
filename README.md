# ferro-fresco-spike

R9 spike: the Ferro Connect + Browser screens on the estate's fresco UI
engine, verifiable headlessly (golden BMPs + interaction assertions), with
a native live-API probe. See RESEARCH.md for the measured gap list.

## Run

```bash
cargo run                      # headless render + interaction assertions
cargo run --features live --bin live-probe -- https://ferro.wyattau.com USER PASS SHARE_TOKEN
```
