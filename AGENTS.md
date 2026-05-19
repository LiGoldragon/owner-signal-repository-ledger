# owner-signal-repository-ledger — Agent Instructions

Read `~/primary/AGENTS.md`, then this file.

This repository is a pure OwnerSignal contract crate. It declares the
owner-only repository-ledger vocabulary and contains no daemon, storage,
actors, or runtime policy.

Do not put ordinary peer queries here. Peer-callable repository reads and event
submissions belong in `signal-repository-ledger`.
