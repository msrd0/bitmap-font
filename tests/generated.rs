// @generated

use bitmap_font::*;
use embedded_graphics::{
	drawable::Drawable,
	fonts::Text,
	geometry::{Dimensions, Point, Size},
	mock_display::MockDisplay,
	pixelcolor::BinaryColor
};

// ### Fonts

#[test]
fn font_size_5x9() {
	assert_eq!(FONT_5x9.width(), 5);
	assert_eq!(FONT_5x9.height(), 9);
}

#[test]
fn text_empty_size_5x9() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_5x9, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_5x9() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_5x9, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(5, 9));
}

#[test]
fn text_multiline_size_5x9() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_5x9, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(15, 27));
}

#[test]
fn text_a_draw_5x9() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_5x9, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"     ",
			"     ",
			"     ",
			" ### ",
			"#  # ",
			"#  # ",
			" ### ",
			"     ",
			"     ",
		])
	);
}

#[test]
fn font_size_6x12() {
	assert_eq!(FONT_6x12.width(), 6);
	assert_eq!(FONT_6x12.height(), 12);
}

#[test]
fn text_empty_size_6x12() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_6x12, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_6x12() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_6x12, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(6, 12));
}

#[test]
fn text_multiline_size_6x12() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_6x12, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(18, 36));
}

#[test]
fn text_a_draw_6x12() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_6x12, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"      ",
			"      ",
			"      ",
			"      ",
			" ###  ",
			"    # ",
			" #### ",
			"#   # ",
			" #### ",
			"      ",
			"      ",
			"      ",
		])
	);
}

#[test]
fn font_size_7x13() {
	assert_eq!(FONT_7x13.width(), 7);
	assert_eq!(FONT_7x13.height(), 13);
}

#[test]
fn text_empty_size_7x13() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_7x13, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_7x13() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_7x13, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(7, 13));
}

#[test]
fn text_multiline_size_7x13() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_7x13, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(21, 39));
}

#[test]
fn text_a_draw_7x13() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_7x13, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"       ",
			"       ",
			"       ",
			"       ",
			"       ",
			"  ###  ",
			"     # ",
			"  #### ",
			" #   # ",
			"  #### ",
			"       ",
			"       ",
			"       ",
		])
	);
}

#[test]
fn font_size_7x14() {
	assert_eq!(FONT_7x14.width(), 7);
	assert_eq!(FONT_7x14.height(), 14);
}

#[test]
fn text_empty_size_7x14() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_7x14, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_7x14() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_7x14, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(7, 14));
}

#[test]
fn text_multiline_size_7x14() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_7x14, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(21, 42));
}

#[test]
fn text_a_draw_7x14() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_7x14, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"       ",
			"       ",
			"       ",
			"       ",
			"       ",
			"  ###  ",
			"     # ",
			"  #### ",
			" #   # ",
			" #   # ",
			"  #### ",
			"       ",
			"       ",
			"       ",
		])
	);
}

#[test]
fn font_size_8x15() {
	assert_eq!(FONT_8x15.width(), 8);
	assert_eq!(FONT_8x15.height(), 15);
}

#[test]
fn text_empty_size_8x15() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_8x15, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_8x15() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_8x15, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(8, 15));
}

#[test]
fn text_multiline_size_8x15() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_8x15, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(24, 45));
}

#[test]
fn text_a_draw_8x15() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_8x15, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"        ",
			"        ",
			"        ",
			"        ",
			"        ",
			"        ",
			"  ####  ",
			"      # ",
			"  ##### ",
			" #    # ",
			" #    # ",
			"  ##### ",
			"        ",
			"        ",
			"        ",
		])
	);
}

#[test]
fn font_size_8x16() {
	assert_eq!(FONT_8x16.width(), 8);
	assert_eq!(FONT_8x16.height(), 16);
}

#[test]
fn text_empty_size_8x16() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_8x16, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_8x16() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_8x16, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(8, 16));
}

#[test]
fn text_multiline_size_8x16() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_8x16, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(24, 48));
}

#[test]
fn text_a_draw_8x16() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_8x16, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"        ",
			"        ",
			"        ",
			"        ",
			"        ",
			"        ",
			"  ####  ",
			"      # ",
			"      # ",
			"  ##### ",
			" #    # ",
			" #   ## ",
			"  ### # ",
			"        ",
			"        ",
			"        ",
		])
	);
}

#[test]
fn font_size_10x20() {
	assert_eq!(FONT_10x20.width(), 10);
	assert_eq!(FONT_10x20.height(), 20);
}

#[test]
fn text_empty_size_10x20() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_10x20, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_10x20() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_10x20, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(10, 20));
}

#[test]
fn text_multiline_size_10x20() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_10x20, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(30, 60));
}

#[test]
fn text_a_draw_10x20() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_10x20, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"          ",
			"          ",
			"          ",
			"          ",
			"          ",
			"          ",
			"  #####   ",
			"       #  ",
			"       #  ",
			"  ######  ",
			" #     #  ",
			" #     #  ",
			" #    ##  ",
			"  #### #  ",
			"          ",
			"          ",
			"          ",
			"          ",
			"          ",
			"          ",
		])
	);
}

// ### Pixel-Double Fonts

#[test]
fn font_size_10x18() {
	assert_eq!(FONT_10x18.width(), 10);
	assert_eq!(FONT_10x18.height(), 18);
}

#[test]
fn text_empty_size_10x18() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_10x18, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_10x18() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_10x18, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(10, 18));
}

#[test]
fn text_multiline_size_10x18() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_10x18, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(30, 54));
}

#[test]
fn text_a_draw_10x18() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_10x18, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"          ",
			"          ",
			"          ",
			"          ",
			"          ",
			"          ",
			"  ######  ",
			"  ######  ",
			"##    ##  ",
			"##    ##  ",
			"##    ##  ",
			"##    ##  ",
			"  ######  ",
			"  ######  ",
			"          ",
			"          ",
			"          ",
			"          ",
		])
	);
}#[test]
fn font_size_12x24() {
	assert_eq!(FONT_12x24.width(), 12);
	assert_eq!(FONT_12x24.height(), 24);
}

#[test]
fn text_empty_size_12x24() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_12x24, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_12x24() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_12x24, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(12, 24));
}

#[test]
fn text_multiline_size_12x24() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_12x24, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(36, 72));
}

#[test]
fn text_a_draw_12x24() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_12x24, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"            ",
			"            ",
			"            ",
			"            ",
			"            ",
			"            ",
			"            ",
			"            ",
			"  ######    ",
			"  ######    ",
			"        ##  ",
			"        ##  ",
			"  ########  ",
			"  ########  ",
			"##      ##  ",
			"##      ##  ",
			"  ########  ",
			"  ########  ",
			"            ",
			"            ",
			"            ",
			"            ",
			"            ",
			"            ",
		])
	);
}#[test]
fn font_size_14x26() {
	assert_eq!(FONT_14x26.width(), 14);
	assert_eq!(FONT_14x26.height(), 26);
}

#[test]
fn text_empty_size_14x26() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_14x26, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_14x26() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_14x26, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(14, 26));
}

#[test]
fn text_multiline_size_14x26() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_14x26, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(42, 78));
}

#[test]
fn text_a_draw_14x26() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_14x26, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"    ######    ",
			"    ######    ",
			"          ##  ",
			"          ##  ",
			"    ########  ",
			"    ########  ",
			"  ##      ##  ",
			"  ##      ##  ",
			"    ########  ",
			"    ########  ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
		])
	);
}#[test]
fn font_size_14x28() {
	assert_eq!(FONT_14x28.width(), 14);
	assert_eq!(FONT_14x28.height(), 28);
}

#[test]
fn text_empty_size_14x28() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_14x28, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_14x28() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_14x28, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(14, 28));
}

#[test]
fn text_multiline_size_14x28() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_14x28, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(42, 84));
}

#[test]
fn text_a_draw_14x28() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_14x28, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"    ######    ",
			"    ######    ",
			"          ##  ",
			"          ##  ",
			"    ########  ",
			"    ########  ",
			"  ##      ##  ",
			"  ##      ##  ",
			"  ##      ##  ",
			"  ##      ##  ",
			"    ########  ",
			"    ########  ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
			"              ",
		])
	);
}#[test]
fn font_size_16x30() {
	assert_eq!(FONT_16x30.width(), 16);
	assert_eq!(FONT_16x30.height(), 30);
}

#[test]
fn text_empty_size_16x30() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_16x30, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_16x30() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_16x30, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(16, 30));
}

#[test]
fn text_multiline_size_16x30() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_16x30, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(48, 90));
}

#[test]
fn text_a_draw_16x30() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_16x30, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"    ########    ",
			"    ########    ",
			"            ##  ",
			"            ##  ",
			"    ##########  ",
			"    ##########  ",
			"  ##        ##  ",
			"  ##        ##  ",
			"  ##        ##  ",
			"  ##        ##  ",
			"    ##########  ",
			"    ##########  ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
		])
	);
}#[test]
fn font_size_16x32() {
	assert_eq!(FONT_16x32.width(), 16);
	assert_eq!(FONT_16x32.height(), 32);
}

#[test]
fn text_empty_size_16x32() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_16x32, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_16x32() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_16x32, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(16, 32));
}

#[test]
fn text_multiline_size_16x32() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_16x32, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(48, 96));
}

#[test]
fn text_a_draw_16x32() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_16x32, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"    ########    ",
			"    ########    ",
			"            ##  ",
			"            ##  ",
			"            ##  ",
			"            ##  ",
			"    ##########  ",
			"    ##########  ",
			"  ##        ##  ",
			"  ##        ##  ",
			"  ##      ####  ",
			"  ##      ####  ",
			"    ######  ##  ",
			"    ######  ##  ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
			"                ",
		])
	);
}#[test]
fn font_size_20x40() {
	assert_eq!(FONT_20x40.width(), 20);
	assert_eq!(FONT_20x40.height(), 40);
}

#[test]
fn text_empty_size_20x40() {
	let size = Text::new("", Point::zero())
		.with_font(FONT_20x40, BinaryColor::On)
		.size();
	assert_eq!(size, Size::zero());
}

#[test]
fn text_a_size_20x40() {
	let size = Text::new("a", Point::zero())
		.with_font(FONT_20x40, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(20, 40));
}

#[test]
fn text_multiline_size_20x40() {
	let size = Text::new("aa\naaa\na", Point::zero())
		.with_font(FONT_20x40, BinaryColor::On)
		.size();
	assert_eq!(size, Size::new(60, 120));
}

#[test]
fn text_a_draw_20x40() {
	let mut display = MockDisplay::new();
	Text::new("a", Point::zero())
		.with_font(FONT_20x40, BinaryColor::On)
		.draw(&mut display)
		.unwrap();
	assert_eq!(
		display,
		MockDisplay::from_pattern(&[
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"    ##########      ",
			"    ##########      ",
			"              ##    ",
			"              ##    ",
			"              ##    ",
			"              ##    ",
			"    ############    ",
			"    ############    ",
			"  ##          ##    ",
			"  ##          ##    ",
			"  ##          ##    ",
			"  ##          ##    ",
			"  ##        ####    ",
			"  ##        ####    ",
			"    ########  ##    ",
			"    ########  ##    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
			"                    ",
		])
	);
}
