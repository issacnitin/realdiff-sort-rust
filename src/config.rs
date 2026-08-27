macro_rules! sort_rules {
	($rules:expr) => {
		$rules.sort_by(|left, right| left.priority.cmp(&right.priority));
	};
}

pub(crate) use sort_rules;
