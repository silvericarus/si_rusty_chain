
# si_rusty_chain

**si_rusty_chain** is a **Rust**-based, learning-oriented **P2P blockchain prototype**. Version **v1** is **non-production** (lab/demo only): it focuses on fundamentals—TCP framing, a Noise-secured transport, deterministic serialization, signed block headers, basic P2P sync, and an explicit peer **FSM**—to build a clear base for a more robust **v2** aimed at real agents.

![Rust](https://img.shields.io/badge/Rust-1.82%2B-orange?logo=rust)
![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey)
![Status](https://img.shields.io/badge/Status-Active%20Development-blue)
![Build](https://img.shields.io/badge/Build-Experimental-yellow)
![Contributions](https://img.shields.io/badge/Contributions-Welcome-brightgreen)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/silvericarus/si_rusty_chain/blank.yml)


## Scope (v1)

- **Goal:** hands-on practice with networking, AEAD, deterministic encoding, block validation, P2P propagation, and a peer **FSM**.  
- **Non-production:** no hardening against Sybil/DoS, limited quotas, and minimal persistence. v2 will raise the bar for real agents.  
- **Topology:** pure **P2P** over **TCP** with a custom binary protocol.

## Architecture at a glance

- **Transport & security**
  - TCP with **length-prefix framing** (8-byte big-endian), **4 MiB** max frame.
  - **Noise XX** (25519, ChaCha20-Poly1305, BLAKE2s) as the secure channel; rekey every **30 min**.
  - No extra AEAD layer beyond Noise in v1.

- **Network envelope (v1)**
  - Fixed header: `version, type, flags, seq, req_id, sender_id, receiver_id`; **body ≤ 2 MiB** (post-decompress).
  - Flag `COMPRESSED` (zstd) negotiated via HELLO — **disabled by default** in v1.

- **Message catalog (v1)**
  - **HELLO**, **ACK/ERROR**, **HB** (heartbeat), **BLOCK**, **REQ_RANGE**, **RANGE**.
  - Planned for later: **PEERS**, **SEND**, **CANCEL**, **BYE** (defined but not required for the v1 happy path).

- **Blocks & validation**
  - Header = **CBOR (deterministic, ARRAY)** with fixed fields; `block_id = BLAKE3("si_rusty_chain/header v1" || cbor)`.
  - **Ed25519** signature over the **raw CBOR header bytes** (signature lives outside the header).
  - Rules: parent link, height = parent+1, time window ≤ **+90 s**, **Lamport** > parent and ≤ parent+16.

- **Consensus (educational)**
  - Canonical chain = **highest height**, with deterministic tiebreakers: **Lamport** → proposer_pk (lexicographic) → block_id.

- **Sync**
  - **HB** every **15 s**; if remote height ≥ local+1 ⇒ start sync.
  - **REQ_RANGE/RANGE**: descending, contiguous, chunk body ≤ **1 MiB**, `last=1` closes the series; `GAP_DETECTED` on holes.

- **FSM (peer lifecycle)**
  - `connecting → handshaking → helloed → syncing → steady → hb-stale → backoff/banned → closing`
  - Defaults: max **8** connections; double-link → keep **oldest**; HB-stale after **5** misses (~75 s); backoff 5→15→60 s with ±20% jitter.

- **Limits & defaults (selected)**
  - Sizes: BLOCK body ≤ **128 KiB**; RANGE chunk ≤ **1 MiB**; SEND ≤ **24 KiB**.
  - Rates/peer: BLOCK ≤ **20/s**; RANGE ≤ **10/s**; HB ≤ **2/s**.
  - Timeouts: handshake **60 s**; HELLO **5 s**; frame read **30 s**.

- **Persistence (v1)**
  - Minimal **WAL** + indices (by height and id); orphan pool TTL **2 min**.

## Development status

- **Active development** on v1 (non-production).  
- v2 will target: hardened quotas/ban/scoring, PEERS discovery, CANCEL/BYE flows, zstd by default (with safeguards), stronger persistence (checksums/fsync), and richer metrics/alerts.

## Requirements

- [Rust](https://www.rust-lang.org/) **1.82+**  
- Cargo

## Quick start (local demo)

```
bash
git clone https://github.com/silvericarus/si_rusty_chain.git
cd si_rusty_chain
cargo run
```

> For WAN tests, run two nodes on different machines, expose a port >1024, and point one node at the other as a bootstrap.

## License

This project is licensed under
[CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/)
You may share and adapt the content with proper attribution, for non-commercial purposes, and under the same license terms.

## Author

Personal project by @silvericarus. PRs and issues are welcome—please align with the v1 scope (non-production) and the documented limits before proposing features.

