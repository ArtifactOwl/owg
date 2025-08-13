# Protocol (v0.1.0)

All messages carry `schema` and `t` (authoritative tick). JSON below is for docs;
binary on the wire is recommended (e.g., rmp-serde).

## Envelope

Client→Server:
```json
{ "kind": "Cmd", "schema": {"major":0,"minor":1}, "t": 0, "id":"client-uuid", "cmd": { "Ping": { "nonce":"abc" } } }
```

Server→Client:
```json
{ "kind": "Evt", "schema": {"major":0,"minor":1}, "t": 0, "evt": { "Pong": { "nonce":"abc", "rttMs": 42 } } }
```

## Commands (subset)
```json
{ "Move": { "entityId":"e1", "axis":[1,0], "thrust":0.6 } }
{ "Fire": { "entityId":"e1", "weapon":"railgun", "aim":[0.2,-0.4] } }
{ "Mine": { "entityId":"e1", "target":"asteroid-42" } }
{ "Craft": { "entityId":"e1", "recipeId":"ingot-fe", "inputs":[["FeOre",10]] } }
{ "UseItem": { "entityId":"e1", "slot":2 } }
{ "Ping": { "nonce": "abc" } }
```

## Events (subset)
```json
{ "Snapshot": { "full": true, "state": { /* see State */ } } }
{ "Delta": { "adds":[...], "updates":[...], "removes":["e17"] } }
{ "Hit": { "projectileId":"p99", "victimId":"asteroid-42", "impulse":1200 } }
{ "Mined": { "minerId":"e1", "nodeId":"asteroid-42", "yields":[["FeOre",5]] } }
{ "CraftResult": { "entityId":"e1", "ok": true, "outputs":[["FeIngot",2]] } }
{ "Desync": { "clientHash":"...", "serverHash":"...", "atTick":123 } }
{ "Pong": { "nonce":"abc", "rttMs":41 } }
```

## State (save/snapshot core)
```json
{
  "world": { "seed":"W-2025-08-A", "time":0 },
  "entities":[{ "id":"e1", "archetype":"ship/player", "pose":{"p":[0,0],"v":[0,0],"θ":0,"ω":0},
                 "physics":{"mass":1200,"radius":1.2},
                 "inventory":{"slots":[["FeOre",10]]}, "owner":"player-uuid"}],
  "projectiles":[]
}
```
