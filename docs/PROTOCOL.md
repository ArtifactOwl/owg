# Protocol (v0.1.0)

All messages carry `schema` and `t` (authoritative tick). JSON below is for docs;
binary on the wire is recommended (e.g., rmp-serde).

## Envelope

Client→Server:
```json
{ "kind": "Cmd", "schema": {"major":0,"minor":1}, "t": 0, "id":"client-uuid", "body": { "Ping": { "nonce":"abc" } } }
```

Server→Client:
```json
{ "kind": "Evt", "schema": {"major":0,"minor":1}, "t": 0, "id": null, "body": { "Pong": { "nonce":"abc", "rtt_ms": 42 } } }
```

## Commands (subset)
```json
{ "Move": { "entity_id":"e1", "axis":[1,0], "thrust":0.6 } }
{ "Fire": { "entity_id":"e1", "weapon":"railgun", "aim":[0.2,-0.4] } }
{ "Mine": { "entity_id":"e1", "target":"asteroid-42" } }
{ "Craft": { "entity_id":"e1", "recipe_id":"ingot-fe", "inputs":[["FeOre",10]] } }
{ "UseItem": { "entity_id":"e1", "slot":2 } }
{ "Ping": { "nonce": "abc" } }
```

## Events (subset)
```json
{ "Snapshot": { "full": true, "state": { /* see State */ } } }
{ "Delta": { "adds":[...], "updates":[...], "removes":["e17"] } }
{ "Hit": { "projectile_id":"p99", "victim_id":"asteroid-42", "impulse":1200 } }
{ "Mined": { "miner_id":"e1", "node_id":"asteroid-42", "yields":[["FeOre",5]] } }
{ "CraftResult": { "entity_id":"e1", "ok": true, "outputs":[["FeIngot",2]] } }
{ "Desync": { "client_hash":"...", "server_hash":"...", "at_tick":123 } }
{ "Pong": { "nonce":"abc", "rtt_ms":41 } }
```

## State (save/snapshot core)
```json
{
  "world": { "seed":"W-2025-08-A", "time":0 },
  "entities":[{ "id":"e1", "archetype":"ship/player", "pose":{"p":[0,0],"v":[0,0],"theta":0,"omega":0},
                 "physics":{"mass":1200,"radius":1.2},
                 "inventory":{"slots":[["FeOre",10]]}, "owner":"player-uuid"}],
  "projectiles":[]
}
```
