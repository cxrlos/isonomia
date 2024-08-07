use isonomia::voting::election::Election;

#[test]
fn test_election_creation() {
    let election = Election::new();
    assert!(election.is_valid());
}

#[test]
fn test_cast_vote() {
    let mut election = Election::new();
    assert!(election.cast_vote("voter1".to_string(), "candidate1".to_string()).is_ok());
    assert!(election.is_valid());
}

#[test]
fn test_get_results() {
    let mut election = Election::new();
    election.cast_vote("voter1".to_string(), "candidate1".to_string()).unwrap();
    election.cast_vote("voter2".to_string(), "candidate2".to_string()).unwrap();
    election.cast_vote("voter3".to_string(), "candidate1".to_string()).unwrap();
    
    let results = election.get_results();
    assert_eq!(results.get("candidate1"), Some(&2));
    assert_eq!(results.get("candidate2"), Some(&1));
}
