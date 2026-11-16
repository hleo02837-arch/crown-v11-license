CROWN V11 — GREEN WOLVES NODE
CROWN V11 is a Rust node for the GREEN WOLVES network.

It runs a TCP server with a built-in health check, and uses ROYAL GUARD as its guard / access layer.

Status: Early development. main is active, no releases published yet.

What it does
Runs a TCP server from src/
Exposes a health check endpoint for liveness / readiness
Enforces ROYAL GUARD rules before serving traffic (see ROYAL_GUARD.md)
Designed as a single node you can run locally and later join to other GREEN WOLVES nodes
Quick start
Requirements
Rust (stable) + Cargo
Git
Run it
git clone https://github.com/hleo02837-arch/crown-v11-license.git
cd crown-v11-license
cargo run
By default the server listens on 127.0.0.1:PORT — replace PORT with the port in your src/ config.
If your build uses a different host/port, update this section to match.

Check health
# Example — update to match your actual health route / message
curl -v 127.0.0.1:PORT/health
# or, if it is raw TCP:
# nc 127.0.0.1 PORT
Expected: server responds and stays up. If it closes immediately, see Troubleshooting below.

Build for release
cargo build --release
./target/release/crown-v11-license
Project structure
crown-v11-license/
├── Cargo.toml         # Rust manifest — must be named exactly this, no backticks
├── README.md          # This file — must be named exactly this
├── ROYAL_GUARD.md     # Guard / access rules (renamed from `ROYAL GUARD`)
├── .gitignore
└── src/
    └── main.rs        # TCP server + health check entrypoint
Filenames matter: GitHub only renders README.md if it is named exactly that.
Cargo only finds Cargo.toml if it is named exactly that.

ROYAL GUARD
ROYAL_GUARD.md is the spec for the guard layer.

It should answer:

What does ROYAL GUARD allow / block?
How does a client prove it should be let in?
What happens on a failed check — drop, log, rate-limit?
How do you test it locally?
If ROYAL GUARD is code instead of a doc, rename it to src/royal_guard.rs and import it from main.rs instead.

Configuration
Document these in one place when you lock them in:

 Host / port
 Health check path / message
 Max connections / timeouts
 Logging level
 Any env vars (e.g. CROWN_PORT, CROWN_HOST)
Development
cargo fmt
cargo clippy
cargo test
Troubleshooting
cargo run says it cannot find Cargo.toml:
Your manifest was named `Cargo.toml` with backticks. Rename it to Cargo.toml.

README shows “Add a README” on GitHub:
Your file was named README.md[package].... Rename it to README.md.

Server starts then exits:
Check that main.rs is actually binding and awaiting connections, not just starting and returning.

Port already in use:
Change the port or kill the old process, then retry.

Roadmap
 Lock in host/port + health check format
 Finish ROYAL GUARD spec + tests
 Add config via env vars
 Add logging
 First release / tag v11.0.0
License
Add your license file here — the repo name suggests licensing is part of this project.
If you are using MIT / Apache-2.0 / other, add a LICENSE file and link it here.

