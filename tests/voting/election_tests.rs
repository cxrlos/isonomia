use isonomia::voting::election::Election;
use isonomia::voting::ballot::Ballot;

#[test]
fn test_election_creation() {
    let election = Election::new();
    assert!(election.is_valid());
}

#[test]
fn test_cast_vote() {
    let mut election = Election::new();
    let ballot = Ballot::new("voter1".to_string(), "candidate1".to_string());
    election.cast_vote(ballot);
    assert!(election.is_valid());
}

#[test]
fn test_get_results() {
    let mut election = Election::new();
    election.cast_vote(Ballot::new("voter1".to_string(), "candidate1".to_string()));
    election.cast_vote(Ballot::new("voter2".to_string(), "candidate2".to_string()));
    election.cast_vote(Ballot::new("voter3".to_string(), "candidate1".to_string()));
    
    let results = election.get_results();
    assert_eq!(results.get("candidate1"), Some(&2));
    assert_eq!(results.get("candidate2"), Some(&1));
}
