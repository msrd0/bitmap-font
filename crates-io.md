# bitmap-font [![Rust 1.57+](https://img.shields.io/badge/rustc-1.57+-orange.svg)](https://blog.rust-lang.org/2021/12/02/Rust-1.57.0.html) [![License Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0) [![GitHub](https://img.shields.io/badge/Code-On%20Github-blue?logo=GitHub)](https://github.com/msrd0/bitmap-font)

This crate provides bitmap fonts for the [`embedded-graphics`][__link0] crate. Those don’t only look better than the [built-in fonts][__link1] by using the good-looking [Tamzen font][__link2] over a font that renders `.` like a `+`, but also allow scaling fonts by pixel-doubling them, giving you two font sizes for the flash size requirements of the smaller one.

See the [`tamzen`][__link3] module for a list of all included fonts.


## Usage


```rust
use bitmap_font::{tamzen::FONT_8x15, BitmapFont, TextStyle};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};

// Draw text 'Hello World!' with the top left corner being the origin
let text = Text::new(
	"Hello World!",
	Point::zero(),
	TextStyle::new(&FONT_8x15, BinaryColor::On)
);
text.draw(&mut display)?;
```



## MSRV Policy

This crate is guaranteed to always build with the latest stable rust version. MSRV is documented; however, for your
information only. Changing the MSRV can be done at any point in time and is not considered a breaking change.

## License (Source Code)

Copyright (C) 2021-2022 Dominic Meiser and [contributors].

```
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

## License (Bitmap Font)

This license applies to the raw bitmap font data included in this crate.

```
Copyright 2011 Suraj N. Kurapati <https://github.com/sunaku/tamzen-font>

Tamzen font is free.  You are hereby granted permission to use, copy, modify,
and distribute it as you see fit.

Tamzen font is provided "as is" without any express or implied warranty.

The author makes no representations about the suitability of this font for
a particular purpose.

In no event will the author be held liable for damages arising from the use
of this font.
```

 [contributors]: https://github.com/msrd0/bitmap-font/graphs/contributors
 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEG8OLOcBnX8tEG0XUTjj8V5LfGxwiScerO4peG3cFs12J67qKYXKEG7QsMysq64PIGyOKYfl4MwNjG-1TTKAJaS94G62BH4qvezKCYWSCg2tiaXRtYXAtZm9udGUwLjIuMmtiaXRtYXBfZm9udIJxZW1iZWRkZWRfZ3JhcGhpY3NlMC43LjE
 [__link0]: https://crates.io/crates/embedded_graphics/0.7.1
 [__link1]: https://docs.rs/embedded_graphics/0.7.1/embedded_graphics/?search=mono_font
 [__link2]: https://github.com/sunaku/tamzen-font
 [__link3]: https://docs.rs/bitmap-font/0.2.2/bitmap_font/tamzen/index.html
