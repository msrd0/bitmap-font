use bit_vec::BitVec;

pub struct Bitmap {
	width: usize,
	width_double: usize,

	height: usize,
	height_double: usize,

	raw: Vec<u8>,
	raw_double: Vec<u8>
}

impl Bitmap {
	pub fn new(min_width: usize, height: usize) -> Self {
		Self {
			width: if min_width % 32 == 0 {
				min_width
			} else {
				(min_width / 32 + 1) * 32
			},
			width_double: if (2 * min_width) % 32 == 0 {
				2 * min_width
			} else {
				(min_width / 16 + 1) * 32
			},

			height,
			height_double: 2 * height,

			raw: Vec::new(),
			raw_double: Vec::new()
		}
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn into_raw(self) -> Vec<u8> {
		self.raw
	}

	pub fn init_lines(&self, char_height: u32) -> (Vec<BitVec>, Vec<BitVec>) {
		let mut lines = Vec::new();
		let mut lines_double = Vec::new();
		for _ in 0..char_height {
			lines.push(BitVec::with_capacity(8 * self.width));
			lines_double.push(BitVec::with_capacity(8 * self.width_double));
		}
		(lines, lines_double)
	}

	pub fn add_lines(&mut self, lines: Vec<BitVec>, lines_double: Vec<BitVec>) {
		for mut line in lines {
			for _ in line.len()..self.width {
				line.push(false);
			}
			self.raw.extend(line.to_bytes());
		}

		for mut line in lines_double {
			for _ in line.len()..self.width_double {
				line.push(false);
			}
			let bytes = line.to_bytes();
			self.raw_double.extend_from_slice(&bytes);
			self.raw_double.extend(bytes);
		}
	}

	fn to_bmp(raw: &[u8], width: i32, height: i32) -> Vec<u8> {
		let mut bmp = Vec::<u8>::new();
		macro_rules! write {
			([$($byte:literal),*]) => {
				bmp.extend(::std::array::IntoIter::new([$($byte),*]))
			};
			($int:expr) => {
				bmp.extend(::std::array::IntoIter::new($int.to_le_bytes()))
			};
		}

		// file header
		write!([0x42, 0x4D]);
		write!(14 + 40 + 8 + raw.len() as u32);
		write!([0, 0, 0, 0]);
		write!(14 + 40 + 8 as u32);

		// info header
		write!(40 as u32);
		write!(width);
		write!(-height);
		write!(1 as u16);
		write!(1 as u16);
		write!([0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

		// color table
		write!([0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0]);

		// pixel data
		bmp.extend_from_slice(raw);

		bmp
	}

	pub fn bmp(&self) -> Vec<u8> {
		Self::to_bmp(&self.raw, self.width as _, self.height as _)
	}

	pub fn bmp_double(&self) -> Vec<u8> {
		Self::to_bmp(&self.raw_double, self.width_double as _, self.height_double as _)
	}
}
