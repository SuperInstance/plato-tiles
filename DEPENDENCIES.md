# DEPENDENCIES — plato-tiles

## Signal Chain Layer

**Cross-cutting (Base Type)** — Every other PLATO crate depends on this.

Typed tile abstraction. Defines the `Tile` struct that flows through the entire PLATO signal chain. This is the foundational type system.

## Ecosystem Dependencies

| Repo | Relationship | Description |
|------|-------------|-------------|
| [plato-rooms](https://github.com/SuperInstance/plato-rooms) | **Depended on by** | Uses tiles as the unit of data within room sensor readings |
| [plato-state](https://github.com/SuperInstance/plato-state) | **Depended on by** | Tiles are the payload format for room state vectors |
| [plato-nervous](https://github.com/SuperInstance/plato-nervous) | **Depended on by** | The signal chain processes tiles through each layer |
| [plato-signal-chain](https://github.com/SuperInstance/plato-signal-chain) | **Depended on by** | Composable pipeline that routes and transforms tiles |
| [plato-coordination](https://github.com/SuperInstance/plato-coordination) | **Depended on by** | Fleet coordination exchanges tiles between rooms |
| [plato-diffusion](https://github.com/SuperInstance/plato-diffusion) | **Depended on by** | Distillation pipeline compresses tile streams |
| [plato-dashboard](https://github.com/SuperInstance/plato-dashboard) | **Related** | Renders tile data for the fleet dashboard |

## Data Flow

```
IN:
  - (none — this is the base type definition)

OUT:
  - Tile struct definition used by all other crates
  - Tile type enumeration (sensor, command, alert, summary)
  - Serialization/deserialization support
```

## Dependency Graph Position

```
plato-tiles ← (root, no ecosystem deps)
  ↓ used by everything
```
