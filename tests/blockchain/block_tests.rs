use isonomia::blockchain::block::Block;

#[test]
fn test_block_creation() {
    let block = Block::new(1, "Test Data".to_string(), "previous_hash".to_string());
    assert_eq!(block.index, 1);
    assert_eq!(block.data, "Test Data");
    assert_eq!(block.previous_hash, "previous_hash");
    assert!(!block.hash.is_empty());
}

#[test]
fn test_block_hash_calculation() {
    let block = Block::new(1, "Test Data".to_string(), "previous_hash".to_string());
    let calculated_hash = block.calculate_hash();
    assert_eq!(block.hash, calculated_hash);
}

