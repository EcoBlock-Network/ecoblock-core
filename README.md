# ecoblock-core

This crate defines the core data models and domain logic for the Ecoblock decentralized ecological data platform.

## Purpose

- Define key types such as `SensorData`, `NodeId`, `TangleBlock`
- Provide serialization/deserialization (e.g. `serde`)
- Act as the base dependency for other Ecoblock modules

## Crate usage

This crate is not intended to be used directly by end-users but by other Ecoblock modules like `ecoblock-gossip`, `ecoblock-storage`, etc.

## License

MIT
