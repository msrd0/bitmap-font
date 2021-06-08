pub trait CeilingDiv {
	fn ceiling_div(self, rhs: Self) -> Self;
}

impl CeilingDiv for u32 {
	fn ceiling_div(self, rhs: Self) -> Self {
		self / rhs + (self % rhs > 0).then(|| 1).unwrap_or(0)
	}
}
