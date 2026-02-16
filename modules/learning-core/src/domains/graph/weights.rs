/// Calculate edge weight from traversal and backup counts.
///
/// Formula: max(0, (traversals - 2 * backups)) / max(1, traversals + backups)
///
/// - Fresh edge (0, 0): weight = 0
/// - 10 traversals, 0 backups: weight = 1.0
/// - 10 traversals, 3 backups: weight ≈ 0.31
/// - 1 traversal, 1 backup: weight = 0
pub fn calculate_weight(traversals: i64, backups: i64) -> f64 {
    let numerator = (traversals - 2 * backups).max(0) as f64;
    let denominator = (traversals + backups).max(1) as f64;
    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_edge() {
        assert_eq!(calculate_weight(0, 0), 0.0);
    }

    #[test]
    fn test_strong_endorsement() {
        assert_eq!(calculate_weight(10, 0), 1.0);
    }

    #[test]
    fn test_mixed_signal() {
        let w = calculate_weight(10, 3);
        assert!((w - 4.0 / 13.0).abs() < 0.01);
    }

    #[test]
    fn test_cancelled_out() {
        assert_eq!(calculate_weight(1, 1), 0.0);
    }

    #[test]
    fn test_more_backups_than_traversals() {
        assert_eq!(calculate_weight(1, 5), 0.0);
    }
}
