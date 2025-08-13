use owg_protocol::{SchemaVersion, Envelope, Kind, Evt, State};

#[test]
fn envelope_roundtrip() {
    let env = Envelope {
        kind: Kind::Evt,
        schema: SchemaVersion::V0_1,
        t: 0,
        id: None,
        body: Evt::Snapshot { full: true, state: State::default() },
    };
    let js = serde_json::to_string(&env).unwrap();
    let de: Envelope<Evt> = serde_json::from_str(&js).unwrap();
    match de.body {
        Evt::Snapshot { full, .. } => assert!(full),
        _ => panic!("wrong variant"),
    }
}
