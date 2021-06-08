// @generated

mod font_5x9 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_5x9;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 5);
		assert_eq!(FONT.height(), 9);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(5, 9));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(15, 27));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(7, 11));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"          ",
				"          ",
				"          ",
				"     #  # ",
				"     #### ",
				"     #  # ",
				"     #  # ",
				"     #  # ",
				"          ",
				"          ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"               ",
				"               ",
				"      ##  ## # ",
				"     #  # # ## ",
				"     #  #      ",
				"     #  #      ",
				"      ##       ",
				"               ",
				"               ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"               ",
				"     #  #   #  ",
				"      ##    #  ",
				" #   #  #   #  ",
				"     #  #      ",
				" #    ##    #  ",
				" #   #  #   #  ",
				" #          #  ",
				" #             ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"               ",
				"               ",
				" ##   ##   ##  ",
				"#  # #  # #  # ",
				" ##   ##   ##  ",
				"               ",
				"               ",
				"               ",
				"               ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"               ",
				"          # #  ",
				"      ###      ",
				"  #  #  # #  # ",
				"     # #  #  # ",
				"  #  #  # #  # ",
				" #   # #   ### ",
				"#            # ",
				" ###       ##  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#        #    #",
				"##      ##   # ",
				"###    ###  #  ",
				"####  #### #   ",
				"###########    ",
				"####  #### #   ",
				"###    ###  #  ",
				"##      ##   # ",
				"#        #    #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"          ",
				"###  ###  ",
				"   #    # ",
				"  #    #  ",
				" #    #   ",
				"          ",
				" #    #   ",
				"          ",
				"          ",
				"          ",
				"     ###  ",
				"        # ",
				"       #  ",
				"      #   ",
				"          ",
				"      #   ",
				"          ",
				"          ",
			])
		);
	}
}

mod font_5x9_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_5x9_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 5);
		assert_eq!(FONT.height(), 9);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(5, 9));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(15, 27));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(7, 11));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"          ",
				"          ",
				"          ",
				"     #  # ",
				"     #### ",
				"     #### ",
				"     #  # ",
				"     #  # ",
				"          ",
				"          ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"               ",
				"           # # ",
				"      ##  #### ",
				"     ## # # #  ",
				"     ## #      ",
				"     ## #      ",
				"      ##       ",
				"               ",
				"               ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"               ",
				"     ## #  ##  ",
				"      ##   ##  ",
				" ##  ## #  ##  ",
				"     ## #      ",
				" ##   ##   ##  ",
				" ##  ## #  ##  ",
				" ##        ##  ",
				" ##            ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"               ",
				"               ",
				" ##   ##   ##  ",
				"## # ## # ## # ",
				" ##   ##   ##  ",
				"               ",
				"               ",
				"               ",
				"               ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"               ",
				"          # #  ",
				"      ###      ",
				" ##  ## # ## # ",
				"     ###  ## # ",
				" ##  ## # ## # ",
				"##   ###   ### ",
				"## #        ## ",
				" ##        ##  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#        #    #",
				"##      ##   # ",
				"###    ###  #  ",
				"####  #### #   ",
				"###########    ",
				"####  #### #   ",
				"###    ###  #  ",
				"##      ##   # ",
				"#        #    #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"          ",
				" ##   ##  ",
				"# ## # ## ",
				"  ##   ## ",
				" ##   ##  ",
				"          ",
				" ##   ##  ",
				"          ",
				"          ",
				"          ",
				"      ##  ",
				"     # ## ",
				"       ## ",
				"      ##  ",
				"          ",
				"      ##  ",
				"          ",
				"          ",
			])
		);
	}
}

mod font_6x12 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_6x12;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 6);
		assert_eq!(FONT.height(), 12);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(6, 12));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(18, 36));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(8, 14));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"           ",
				"           ",
				"           ",
				"     #   # ",
				"     ## ## ",
				"     # # # ",
				"     # # # ",
				"     #   # ",
				"     #   # ",
				"     #   # ",
				"           ",
				"           ",
				"           ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                  ",
				"                  ",
				"       ###   #  # ",
				"      #   # # # # ",
				"      #   # #  #  ",
				"      #   #       ",
				"      #   #       ",
				"      #   #       ",
				"       ###        ",
				"                  ",
				"                  ",
				"                  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                  ",
				"              #   ",
				"              #   ",
				"  #   #   #   #   ",
				"       ###    #   ",
				"       # #        ",
				"  #    ###    #   ",
				"  #   #   #   #   ",
				"  #           #   ",
				"  #           #   ",
				"  #               ",
				"                  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                  ",
				"                  ",
				" ##    ##    ##   ",
				"#  #  #  #  #  #  ",
				"#  #  #  #  #  #  ",
				" ##    ##    ##   ",
				"                  ",
				"                  ",
				"                  ",
				"                  ",
				"                  ",
				"                  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                  ",
				"                  ",
				"       ###   # #  ",
				"  #   #   #       ",
				"      #  #  #   # ",
				"      # ##  #   # ",
				"  #   #   # #   # ",
				" #    #   # #   # ",
				"#     # ##   #### ",
				"#   #           # ",
				" ###         ###  ",
				"                  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#          #     #",
				"##        ##    # ",
				"###      ###   #  ",
				"####    ####  #   ",
				"#####  ##### #    ",
				"#############     ",
				"#############     ",
				"#####  ##### #    ",
				"####    ####  #   ",
				"###      ###   #  ",
				"##        ##    # ",
				"#          #     #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"            ",
				" ###   ###  ",
				"#   # #   # ",
				"    #     # ",
				"   #     #  ",
				"  #     #   ",
				"            ",
				"            ",
				"  #     #   ",
				"            ",
				"            ",
				"            ",
				"            ",
				"       ###  ",
				"      #   # ",
				"          # ",
				"         #  ",
				"        #   ",
				"            ",
				"            ",
				"        #   ",
				"            ",
				"            ",
				"            ",
			])
		);
	}
}

mod font_6x12_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_6x12_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 6);
		assert_eq!(FONT.height(), 12);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(6, 12));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(18, 36));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(8, 14));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"           ",
				"           ",
				"           ",
				"     #   # ",
				"     ## ## ",
				"     ##### ",
				"     ##### ",
				"     # # # ",
				"     #   # ",
				"     #   # ",
				"           ",
				"           ",
				"           ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                  ",
				"                  ",
				"       ###   ## # ",
				"      ##  # ##### ",
				"      ##  # # ##  ",
				"      ##  #       ",
				"      ##  #       ",
				"      ##  #       ",
				"       ###        ",
				"                  ",
				"                  ",
				"                  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                  ",
				"              ##  ",
				"      ##  #   ##  ",
				" ##    ###    ##  ",
				"      ##  #   ##  ",
				"      ##  #       ",
				" ##    ###    ##  ",
				" ##   ##  #   ##  ",
				"####          ##  ",
				"####          ##  ",
				" ##               ",
				"                  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                  ",
				"                  ",
				" ###   ###   ###  ",
				"##  # ##  # ##  # ",
				"##  # ##  # ##  # ",
				" ###   ###   ###  ",
				"                  ",
				"                  ",
				"                  ",
				"                  ",
				"                  ",
				"                  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                  ",
				"       ###  ## ## ",
				"      ##  # ## ## ",
				"  ##  ##  #       ",
				"      ## #  ##  # ",
				"      ##  # ##  # ",
				"  ##  ##  # ##  # ",
				" ##   ##  # ##  # ",
				"##    ## #   #### ",
				"##  # ##       ## ",
				" ###         ###  ",
				"                  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#          #     #",
				"##        ##    # ",
				"###      ###   #  ",
				"####    ####  #   ",
				"#####  ##### #    ",
				"#############     ",
				"#############     ",
				"#####  ##### #    ",
				"####    ####  #   ",
				"###      ###   #  ",
				"##        ##    # ",
				"#          #     #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"            ",
				" ###   ###  ",
				"#  ## #  ## ",
				"   ##    ## ",
				"  ##    ##  ",
				" ##    ##   ",
				"            ",
				"            ",
				" ##    ##   ",
				"            ",
				"            ",
				"            ",
				"            ",
				"       ###  ",
				"      #  ## ",
				"         ## ",
				"        ##  ",
				"       ##   ",
				"            ",
				"            ",
				"       ##   ",
				"            ",
				"            ",
				"            ",
			])
		);
	}
}

mod font_7x13 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_7x13;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 7);
		assert_eq!(FONT.height(), 13);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(7, 13));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(21, 39));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(9, 15));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"            ",
				"            ",
				"            ",
				"            ",
				"      #   # ",
				"      ## ## ",
				"      # # # ",
				"      # # # ",
				"      #   # ",
				"      #   # ",
				"      #   # ",
				"            ",
				"            ",
				"            ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"         ###    #  # ",
				"        #   #  # # # ",
				"        #   #  #  #  ",
				"        #   #        ",
				"        #   #        ",
				"        #   #        ",
				"         ###         ",
				"                     ",
				"                     ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                 #   ",
				"                 #   ",
				"   #    #   #    #   ",
				"         ###     #   ",
				"         # #         ",
				"   #     ###     #   ",
				"   #    #   #    #   ",
				"   #             #   ",
				"   #             #   ",
				"   #                 ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"  ##     ##     ##   ",
				" #  #   #  #   #  #  ",
				" #  #   #  #   #  #  ",
				"  ##     ##     ##   ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"         ###    # #  ",
				"   #    #   #        ",
				"        #  #   #   # ",
				"        # ##   #   # ",
				"   #    #   #  #   # ",
				"  #     #   #  #   # ",
				" #      # ##    #### ",
				" #   #             # ",
				"  ###           ###  ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#            #      #",
				"##          ##     # ",
				"###        ###    #  ",
				"####      ####   #   ",
				"#####    #####  #    ",
				"######  ###### #     ",
				"###############      ",
				"######  ###### #     ",
				"#####    #####  #    ",
				"####      ####   #   ",
				"###        ###    #  ",
				"##          ##     # ",
				"#            #      #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"              ",
				"              ",
				"  ###    ###  ",
				" #   #  #   # ",
				"     #      # ",
				"    #      #  ",
				"   #      #   ",
				"              ",
				"              ",
				"   #      #   ",
				"              ",
				"              ",
				"              ",
				"              ",
				"              ",
				"         ###  ",
				"        #   # ",
				"            # ",
				"           #  ",
				"          #   ",
				"              ",
				"              ",
				"          #   ",
				"              ",
				"              ",
				"              ",
			])
		);
	}
}

mod font_7x13_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_7x13_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 7);
		assert_eq!(FONT.height(), 13);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(7, 13));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(21, 39));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(9, 15));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"            ",
				"            ",
				"            ",
				"            ",
				"     ##  ## ",
				"     ###### ",
				"     ###### ",
				"     ##  ## ",
				"     ##  ## ",
				"     ##  ## ",
				"     ##  ## ",
				"            ",
				"            ",
				"            ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"        ####   ##  # ",
				"       ##  ## ###### ",
				"       ##  ## #  ##  ",
				"       ##  ##        ",
				"       ##  ##        ",
				"       ##  ##        ",
				"        ####         ",
				"                     ",
				"                     ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                ##   ",
				"                ##   ",
				"  ##   ##  ##   ##   ",
				"        ####    ##   ",
				"        ####         ",
				"  ##    ####    ##   ",
				"  ##   ##  ##   ##   ",
				"  ##            ##   ",
				"  ##            ##   ",
				"  ##                 ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				" ###    ###    ###   ",
				"## ##  ## ##  ## ##  ",
				"## ##  ## ##  ## ##  ",
				" ###    ###    ###   ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"              ##  ## ",
				"        ####  ##  ## ",
				"  ##   ##  ##        ",
				"       ## ##  ##  ## ",
				"       ##  ## ##  ## ",
				"  ##   ##  ## ##  ## ",
				" ##    ##  ## ##  ## ",
				"##     ## ##   ##### ",
				"##   #            ## ",
				" ####          ####  ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#            #      #",
				"##          ##     # ",
				"###        ###    #  ",
				"####      ####   #   ",
				"#####    #####  #    ",
				"######  ###### #     ",
				"###############      ",
				"######  ###### #     ",
				"#####    #####  #    ",
				"####      ####   #   ",
				"###        ###    #  ",
				"##          ##     # ",
				"#            #      #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"              ",
				"              ",
				" ####   ####  ",
				"##  ## ##  ## ",
				"    ##     ## ",
				"   ##     ##  ",
				"  ##     ##   ",
				"              ",
				"              ",
				"  ##     ##   ",
				"              ",
				"              ",
				"              ",
				"              ",
				"              ",
				"        ####  ",
				"       ##  ## ",
				"           ## ",
				"          ##  ",
				"         ##   ",
				"              ",
				"              ",
				"         ##   ",
				"              ",
				"              ",
				"              ",
			])
		);
	}
}

mod font_7x14 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_7x14;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 7);
		assert_eq!(FONT.height(), 14);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(7, 14));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(21, 42));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(9, 16));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"            ",
				"            ",
				"            ",
				"            ",
				"      #   # ",
				"      ## ## ",
				"      # # # ",
				"      # # # ",
				"      #   # ",
				"      #   # ",
				"      #   # ",
				"      #   # ",
				"            ",
				"            ",
				"            ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"         ###    #  # ",
				"        #   #  # # # ",
				"        #   #  #  #  ",
				"        #   #        ",
				"        #   #        ",
				"        #   #        ",
				"        #   #        ",
				"         ###         ",
				"                     ",
				"                     ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"                 #   ",
				"   #    #   #    #   ",
				"   #     ###     #   ",
				"         # #     #   ",
				"         ###         ",
				"   #    #   #    #   ",
				"   #             #   ",
				"   #             #   ",
				"   #             #   ",
				"   #                 ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"  ##     ##     ##   ",
				" #  #   #  #   #  #  ",
				" #  #   #  #   #  #  ",
				"  ##     ##     ##   ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"         ###    # #  ",
				"   #    #   #        ",
				"   #    #  #   #   # ",
				"        # ##   #   # ",
				"        #   #  #   # ",
				"   #    #   #  #   # ",
				"  #     #   #  #   # ",
				" #      # ##    #### ",
				" #   #             # ",
				"  ###           ###  ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#            #      #",
				"##          ##     # ",
				"###        ###    #  ",
				"####      ####   #   ",
				"#####    #####  #    ",
				"######  ###### #     ",
				"###############      ",
				"###############      ",
				"######  ###### #     ",
				"#####    #####  #    ",
				"####      ####   #   ",
				"###        ###    #  ",
				"##          ##     # ",
				"#            #      #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"              ",
				"              ",
				"  ###    ###  ",
				" #   #  #   # ",
				"     #      # ",
				"    #      #  ",
				"   #      #   ",
				"              ",
				"              ",
				"   #      #   ",
				"   #      #   ",
				"              ",
				"              ",
				"              ",
				"              ",
				"              ",
				"         ###  ",
				"        #   # ",
				"            # ",
				"           #  ",
				"          #   ",
				"              ",
				"              ",
				"          #   ",
				"          #   ",
				"              ",
				"              ",
				"              ",
			])
		);
	}
}

mod font_7x14_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_7x14_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 7);
		assert_eq!(FONT.height(), 14);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(7, 14));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(21, 42));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(9, 16));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"            ",
				"            ",
				"            ",
				"            ",
				"     ##  ## ",
				"     ###### ",
				"     ###### ",
				"     ###### ",
				"     ##  ## ",
				"     ##  ## ",
				"     ##  ## ",
				"     ##  ## ",
				"            ",
				"            ",
				"            ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"        ####   ##  # ",
				"       ##  ## ###### ",
				"       ##  ## #  ##  ",
				"       ##  ##        ",
				"       ##  ##        ",
				"       ##  ##        ",
				"       ##  ##        ",
				"        ####         ",
				"                     ",
				"                     ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"       ##  ##   ##   ",
				"  ##    ####    ##   ",
				"  ##   ##  ##   ##   ",
				"       ##  ##   ##   ",
				"       ##  ##        ",
				"  ##    ####    ##   ",
				"  ##   ##  ##   ##   ",
				" ####           ##   ",
				" ####           ##   ",
				"  ##                 ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				" ###    ###    ###   ",
				"## ##  ## ##  ## ##  ",
				"## ##  ## ##  ## ##  ",
				" ###    ###    ###   ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"               ## ## ",
				"        ####   ## ## ",
				"  ##   ##  ##        ",
				"  ##   ##  ## ##  ## ",
				"       ## ##  ##  ## ",
				"       ##  ## ##  ## ",
				"  ##   ##  ## ##  ## ",
				" ##    ##  ## ##  ## ",
				"##     ## ##   ##### ",
				"##   #            ## ",
				" ####          ####  ",
				"                     ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#            #      #",
				"##          ##     # ",
				"###        ###    #  ",
				"####      ####   #   ",
				"#####    #####  #    ",
				"######  ###### #     ",
				"###############      ",
				"###############      ",
				"######  ###### #     ",
				"#####    #####  #    ",
				"####      ####   #   ",
				"###        ###    #  ",
				"##          ##     # ",
				"#            #      #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"              ",
				"              ",
				" ####   ####  ",
				"#   ## #   ## ",
				"    ##     ## ",
				"   ##     ##  ",
				"  ##     ##   ",
				"              ",
				"              ",
				"  ##     ##   ",
				"  ##     ##   ",
				"              ",
				"              ",
				"              ",
				"              ",
				"              ",
				"        ####  ",
				"       #   ## ",
				"           ## ",
				"          ##  ",
				"         ##   ",
				"              ",
				"              ",
				"         ##   ",
				"         ##   ",
				"              ",
				"              ",
				"              ",
			])
		);
	}
}

mod font_8x15 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_8x15;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 8);
		assert_eq!(FONT.height(), 15);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(8, 15));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(24, 45));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(10, 17));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"             ",
				"             ",
				"             ",
				"             ",
				"             ",
				"      #     #",
				"      ##   ##",
				"      # # # #",
				"      #  #  #",
				"      #  #  #",
				"      #     #",
				"      #     #",
				"      #     #",
				"             ",
				"             ",
				"             ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"          ####    ##   #",
				"         #    #  #  #  #",
				"         #    #  #   ## ",
				"         #    #         ",
				"         #    #         ",
				"         #    #         ",
				"         #    #         ",
				"          ####          ",
				"                        ",
				"                        ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                    #   ",
				"         #    #     #   ",
				"   #      ####      #   ",
				"   #      #  #      #   ",
				"          #  #          ",
				"          ####          ",
				"   #     #    #     #   ",
				"   #                #   ",
				"   #                #   ",
				"   #                #   ",
				"   #                    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"  ###     ###     ###   ",
				" #   #   #   #   #   #  ",
				" #   #   #   #   #   #  ",
				" #   #   #   #   #   #  ",
				"  ###     ###     ###   ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"                  #  #  ",
				"          ####    #  #  ",
				"         #    #         ",
				"    #    #   #   #    # ",
				"    #    #  ##   #    # ",
				"         #    #  #    # ",
				"         #    #  #    # ",
				"    #    #    #  #    # ",
				"   #     #  ##    ##### ",
				"  #                   # ",
				"  #   #               # ",
				"   ###            ####  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#              #       #",
				"##            ##      # ",
				"###          ###     #  ",
				"####        ####    #   ",
				"#####      #####   #    ",
				"######    ######  #     ",
				"#######  ####### #      ",
				"#################       ",
				"#######  ####### #      ",
				"######    ######  #     ",
				"#####      #####   #    ",
				"####        ####    #   ",
				"###          ###     #  ",
				"##            ##      # ",
				"#              #       #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                ",
				"                ",
				"                ",
				"  ####    ####  ",
				" #    #  #    # ",
				"     #       #  ",
				"    #       #   ",
				"   #       #    ",
				"                ",
				"                ",
				"   #       #    ",
				"   #       #    ",
				"                ",
				"                ",
				"                ",
				"                ",
				"                ",
				"                ",
				"          ####  ",
				"         #    # ",
				"             #  ",
				"            #   ",
				"           #    ",
				"                ",
				"                ",
				"           #    ",
				"           #    ",
				"                ",
				"                ",
				"                ",
			])
		);
	}
}

mod font_8x15_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_8x15_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 8);
		assert_eq!(FONT.height(), 15);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(8, 15));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(24, 45));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(10, 17));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"             ",
				"             ",
				"             ",
				"             ",
				"             ",
				"      ##   ##",
				"      ### ###",
				"      #######",
				"      ## # ##",
				"      ##   ##",
				"      ##   ##",
				"      ##   ##",
				"      ##   ##",
				"             ",
				"             ",
				"             ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"          ####    ###  #",
				"         ##  ##  #######",
				"         ##  ##  #  ### ",
				"         ##  ##         ",
				"         ##  ##         ",
				"         ##  ##         ",
				"         ##  ##         ",
				"          ####          ",
				"                        ",
				"                        ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"         ##   ##   ##   ",
				"          #####    ##   ",
				"   ##     ## ##    ##   ",
				"   ##    ##   ##   ##   ",
				"         ##   ##        ",
				"          ## ##         ",
				"   ##     #####    ##   ",
				"   ##    ##   ##   ##   ",
				"  ####             ##   ",
				"  ####             ##   ",
				"   ##                   ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"  ####    ####    ####  ",
				" ##  ##  ##  ##  ##  ## ",
				" ##  ##  ##  ##  ##  ## ",
				" ##  ##  ##  ##  ##  ## ",
				"  ####    ####    ####  ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"                 ##  ## ",
				"          ####   ##  ## ",
				"         ##  ##         ",
				"   ##    ##  ##  ##  ## ",
				"   ##    ## ##   ##  ## ",
				"         ##  ##  ##  ## ",
				"         ##  ##  ##  ## ",
				"   ##    ##  ##  ##  ## ",
				"  ##     ## ##    ##### ",
				" ##                  ## ",
				" ##   #              ## ",
				"  ####            ####  ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#              #       #",
				"##            ##      # ",
				"###          ###     #  ",
				"####        ####    #   ",
				"#####      #####   #    ",
				"######    ######  #     ",
				"#######  ####### #      ",
				"#################       ",
				"#######  ####### #      ",
				"######    ######  #     ",
				"#####      #####   #    ",
				"####        ####    #   ",
				"###          ###     #  ",
				"##            ##      # ",
				"#              #       #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                ",
				"                ",
				"                ",
				"  ####    ####  ",
				" #   ##  #   ## ",
				"     ##      ## ",
				"    ##      ##  ",
				"   ##      ##   ",
				"                ",
				"                ",
				"   ##      ##   ",
				"   ##      ##   ",
				"                ",
				"                ",
				"                ",
				"                ",
				"                ",
				"                ",
				"          ####  ",
				"         #   ## ",
				"             ## ",
				"            ##  ",
				"           ##   ",
				"                ",
				"                ",
				"           ##   ",
				"           ##   ",
				"                ",
				"                ",
				"                ",
			])
		);
	}
}

mod font_8x16 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_8x16;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 8);
		assert_eq!(FONT.height(), 16);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(8, 16));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(24, 48));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(10, 18));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"             ",
				"             ",
				"             ",
				"             ",
				"      #     #",
				"      ##   ##",
				"      # # # #",
				"      #  #  #",
				"      #  #  #",
				"      #     #",
				"      #     #",
				"      #     #",
				"      #     #",
				"             ",
				"             ",
				"             ",
				"             ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                  ##   #",
				"                 #  #  #",
				"          ####   #   ## ",
				"         #    #         ",
				"         #    #         ",
				"         #    #         ",
				"         #    #         ",
				"         #    #         ",
				"         #    #         ",
				"         #    #         ",
				"          ####          ",
				"                        ",
				"                        ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"                    #   ",
				"         #    #     #   ",
				"   #      ####      #   ",
				"   #      #  #      #   ",
				"          #  #          ",
				"          #  #          ",
				"   #      ####          ",
				"   #     #    #     #   ",
				"   #                #   ",
				"   #                #   ",
				"   #                #   ",
				"   #                    ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"  ###     ###     ###   ",
				" #   #   #   #   #   #  ",
				" #   #   #   #   #   #  ",
				" #   #   #   #   #   #  ",
				"  ###     ###     ###   ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                  #  #  ",
				"          ####    #  #  ",
				"         #    #         ",
				"    #    #   #   #    # ",
				"    #    #  ##   #    # ",
				"         #    #  #    # ",
				"         #    #  #    # ",
				"    #    #    #  #    # ",
				"   #     #   #   #    # ",
				"  #      # ##     ##### ",
				" #                    # ",
				" #    #               # ",
				"  ####            ####  ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"#              #       #",
				"##            ##      # ",
				"###          ###     #  ",
				"####        ####    #   ",
				"#####      #####   #    ",
				"######    ######  #     ",
				"#######  ####### #      ",
				"#################       ",
				"#################       ",
				"#######  ####### #      ",
				"######    ######  #     ",
				"#####      #####   #    ",
				"####        ####    #   ",
				"###          ###     #  ",
				"##            ##      # ",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                ",
				"                ",
				"  ####    ####  ",
				" #    #  #    # ",
				"      #       # ",
				"     #       #  ",
				"    #       #   ",
				"   #       #    ",
				"                ",
				"                ",
				"   #       #    ",
				"   #       #    ",
				"                ",
				"                ",
				"                ",
				"                ",
				"                ",
				"                ",
				"          ####  ",
				"         #    # ",
				"              # ",
				"             #  ",
				"            #   ",
				"           #    ",
				"                ",
				"                ",
				"           #    ",
				"           #    ",
				"                ",
				"                ",
				"                ",
				"                ",
			])
		);
	}
}

mod font_8x16_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_8x16_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 8);
		assert_eq!(FONT.height(), 16);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(8, 16));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(24, 48));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(10, 18));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"             ",
				"             ",
				"             ",
				"             ",
				"      ##   ##",
				"      ### ###",
				"      #######",
				"      ## # ##",
				"      ##   ##",
				"      ##   ##",
				"      ##   ##",
				"      ##   ##",
				"      ##   ##",
				"             ",
				"             ",
				"             ",
				"             ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                  ##  ##",
				"                 #### ##",
				"          ####   ## ####",
				"         ##  ##  ##  ## ",
				"         ##  ##         ",
				"         ##  ##         ",
				"         ##  ##         ",
				"         ##  ##         ",
				"         ##  ##         ",
				"         ##  ##         ",
				"          ####          ",
				"                        ",
				"                        ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"                   ##   ",
				"         ##   ##   ##   ",
				"   ##     #####    ##   ",
				"   ##     ## ##    ##   ",
				"         ##   ##        ",
				"         ##   ##        ",
				"   ##     ## ##         ",
				"   ##     #####    ##   ",
				"  ####   ##   ##   ##   ",
				"  ####             ##   ",
				"  ####             ##   ",
				"   ##                   ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                        ",
				"  ####    ####    ####  ",
				" ##  ##  ##  ##  ##  ## ",
				" ##  ##  ##  ##  ##  ## ",
				" ##  ##  ##  ##  ##  ## ",
				"  ####    ####    ####  ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"                 ##  ## ",
				"          ####   ##  ## ",
				"         ##  ##         ",
				"   ##    ##  ##  ##  ## ",
				"   ##    ## ##   ##  ## ",
				"         ##  ##  ##  ## ",
				"         ##  ##  ##  ## ",
				"   ##    ##  ##  ##  ## ",
				"  ##     ##  ##  ##  ## ",
				" ##      ## ##    ##### ",
				" ##                  ## ",
				" ##   #              ## ",
				"  ####            ####  ",
				"                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"#              #       #",
				"##            ##      # ",
				"###          ###     #  ",
				"####        ####    #   ",
				"#####      #####   #    ",
				"######    ######  #     ",
				"#######  ####### #      ",
				"#################       ",
				"#################       ",
				"#######  ####### #      ",
				"######    ######  #     ",
				"#####      #####   #    ",
				"####        ####    #   ",
				"###          ###     #  ",
				"##            ##      # ",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                ",
				"                ",
				"  ####    ####  ",
				" #   ##  #   ## ",
				"     ##      ## ",
				"     ##      ## ",
				"    ##      ##  ",
				"   ##      ##   ",
				"                ",
				"                ",
				"   ##      ##   ",
				"   ##      ##   ",
				"                ",
				"                ",
				"                ",
				"                ",
				"                ",
				"                ",
				"          ####  ",
				"         #   ## ",
				"             ## ",
				"             ## ",
				"            ##  ",
				"           ##   ",
				"                ",
				"                ",
				"           ##   ",
				"           ##   ",
				"                ",
				"                ",
				"                ",
				"                ",
			])
		);
	}
}

mod font_10x18 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_10x18;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 10);
		assert_eq!(FONT.height(), 18);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(10, 18));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(30, 54));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(12, 20));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"               ",
				"               ",
				"               ",
				"               ",
				"               ",
				"     ##    ##  ",
				"     ##    ##  ",
				"     ########  ",
				"     ########  ",
				"     ##    ##  ",
				"     ##    ##  ",
				"     ##    ##  ",
				"     ##    ##  ",
				"     ##    ##  ",
				"     ##    ##  ",
				"               ",
				"               ",
				"               ",
				"               ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"            ####    ####  ##  ",
				"            ####    ####  ##  ",
				"          ##    ##  ##  ####  ",
				"          ##    ##  ##  ####  ",
				"          ##    ##            ",
				"          ##    ##            ",
				"          ##    ##            ",
				"          ##    ##            ",
				"            ####              ",
				"            ####              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"          ##    ##      ##    ",
				"          ##    ##      ##    ",
				"            ####        ##    ",
				"            ####        ##    ",
				"  ##      ##    ##      ##    ",
				"  ##      ##    ##      ##    ",
				"          ##    ##            ",
				"          ##    ##            ",
				"  ##        ####        ##    ",
				"  ##        ####        ##    ",
				"  ##      ##    ##      ##    ",
				"  ##      ##    ##      ##    ",
				"  ##                    ##    ",
				"  ##                    ##    ",
				"  ##                          ",
				"  ##                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"  ####      ####      ####    ",
				"  ####      ####      ####    ",
				"##    ##  ##    ##  ##    ##  ",
				"##    ##  ##    ##  ##    ##  ",
				"  ####      ####      ####    ",
				"  ####      ####      ####    ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                    ##  ##    ",
				"                    ##  ##    ",
				"            ######            ",
				"            ######            ",
				"    ##    ##    ##  ##    ##  ",
				"    ##    ##    ##  ##    ##  ",
				"          ##  ##    ##    ##  ",
				"          ##  ##    ##    ##  ",
				"    ##    ##    ##  ##    ##  ",
				"    ##    ##    ##  ##    ##  ",
				"  ##      ##  ##      ######  ",
				"  ##      ##  ##      ######  ",
				"##                        ##  ",
				"##                        ##  ",
				"  ######              ####    ",
				"  ######              ####    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                ##        ##",
				"##                ##        ##",
				"####            ####      ##  ",
				"####            ####      ##  ",
				"######        ######    ##    ",
				"######        ######    ##    ",
				"########    ########  ##      ",
				"########    ########  ##      ",
				"######################        ",
				"######################        ",
				"########    ########  ##      ",
				"########    ########  ##      ",
				"######        ######    ##    ",
				"######        ######    ##    ",
				"####            ####      ##  ",
				"####            ####      ##  ",
				"##                ##        ##",
				"##                ##        ##",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                    ",
				"                    ",
				"######    ######    ",
				"######    ######    ",
				"      ##        ##  ",
				"      ##        ##  ",
				"    ##        ##    ",
				"    ##        ##    ",
				"  ##        ##      ",
				"  ##        ##      ",
				"                    ",
				"                    ",
				"  ##        ##      ",
				"  ##        ##      ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"          ######    ",
				"          ######    ",
				"                ##  ",
				"                ##  ",
				"              ##    ",
				"              ##    ",
				"            ##      ",
				"            ##      ",
				"                    ",
				"                    ",
				"            ##      ",
				"            ##      ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
			])
		);
	}
}

mod font_10x18_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_10x18_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 10);
		assert_eq!(FONT.height(), 18);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(10, 18));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(30, 54));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(12, 20));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"               ",
				"               ",
				"               ",
				"               ",
				"               ",
				"     ##    ##  ",
				"     ##    ##  ",
				"     ########  ",
				"     ########  ",
				"     ########  ",
				"     ########  ",
				"     ##    ##  ",
				"     ##    ##  ",
				"     ##    ##  ",
				"     ##    ##  ",
				"               ",
				"               ",
				"               ",
				"               ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                      ##  ##  ",
				"                      ##  ##  ",
				"            ####    ########  ",
				"            ####    ########  ",
				"          ####  ##  ##  ##    ",
				"          ####  ##  ##  ##    ",
				"          ####  ##            ",
				"          ####  ##            ",
				"          ####  ##            ",
				"          ####  ##            ",
				"            ####              ",
				"            ####              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"          ####  ##    ####    ",
				"          ####  ##    ####    ",
				"            ####      ####    ",
				"            ####      ####    ",
				"  ####    ####  ##    ####    ",
				"  ####    ####  ##    ####    ",
				"          ####  ##            ",
				"          ####  ##            ",
				"  ####      ####      ####    ",
				"  ####      ####      ####    ",
				"  ####    ####  ##    ####    ",
				"  ####    ####  ##    ####    ",
				"  ####                ####    ",
				"  ####                ####    ",
				"  ####                        ",
				"  ####                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"  ####      ####      ####    ",
				"  ####      ####      ####    ",
				"####  ##  ####  ##  ####  ##  ",
				"####  ##  ####  ##  ####  ##  ",
				"  ####      ####      ####    ",
				"  ####      ####      ####    ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                    ##  ##    ",
				"                    ##  ##    ",
				"            ######            ",
				"            ######            ",
				"  ####    ####  ##  ####  ##  ",
				"  ####    ####  ##  ####  ##  ",
				"          ######    ####  ##  ",
				"          ######    ####  ##  ",
				"  ####    ####  ##  ####  ##  ",
				"  ####    ####  ##  ####  ##  ",
				"####      ######      ######  ",
				"####      ######      ######  ",
				"####  ##                ####  ",
				"####  ##                ####  ",
				"  ####                ####    ",
				"  ####                ####    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                ##        ##",
				"##                ##        ##",
				"####            ####      ##  ",
				"####            ####      ##  ",
				"######        ######    ##    ",
				"######        ######    ##    ",
				"########    ########  ##      ",
				"########    ########  ##      ",
				"######################        ",
				"######################        ",
				"########    ########  ##      ",
				"########    ########  ##      ",
				"######        ######    ##    ",
				"######        ######    ##    ",
				"####            ####      ##  ",
				"####            ####      ##  ",
				"##                ##        ##",
				"##                ##        ##",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                    ",
				"                    ",
				"  ####      ####    ",
				"  ####      ####    ",
				"##  ####  ##  ####  ",
				"##  ####  ##  ####  ",
				"    ####      ####  ",
				"    ####      ####  ",
				"  ####      ####    ",
				"  ####      ####    ",
				"                    ",
				"                    ",
				"  ####      ####    ",
				"  ####      ####    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"            ####    ",
				"            ####    ",
				"          ##  ####  ",
				"          ##  ####  ",
				"              ####  ",
				"              ####  ",
				"            ####    ",
				"            ####    ",
				"                    ",
				"                    ",
				"            ####    ",
				"            ####    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
			])
		);
	}
}

mod font_10x20 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_10x20;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 10);
		assert_eq!(FONT.height(), 20);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(10, 20));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(30, 60));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(12, 22));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"               ",
				"               ",
				"               ",
				"               ",
				"               ",
				"      #     #  ",
				"      ##   ##  ",
				"      # # # #  ",
				"      #  #  #  ",
				"      #  #  #  ",
				"      #     #  ",
				"      #     #  ",
				"      #     #  ",
				"      #     #  ",
				"      #     #  ",
				"               ",
				"               ",
				"               ",
				"               ",
				"               ",
				"               ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                              ",
				"                      ##    # ",
				"             ####    #  #   # ",
				"            #    #   #   #  # ",
				"           #      #  #    ##  ",
				"           #      #           ",
				"           #      #           ",
				"           #      #           ",
				"           #      #           ",
				"           #      #           ",
				"            #    #            ",
				"             ####             ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                              ",
				"                        #     ",
				"           #       #    #     ",
				"            # ### #     #     ",
				"    #        #   #      #     ",
				"    #       #     #     #     ",
				"            #     #           ",
				"            #     #           ",
				"             #   #            ",
				"    #       # ### #     #     ",
				"    #      #       #    #     ",
				"    #                   #     ",
				"    #                   #     ",
				"    #                   #     ",
				"    #                         ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"   ###       ###       ###    ",
				"  #   #     #   #     #   #   ",
				"  #   #     #   #     #   #   ",
				"  #   #     #   #     #   #   ",
				"   ###       ###       ###    ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                      #   #   ",
				"                      #   #   ",
				"             ####             ",
				"            #    #            ",
				"    #      #     #   #     #  ",
				"    #      #    #    #     #  ",
				"           #   ##    #     #  ",
				"           #     #   #     #  ",
				"           #     #   #     #  ",
				"    #      #     #   #     #  ",
				"   #       #    #    #     #  ",
				"  #        #  ##      ######  ",
				" #                         #  ",
				" #     #                   #  ",
				"  #####                    #  ",
				"                      #####   ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#                  #         #",
				"##                ##        # ",
				"###              ###       #  ",
				"####            ####      #   ",
				"#####          #####     #    ",
				"######        ######    #     ",
				"#######      #######   #      ",
				"########    ########  #       ",
				"#########  ######### #        ",
				"#####################         ",
				"#####################         ",
				"#########  ######### #        ",
				"########    ########  #       ",
				"#######      #######   #      ",
				"######        ######    #     ",
				"#####          #####     #    ",
				"####            ####      #   ",
				"###              ###       #  ",
				"##                ##        # ",
				"#                  #         #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                    ",
				"                    ",
				"                    ",
				"  #####     #####   ",
				" #     #   #     #  ",
				"       #         #  ",
				"      #         #   ",
				"     #         #    ",
				"    #         #     ",
				"                    ",
				"                    ",
				"                    ",
				"    #         #     ",
				"    #         #     ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"            #####   ",
				"           #     #  ",
				"                 #  ",
				"                #   ",
				"               #    ",
				"              #     ",
				"                    ",
				"                    ",
				"                    ",
				"              #     ",
				"              #     ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
			])
		);
	}
}

mod font_10x20_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_10x20_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 10);
		assert_eq!(FONT.height(), 20);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(10, 20));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(30, 60));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(12, 22));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"               ",
				"               ",
				"               ",
				"               ",
				"               ",
				"      ##    ## ",
				"      ###  ### ",
				"      ######## ",
				"      ## ## ## ",
				"      ##    ## ",
				"      ##    ## ",
				"      ##    ## ",
				"      ##    ## ",
				"      ##    ## ",
				"      ##    ## ",
				"               ",
				"               ",
				"               ",
				"               ",
				"               ",
				"               ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                              ",
				"                     ##   ##  ",
				"             ####   ####  ##  ",
				"            ##  ##  ##  ####  ",
				"           ##    ## ##   ##   ",
				"           ##    ##           ",
				"           ##    ##           ",
				"           ##    ##           ",
				"           ##    ##           ",
				"           ##    ##           ",
				"            ##  ##            ",
				"             ####             ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                              ",
				"                       ##     ",
				"                       ##     ",
				"          ##     ##    ##     ",
				"   ##      #######     ##     ",
				"   ##       ## ##      ##     ",
				"           ##   ##            ",
				"           ##   ##            ",
				"            ## ##             ",
				"   ##      #######     ##     ",
				"   ##     ##     ##    ##     ",
				"  ####                 ##     ",
				"  ####                 ##     ",
				"  ####                 ##     ",
				"   ##                         ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"  ####      ####      ####    ",
				" ##  ##    ##  ##    ##  ##   ",
				" ##  ##    ##  ##    ##  ##   ",
				" ##  ##    ##  ##    ##  ##   ",
				"  ####      ####      ####    ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                              ",
				"                              ",
				"                     ##  ##   ",
				"                     ##  ##   ",
				"            #####             ",
				"           ##   ##            ",
				"    ##     ##   ##   ##   ##  ",
				"    ##     ##  ##    ##   ##  ",
				"           ## ###    ##   ##  ",
				"           ##   ##   ##   ##  ",
				"           ##   ##   ##   ##  ",
				"    ##     ##   ##   ##   ##  ",
				"   ##      ##   ##   ##   ##  ",
				"  ##       ##  ##     ######  ",
				" ##                       ##  ",
				" ##    ##                 ##  ",
				"  ######              #####   ",
				"                              ",
				"                              ",
				"                              ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"#                  #         #",
				"##                ##        # ",
				"###              ###       #  ",
				"####            ####      #   ",
				"#####          #####     #    ",
				"######        ######    #     ",
				"#######      #######   #      ",
				"########    ########  #       ",
				"#########  ######### #        ",
				"#####################         ",
				"#####################         ",
				"#########  ######### #        ",
				"########    ########  #       ",
				"#######      #######   #      ",
				"######        ######    #     ",
				"#####          #####     #    ",
				"####            ####      #   ",
				"###              ###       #  ",
				"##                ##        # ",
				"#                  #         #",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                    ",
				"                    ",
				"                    ",
				"  ######    ######  ",
				" ##    ##  ##    ## ",
				"       ##        ## ",
				"      ##        ##  ",
				"     ##        ##   ",
				"    ##        ##    ",
				"                    ",
				"                    ",
				"                    ",
				"    ##        ##    ",
				"    ##        ##    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"            ######  ",
				"           ##    ## ",
				"                 ## ",
				"                ##  ",
				"               ##   ",
				"              ##    ",
				"                    ",
				"                    ",
				"                    ",
				"              ##    ",
				"              ##    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
				"                    ",
			])
		);
	}
}

mod font_12x24 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_12x24;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 12);
		assert_eq!(FONT.height(), 24);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(12, 24));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(36, 72));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(14, 26));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                 ",
				"                 ",
				"                 ",
				"                 ",
				"                 ",
				"     ##      ##  ",
				"     ##      ##  ",
				"     ####  ####  ",
				"     ####  ####  ",
				"     ##  ##  ##  ",
				"     ##  ##  ##  ",
				"     ##  ##  ##  ",
				"     ##  ##  ##  ",
				"     ##      ##  ",
				"     ##      ##  ",
				"     ##      ##  ",
				"     ##      ##  ",
				"     ##      ##  ",
				"     ##      ##  ",
				"                 ",
				"                 ",
				"                 ",
				"                 ",
				"                 ",
				"                 ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"              ######      ##    ##  ",
				"              ######      ##    ##  ",
				"            ##      ##  ##  ##  ##  ",
				"            ##      ##  ##  ##  ##  ",
				"            ##      ##  ##    ##    ",
				"            ##      ##  ##    ##    ",
				"            ##      ##              ",
				"            ##      ##              ",
				"            ##      ##              ",
				"            ##      ##              ",
				"            ##      ##              ",
				"            ##      ##              ",
				"              ######                ",
				"              ######                ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                    ",
				"                                    ",
				"                            ##      ",
				"                            ##      ",
				"                            ##      ",
				"                            ##      ",
				"    ##      ##      ##      ##      ",
				"    ##      ##      ##      ##      ",
				"              ######        ##      ",
				"              ######        ##      ",
				"              ##  ##                ",
				"              ##  ##                ",
				"    ##        ######        ##      ",
				"    ##        ######        ##      ",
				"    ##      ##      ##      ##      ",
				"    ##      ##      ##      ##      ",
				"    ##                      ##      ",
				"    ##                      ##      ",
				"    ##                      ##      ",
				"    ##                      ##      ",
				"    ##                              ",
				"    ##                              ",
				"                                    ",
				"                                    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"  ####        ####        ####      ",
				"  ####        ####        ####      ",
				"##    ##    ##    ##    ##    ##    ",
				"##    ##    ##    ##    ##    ##    ",
				"##    ##    ##    ##    ##    ##    ",
				"##    ##    ##    ##    ##    ##    ",
				"  ####        ####        ####      ",
				"  ####        ####        ####      ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"              ######      ##  ##    ",
				"              ######      ##  ##    ",
				"    ##      ##      ##              ",
				"    ##      ##      ##              ",
				"            ##    ##    ##      ##  ",
				"            ##    ##    ##      ##  ",
				"            ##  ####    ##      ##  ",
				"            ##  ####    ##      ##  ",
				"    ##      ##      ##  ##      ##  ",
				"    ##      ##      ##  ##      ##  ",
				"  ##        ##      ##  ##      ##  ",
				"  ##        ##      ##  ##      ##  ",
				"##          ##  ####      ########  ",
				"##          ##  ####      ########  ",
				"##      ##                      ##  ",
				"##      ##                      ##  ",
				"  ######                  ######    ",
				"  ######                  ######    ",
				"                                    ",
				"                                    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                    ##          ##",
				"##                    ##          ##",
				"####                ####        ##  ",
				"####                ####        ##  ",
				"######            ######      ##    ",
				"######            ######      ##    ",
				"########        ########    ##      ",
				"########        ########    ##      ",
				"##########    ##########  ##        ",
				"##########    ##########  ##        ",
				"##########################          ",
				"##########################          ",
				"##########################          ",
				"##########################          ",
				"##########    ##########  ##        ",
				"##########    ##########  ##        ",
				"########        ########    ##      ",
				"########        ########    ##      ",
				"######            ######      ##    ",
				"######            ######      ##    ",
				"####                ####        ##  ",
				"####                ####        ##  ",
				"##                    ##          ##",
				"##                    ##          ##",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"  ######      ######    ",
				"  ######      ######    ",
				"##      ##  ##      ##  ",
				"##      ##  ##      ##  ",
				"        ##          ##  ",
				"        ##          ##  ",
				"      ##          ##    ",
				"      ##          ##    ",
				"    ##          ##      ",
				"    ##          ##      ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"    ##          ##      ",
				"    ##          ##      ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"              ######    ",
				"              ######    ",
				"            ##      ##  ",
				"            ##      ##  ",
				"                    ##  ",
				"                    ##  ",
				"                  ##    ",
				"                  ##    ",
				"                ##      ",
				"                ##      ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                ##      ",
				"                ##      ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
			])
		);
	}
}

mod font_12x24_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_12x24_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 12);
		assert_eq!(FONT.height(), 24);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(12, 24));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(36, 72));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(14, 26));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                 ",
				"                 ",
				"                 ",
				"                 ",
				"                 ",
				"     ##      ##  ",
				"     ##      ##  ",
				"     ####  ####  ",
				"     ####  ####  ",
				"     ##########  ",
				"     ##########  ",
				"     ##########  ",
				"     ##########  ",
				"     ##  ##  ##  ",
				"     ##  ##  ##  ",
				"     ##      ##  ",
				"     ##      ##  ",
				"     ##      ##  ",
				"     ##      ##  ",
				"                 ",
				"                 ",
				"                 ",
				"                 ",
				"                 ",
				"                 ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"              ######      ####  ##  ",
				"              ######      ####  ##  ",
				"            ####    ##  ##########  ",
				"            ####    ##  ##########  ",
				"            ####    ##  ##  ####    ",
				"            ####    ##  ##  ####    ",
				"            ####    ##              ",
				"            ####    ##              ",
				"            ####    ##              ",
				"            ####    ##              ",
				"            ####    ##              ",
				"            ####    ##              ",
				"              ######                ",
				"              ######                ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                    ",
				"                                    ",
				"                            ####    ",
				"                            ####    ",
				"            ####    ##      ####    ",
				"            ####    ##      ####    ",
				"  ####        ######        ####    ",
				"  ####        ######        ####    ",
				"            ####    ##      ####    ",
				"            ####    ##      ####    ",
				"            ####    ##              ",
				"            ####    ##              ",
				"  ####        ######        ####    ",
				"  ####        ######        ####    ",
				"  ####      ####    ##      ####    ",
				"  ####      ####    ##      ####    ",
				"########                    ####    ",
				"########                    ####    ",
				"########                    ####    ",
				"########                    ####    ",
				"  ####                              ",
				"  ####                              ",
				"                                    ",
				"                                    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"  ######      ######      ######    ",
				"  ######      ######      ######    ",
				"####    ##  ####    ##  ####    ##  ",
				"####    ##  ####    ##  ####    ##  ",
				"####    ##  ####    ##  ####    ##  ",
				"####    ##  ####    ##  ####    ##  ",
				"  ######      ######      ######    ",
				"  ######      ######      ######    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
				"                                    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                    ",
				"                                    ",
				"              ######    ####  ####  ",
				"              ######    ####  ####  ",
				"            ####    ##  ####  ####  ",
				"            ####    ##  ####  ####  ",
				"    ####    ####    ##              ",
				"    ####    ####    ##              ",
				"            ####  ##    ####    ##  ",
				"            ####  ##    ####    ##  ",
				"            ####    ##  ####    ##  ",
				"            ####    ##  ####    ##  ",
				"    ####    ####    ##  ####    ##  ",
				"    ####    ####    ##  ####    ##  ",
				"  ####      ####    ##  ####    ##  ",
				"  ####      ####    ##  ####    ##  ",
				"####        ####  ##      ########  ",
				"####        ####  ##      ########  ",
				"####    ##  ####              ####  ",
				"####    ##  ####              ####  ",
				"  ######                  ######    ",
				"  ######                  ######    ",
				"                                    ",
				"                                    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                    ##          ##",
				"##                    ##          ##",
				"####                ####        ##  ",
				"####                ####        ##  ",
				"######            ######      ##    ",
				"######            ######      ##    ",
				"########        ########    ##      ",
				"########        ########    ##      ",
				"##########    ##########  ##        ",
				"##########    ##########  ##        ",
				"##########################          ",
				"##########################          ",
				"##########################          ",
				"##########################          ",
				"##########    ##########  ##        ",
				"##########    ##########  ##        ",
				"########        ########    ##      ",
				"########        ########    ##      ",
				"######            ######      ##    ",
				"######            ######      ##    ",
				"####                ####        ##  ",
				"####                ####        ##  ",
				"##                    ##          ##",
				"##                    ##          ##",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                        ",
				"                        ",
				"  ######      ######    ",
				"  ######      ######    ",
				"##    ####  ##    ####  ",
				"##    ####  ##    ####  ",
				"      ####        ####  ",
				"      ####        ####  ",
				"    ####        ####    ",
				"    ####        ####    ",
				"  ####        ####      ",
				"  ####        ####      ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"  ####        ####      ",
				"  ####        ####      ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"              ######    ",
				"              ######    ",
				"            ##    ####  ",
				"            ##    ####  ",
				"                  ####  ",
				"                  ####  ",
				"                ####    ",
				"                ####    ",
				"              ####      ",
				"              ####      ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"              ####      ",
				"              ####      ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
				"                        ",
			])
		);
	}
}

mod font_14x26 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_14x26;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 14);
		assert_eq!(FONT.height(), 26);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(14, 26));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(42, 78));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(16, 28));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ####  ####  ",
				"       ####  ####  ",
				"       ##  ##  ##  ",
				"       ##  ##  ##  ",
				"       ##  ##  ##  ",
				"       ##  ##  ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                  ######        ##    ##  ",
				"                  ######        ##    ##  ",
				"                ##      ##    ##  ##  ##  ",
				"                ##      ##    ##  ##  ##  ",
				"                ##      ##    ##    ##    ",
				"                ##      ##    ##    ##    ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                  ######                  ",
				"                  ######                  ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                  ##      ",
				"                                  ##      ",
				"                                  ##      ",
				"                                  ##      ",
				"      ##        ##      ##        ##      ",
				"      ##        ##      ##        ##      ",
				"                  ######          ##      ",
				"                  ######          ##      ",
				"                  ##  ##                  ",
				"                  ##  ##                  ",
				"      ##          ######          ##      ",
				"      ##          ######          ##      ",
				"      ##        ##      ##        ##      ",
				"      ##        ##      ##        ##      ",
				"      ##                          ##      ",
				"      ##                          ##      ",
				"      ##                          ##      ",
				"      ##                          ##      ",
				"      ##                                  ",
				"      ##                                  ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"    ####          ####          ####      ",
				"    ####          ####          ####      ",
				"  ##    ##      ##    ##      ##    ##    ",
				"  ##    ##      ##    ##      ##    ##    ",
				"  ##    ##      ##    ##      ##    ##    ",
				"  ##    ##      ##    ##      ##    ##    ",
				"    ####          ####          ####      ",
				"    ####          ####          ####      ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                  ######        ##  ##    ",
				"                  ######        ##  ##    ",
				"      ##        ##      ##                ",
				"      ##        ##      ##                ",
				"                ##    ##      ##      ##  ",
				"                ##    ##      ##      ##  ",
				"                ##  ####      ##      ##  ",
				"                ##  ####      ##      ##  ",
				"      ##        ##      ##    ##      ##  ",
				"      ##        ##      ##    ##      ##  ",
				"    ##          ##      ##    ##      ##  ",
				"    ##          ##      ##    ##      ##  ",
				"  ##            ##  ####        ########  ",
				"  ##            ##  ####        ########  ",
				"  ##      ##                          ##  ",
				"  ##      ##                          ##  ",
				"    ######                      ######    ",
				"    ######                      ######    ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                        ##            ##",
				"##                        ##            ##",
				"####                    ####          ##  ",
				"####                    ####          ##  ",
				"######                ######        ##    ",
				"######                ######        ##    ",
				"########            ########      ##      ",
				"########            ########      ##      ",
				"##########        ##########    ##        ",
				"##########        ##########    ##        ",
				"############    ############  ##          ",
				"############    ############  ##          ",
				"##############################            ",
				"##############################            ",
				"############    ############  ##          ",
				"############    ############  ##          ",
				"##########        ##########    ##        ",
				"##########        ##########    ##        ",
				"########            ########      ##      ",
				"########            ########      ##      ",
				"######                ######        ##    ",
				"######                ######        ##    ",
				"####                    ####          ##  ",
				"####                    ####          ##  ",
				"##                        ##            ##",
				"##                        ##            ##",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"    ######        ######    ",
				"    ######        ######    ",
				"  ##      ##    ##      ##  ",
				"  ##      ##    ##      ##  ",
				"          ##            ##  ",
				"          ##            ##  ",
				"        ##            ##    ",
				"        ##            ##    ",
				"      ##            ##      ",
				"      ##            ##      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"      ##            ##      ",
				"      ##            ##      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                  ######    ",
				"                  ######    ",
				"                ##      ##  ",
				"                ##      ##  ",
				"                        ##  ",
				"                        ##  ",
				"                      ##    ",
				"                      ##    ",
				"                    ##      ",
				"                    ##      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                    ##      ",
				"                    ##      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
			])
		);
	}
}

mod font_14x26_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_14x26_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 14);
		assert_eq!(FONT.height(), 26);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(14, 26));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(42, 78));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(16, 28));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ############  ",
				"     ############  ",
				"     ############  ",
				"     ############  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                ########      ####    ##  ",
				"                ########      ####    ##  ",
				"              ####    ####  ############  ",
				"              ####    ####  ############  ",
				"              ####    ####  ##    ####    ",
				"              ####    ####  ##    ####    ",
				"              ####    ####                ",
				"              ####    ####                ",
				"              ####    ####                ",
				"              ####    ####                ",
				"              ####    ####                ",
				"              ####    ####                ",
				"                ########                  ",
				"                ########                  ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                ####      ",
				"                                ####      ",
				"                                ####      ",
				"                                ####      ",
				"    ####      ####    ####      ####      ",
				"    ####      ####    ####      ####      ",
				"                ########        ####      ",
				"                ########        ####      ",
				"                ########                  ",
				"                ########                  ",
				"    ####        ########        ####      ",
				"    ####        ########        ####      ",
				"    ####      ####    ####      ####      ",
				"    ####      ####    ####      ####      ",
				"    ####                        ####      ",
				"    ####                        ####      ",
				"    ####                        ####      ",
				"    ####                        ####      ",
				"    ####                                  ",
				"    ####                                  ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"  ######        ######        ######      ",
				"  ######        ######        ######      ",
				"####  ####    ####  ####    ####  ####    ",
				"####  ####    ####  ####    ####  ####    ",
				"####  ####    ####  ####    ####  ####    ",
				"####  ####    ####  ####    ####  ####    ",
				"  ######        ######        ######      ",
				"  ######        ######        ######      ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                            ####    ####  ",
				"                            ####    ####  ",
				"                ########    ####    ####  ",
				"                ########    ####    ####  ",
				"    ####      ####    ####                ",
				"    ####      ####    ####                ",
				"              ####  ####    ####    ####  ",
				"              ####  ####    ####    ####  ",
				"              ####    ####  ####    ####  ",
				"              ####    ####  ####    ####  ",
				"    ####      ####    ####  ####    ####  ",
				"    ####      ####    ####  ####    ####  ",
				"  ####        ####    ####  ####    ####  ",
				"  ####        ####    ####  ####    ####  ",
				"####          ####  ####      ##########  ",
				"####          ####  ####      ##########  ",
				"####      ##                        ####  ",
				"####      ##                        ####  ",
				"  ########                    ########    ",
				"  ########                    ########    ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                        ##            ##",
				"##                        ##            ##",
				"####                    ####          ##  ",
				"####                    ####          ##  ",
				"######                ######        ##    ",
				"######                ######        ##    ",
				"########            ########      ##      ",
				"########            ########      ##      ",
				"##########        ##########    ##        ",
				"##########        ##########    ##        ",
				"############    ############  ##          ",
				"############    ############  ##          ",
				"##############################            ",
				"##############################            ",
				"############    ############  ##          ",
				"############    ############  ##          ",
				"##########        ##########    ##        ",
				"##########        ##########    ##        ",
				"########            ########      ##      ",
				"########            ########      ##      ",
				"######                ######        ##    ",
				"######                ######        ##    ",
				"####                    ####          ##  ",
				"####                    ####          ##  ",
				"##                        ##            ##",
				"##                        ##            ##",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"  ########      ########    ",
				"  ########      ########    ",
				"####    ####  ####    ####  ",
				"####    ####  ####    ####  ",
				"        ####          ####  ",
				"        ####          ####  ",
				"      ####          ####    ",
				"      ####          ####    ",
				"    ####          ####      ",
				"    ####          ####      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"    ####          ####      ",
				"    ####          ####      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                ########    ",
				"                ########    ",
				"              ####    ####  ",
				"              ####    ####  ",
				"                      ####  ",
				"                      ####  ",
				"                    ####    ",
				"                    ####    ",
				"                  ####      ",
				"                  ####      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                  ####      ",
				"                  ####      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
			])
		);
	}
}

mod font_14x28 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_14x28;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 14);
		assert_eq!(FONT.height(), 28);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(14, 28));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(42, 84));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(16, 30));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ####  ####  ",
				"       ####  ####  ",
				"       ##  ##  ##  ",
				"       ##  ##  ##  ",
				"       ##  ##  ##  ",
				"       ##  ##  ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"       ##      ##  ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                  ######        ##    ##  ",
				"                  ######        ##    ##  ",
				"                ##      ##    ##  ##  ##  ",
				"                ##      ##    ##  ##  ##  ",
				"                ##      ##    ##    ##    ",
				"                ##      ##    ##    ##    ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                ##      ##                ",
				"                  ######                  ",
				"                  ######                  ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                  ##      ",
				"                                  ##      ",
				"      ##        ##      ##        ##      ",
				"      ##        ##      ##        ##      ",
				"      ##          ######          ##      ",
				"      ##          ######          ##      ",
				"                  ##  ##          ##      ",
				"                  ##  ##          ##      ",
				"                  ######                  ",
				"                  ######                  ",
				"      ##        ##      ##        ##      ",
				"      ##        ##      ##        ##      ",
				"      ##                          ##      ",
				"      ##                          ##      ",
				"      ##                          ##      ",
				"      ##                          ##      ",
				"      ##                          ##      ",
				"      ##                          ##      ",
				"      ##                                  ",
				"      ##                                  ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"    ####          ####          ####      ",
				"    ####          ####          ####      ",
				"  ##    ##      ##    ##      ##    ##    ",
				"  ##    ##      ##    ##      ##    ##    ",
				"  ##    ##      ##    ##      ##    ##    ",
				"  ##    ##      ##    ##      ##    ##    ",
				"    ####          ####          ####      ",
				"    ####          ####          ####      ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                  ######        ##  ##    ",
				"                  ######        ##  ##    ",
				"      ##        ##      ##                ",
				"      ##        ##      ##                ",
				"      ##        ##    ##      ##      ##  ",
				"      ##        ##    ##      ##      ##  ",
				"                ##  ####      ##      ##  ",
				"                ##  ####      ##      ##  ",
				"                ##      ##    ##      ##  ",
				"                ##      ##    ##      ##  ",
				"      ##        ##      ##    ##      ##  ",
				"      ##        ##      ##    ##      ##  ",
				"    ##          ##      ##    ##      ##  ",
				"    ##          ##      ##    ##      ##  ",
				"  ##            ##  ####        ########  ",
				"  ##            ##  ####        ########  ",
				"  ##      ##                          ##  ",
				"  ##      ##                          ##  ",
				"    ######                      ######    ",
				"    ######                      ######    ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                        ##            ##",
				"##                        ##            ##",
				"####                    ####          ##  ",
				"####                    ####          ##  ",
				"######                ######        ##    ",
				"######                ######        ##    ",
				"########            ########      ##      ",
				"########            ########      ##      ",
				"##########        ##########    ##        ",
				"##########        ##########    ##        ",
				"############    ############  ##          ",
				"############    ############  ##          ",
				"##############################            ",
				"##############################            ",
				"##############################            ",
				"##############################            ",
				"############    ############  ##          ",
				"############    ############  ##          ",
				"##########        ##########    ##        ",
				"##########        ##########    ##        ",
				"########            ########      ##      ",
				"########            ########      ##      ",
				"######                ######        ##    ",
				"######                ######        ##    ",
				"####                    ####          ##  ",
				"####                    ####          ##  ",
				"##                        ##            ##",
				"##                        ##            ##",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"    ######        ######    ",
				"    ######        ######    ",
				"  ##      ##    ##      ##  ",
				"  ##      ##    ##      ##  ",
				"          ##            ##  ",
				"          ##            ##  ",
				"        ##            ##    ",
				"        ##            ##    ",
				"      ##            ##      ",
				"      ##            ##      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"      ##            ##      ",
				"      ##            ##      ",
				"      ##            ##      ",
				"      ##            ##      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                  ######    ",
				"                  ######    ",
				"                ##      ##  ",
				"                ##      ##  ",
				"                        ##  ",
				"                        ##  ",
				"                      ##    ",
				"                      ##    ",
				"                    ##      ",
				"                    ##      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                    ##      ",
				"                    ##      ",
				"                    ##      ",
				"                    ##      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
			])
		);
	}
}

mod font_14x28_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_14x28_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 14);
		assert_eq!(FONT.height(), 28);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(14, 28));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(42, 84));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(16, 30));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ############  ",
				"     ############  ",
				"     ############  ",
				"     ############  ",
				"     ############  ",
				"     ############  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"     ####    ####  ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
				"                   ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                ########      ####    ##  ",
				"                ########      ####    ##  ",
				"              ####    ####  ############  ",
				"              ####    ####  ############  ",
				"              ####    ####  ##    ####    ",
				"              ####    ####  ##    ####    ",
				"              ####    ####                ",
				"              ####    ####                ",
				"              ####    ####                ",
				"              ####    ####                ",
				"              ####    ####                ",
				"              ####    ####                ",
				"              ####    ####                ",
				"              ####    ####                ",
				"                ########                  ",
				"                ########                  ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"              ####    ####      ####      ",
				"              ####    ####      ####      ",
				"    ####        ########        ####      ",
				"    ####        ########        ####      ",
				"    ####      ####    ####      ####      ",
				"    ####      ####    ####      ####      ",
				"              ####    ####      ####      ",
				"              ####    ####      ####      ",
				"              ####    ####                ",
				"              ####    ####                ",
				"    ####        ########        ####      ",
				"    ####        ########        ####      ",
				"    ####      ####    ####      ####      ",
				"    ####      ####    ####      ####      ",
				"  ########                      ####      ",
				"  ########                      ####      ",
				"  ########                      ####      ",
				"  ########                      ####      ",
				"    ####                                  ",
				"    ####                                  ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"  ######        ######        ######      ",
				"  ######        ######        ######      ",
				"####  ####    ####  ####    ####  ####    ",
				"####  ####    ####  ####    ####  ####    ",
				"####  ####    ####  ####    ####  ####    ",
				"####  ####    ####  ####    ####  ####    ",
				"  ######        ######        ######      ",
				"  ######        ######        ######      ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                          ",
				"                                          ",
				"                                          ",
				"                                          ",
				"                              ####  ####  ",
				"                              ####  ####  ",
				"                ########      ####  ####  ",
				"                ########      ####  ####  ",
				"    ####      ####    ####                ",
				"    ####      ####    ####                ",
				"    ####      ####    ####  ####    ####  ",
				"    ####      ####    ####  ####    ####  ",
				"              ####  ####    ####    ####  ",
				"              ####  ####    ####    ####  ",
				"              ####    ####  ####    ####  ",
				"              ####    ####  ####    ####  ",
				"    ####      ####    ####  ####    ####  ",
				"    ####      ####    ####  ####    ####  ",
				"  ####        ####    ####  ####    ####  ",
				"  ####        ####    ####  ####    ####  ",
				"####          ####  ####      ##########  ",
				"####          ####  ####      ##########  ",
				"####      ##                        ####  ",
				"####      ##                        ####  ",
				"  ########                    ########    ",
				"  ########                    ########    ",
				"                                          ",
				"                                          ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                        ##            ##",
				"##                        ##            ##",
				"####                    ####          ##  ",
				"####                    ####          ##  ",
				"######                ######        ##    ",
				"######                ######        ##    ",
				"########            ########      ##      ",
				"########            ########      ##      ",
				"##########        ##########    ##        ",
				"##########        ##########    ##        ",
				"############    ############  ##          ",
				"############    ############  ##          ",
				"##############################            ",
				"##############################            ",
				"##############################            ",
				"##############################            ",
				"############    ############  ##          ",
				"############    ############  ##          ",
				"##########        ##########    ##        ",
				"##########        ##########    ##        ",
				"########            ########      ##      ",
				"########            ########      ##      ",
				"######                ######        ##    ",
				"######                ######        ##    ",
				"####                    ####          ##  ",
				"####                    ####          ##  ",
				"##                        ##            ##",
				"##                        ##            ##",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"  ########      ########    ",
				"  ########      ########    ",
				"##      ####  ##      ####  ",
				"##      ####  ##      ####  ",
				"        ####          ####  ",
				"        ####          ####  ",
				"      ####          ####    ",
				"      ####          ####    ",
				"    ####          ####      ",
				"    ####          ####      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"    ####          ####      ",
				"    ####          ####      ",
				"    ####          ####      ",
				"    ####          ####      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                ########    ",
				"                ########    ",
				"              ##      ####  ",
				"              ##      ####  ",
				"                      ####  ",
				"                      ####  ",
				"                    ####    ",
				"                    ####    ",
				"                  ####      ",
				"                  ####      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                  ####      ",
				"                  ####      ",
				"                  ####      ",
				"                  ####      ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
				"                            ",
			])
		);
	}
}

mod font_16x30 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_16x30;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 16);
		assert_eq!(FONT.height(), 30);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(16, 30));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(48, 90));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(18, 32));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"       ##          ##",
				"       ##          ##",
				"       ####      ####",
				"       ####      ####",
				"       ##  ##  ##  ##",
				"       ##  ##  ##  ##",
				"       ##    ##    ##",
				"       ##    ##    ##",
				"       ##    ##    ##",
				"       ##    ##    ##",
				"       ##          ##",
				"       ##          ##",
				"       ##          ##",
				"       ##          ##",
				"       ##          ##",
				"       ##          ##",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                    ########        ####      ##",
				"                    ########        ####      ##",
				"                  ##        ##    ##    ##    ##",
				"                  ##        ##    ##    ##    ##",
				"                  ##        ##    ##      ####  ",
				"                  ##        ##    ##      ####  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                    ########                    ",
				"                    ########                    ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                        ##      ",
				"                                        ##      ",
				"                  ##        ##          ##      ",
				"                  ##        ##          ##      ",
				"      ##            ########            ##      ",
				"      ##            ########            ##      ",
				"      ##            ##    ##            ##      ",
				"      ##            ##    ##            ##      ",
				"                    ##    ##                    ",
				"                    ##    ##                    ",
				"                    ########                    ",
				"                    ########                    ",
				"      ##          ##        ##          ##      ",
				"      ##          ##        ##          ##      ",
				"      ##                                ##      ",
				"      ##                                ##      ",
				"      ##                                ##      ",
				"      ##                                ##      ",
				"      ##                                ##      ",
				"      ##                                ##      ",
				"      ##                                        ",
				"      ##                                        ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"    ######          ######          ######      ",
				"    ######          ######          ######      ",
				"  ##      ##      ##      ##      ##      ##    ",
				"  ##      ##      ##      ##      ##      ##    ",
				"  ##      ##      ##      ##      ##      ##    ",
				"  ##      ##      ##      ##      ##      ##    ",
				"  ##      ##      ##      ##      ##      ##    ",
				"  ##      ##      ##      ##      ##      ##    ",
				"    ######          ######          ######      ",
				"    ######          ######          ######      ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                    ##    ##    ",
				"                                    ##    ##    ",
				"                    ########        ##    ##    ",
				"                    ########        ##    ##    ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"        ##        ##      ##      ##        ##  ",
				"        ##        ##      ##      ##        ##  ",
				"        ##        ##    ####      ##        ##  ",
				"        ##        ##    ####      ##        ##  ",
				"                  ##        ##    ##        ##  ",
				"                  ##        ##    ##        ##  ",
				"                  ##        ##    ##        ##  ",
				"                  ##        ##    ##        ##  ",
				"        ##        ##        ##    ##        ##  ",
				"        ##        ##        ##    ##        ##  ",
				"      ##          ##    ####        ##########  ",
				"      ##          ##    ####        ##########  ",
				"    ##                                      ##  ",
				"    ##                                      ##  ",
				"    ##      ##                              ##  ",
				"    ##      ##                              ##  ",
				"      ######                        ########    ",
				"      ######                        ########    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                            ##              ##",
				"##                            ##              ##",
				"####                        ####            ##  ",
				"####                        ####            ##  ",
				"######                    ######          ##    ",
				"######                    ######          ##    ",
				"########                ########        ##      ",
				"########                ########        ##      ",
				"##########            ##########      ##        ",
				"##########            ##########      ##        ",
				"############        ############    ##          ",
				"############        ############    ##          ",
				"##############    ##############  ##            ",
				"##############    ##############  ##            ",
				"##################################              ",
				"##################################              ",
				"##############    ##############  ##            ",
				"##############    ##############  ##            ",
				"############        ############    ##          ",
				"############        ############    ##          ",
				"##########            ##########      ##        ",
				"##########            ##########      ##        ",
				"########                ########        ##      ",
				"########                ########        ##      ",
				"######                    ######          ##    ",
				"######                    ######          ##    ",
				"####                        ####            ##  ",
				"####                        ####            ##  ",
				"##                            ##              ##",
				"##                            ##              ##",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"    ########        ########    ",
				"    ########        ########    ",
				"  ##        ##    ##        ##  ",
				"  ##        ##    ##        ##  ",
				"          ##              ##    ",
				"          ##              ##    ",
				"        ##              ##      ",
				"        ##              ##      ",
				"      ##              ##        ",
				"      ##              ##        ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"      ##              ##        ",
				"      ##              ##        ",
				"      ##              ##        ",
				"      ##              ##        ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                    ########    ",
				"                    ########    ",
				"                  ##        ##  ",
				"                  ##        ##  ",
				"                          ##    ",
				"                          ##    ",
				"                        ##      ",
				"                        ##      ",
				"                      ##        ",
				"                      ##        ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                      ##        ",
				"                      ##        ",
				"                      ##        ",
				"                      ##        ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
			])
		);
	}
}

mod font_16x30_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_16x30_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 16);
		assert_eq!(FONT.height(), 30);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(16, 30));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(48, 90));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(18, 32));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"       ####      ####",
				"       ####      ####",
				"       ######  ######",
				"       ######  ######",
				"       ##############",
				"       ##############",
				"       ####  ##  ####",
				"       ####  ##  ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                    ########        ######    ##",
				"                    ########        ######    ##",
				"                  ####    ####    ##############",
				"                  ####    ####    ##############",
				"                  ####    ####    ##    ######  ",
				"                  ####    ####    ##    ######  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                    ########                    ",
				"                    ########                    ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                  ####      ####      ####      ",
				"                  ####      ####      ####      ",
				"                    ##########        ####      ",
				"                    ##########        ####      ",
				"      ####          ####  ####        ####      ",
				"      ####          ####  ####        ####      ",
				"      ####        ####      ####      ####      ",
				"      ####        ####      ####      ####      ",
				"                  ####      ####                ",
				"                  ####      ####                ",
				"                    ####  ####                  ",
				"                    ####  ####                  ",
				"      ####          ##########        ####      ",
				"      ####          ##########        ####      ",
				"      ####        ####      ####      ####      ",
				"      ####        ####      ####      ####      ",
				"    ########                          ####      ",
				"    ########                          ####      ",
				"    ########                          ####      ",
				"    ########                          ####      ",
				"      ####                                      ",
				"      ####                                      ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"    ########        ########        ########    ",
				"    ########        ########        ########    ",
				"  ####    ####    ####    ####    ####    ####  ",
				"  ####    ####    ####    ####    ####    ####  ",
				"  ####    ####    ####    ####    ####    ####  ",
				"  ####    ####    ####    ####    ####    ####  ",
				"  ####    ####    ####    ####    ####    ####  ",
				"  ####    ####    ####    ####    ####    ####  ",
				"    ########        ########        ########    ",
				"    ########        ########        ########    ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                  ####    ####  ",
				"                                  ####    ####  ",
				"                    ########      ####    ####  ",
				"                    ########      ####    ####  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"      ####        ####    ####    ####    ####  ",
				"      ####        ####    ####    ####    ####  ",
				"      ####        ####  ####      ####    ####  ",
				"      ####        ####  ####      ####    ####  ",
				"                  ####    ####    ####    ####  ",
				"                  ####    ####    ####    ####  ",
				"                  ####    ####    ####    ####  ",
				"                  ####    ####    ####    ####  ",
				"      ####        ####    ####    ####    ####  ",
				"      ####        ####    ####    ####    ####  ",
				"    ####          ####  ####        ##########  ",
				"    ####          ####  ####        ##########  ",
				"  ####                                    ####  ",
				"  ####                                    ####  ",
				"  ####      ##                            ####  ",
				"  ####      ##                            ####  ",
				"    ########                        ########    ",
				"    ########                        ########    ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                            ##              ##",
				"##                            ##              ##",
				"####                        ####            ##  ",
				"####                        ####            ##  ",
				"######                    ######          ##    ",
				"######                    ######          ##    ",
				"########                ########        ##      ",
				"########                ########        ##      ",
				"##########            ##########      ##        ",
				"##########            ##########      ##        ",
				"############        ############    ##          ",
				"############        ############    ##          ",
				"##############    ##############  ##            ",
				"##############    ##############  ##            ",
				"##################################              ",
				"##################################              ",
				"##############    ##############  ##            ",
				"##############    ##############  ##            ",
				"############        ############    ##          ",
				"############        ############    ##          ",
				"##########            ##########      ##        ",
				"##########            ##########      ##        ",
				"########                ########        ##      ",
				"########                ########        ##      ",
				"######                    ######          ##    ",
				"######                    ######          ##    ",
				"####                        ####            ##  ",
				"####                        ####            ##  ",
				"##                            ##              ##",
				"##                            ##              ##",
			])
		);
	}
	
	#[test]
	fn text_fallback() {
		let mut display = MockDisplay::new();
		Text::new("§?\n µ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"    ########        ########    ",
				"    ########        ########    ",
				"  ##      ####    ##      ####  ",
				"  ##      ####    ##      ####  ",
				"          ####            ####  ",
				"          ####            ####  ",
				"        ####            ####    ",
				"        ####            ####    ",
				"      ####            ####      ",
				"      ####            ####      ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"      ####            ####      ",
				"      ####            ####      ",
				"      ####            ####      ",
				"      ####            ####      ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                    ########    ",
				"                    ########    ",
				"                  ##      ####  ",
				"                  ##      ####  ",
				"                          ####  ",
				"                          ####  ",
				"                        ####    ",
				"                        ####    ",
				"                      ####      ",
				"                      ####      ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                      ####      ",
				"                      ####      ",
				"                      ####      ",
				"                      ####      ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
				"                                ",
			])
		);
	}
}

mod font_16x32 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_16x32;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 16);
		assert_eq!(FONT.height(), 32);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(16, 32));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(48, 96));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(18, 34));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"       ##          ##",
				"       ##          ##",
				"       ####      ####",
				"       ####      ####",
				"       ##  ##  ##  ##",
				"       ##  ##  ##  ##",
				"       ##    ##    ##",
				"       ##    ##    ##",
				"       ##    ##    ##",
				"       ##    ##    ##",
				"       ##          ##",
				"       ##          ##",
				"       ##          ##",
				"       ##          ##",
				"       ##          ##",
				"       ##          ##",
				"       ##          ##",
				"       ##          ##",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                    ####      ##",
				"                                    ####      ##",
				"                                  ##    ##    ##",
				"                                  ##    ##    ##",
				"                    ########      ##      ####  ",
				"                    ########      ##      ####  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"                    ########                    ",
				"                    ########                    ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                        ##      ",
				"                                        ##      ",
				"                  ##        ##          ##      ",
				"                  ##        ##          ##      ",
				"      ##            ########            ##      ",
				"      ##            ########            ##      ",
				"      ##            ##    ##            ##      ",
				"      ##            ##    ##            ##      ",
				"                    ##    ##                    ",
				"                    ##    ##                    ",
				"                    ##    ##                    ",
				"                    ##    ##                    ",
				"      ##            ########                    ",
				"      ##            ########                    ",
				"      ##          ##        ##          ##      ",
				"      ##          ##        ##          ##      ",
				"      ##                                ##      ",
				"      ##                                ##      ",
				"      ##                                ##      ",
				"      ##                                ##      ",
				"      ##                                ##      ",
				"      ##                                ##      ",
				"      ##                                        ",
				"      ##                                        ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"    ######          ######          ######      ",
				"    ######          ######          ######      ",
				"  ##      ##      ##      ##      ##      ##    ",
				"  ##      ##      ##      ##      ##      ##    ",
				"  ##      ##      ##      ##      ##      ##    ",
				"  ##      ##      ##      ##      ##      ##    ",
				"  ##      ##      ##      ##      ##      ##    ",
				"  ##      ##      ##      ##      ##      ##    ",
				"    ######          ######          ######      ",
				"    ######          ######          ######      ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                    ##    ##    ",
				"                                    ##    ##    ",
				"                    ########        ##    ##    ",
				"                    ########        ##    ##    ",
				"                  ##        ##                  ",
				"                  ##        ##                  ",
				"        ##        ##      ##      ##        ##  ",
				"        ##        ##      ##      ##        ##  ",
				"        ##        ##    ####      ##        ##  ",
				"        ##        ##    ####      ##        ##  ",
				"                  ##        ##    ##        ##  ",
				"                  ##        ##    ##        ##  ",
				"                  ##        ##    ##        ##  ",
				"                  ##        ##    ##        ##  ",
				"        ##        ##        ##    ##        ##  ",
				"        ##        ##        ##    ##        ##  ",
				"      ##          ##      ##      ##        ##  ",
				"      ##          ##      ##      ##        ##  ",
				"    ##            ##  ####          ##########  ",
				"    ##            ##  ####          ##########  ",
				"  ##                                        ##  ",
				"  ##                                        ##  ",
				"  ##        ##                              ##  ",
				"  ##        ##                              ##  ",
				"    ########                        ########    ",
				"    ########                        ########    ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"##                            ##              ##",
				"##                            ##              ##",
				"####                        ####            ##  ",
				"####                        ####            ##  ",
				"######                    ######          ##    ",
				"######                    ######          ##    ",
				"########                ########        ##      ",
				"########                ########        ##      ",
				"##########            ##########      ##        ",
				"##########            ##########      ##        ",
				"############        ############    ##          ",
				"############        ############    ##          ",
				"##############    ##############  ##            ",
				"##############    ##############  ##            ",
				"##################################              ",
				"##################################              ",
				"##################################              ",
				"##################################              ",
				"##############    ##############  ##            ",
				"##############    ##############  ##            ",
				"############        ############    ##          ",
				"############        ############    ##          ",
				"##########            ##########      ##        ",
				"##########            ##########      ##        ",
				"########                ########        ##      ",
				"########                ########        ##      ",
				"######                    ######          ##    ",
				"######                    ######          ##    ",
				"####                        ####            ##  ",
				"####                        ####            ##  ",
			])
		);
	}
}

mod font_16x32_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_16x32_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 16);
		assert_eq!(FONT.height(), 32);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(16, 32));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(48, 96));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(18, 34));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"       ####      ####",
				"       ####      ####",
				"       ######  ######",
				"       ######  ######",
				"       ##############",
				"       ##############",
				"       ####  ##  ####",
				"       ####  ##  ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"       ####      ####",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
				"                     ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                    ####    ####",
				"                                    ####    ####",
				"                                  ########  ####",
				"                                  ########  ####",
				"                    ########      ####  ########",
				"                    ########      ####  ########",
				"                  ####    ####    ####    ####  ",
				"                  ####    ####    ####    ####  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"                    ########                    ",
				"                    ########                    ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                      ####      ",
				"                                      ####      ",
				"                  ####      ####      ####      ",
				"                  ####      ####      ####      ",
				"      ####          ##########        ####      ",
				"      ####          ##########        ####      ",
				"      ####          ####  ####        ####      ",
				"      ####          ####  ####        ####      ",
				"                  ####      ####                ",
				"                  ####      ####                ",
				"                  ####      ####                ",
				"                  ####      ####                ",
				"      ####          ####  ####                  ",
				"      ####          ####  ####                  ",
				"      ####          ##########        ####      ",
				"      ####          ##########        ####      ",
				"    ########      ####      ####      ####      ",
				"    ########      ####      ####      ####      ",
				"    ########                          ####      ",
				"    ########                          ####      ",
				"    ########                          ####      ",
				"    ########                          ####      ",
				"      ####                                      ",
				"      ####                                      ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"    ########        ########        ########    ",
				"    ########        ########        ########    ",
				"  ####    ####    ####    ####    ####    ####  ",
				"  ####    ####    ####    ####    ####    ####  ",
				"  ####    ####    ####    ####    ####    ####  ",
				"  ####    ####    ####    ####    ####    ####  ",
				"  ####    ####    ####    ####    ####    ####  ",
				"  ####    ####    ####    ####    ####    ####  ",
				"    ########        ########        ########    ",
				"    ########        ########        ########    ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"                                                ",
				"                                                ",
				"                                  ####    ####  ",
				"                                  ####    ####  ",
				"                    ########      ####    ####  ",
				"                    ########      ####    ####  ",
				"                  ####    ####                  ",
				"                  ####    ####                  ",
				"      ####        ####    ####    ####    ####  ",
				"      ####        ####    ####    ####    ####  ",
				"      ####        ####  ####      ####    ####  ",
				"      ####        ####  ####      ####    ####  ",
				"                  ####    ####    ####    ####  ",
				"                  ####    ####    ####    ####  ",
				"                  ####    ####    ####    ####  ",
				"                  ####    ####    ####    ####  ",
				"      ####        ####    ####    ####    ####  ",
				"      ####        ####    ####    ####    ####  ",
				"    ####          ####    ####    ####    ####  ",
				"    ####          ####    ####    ####    ####  ",
				"  ####            ####  ####        ##########  ",
				"  ####            ####  ####        ##########  ",
				"  ####                                    ####  ",
				"  ####                                    ####  ",
				"  ####      ##                            ####  ",
				"  ####      ##                            ####  ",
				"    ########                        ########    ",
				"    ########                        ########    ",
				"                                                ",
				"                                                ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                ",
				"                                                ",
				"##                            ##              ##",
				"##                            ##              ##",
				"####                        ####            ##  ",
				"####                        ####            ##  ",
				"######                    ######          ##    ",
				"######                    ######          ##    ",
				"########                ########        ##      ",
				"########                ########        ##      ",
				"##########            ##########      ##        ",
				"##########            ##########      ##        ",
				"############        ############    ##          ",
				"############        ############    ##          ",
				"##############    ##############  ##            ",
				"##############    ##############  ##            ",
				"##################################              ",
				"##################################              ",
				"##################################              ",
				"##################################              ",
				"##############    ##############  ##            ",
				"##############    ##############  ##            ",
				"############        ############    ##          ",
				"############        ############    ##          ",
				"##########            ##########      ##        ",
				"##########            ##########      ##        ",
				"########                ########        ##      ",
				"########                ########        ##      ",
				"######                    ######          ##    ",
				"######                    ######          ##    ",
				"####                        ####            ##  ",
				"####                        ####            ##  ",
			])
		);
	}
}

mod font_20x40 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_20x40;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 20);
		assert_eq!(FONT.height(), 40);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(20, 40));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(60, 120));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(22, 42));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"       ##          ##    ",
				"       ##          ##    ",
				"       ####      ####    ",
				"       ####      ####    ",
				"       ##  ##  ##  ##    ",
				"       ##  ##  ##  ##    ",
				"       ##    ##    ##    ",
				"       ##    ##    ##    ",
				"       ##    ##    ##    ",
				"       ##    ##    ##    ",
				"       ##          ##    ",
				"       ##          ##    ",
				"       ##          ##    ",
				"       ##          ##    ",
				"       ##          ##    ",
				"       ##          ##    ",
				"       ##          ##    ",
				"       ##          ##    ",
				"       ##          ##    ",
				"       ##          ##    ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                            ####        ##  ",
				"                                            ####        ##  ",
				"                          ########        ##    ##      ##  ",
				"                          ########        ##    ##      ##  ",
				"                        ##        ##      ##      ##    ##  ",
				"                        ##        ##      ##      ##    ##  ",
				"                      ##            ##    ##        ####    ",
				"                      ##            ##    ##        ####    ",
				"                      ##            ##                      ",
				"                      ##            ##                      ",
				"                      ##            ##                      ",
				"                      ##            ##                      ",
				"                      ##            ##                      ",
				"                      ##            ##                      ",
				"                      ##            ##                      ",
				"                      ##            ##                      ",
				"                      ##            ##                      ",
				"                      ##            ##                      ",
				"                        ##        ##                        ",
				"                        ##        ##                        ",
				"                          ########                          ",
				"                          ########                          ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                ##          ",
				"                                                ##          ",
				"                      ##              ##        ##          ",
				"                      ##              ##        ##          ",
				"                        ##  ######  ##          ##          ",
				"                        ##  ######  ##          ##          ",
				"        ##                ##      ##            ##          ",
				"        ##                ##      ##            ##          ",
				"        ##              ##          ##          ##          ",
				"        ##              ##          ##          ##          ",
				"                        ##          ##                      ",
				"                        ##          ##                      ",
				"                        ##          ##                      ",
				"                        ##          ##                      ",
				"                          ##      ##                        ",
				"                          ##      ##                        ",
				"        ##              ##  ######  ##          ##          ",
				"        ##              ##  ######  ##          ##          ",
				"        ##            ##              ##        ##          ",
				"        ##            ##              ##        ##          ",
				"        ##                                      ##          ",
				"        ##                                      ##          ",
				"        ##                                      ##          ",
				"        ##                                      ##          ",
				"        ##                                      ##          ",
				"        ##                                      ##          ",
				"        ##                                                  ",
				"        ##                                                  ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"      ######              ######              ######        ",
				"      ######              ######              ######        ",
				"    ##      ##          ##      ##          ##      ##      ",
				"    ##      ##          ##      ##          ##      ##      ",
				"    ##      ##          ##      ##          ##      ##      ",
				"    ##      ##          ##      ##          ##      ##      ",
				"    ##      ##          ##      ##          ##      ##      ",
				"    ##      ##          ##      ##          ##      ##      ",
				"      ######              ######              ######        ",
				"      ######              ######              ######        ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                            ##      ##      ",
				"                                            ##      ##      ",
				"                                            ##      ##      ",
				"                                            ##      ##      ",
				"                          ########                          ",
				"                          ########                          ",
				"                        ##        ##                        ",
				"                        ##        ##                        ",
				"        ##            ##          ##      ##          ##    ",
				"        ##            ##          ##      ##          ##    ",
				"        ##            ##        ##        ##          ##    ",
				"        ##            ##        ##        ##          ##    ",
				"                      ##      ####        ##          ##    ",
				"                      ##      ####        ##          ##    ",
				"                      ##          ##      ##          ##    ",
				"                      ##          ##      ##          ##    ",
				"                      ##          ##      ##          ##    ",
				"                      ##          ##      ##          ##    ",
				"        ##            ##          ##      ##          ##    ",
				"        ##            ##          ##      ##          ##    ",
				"      ##              ##        ##        ##          ##    ",
				"      ##              ##        ##        ##          ##    ",
				"    ##                ##    ####            ############    ",
				"    ##                ##    ####            ############    ",
				"  ##                                                  ##    ",
				"  ##                                                  ##    ",
				"  ##          ##                                      ##    ",
				"  ##          ##                                      ##    ",
				"    ##########                                        ##    ",
				"    ##########                                        ##    ",
				"                                            ##########      ",
				"                                            ##########      ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                                    ##                  ##",
				"##                                    ##                  ##",
				"####                                ####                ##  ",
				"####                                ####                ##  ",
				"######                            ######              ##    ",
				"######                            ######              ##    ",
				"########                        ########            ##      ",
				"########                        ########            ##      ",
				"##########                    ##########          ##        ",
				"##########                    ##########          ##        ",
				"############                ############        ##          ",
				"############                ############        ##          ",
				"##############            ##############      ##            ",
				"##############            ##############      ##            ",
				"################        ################    ##              ",
				"################        ################    ##              ",
				"##################    ##################  ##                ",
				"##################    ##################  ##                ",
				"##########################################                  ",
				"##########################################                  ",
				"##########################################                  ",
				"##########################################                  ",
				"##################    ##################  ##                ",
				"##################    ##################  ##                ",
				"################        ################    ##              ",
				"################        ################    ##              ",
				"##############            ##############      ##            ",
				"##############            ##############      ##            ",
				"############                ############        ##          ",
				"############                ############        ##          ",
				"##########                    ##########          ##        ",
				"##########                    ##########          ##        ",
				"########                        ########            ##      ",
				"########                        ########            ##      ",
				"######                            ######              ##    ",
				"######                            ######              ##    ",
				"####                                ####                ##  ",
				"####                                ####                ##  ",
				"##                                    ##                  ##",
				"##                                    ##                  ##",
			])
		);
	}
}

mod font_20x40_bold {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor,
		transform::Transform
	};
	
	const FONT: BitmapFont = FONT_20x40_BOLD;
	
	#[test]
	fn font_size() {
		assert_eq!(FONT.width(), 20);
		assert_eq!(FONT.height(), 40);
	}
	
	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}
	
	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(20, 40));
	}
	
	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(60, 120));
	}
	
	#[test]
	fn text_translate() {
		let mut text = Text::new("M", Point::zero())
			.with_font(FONT, BinaryColor::On);
		text.translate_mut(Point::new(2, 2));
		assert_eq!(text.top_left(), Point::new(2, 2));
		assert_eq!(text.bottom_right(), Point::new(22, 42));
		
		let mut display = MockDisplay::new();
		text.translate(Point::new(3, -1)).draw(&mut display).unwrap();
		assert_eq!(
			display,
			 MockDisplay::from_pattern(&[
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ######    ######  ",
				"       ######    ######  ",
				"       ################  ",
				"       ################  ",
				"       ####  ####  ####  ",
				"       ####  ####  ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"       ####        ####  ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
				"                         ",
			 ])
		);
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                          ####      ####    ",
				"                                          ####      ####    ",
				"                          ########      ########    ####    ",
				"                          ########      ########    ####    ",
				"                        ####    ####    ####    ########    ",
				"                        ####    ####    ####    ########    ",
				"                      ####        ####  ####      ####      ",
				"                      ####        ####  ####      ####      ",
				"                      ####        ####                      ",
				"                      ####        ####                      ",
				"                      ####        ####                      ",
				"                      ####        ####                      ",
				"                      ####        ####                      ",
				"                      ####        ####                      ",
				"                      ####        ####                      ",
				"                      ####        ####                      ",
				"                      ####        ####                      ",
				"                      ####        ####                      ",
				"                        ####    ####                        ",
				"                        ####    ####                        ",
				"                          ########                          ",
				"                          ########                          ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
			])
		);
	}
	
	#[test]
	fn text_char_range_2() {
		let mut display = MockDisplay::new();
		Text::new("¡¤¦", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                              ####          ",
				"                                              ####          ",
				"                                              ####          ",
				"                                              ####          ",
				"                    ####          ####        ####          ",
				"                    ####          ####        ####          ",
				"      ####            ##############          ####          ",
				"      ####            ##############          ####          ",
				"      ####              ####  ####            ####          ",
				"      ####              ####  ####            ####          ",
				"                      ####      ####                        ",
				"                      ####      ####                        ",
				"                      ####      ####                        ",
				"                      ####      ####                        ",
				"                        ####  ####                          ",
				"                        ####  ####                          ",
				"      ####            ##############          ####          ",
				"      ####            ##############          ####          ",
				"      ####          ####          ####        ####          ",
				"      ####          ####          ####        ####          ",
				"    ########                                  ####          ",
				"    ########                                  ####          ",
				"    ########                                  ####          ",
				"    ########                                  ####          ",
				"    ########                                  ####          ",
				"    ########                                  ####          ",
				"      ####                                                  ",
				"      ####                                                  ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
			])
		);
	}
	
	#[test]
	fn text_char_range_3() {
		let mut display = MockDisplay::new();
		Text::new("°°°", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"    ########            ########            ########        ",
				"    ########            ########            ########        ",
				"  ####    ####        ####    ####        ####    ####      ",
				"  ####    ####        ####    ####        ####    ####      ",
				"  ####    ####        ####    ####        ####    ####      ",
				"  ####    ####        ####    ####        ####    ####      ",
				"  ####    ####        ####    ####        ####    ####      ",
				"  ####    ####        ####    ####        ####    ####      ",
				"    ########            ########            ########        ",
				"    ########            ########            ########        ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
			])
		);
	}
	
	#[test]
	fn text_char_range_4() {
		let mut display = MockDisplay::new();
		Text::new("¿ßÿ", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                          ####    ####      ",
				"                                          ####    ####      ",
				"                                          ####    ####      ",
				"                                          ####    ####      ",
				"                        ##########                          ",
				"                        ##########                          ",
				"                      ####      ####                        ",
				"                      ####      ####                        ",
				"        ####          ####      ####      ####      ####    ",
				"        ####          ####      ####      ####      ####    ",
				"        ####          ####    ####        ####      ####    ",
				"        ####          ####    ####        ####      ####    ",
				"                      ####  ######        ####      ####    ",
				"                      ####  ######        ####      ####    ",
				"                      ####      ####      ####      ####    ",
				"                      ####      ####      ####      ####    ",
				"                      ####      ####      ####      ####    ",
				"                      ####      ####      ####      ####    ",
				"        ####          ####      ####      ####      ####    ",
				"        ####          ####      ####      ####      ####    ",
				"      ####            ####      ####      ####      ####    ",
				"      ####            ####      ####      ####      ####    ",
				"    ####              ####    ####          ############    ",
				"    ####              ####    ####          ############    ",
				"  ####                                              ####    ",
				"  ####                                              ####    ",
				"  ####        ####                                  ####    ",
				"  ####        ####                                  ####    ",
				"    ############                            ##########      ",
				"    ############                            ##########      ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
				"                                                            ",
			])
		);
	}
	
	#[test]
	fn text_char_range_5() {
		let mut display = MockDisplay::new();
		Text::new("", Point::zero())
			.with_font(FONT, BinaryColor::On)
			.draw(&mut display)
			.unwrap();
		assert_eq!(
			display,
			MockDisplay::from_pattern(&[
				"##                                    ##                  ##",
				"##                                    ##                  ##",
				"####                                ####                ##  ",
				"####                                ####                ##  ",
				"######                            ######              ##    ",
				"######                            ######              ##    ",
				"########                        ########            ##      ",
				"########                        ########            ##      ",
				"##########                    ##########          ##        ",
				"##########                    ##########          ##        ",
				"############                ############        ##          ",
				"############                ############        ##          ",
				"##############            ##############      ##            ",
				"##############            ##############      ##            ",
				"################        ################    ##              ",
				"################        ################    ##              ",
				"##################    ##################  ##                ",
				"##################    ##################  ##                ",
				"##########################################                  ",
				"##########################################                  ",
				"##########################################                  ",
				"##########################################                  ",
				"##################    ##################  ##                ",
				"##################    ##################  ##                ",
				"################        ################    ##              ",
				"################        ################    ##              ",
				"##############            ##############      ##            ",
				"##############            ##############      ##            ",
				"############                ############        ##          ",
				"############                ############        ##          ",
				"##########                    ##########          ##        ",
				"##########                    ##########          ##        ",
				"########                        ########            ##      ",
				"########                        ########            ##      ",
				"######                            ######              ##    ",
				"######                            ######              ##    ",
				"####                                ####                ##  ",
				"####                                ####                ##  ",
				"##                                    ##                  ##",
				"##                                    ##                  ##",
			])
		);
	}
}
