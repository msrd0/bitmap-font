// @generated

mod font_5x9 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_5x9.width(), 5);
		assert_eq!(FONT_5x9.height(), 9);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_5x9, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_5x9, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(5, 9));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_5x9, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(15, 27));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_5x9, BinaryColor::On)
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
			.with_font(FONT_5x9, BinaryColor::On)
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
			.with_font(FONT_5x9, BinaryColor::On)
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
			.with_font(FONT_5x9, BinaryColor::On)
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
			.with_font(FONT_5x9, BinaryColor::On)
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
			.with_font(FONT_5x9, BinaryColor::On)
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

mod font_6x12 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_6x12.width(), 6);
		assert_eq!(FONT_6x12.height(), 12);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_6x12, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_6x12, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(6, 12));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_6x12, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(18, 36));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_6x12, BinaryColor::On)
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
			.with_font(FONT_6x12, BinaryColor::On)
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
			.with_font(FONT_6x12, BinaryColor::On)
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
			.with_font(FONT_6x12, BinaryColor::On)
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
			.with_font(FONT_6x12, BinaryColor::On)
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
			.with_font(FONT_6x12, BinaryColor::On)
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

mod font_7x13 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_7x13.width(), 7);
		assert_eq!(FONT_7x13.height(), 13);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_7x13, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_7x13, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(7, 13));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_7x13, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(21, 39));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_7x13, BinaryColor::On)
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
			.with_font(FONT_7x13, BinaryColor::On)
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
			.with_font(FONT_7x13, BinaryColor::On)
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
			.with_font(FONT_7x13, BinaryColor::On)
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
			.with_font(FONT_7x13, BinaryColor::On)
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
			.with_font(FONT_7x13, BinaryColor::On)
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

mod font_7x14 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_7x14.width(), 7);
		assert_eq!(FONT_7x14.height(), 14);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_7x14, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_7x14, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(7, 14));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_7x14, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(21, 42));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_7x14, BinaryColor::On)
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
			.with_font(FONT_7x14, BinaryColor::On)
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
			.with_font(FONT_7x14, BinaryColor::On)
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
			.with_font(FONT_7x14, BinaryColor::On)
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
			.with_font(FONT_7x14, BinaryColor::On)
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
			.with_font(FONT_7x14, BinaryColor::On)
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

mod font_8x15 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_8x15.width(), 8);
		assert_eq!(FONT_8x15.height(), 15);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_8x15, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_8x15, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(8, 15));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_8x15, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(24, 45));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_8x15, BinaryColor::On)
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
			.with_font(FONT_8x15, BinaryColor::On)
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
			.with_font(FONT_8x15, BinaryColor::On)
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
			.with_font(FONT_8x15, BinaryColor::On)
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
			.with_font(FONT_8x15, BinaryColor::On)
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
			.with_font(FONT_8x15, BinaryColor::On)
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

mod font_8x16 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_8x16.width(), 8);
		assert_eq!(FONT_8x16.height(), 16);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_8x16, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_8x16, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(8, 16));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_8x16, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(24, 48));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_8x16, BinaryColor::On)
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
			.with_font(FONT_8x16, BinaryColor::On)
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
			.with_font(FONT_8x16, BinaryColor::On)
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
			.with_font(FONT_8x16, BinaryColor::On)
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
			.with_font(FONT_8x16, BinaryColor::On)
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
			.with_font(FONT_8x16, BinaryColor::On)
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

mod font_10x18 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_10x18.width(), 10);
		assert_eq!(FONT_10x18.height(), 18);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_10x18, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_10x18, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(10, 18));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_10x18, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(30, 54));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_10x18, BinaryColor::On)
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
			.with_font(FONT_10x18, BinaryColor::On)
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
			.with_font(FONT_10x18, BinaryColor::On)
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
			.with_font(FONT_10x18, BinaryColor::On)
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
			.with_font(FONT_10x18, BinaryColor::On)
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
			.with_font(FONT_10x18, BinaryColor::On)
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

mod font_10x20 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_10x20.width(), 10);
		assert_eq!(FONT_10x20.height(), 20);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_10x20, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_10x20, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(10, 20));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_10x20, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(30, 60));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_10x20, BinaryColor::On)
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
			.with_font(FONT_10x20, BinaryColor::On)
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
			.with_font(FONT_10x20, BinaryColor::On)
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
			.with_font(FONT_10x20, BinaryColor::On)
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
			.with_font(FONT_10x20, BinaryColor::On)
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
			.with_font(FONT_10x20, BinaryColor::On)
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

mod font_12x24 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_12x24.width(), 12);
		assert_eq!(FONT_12x24.height(), 24);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_12x24, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_12x24, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(12, 24));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_12x24, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(36, 72));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_12x24, BinaryColor::On)
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
			.with_font(FONT_12x24, BinaryColor::On)
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
			.with_font(FONT_12x24, BinaryColor::On)
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
			.with_font(FONT_12x24, BinaryColor::On)
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
			.with_font(FONT_12x24, BinaryColor::On)
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
			.with_font(FONT_12x24, BinaryColor::On)
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

mod font_14x26 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_14x26.width(), 14);
		assert_eq!(FONT_14x26.height(), 26);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_14x26, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_14x26, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(14, 26));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_14x26, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(42, 78));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_14x26, BinaryColor::On)
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
			.with_font(FONT_14x26, BinaryColor::On)
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
			.with_font(FONT_14x26, BinaryColor::On)
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
			.with_font(FONT_14x26, BinaryColor::On)
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
			.with_font(FONT_14x26, BinaryColor::On)
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
			.with_font(FONT_14x26, BinaryColor::On)
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

mod font_14x28 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_14x28.width(), 14);
		assert_eq!(FONT_14x28.height(), 28);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_14x28, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_14x28, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(14, 28));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_14x28, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(42, 84));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_14x28, BinaryColor::On)
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
			.with_font(FONT_14x28, BinaryColor::On)
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
			.with_font(FONT_14x28, BinaryColor::On)
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
			.with_font(FONT_14x28, BinaryColor::On)
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
			.with_font(FONT_14x28, BinaryColor::On)
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
			.with_font(FONT_14x28, BinaryColor::On)
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

mod font_16x30 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_16x30.width(), 16);
		assert_eq!(FONT_16x30.height(), 30);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_16x30, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_16x30, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(16, 30));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_16x30, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(48, 90));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_16x30, BinaryColor::On)
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
			.with_font(FONT_16x30, BinaryColor::On)
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
			.with_font(FONT_16x30, BinaryColor::On)
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
			.with_font(FONT_16x30, BinaryColor::On)
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
			.with_font(FONT_16x30, BinaryColor::On)
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
			.with_font(FONT_16x30, BinaryColor::On)
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

mod font_16x32 {
	use bitmap_font::*;
	use embedded_graphics::{
		drawable::Drawable,
		fonts::Text,
		geometry::{Dimensions, Point, Size},
		mock_display::MockDisplay,
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_16x32.width(), 16);
		assert_eq!(FONT_16x32.height(), 32);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_16x32, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_16x32, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(16, 32));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_16x32, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(48, 96));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_16x32, BinaryColor::On)
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
			.with_font(FONT_16x32, BinaryColor::On)
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
			.with_font(FONT_16x32, BinaryColor::On)
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
			.with_font(FONT_16x32, BinaryColor::On)
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
			.with_font(FONT_16x32, BinaryColor::On)
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
		pixelcolor::BinaryColor
	};

	#[test]
	fn font_size() {
		assert_eq!(FONT_20x40.width(), 20);
		assert_eq!(FONT_20x40.height(), 40);
	}

	#[test]
	fn text_empty_size() {
		let size = Text::new("", Point::zero())
			.with_font(FONT_20x40, BinaryColor::On)
			.size();
		assert_eq!(size, Size::zero());
	}

	#[test]
	fn text_a_size() {
		let size = Text::new("a", Point::zero())
			.with_font(FONT_20x40, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(20, 40));
	}

	#[test]
	fn text_multiline_size() {
		let size = Text::new("aa\naaa\na", Point::zero())
			.with_font(FONT_20x40, BinaryColor::On)
			.size();
		assert_eq!(size, Size::new(60, 120));
	}
	
	#[test]
	fn text_char_range_1() {
		let mut display = MockDisplay::new();
		Text::new(" O~", Point::zero())
			.with_font(FONT_20x40, BinaryColor::On)
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
			.with_font(FONT_20x40, BinaryColor::On)
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
			.with_font(FONT_20x40, BinaryColor::On)
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
			.with_font(FONT_20x40, BinaryColor::On)
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
			.with_font(FONT_20x40, BinaryColor::On)
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
