use std::collections::BTreeMap;
use nexus_host::moderation::*;

#[test]
fn test_blacklist_and_appeal() {
    let mut participants = BTreeMap::new();
    let id = "bank1".to_string();
    participants.insert(id.clone(), Participant {
        id: id.clone(),
        strikes: 3,
        reputation: 50,
        blacklisted: true,
        blacklist_reason: Some("Too many invalid actions".to_string()),
        last_action: None,
    });
    check_blacklist(&participants, &id);
    submit_appeal(&mut participants, &id, "We fixed our process");
    assert!(!participants[&id].blacklisted);
    assert_eq!(participants[&id].strikes, 0);
}

#[test]
fn test_reset_strikes() {
    let mut participants = BTreeMap::new();
    let id = "bank2".to_string();
    participants.insert(id.clone(), Participant {
        id: id.clone(),
        strikes: 2,
        reputation: 80,
        blacklisted: false,
        blacklist_reason: None,
        last_action: None,
    });
    reset_strikes(&mut participants, &id);
    assert_eq!(participants[&id].strikes, 0);
}
