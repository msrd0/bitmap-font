#![cfg_attr(not(test), no_std)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

/// **This version is for embedded-graphics 0.7. The latest release works with embedded-graphics 0.6.**
///
/// This crate provides bitmap fonts for the [`embedded-graphics`] crate. Those don't only look
/// better than the [built-in fonts](embedded_graphics::mono_font) by using the good-looking
/// [Tamzen font](https://github.com/sunaku/tamzen-font) over a font that renders `.` like a `+`,
/// but also allow scaling fonts by pixel-doubling them, giving you two font sizes for the flash
/// size requirements of the smaller one.
///
///  [`embedded-graphics`]: embedded_graphics
use embedded_graphics::{
	draw_target::DrawTarget,
	geometry::{OriginDimensions, Point, Size},
	image::{ImageDrawable, ImageRaw},
	mono_font::mapping::GlyphMapping,
	pixelcolor::BinaryColor,
	primitives::Rectangle,
	Pixel
};

mod generated;
pub use generated::*;

/// Stores the font bitmap and some additional info for each font.
#[derive(Clone, Copy)]
pub struct BitmapFont<'a> {
	/// The raw bitmap data for the font.
	bitmap: ImageRaw<'a, BinaryColor>,

	/// The char to glyph mapping.
	glyph_mapping: &'a dyn GlyphMapping,

	/// The size of each character in the raw bitmap data.
	size: Size,

	/// The amount of pixels to draw per each font pixel. Must not be `0`. Set to `2` for
	/// pixel-double font.
	pixels: u8
}

impl<'a> BitmapFont<'a> {
	/// Return the width of each character.
	pub const fn width(self) -> u32 {
		self.size.width * self.pixels as u32
	}

	/// Return the height of each character.
	pub const fn height(self) -> u32 {
		self.size.height * self.pixels as u32
	}

	/// Draw a glyph to the `target` with `color` at position `pos`.
	pub fn draw<D>(self, idx: u32, target: &mut D, color: BinaryColor, pos: Point) -> Result<(), D::Error>
	where
		D: DrawTarget<Color = BinaryColor> + OriginDimensions
	{
		let char_per_row = self.bitmap.size().width / self.size.width;
		let row = idx / char_per_row;

		// index in the raw bitmap
		let char_x = (idx - (row * char_per_row)) * self.size.width;
		let char_y = row * self.size.height;
		let area = Rectangle::new(Point::new(char_x as _, char_y as _), self.size);

		// draw the glyph, suitably pixel-doubled
		let mut pixel_doubling_target = PixelDoublingDrawTarget {
			target,
			color,
			offset: pos,
			pixels: self.pixels
		};
		self.bitmap.draw_sub_image(&mut pixel_doubling_target, &area)?;

		Ok(())
	}

	/// Returns a pixel-double version of this font.
	pub const fn pixel_double(mut self) -> Self {
		self.pixels *= 2;
		self
	}
}

/// A pixel-doubling draw target. This works by intercepting all drawing operations, and doubling
/// all pixels from the point of the origin. **This means you need to carefully select your
/// origin!** All drawing operations, after being pixel-doubled (i.e. multiplied by the amount
/// of pixels specified), will be offseted to map your custom origin to the `target`'s origin.
///
/// Also, this target draws `color` for all pixels that are on, and nothing for all pixels
/// that are off.
struct PixelDoublingDrawTarget<'a, D: DrawTarget<Color = BinaryColor> + OriginDimensions> {
	target: &'a mut D,
	color: BinaryColor,
	offset: Point,
	pixels: u8
}

impl<'a, D> OriginDimensions for PixelDoublingDrawTarget<'a, D>
where
	D: DrawTarget<Color = BinaryColor> + OriginDimensions
{
	fn size(&self) -> Size {
		let target_size = self.target.size();
		let width = match self.offset.x % 2 {
			0 => target_size.width / 2,
			1 => target_size.width / 2 - 1,
			_ => unreachable!()
		};
		let height = match self.offset.y % 2 {
			0 => target_size.height / 2,
			1 => target_size.height / 2 - 1,
			_ => unreachable!()
		};
		Size::new(width, height)
	}
}

impl<'a, D> DrawTarget for PixelDoublingDrawTarget<'a, D>
where
	D: DrawTarget<Color = BinaryColor> + OriginDimensions
{
	type Color = BinaryColor;
	type Error = D::Error;

	fn draw_iter<I>(&mut self, pixel_iter: I) -> Result<(), Self::Error>
	where
		I: IntoIterator<Item = Pixel<BinaryColor>>
	{
		let color = self.color;
		let offset = self.offset;
		let pixels = self.pixels;
		self.target.draw_iter(
			pixel_iter
				.into_iter()
				.filter(|pixel| pixel.1 == BinaryColor::On)
				.flat_map(|pixel| {
					PixelDoublingIterator::new(Pixel(pixel.0 - offset, color), pixels)
						.map(|pixel| Pixel(pixel.0 + offset, pixel.1))
				})
		)
	}
}

struct PixelDoublingIterator {
	color: BinaryColor,
	pos: Point,
	// pixels traveled in x direction
	x: u8,
	// remaining after this one
	remaining_x: u8,
	// remaining including this one
	remaining_y: u8
}

impl PixelDoublingIterator {
	fn new(pixel: Pixel<BinaryColor>, pixels: u8) -> Self {
		Self {
			color: pixel.1,
			pos: pixel.0 * pixels as _,
			x: 0,
			remaining_x: pixels - 1,
			remaining_y: pixels
		}
	}
}

impl Iterator for PixelDoublingIterator {
	type Item = Pixel<BinaryColor>;

	fn next(&mut self) -> Option<Pixel<BinaryColor>> {
		if self.remaining_y == 0 {
			return None;
		}

		let pixel = Pixel(self.pos, self.color);
		if self.remaining_x > 0 {
			self.remaining_x -= 1;
			self.x += 1;
			self.pos.x += 1;
		} else {
			self.pos.x -= self.x as i32;
			self.remaining_x = self.x;
			self.x = 0;

			self.remaining_y -= 1;
			self.pos.y += 1;
		}
		Some(pixel)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn px(x: i32, y: i32) -> Pixel<BinaryColor> {
		Pixel(Point::new(x, y), BinaryColor::On)
	}

	#[test]
	fn pixel_doubling_iterator() {
		assert_eq!(PixelDoublingIterator::new(px(0, 0), 1).collect::<Vec<_>>(), vec![px(0, 0)]);
		assert_eq!(PixelDoublingIterator::new(px(1, 2), 1).collect::<Vec<_>>(), vec![px(1, 2)]);

		assert_eq!(PixelDoublingIterator::new(px(0, 0), 2).collect::<Vec<_>>(), vec![
			px(0, 0),
			px(1, 0),
			px(0, 1),
			px(1, 1)
		]);
		assert_eq!(PixelDoublingIterator::new(px(1, 2), 2).collect::<Vec<_>>(), vec![
			px(2, 4),
			px(3, 4),
			px(2, 5),
			px(3, 5)
		]);

		assert_eq!(PixelDoublingIterator::new(px(0, 0), 3).collect::<Vec<_>>(), vec![
			px(0, 0),
			px(1, 0),
			px(2, 0),
			px(0, 1),
			px(1, 1),
			px(2, 1),
			px(0, 2),
			px(1, 2),
			px(2, 2)
		]);
		assert_eq!(PixelDoublingIterator::new(px(1, 2), 3).collect::<Vec<_>>(), vec![
			px(3, 6),
			px(4, 6),
			px(5, 6),
			px(3, 7),
			px(4, 7),
			px(5, 7),
			px(3, 8),
			px(4, 8),
			px(5, 8)
		]);
	}
}
