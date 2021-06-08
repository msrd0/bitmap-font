pub trait CeilingDiv {
	fn ceiling_div(self, rhs: Self) -> Self;
}

macro_rules! impl_ceiling_div {
	($($ty:ty),*) => {
		$(impl CeilingDiv for $ty {
			fn ceiling_div(self, rhs: Self) -> Self {
				self / rhs + (self % rhs > 0).then(|| 1).unwrap_or(0)
			}
		})*
	};
}
impl_ceiling_div!(usize, u32);
