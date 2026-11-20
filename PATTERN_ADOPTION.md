# Pattern adoption

R9 render spike: golden-master machinery exists in the spike binary; no
production service surface, so few patterns apply.

| Pattern | Status | Evidence |
|---|---|---|
| T13 | partial | src/ferro.rs:16 (pure tree constructors; rendering and assertions are the shell in src/main.rs) |
| P65 | partial | src/main.rs:22 (golden BMPs written to goldens/, interaction asserted at src/main.rs:50; fnv1a hashes printed for manual diff, no in-code byte compare) |
