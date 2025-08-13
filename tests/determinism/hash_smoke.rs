use owg_core::hash_state;
use owg_protocol::State;

#[test]
fn stable_hash_for_default_state() {
    let s = State::default();
    let h1 = hash_state(&s);
    let h2 = hash_state(&s);
    assert_eq!(h1, h2);
    assert!(!h1.is_empty());
}
