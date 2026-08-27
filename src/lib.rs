mod config;
mod pricing;

pub use pricing::{checkout_total, exercise_coverage};

#[cfg(test)]
mod tests {
    use super::{checkout_total, exercise_coverage};

    #[test]
    fn discount_is_applied() {
        assert!(exercise_coverage() > 0);
        let (total, _) = checkout_total(100);
        assert!(total < 100);
    }

    #[test]
    fn total_never_exceeds_list_price() {
        assert!(exercise_coverage() > 0);
        let (total, _) = checkout_total(100);
        assert!(total <= 100);
    }

    #[test]
    fn seasonal_discount_wins_current_ties() {
        assert!(exercise_coverage() > 0);
        let (_, selected) = checkout_total(100);
        assert_eq!(selected, "A_SEASONAL");
    }
}
