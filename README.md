# plato-tiles

Typed tiles flowing through the PLATO signal chain.

## What It Does

Defines the `Tile` — the fundamental unit of data in the PLATO nervous system. Every sensor reading, command, alert, and summary flows through the system as a typed tile. This crate provides the shared vocabulary that all other PLATO crates speak.

## Ecosystem

This is the **base crate** of the PLATO ecosystem. Everything depends on it:

- **[plato-rooms](https://github.com/SuperInstance/plato-rooms)** — Rooms contain sensors that produce tiles
- **[plato-state](https://github.com/SuperInstance/plato-state)** — Room state vectors are built from tile streams
- **[plato-nervous](https://github.com/SuperInstance/plato-nervous)** — The signal chain processes tiles through each layer
- **[plato-signal-chain](https://github.com/SuperInstance/plato-signal-chain)** — Composable pipeline that routes and transforms tiles
- **[plato-coordination](https://github.com/SuperInstance/plato-coordination)** — Fleet coordination exchanges tiles between rooms
- **[plato-diffusion](https://github.com/SuperInstance/plato-diffusion)** — Progressive distillation compresses tile streams
- **[plato-dashboard](https://github.com/SuperInstance/plato-dashboard)** — Renders tile data for fleet monitoring

See [DEPENDENCIES.md](./DEPENDENCIES.md) for the full dependency map.
