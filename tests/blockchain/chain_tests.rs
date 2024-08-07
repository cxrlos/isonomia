use isonomia::blockchain::chain::Isonomia;

#[test]
fn test_chain_creation() {
    let isonomia = Isonomia::new();
    assert_eq!(isonomia.get_chain().len(), 1);
}

#[test]
fn test_add_block() {
    let mut isonomia = Isonomia::new();
    isonomia.add_block("Test Block".to_string());
    assert_eq!(isonomia.get_chain().len(), 2);
}

#[test]
fn test_chain_validity() {
    let mut isonomia = Isonomia::new();
    isonomia.add_block("Test Block".to_string());
    assert!(isonomia.is_chain_valid());
}
