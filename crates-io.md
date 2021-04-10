# bitmap-font [![Rust 1.50+](https://img.shields.io/badge/rustc-1.50+-orange.svg)](https://blog.rust-lang.org/2021/02/11/Rust-1.50.0.html) [![License Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0) [![GitHub](https://img.shields.io/badge/Code-On%20Github-blue?logo=GitHub)](https://github.com/msrd0/bitmap-font)

This crate provides bitmap fonts for the `embedded-graphics` crate without requiring generics. All
fonts provided are concrete, constant instances of [`BitmapFont`]. This means you can use these
bitmap fonts without any generics, unlike those fonts shipped with `embedded-graphics` where each
font is implemented via its own struct. Also, this allows pixel-double fonts to share their bitmap
data with the non-doubled font, reducing the flash size required.

## Usage Example

```rust
use bitmap_font::{BitmapFont, WithFont, FONT_7x13};
use embedded_graphics::{fonts::Text, prelude::*};

let font: BitmapFont = FONT_7x13;
let text = Text::new("Hello World!", Point::zero());
text.with_font(font, BinaryColor::On).draw(&mut display)?;
```

## MSRV Policy

This crate is guaranteed to always build with the latest stable rust version. MSRV is documented; however, for your
information only. Changing the MSRV can be done at any point in time and is not considered a breaking change.

## License (Source Code)

Copyright (C) 2021 Dominic Meiser and [contributors](https://github.com/msrd0/bitmap-font/graphs/contributors).

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