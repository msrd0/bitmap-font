(function() {var implementors = {};
implementors["embedded_graphics"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"embedded_graphics/geometry/trait.OriginDimensions.html\" title=\"trait embedded_graphics::geometry::OriginDimensions\">OriginDimensions</a> for <a class=\"struct\" href=\"embedded_graphics/draw_target/struct.Cropped.html\" title=\"struct embedded_graphics::draw_target::Cropped\">Cropped</a>&lt;'_, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"embedded_graphics/draw_target/trait.DrawTarget.html\" title=\"trait embedded_graphics::draw_target::DrawTarget\">DrawTarget</a>,&nbsp;</span>","synthetic":false,"types":["embedded_graphics::draw_target::cropped::Cropped"]},{"text":"impl&lt;C, BO&gt; <a class=\"trait\" href=\"embedded_graphics/geometry/trait.OriginDimensions.html\" title=\"trait embedded_graphics::geometry::OriginDimensions\">OriginDimensions</a> for <a class=\"struct\" href=\"embedded_graphics/image/struct.ImageRaw.html\" title=\"struct embedded_graphics::image::ImageRaw\">ImageRaw</a>&lt;'_, C, BO&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.57.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&lt;C as <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&gt;::<a class=\"type\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html#associatedtype.Raw\" title=\"type embedded_graphics::pixelcolor::PixelColor::Raw\">Raw</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;BO: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/raw/trait.ByteOrder.html\" title=\"trait embedded_graphics::pixelcolor::raw::ByteOrder\">ByteOrder</a>,&nbsp;</span>","synthetic":false,"types":["embedded_graphics::image::image_raw::ImageRaw"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"embedded_graphics/geometry/trait.OriginDimensions.html\" title=\"trait embedded_graphics::geometry::OriginDimensions\">OriginDimensions</a> for <a class=\"struct\" href=\"embedded_graphics/image/struct.SubImage.html\" title=\"struct embedded_graphics::image::SubImage\">SubImage</a>&lt;'_, T&gt;","synthetic":false,"types":["embedded_graphics::image::sub_image::SubImage"]},{"text":"impl&lt;C&gt; <a class=\"trait\" href=\"embedded_graphics/geometry/trait.OriginDimensions.html\" title=\"trait embedded_graphics::geometry::OriginDimensions\">OriginDimensions</a> for <a class=\"struct\" href=\"embedded_graphics/mock_display/struct.MockDisplay.html\" title=\"struct embedded_graphics::mock_display::MockDisplay\">MockDisplay</a>&lt;C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>,&nbsp;</span>","synthetic":false,"types":["embedded_graphics::mock_display::MockDisplay"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()