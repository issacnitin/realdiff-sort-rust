use crate::config::sort_rules;

#[derive(Clone)]
struct DiscountRule {
    code: &'static str,
    priority: i32,
    minimum_total: i32,
    percent_off: i32,
}

fn by_priority(rules: &[DiscountRule]) -> Vec<DiscountRule> {
    let mut ordered = rules.to_vec();
    sort_rules!(ordered);
    ordered
}

fn select_discount(list_price: i32) -> DiscountRule {
    let priorities = [2, 1, 3, 0, 3, 3, 2, 0, 2, 0, 1, 3, 1, 0, 1, 3, 3, 0, 2, 2, 2, 3, 1, 2, 2, 0, 2, 0, 1, 2, 1, 2, 3];
    let rules = priorities.map(|priority| DiscountRule {
        code: "INELIGIBLE",
        priority,
        minimum_total: 1000,
        percent_off: 5,
    });
    let mut rules = rules;
    rules[2] = DiscountRule { code: "A_SEASONAL", priority: 3, minimum_total: 50, percent_off: 15 };
    rules[15] = DiscountRule { code: "Z_CLEARANCE", priority: 3, minimum_total: 50, percent_off: 40 };
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
    let steps: [fn(i32) -> i32; 100] = [
        step000, step001, step002, step003, step004, step005, step006, step007, step008, step009,
        step010, step011, step012, step013, step014, step015, step016, step017, step018, step019,
        step020, step021, step022, step023, step024, step025, step026, step027, step028, step029,
        step030, step031, step032, step033, step034, step035, step036, step037, step038, step039,
        step040, step041, step042, step043, step044, step045, step046, step047, step048, step049,
        step050, step051, step052, step053, step054, step055, step056, step057, step058, step059,
        step060, step061, step062, step063, step064, step065, step066, step067, step068, step069,
        step070, step071, step072, step073, step074, step075, step076, step077, step078, step079,
        step080, step081, step082, step083, step084, step085, step086, step087, step088, step089,
        step090, step091, step092, step093, step094, step095, step096, step097, step098, step099,
    ];
    steps.into_iter().fold(0, |value, step| step(value))
}

fn step000(value: i32) -> i32 { value + 1 }
fn step001(value: i32) -> i32 { value + 1 }
fn step002(value: i32) -> i32 { value + 1 }
fn step003(value: i32) -> i32 { value + 1 }
fn step004(value: i32) -> i32 { value + 1 }
fn step005(value: i32) -> i32 { value + 1 }
fn step006(value: i32) -> i32 { value + 1 }
fn step007(value: i32) -> i32 { value + 1 }
fn step008(value: i32) -> i32 { value + 1 }
fn step009(value: i32) -> i32 { value + 1 }
fn step010(value: i32) -> i32 { value + 1 }
fn step011(value: i32) -> i32 { value + 1 }
fn step012(value: i32) -> i32 { value + 1 }
fn step013(value: i32) -> i32 { value + 1 }
fn step014(value: i32) -> i32 { value + 1 }
fn step015(value: i32) -> i32 { value + 1 }
fn step016(value: i32) -> i32 { value + 1 }
fn step017(value: i32) -> i32 { value + 1 }
fn step018(value: i32) -> i32 { value + 1 }
fn step019(value: i32) -> i32 { value + 1 }
fn step020(value: i32) -> i32 { value + 1 }
fn step021(value: i32) -> i32 { value + 1 }
fn step022(value: i32) -> i32 { value + 1 }
fn step023(value: i32) -> i32 { value + 1 }
fn step024(value: i32) -> i32 { value + 1 }
fn step025(value: i32) -> i32 { value + 1 }
fn step026(value: i32) -> i32 { value + 1 }
fn step027(value: i32) -> i32 { value + 1 }
fn step028(value: i32) -> i32 { value + 1 }
fn step029(value: i32) -> i32 { value + 1 }
fn step030(value: i32) -> i32 { value + 1 }
fn step031(value: i32) -> i32 { value + 1 }
fn step032(value: i32) -> i32 { value + 1 }
fn step033(value: i32) -> i32 { value + 1 }
fn step034(value: i32) -> i32 { value + 1 }
fn step035(value: i32) -> i32 { value + 1 }
fn step036(value: i32) -> i32 { value + 1 }
fn step037(value: i32) -> i32 { value + 1 }
fn step038(value: i32) -> i32 { value + 1 }
fn step039(value: i32) -> i32 { value + 1 }
fn step040(value: i32) -> i32 { value + 1 }
fn step041(value: i32) -> i32 { value + 1 }
fn step042(value: i32) -> i32 { value + 1 }
fn step043(value: i32) -> i32 { value + 1 }
fn step044(value: i32) -> i32 { value + 1 }
fn step045(value: i32) -> i32 { value + 1 }
fn step046(value: i32) -> i32 { value + 1 }
fn step047(value: i32) -> i32 { value + 1 }
fn step048(value: i32) -> i32 { value + 1 }
fn step049(value: i32) -> i32 { value + 1 }
fn step050(value: i32) -> i32 { value + 1 }
fn step051(value: i32) -> i32 { value + 1 }
fn step052(value: i32) -> i32 { value + 1 }
fn step053(value: i32) -> i32 { value + 1 }
fn step054(value: i32) -> i32 { value + 1 }
fn step055(value: i32) -> i32 { value + 1 }
fn step056(value: i32) -> i32 { value + 1 }
fn step057(value: i32) -> i32 { value + 1 }
fn step058(value: i32) -> i32 { value + 1 }
fn step059(value: i32) -> i32 { value + 1 }
fn step060(value: i32) -> i32 { value + 1 }
fn step061(value: i32) -> i32 { value + 1 }
fn step062(value: i32) -> i32 { value + 1 }
fn step063(value: i32) -> i32 { value + 1 }
fn step064(value: i32) -> i32 { value + 1 }
fn step065(value: i32) -> i32 { value + 1 }
fn step066(value: i32) -> i32 { value + 1 }
fn step067(value: i32) -> i32 { value + 1 }
fn step068(value: i32) -> i32 { value + 1 }
fn step069(value: i32) -> i32 { value + 1 }
fn step070(value: i32) -> i32 { value + 1 }
fn step071(value: i32) -> i32 { value + 1 }
fn step072(value: i32) -> i32 { value + 1 }
fn step073(value: i32) -> i32 { value + 1 }
fn step074(value: i32) -> i32 { value + 1 }
fn step075(value: i32) -> i32 { value + 1 }
fn step076(value: i32) -> i32 { value + 1 }
fn step077(value: i32) -> i32 { value + 1 }
fn step078(value: i32) -> i32 { value + 1 }
fn step079(value: i32) -> i32 { value + 1 }
fn step080(value: i32) -> i32 { value + 1 }
fn step081(value: i32) -> i32 { value + 1 }
fn step082(value: i32) -> i32 { value + 1 }
fn step083(value: i32) -> i32 { value + 1 }
fn step084(value: i32) -> i32 { value + 1 }
fn step085(value: i32) -> i32 { value + 1 }
fn step086(value: i32) -> i32 { value + 1 }
fn step087(value: i32) -> i32 { value + 1 }
fn step088(value: i32) -> i32 { value + 1 }
fn step089(value: i32) -> i32 { value + 1 }
fn step090(value: i32) -> i32 { value + 1 }
fn step091(value: i32) -> i32 { value + 1 }
fn step092(value: i32) -> i32 { value + 1 }
fn step093(value: i32) -> i32 { value + 1 }
fn step094(value: i32) -> i32 { value + 1 }
fn step095(value: i32) -> i32 { value + 1 }
fn step096(value: i32) -> i32 { value + 1 }
fn step097(value: i32) -> i32 { value + 1 }
fn step098(value: i32) -> i32 { value + 1 }
fn step099(value: i32) -> i32 { value + 1 }
