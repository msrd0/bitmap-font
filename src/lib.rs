#![no_std]
#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

/*!
This crate provides bitmap fonts for the `embedded-graphics` crate without requiring generics. All
fonts provided are concrete, constant instances of [`BitmapFont`]. This means you can use these
bitmap fonts without any generics, unlike those fonts shipped with `embedded-graphics` where each
font is implemented via its own struct. Also, this allows pixel-double fonts to share their bitmap
data with the non-doubled font, reducing the flash size required.

# Usage Example

```rust
use bitmap_font::{BitmapFont, WithFont, FONT_7x13};
use embedded_graphics::{fonts::Text, prelude::*};
# use embedded_graphics::{drawable::Pixel, geometry::Size, pixelcolor::BinaryColor};
# use core::convert::Infallible;
# struct Display;
# impl DrawTarget<BinaryColor> for Display {
#   type Error = Infallible;
#   fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) -> Result<(), Infallible> { Ok(()) }
#   fn size(&self) -> Size { unimplemented!() }
# }
# fn main() -> Result<(), Infallible> {
# let mut display = Display;

let font: BitmapFont = FONT_7x13;
let text = Text::new("Hello World!", Point::zero());
text.with_font(font, BinaryColor::On).draw(&mut display)?;
# Ok(())
# }
```
*/

use embedded_graphics::{
	drawable::{Drawable, Pixel},
	fonts::Text,
	geometry::{Dimensions, Point, Size},
	pixelcolor::BinaryColor,
	transform::Transform,
	DrawTarget
};

mod generated;
pub use generated::*;

/// Stores the font bitmap and some additional info for each font.
#[derive(Clone, Copy)]
pub struct BitmapFont {
	/// The raw bitmap data for the font.
	bitmap: &'static [u8],

	/// The width of the raw bitmap data.
	bitmap_width: u32,

	/// The width of each character in the raw bitmap data.
	width: u32,

	/// The height of each character in the raw bitmap data.
	height: u32,

	/// The amount of pixels to draw per each font pixel. Set to `2` for pixel-double font.
	pixels: u32
}

impl BitmapFont {
	/// Return the width of each character.
	pub const fn width(self) -> u32 {
		self.width * self.pixels
	}

	/// Return the height of each character.
	pub const fn height(self) -> u32 {
		self.height * self.pixels
	}

	/// Returns `true` if the pixel `(x, y)` is turned on in the character `c`.
	// inspired by https://docs.rs/embedded-graphics/0.6.2/src/embedded_graphics/fonts/mod.rs.html#246
	pub fn pixel(self, c: char, x: u32, y: u32) -> bool {
		let x = x / self.pixels;
		let y = y / self.pixels;

		assert!(x < self.width);
		assert!(y < self.height);

		let char_per_row = self.bitmap_width / self.width;
		let offset = char_offset(c);
		let row = offset / char_per_row;

		extern crate std;
		std::println!("row = {}", row);

		let char_x = (offset - (row * char_per_row)) * self.width;
		let char_y = row * self.height;

		let bitmap_bit_index = char_x + x + ((char_y + y) * self.bitmap_width);
		let bitmap_byte = bitmap_bit_index / 8;
		let bitmap_bit = 7 - (bitmap_bit_index % 8);

		self.bitmap[bitmap_byte as usize] & (1 << bitmap_bit) != 0
	}

	/// Returns a pixel-double version of this font.
	pub const fn pixel_double(mut self) -> Self {
		self.pixels *= 2;
		self
	}
}

mod private {
	use super::BitmapFont;
	use embedded_graphics::pixelcolor::BinaryColor;

	pub struct Styled<T> {
		pub(super) primitive: T,
		pub(super) font: BitmapFont,
		pub(super) color: BinaryColor
	}

	pub struct Sealed;
}
use private::*;

/// Style a [Text] with a font and color.
pub trait WithFont: Sized {
	#[doc(hidden)]
	const SEALED: Sealed;

	/// Style this text with a font and color.
	fn with_font(self, font: BitmapFont, color: BinaryColor) -> Styled<Self>;
}

impl WithFont for Text<'_> {
	#[doc(hidden)]
	const SEALED: Sealed = Sealed;

	fn with_font(self, font: BitmapFont, color: BinaryColor) -> Styled<Self> {
		Styled {
			primitive: self,
			font,
			color
		}
	}
}

impl Transform for Styled<Text<'_>> {
	fn translate(&self, by: Point) -> Self {
		Self {
			primitive: self.primitive.translate(by),
			font: self.font,
			color: self.color
		}
	}

	fn translate_mut(&mut self, by: Point) -> &mut Self {
		self.primitive.translate_mut(by);
		self
	}
}

impl Dimensions for Styled<Text<'_>> {
	fn top_left(&self) -> Point {
		self.primitive.position
	}

	fn bottom_right(&self) -> Point {
		self.top_left() + self.size()
	}

	// inspired by https://docs.rs/embedded-graphics/0.6.2/src/embedded_graphics/fonts/text.rs.html#118
	fn size(&self) -> Size {
		if self.primitive.text.is_empty() {
			return Size::zero();
		}

		let width = self
			.primitive
			.text
			.lines()
			.map(|l| l.len() as u32 * self.font.width())
			.max()
			.unwrap_or(0);
		let height = self.primitive.text.lines().count() as u32 * self.font.height();
		Size::new(width, height)
	}
}

struct PixelIterator<'a> {
	text: &'a str,
	idx: usize,
	font: BitmapFont,
	color: BinaryColor,

	top_left: Point,
	pos: Point,

	char_width: u32,
	char_walk_x: i32,
	char_walk_y: i32,
	current_char: Option<char>
}

impl Iterator for PixelIterator<'_> {
	type Item = Pixel<BinaryColor>;

	// inspired by https://docs.rs/embedded-graphics/0.6.2/src/embedded_graphics/fonts/text.rs.html#170
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if self.current_char == Some('\n') {
				self.pos.x = self.top_left.x;
				self.pos.y += self.font.height() as i32;
				self.idx += 1;
				self.current_char = self.text.chars().nth(self.idx);
			} else if self.char_walk_x < 0 {
				self.char_walk_y += 1;
				if self.char_walk_y >= self.font.height() as i32 {
					self.char_walk_y = 0;
					self.char_walk_x += 1;
				}
			} else if let Some(current_char) = self.current_char {
				if self.char_width == 0 {
					self.char_width = self.font.width();
				}

				let color = self
					.font
					.pixel(current_char, self.char_walk_x as _, self.char_walk_y as _)
					.then(|| self.color);
				let x = self.pos.x + self.char_walk_x;
				let y = self.pos.y + self.char_walk_y;
				self.char_walk_x += 1;

				if self.char_walk_x >= self.font.width() as i32 {
					self.char_walk_x = 0;
					self.char_walk_y += 1;

					if self.char_walk_y >= self.font.height() as i32 {
						self.pos.x += self.char_width as i32;
						self.char_width = 0;
						self.char_walk_y = 0;
						self.idx += 1;
						self.current_char = self.text.chars().nth(self.idx);
					}
				}

				if let Some(color) = color {
					break Some(Pixel(Point::new(x, y), color));
				}
			} else {
				break None;
			}
		}
	}
}

impl Drawable<BinaryColor> for &Styled<Text<'_>> {
	fn draw<D: DrawTarget<BinaryColor>>(self, display: &mut D) -> Result<(), D::Error> {
		let iter = PixelIterator {
			text: self.primitive.text,
			idx: 0,
			font: self.font,
			color: self.color,

			top_left: self.primitive.position,
			pos: self.primitive.position,

			char_width: 0,
			char_walk_x: 0,
			char_walk_y: 0,
			current_char: self.primitive.text.chars().next()
		};
		display.draw_iter(iter)
	}
}

impl Drawable<BinaryColor> for Styled<Text<'_>> {
	fn draw<D: DrawTarget<BinaryColor>>(self, display: &mut D) -> Result<(), D::Error> {
		<&Self as Drawable<BinaryColor>>::draw(&self, display)
	}
}
