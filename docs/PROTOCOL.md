# Protocol (v0.1.0) quick view
Envelope fields: `kind`, `schema {major,minor}`, `t`, `id`, `body`.

`kind` accepts `"cmd"` or `"Cmd"`, `"evt"` or `"Evt"` for robustness.
