use insomnia::voting::ballot::Ballot;

#[test]
fn test_ballot_creation() {
    let ballot = Ballot::new("voter1".to_string(), "candidate1".to_string());
    assert_eq!(ballot.voter_id, "voter1");
    assert_eq!(ballot.choice, "candidate1");
}
