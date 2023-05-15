(function() {var implementors = {
"embedded_graphics":[["impl&lt;C, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU32.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU32\">RawU32</a>, <a class=\"enum\" href=\"embedded_graphics/pixelcolor/raw/enum.BigEndian.html\" title=\"enum embedded_graphics::pixelcolor::raw::BigEndian\">BigEndian</a>, WIDTH, HEIGHT, N&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&lt;Raw = <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU32.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU32\">RawU32</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU32.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU32\">RawU32</a>&gt;,</span>"],["impl&lt;T, C&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/draw_target/struct.ColorConverted.html\" title=\"struct embedded_graphics::draw_target::ColorConverted\">ColorConverted</a>&lt;'_, T, C&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a>,\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;T::<a class=\"associatedtype\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html#associatedtype.Color\" title=\"type embedded_graphics::draw_target::DrawTarget::Color\">Color</a>&gt;,</span>"],["impl&lt;C&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/mock_display/struct.MockDisplay.html\" title=\"struct embedded_graphics::mock_display::MockDisplay\">MockDisplay</a>&lt;C&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>,</span>"],["impl&lt;C, BO, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU1.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU1\">RawU1</a>, BO, WIDTH, HEIGHT, N&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&lt;Raw = <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU1.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU1\">RawU1</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU1.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU1\">RawU1</a>&gt;,</span>"],["impl&lt;C, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU32.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU32\">RawU32</a>, <a class=\"enum\" href=\"embedded_graphics/pixelcolor/raw/enum.LittleEndian.html\" title=\"enum embedded_graphics::pixelcolor::raw::LittleEndian\">LittleEndian</a>, WIDTH, HEIGHT, N&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&lt;Raw = <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU32.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU32\">RawU32</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU32.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU32\">RawU32</a>&gt;,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/draw_target/struct.Cropped.html\" title=\"struct embedded_graphics::draw_target::Cropped\">Cropped</a>&lt;'_, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a>,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/draw_target/struct.Clipped.html\" title=\"struct embedded_graphics::draw_target::Clipped\">Clipped</a>&lt;'_, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a>,</span>"],["impl&lt;C, BO, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU4.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU4\">RawU4</a>, BO, WIDTH, HEIGHT, N&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&lt;Raw = <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU4.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU4\">RawU4</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU4.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU4\">RawU4</a>&gt;,</span>"],["impl&lt;C, BO, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU2.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU2\">RawU2</a>, BO, WIDTH, HEIGHT, N&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&lt;Raw = <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU2.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU2\">RawU2</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU2.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU2\">RawU2</a>&gt;,</span>"],["impl&lt;T&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/draw_target/struct.Translated.html\" title=\"struct embedded_graphics::draw_target::Translated\">Translated</a>&lt;'_, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a>,</span>"],["impl&lt;C, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU16.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU16\">RawU16</a>, <a class=\"enum\" href=\"embedded_graphics/pixelcolor/raw/enum.LittleEndian.html\" title=\"enum embedded_graphics::pixelcolor::raw::LittleEndian\">LittleEndian</a>, WIDTH, HEIGHT, N&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&lt;Raw = <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU16.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU16\">RawU16</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU16.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU16\">RawU16</a>&gt;,</span>"],["impl&lt;C, BO, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU8.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU8\">RawU8</a>, BO, WIDTH, HEIGHT, N&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&lt;Raw = <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU8.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU8\">RawU8</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU8.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU8\">RawU8</a>&gt;,</span>"],["impl&lt;C, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU24.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU24\">RawU24</a>, <a class=\"enum\" href=\"embedded_graphics/pixelcolor/raw/enum.LittleEndian.html\" title=\"enum embedded_graphics::pixelcolor::raw::LittleEndian\">LittleEndian</a>, WIDTH, HEIGHT, N&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&lt;Raw = <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU24.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU24\">RawU24</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU24.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU24\">RawU24</a>&gt;,</span>"],["impl&lt;C, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU24.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU24\">RawU24</a>, <a class=\"enum\" href=\"embedded_graphics/pixelcolor/raw/enum.BigEndian.html\" title=\"enum embedded_graphics::pixelcolor::raw::BigEndian\">BigEndian</a>, WIDTH, HEIGHT, N&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&lt;Raw = <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU24.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU24\">RawU24</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU24.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU24\">RawU24</a>&gt;,</span>"],["impl&lt;C, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.69.0/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU16.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU16\">RawU16</a>, <a class=\"enum\" href=\"embedded_graphics/pixelcolor/raw/enum.BigEndian.html\" title=\"enum embedded_graphics::pixelcolor::raw::BigEndian\">BigEndian</a>, WIDTH, HEIGHT, N&gt;<span class=\"where fmt-newline\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&lt;Raw = <a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU16.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU16\">RawU16</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"embedded_graphics/pixelcolor/raw/struct.RawU16.html\" title=\"struct embedded_graphics::pixelcolor::raw::RawU16\">RawU16</a>&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()