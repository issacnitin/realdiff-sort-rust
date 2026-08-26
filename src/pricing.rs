use crate::config::BREAK_PRIORITY_TIES_BY_CODE;

#[derive(Clone)]
struct DiscountRule {
    code: &'static str,
    priority: i32,
    minimum_total: i32,
    percent_off: i32,
}

fn by_priority(rules: &[DiscountRule]) -> Vec<DiscountRule> {
    let mut ordered = rules.to_vec();
    ordered.sort_by(|left, right| {
        let priority = left.priority.cmp(&right.priority);
        if BREAK_PRIORITY_TIES_BY_CODE && priority.is_eq() {
            left.code.cmp(right.code)
        } else {
            priority
        }
    });
    ordered
}

fn select_discount(list_price: i32) -> DiscountRule {
    let rules = [
        DiscountRule { code: "Z_CLEARANCE", priority: 10, minimum_total: 50, percent_off: 40 },
        DiscountRule { code: "A_SEASONAL", priority: 10, minimum_total: 50, percent_off: 15 },
        DiscountRule { code: "INELIGIBLE", priority: 10, minimum_total: 1000, percent_off: 5 },
    ];
    by_priority(&rules)
        .into_iter()
        .find(|rule| list_price >= rule.minimum_total)
        .expect("an eligible discount")
}

pub fn checkout_total(list_price: i32) -> (i32, &'static str) {
    let selected = select_discount(list_price);
    (list_price * (100 - selected.percent_off) / 100, selected.code)
}

pub fn exercise_coverage() -> i32 {
    (0..40).map(normalize).sum()
}

fn normalize(value: i32) -> i32 {
    value + 1
}
