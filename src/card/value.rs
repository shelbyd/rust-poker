#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone)]
pub enum Value {
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
}


mod tests {
	use super::Value::*;
	#[test] fn values_equal_themselves() {
		assert!(Ace == Ace);
		assert!(Two == Two);
		assert!(Three == Three);
		assert!(Four == Four);
		assert!(Five == Five);
		assert!(Six == Six);
		assert!(Seven == Seven);
		assert!(Eight == Eight);
		assert!(Nine == Nine);
		assert!(Ten == Ten);
		assert!(Jack == Jack);
		assert!(Queen == Queen);
		assert!(King == King);
	}

	#[test] fn values_dont_equal_others() {
		assert!(Ace != Two);
		assert!(Two != Three);
		assert!(Three != Four);
		assert!(Four != Five);
		assert!(Five != Six);
		assert!(Six != Seven);
		assert!(Seven != Eight);
		assert!(Eight != Nine);
		assert!(Nine != Ten);
		assert!(Ten != Jack);
		assert!(Jack != Queen);
		assert!(Queen != King);
		assert!(King != Ace);
	}

	#[test] fn values_are_ordered() {
		assert!(Two < Three);
		assert!(Three < Four);
		assert!(Four < Five);
		assert!(Five < Six);
		assert!(Six < Seven);
		assert!(Seven < Eight);
		assert!(Eight < Nine);
		assert!(Nine < Ten);
		assert!(Ten < Jack);
		assert!(Jack < Queen);
		assert!(Queen < King);
		assert!(King < Ace);
		assert!(Two < Ace);
	}
}
