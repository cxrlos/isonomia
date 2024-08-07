use isonomia::voting::ballot::Ballot;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

#[test]
fn test_ballot_creation_and_verification() {
    let mut csprng = OsRng{};
    let keypair = Keypair::generate(&mut csprng);
    
    let ballot = Ballot::new("voter1".to_string(), "candidate1".to_string(), &keypair);
    
    assert_eq!(ballot.voter_id, "voter1");
    assert_eq!(ballot.choice, "candidate1");
    assert!(ballot.verify());
}

#[test]
fn test_ballot_tampering() {
    let mut csprng = OsRng{};
    let keypair = Keypair::generate(&mut csprng);
    
    let mut ballot = Ballot::new("voter1".to_string(), "candidate1".to_string(), &keypair);
    
    // Tamper with the ballot
    ballot.choice = "candidate2".to_string();
    
    assert!(!ballot.verify());
}
