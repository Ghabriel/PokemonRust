use super::super::{prelude::*, TestMethods};

#[test]
fn splash_does_nothing() {
    let mut backend = battle! {
        "Magikarp" 10 (max ivs, Serious) vs "Magikarp" 10 (max ivs, Serious)
    };

    let events = backend.process_turn("Splash", "Splash");

    assert_eq!(events.len(), 3);
}
