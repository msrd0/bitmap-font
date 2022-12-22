#![warn(rust_2018_idioms)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]
// This suggestions makes the code soo much less readable
#![allow(clippy::needless_range_loop)]

use anyhow::bail;
use askama::Template;
use bdf_reader::Glyph;
use bit_vec::BitVec;
use linked_hash_map::LinkedHashMap;
use num_format::{Locale, ToFormattedString};
use std::{
	cmp::Ordering,
	convert::TryInto,
	fs::{self, File},
	io::{BufReader, Write}
};

mod bitmap;
use bitmap::Bitmap;

mod util;
use util::CeilingDiv;

const CHAR_RANGES: &[(char, char)] = &[
	// printable ascii chars
	(' ', '~'),
	// printable latin-1 chars
	('¡', '¦'),
	('°', '°'),
	('¿', 'ÿ'),
	// powerline
	('', '')
];
const MIN_ROWS: usize = 4;
const MAX_ROWS: usize = 20;

mod filters {
	pub fn chunks(src: &[u8], size: usize) -> askama::Result<Vec<&[u8]>> {
		Ok(src.chunks(size).collect())
	}
}

#[derive(Template)]
#[template(path = "src.rs.j2", escape = "none")]
struct RustSource<'a> {
	char_ranges: &'a Vec<CharRange>,
	fonts: &'a Vec<Font>
}

#[derive(Template)]
#[template(path = "tests.rs.j2", escape = "none")]
struct RustTests<'a, 'b> {
	char_ranges: &'a Vec<CharRange>,
	fonts: &'a Vec<VirtualFont<'b>>
}

struct CharRange {
	start: char,
	mid: char,
	end: char,
	skip: u32
}

struct Font {
	width: usize,
	height: usize,
	bold: bool,
	bitmap: Vec<u8>,
	bitmap_len: usize,
	bitmap_len_str: String,
	img_width: usize,
	img_height: usize,
	bmp: String,
	bmp_double: String
}

impl PartialEq for Font {
	fn eq(&self, other: &Self) -> bool {
		self.width == other.width && self.height == other.height && self.bold == other.bold
	}
}

impl Eq for Font {}

impl PartialOrd for Font {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Font {
	fn cmp(&self, other: &Self) -> Ordering {
		(self.width, self.height, self.bold).cmp(&(other.width, other.height, other.bold))
	}
}

struct VirtualFont<'a> {
	width: usize,
	height: usize,
	bold: bool,
	glyphs: LinkedHashMap<char, GlyphData<'a>>,
	pixels: usize
}

// weird trait because askama doesn't want to learn proper borrowing/copying
trait Char {
	fn char(self) -> char;
}

impl Char for char {
	fn char(self) -> char {
		self
	}
}

impl Char for &char {
	fn char(self) -> char {
		*self
	}
}

impl<'a> VirtualFont<'a> {
	fn glyph<C>(&self, c: C) -> GlyphData<'a>
	where
		C: Char
	{
		let c = c.char();
		let mut glyph = self
			.glyphs
			.get(&c)
			.unwrap_or_else(|| panic!("No glyph found for char '{}'", c))
			.clone();
		glyph.pixels = self.pixels;
		glyph
	}
}

impl PartialEq for VirtualFont<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.width == other.width && self.height == other.height && self.bold == other.bold
	}
}

impl Eq for VirtualFont<'_> {}

impl PartialOrd for VirtualFont<'_> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for VirtualFont<'_> {
	fn cmp(&self, other: &Self) -> Ordering {
		(self.width, self.height, self.bold).cmp(&(other.width, other.height, other.bold))
	}
}

#[derive(Clone)]
struct GlyphData<'a> {
	bitmap: bdf_reader::Bitmap<'a>,
	width: usize,
	height: usize,
	pixels: usize
}

impl<'a> GlyphData<'a> {
	fn new(glyph: &'a Glyph, width: usize, height: usize) -> anyhow::Result<Self> {
		let bitmap = glyph.bitmap();

		if bitmap.width() != width || bitmap.height() != height {
			let msg = format!(
				"Font is not a monospace font (width={}, height={}, bitmap={}x{})",
				width,
				height,
				bitmap.width(),
				bitmap.height()
			);
			if bitmap.width() >= width && bitmap.height() >= height {
				println!("[WARN] {} - clipping", msg);
			} else {
				bail!("{}", msg);
			}
		}

		Ok(Self {
			bitmap,
			width,
			height,
			pixels: 1
		})
	}

	fn push_to_bitmap(&self, lines: &mut [BitVec]) {
		for y in 0..self.height {
			for x in 0..self.width {
				lines[y].push(self.bitmap.get(x, y).unwrap());
			}
		}
	}

	fn push_to_bitmap_double(&self, lines_double: &mut [BitVec]) {
		for y in 0..self.height {
			for x in 0..self.width {
				for _ in 0..2 {
					lines_double[y].push(self.bitmap.get(x, y).unwrap());
				}
			}
		}
	}

	// y is a reference because askama is stupid
	fn mock_line(&self, y: &usize) -> String {
		let y = y / self.pixels;
		let mut line = String::new();
		for x in 0..self.width {
			for _ in 0..self.pixels {
				line.push(match self.bitmap.get(x, y).unwrap() {
					true => '#',
					false => ' '
				});
			}
		}
		line
	}

	// y is a reference because askama is stupid
	fn mock_line_inverted(&self, y: &usize) -> String {
		let y = y / self.pixels;
		let mut line = String::new();
		for x in 0..self.width {
			for _ in 0..self.pixels {
				line.push(match self.bitmap.get(x, y).unwrap() {
					true => '#',
					false => '.'
				});
			}
		}
		line
	}
}

#[derive(Clone, Copy)]
struct FontMetadata {
	width: usize,
	height: usize,
	bold: bool,
	per_line: usize
}

fn main() -> anyhow::Result<()> {
	let mut fonts_with_metadata = Vec::new();
	let mut fonts = Vec::new();
	let mut virtual_fonts = Vec::new();

	let char_count: usize = CHAR_RANGES
		.iter()
		.map(|(start, end)| *end as usize - *start as usize + 1)
		.sum();

	let mut char_ranges = Vec::new();
	let mut skip = 0;
	for (start, end) in CHAR_RANGES {
		char_ranges.push(CharRange {
			start: *start,
			mid: ((*start as u32 + *end as u32).ceiling_div(2)).try_into().unwrap(),
			end: *end,
			skip
		});
		skip += *end as u32 - *start as u32 + 1;
	}

	let dir = "tamzen-font/bdf";
	for file in fs::read_dir(dir)? {
		let file = file?;
		let path = file.path().display().to_string();
		println!("Inspecting file {}", path);
		let path = match path
			.strip_prefix("tamzen-font/bdf/TamzenForPowerline")
			.and_then(|path| path.strip_suffix(".bdf"))
		{
			Some(path) => path,
			None => continue
		};
		let bold = if path.ends_with('b') {
			true
		} else if path.ends_with('r') {
			false
		} else {
			continue;
		};
		let path = &path[..path.len() - 1];

		let mut size = path.split('x');
		let (width, height) = match (
			size.next().and_then(|s| s.parse::<usize>().ok()),
			size.next().and_then(|s| s.parse::<usize>().ok())
		) {
			(Some(width), Some(height)) => (width, height),
			_ => continue
		};
		println!(
			" -> Found font {}x{} ({})",
			width,
			height,
			if bold { "bold" } else { "regular" }
		);

		let mut rows = 1;
		let mut per_line = char_count;
		let mut min_raw_size = usize::MAX;
		for r in MIN_ROWS..=MAX_ROWS {
			// characters that can be displayed per line
			let pl = char_count.ceiling_div(r);
			// the minimum width of the image
			let min_width = pl * width;
			// the width the image will have
			let img_width = min_width.ceiling_div(32) * 32;
			// the height the image will have
			let img_height = r * height;
			// the space the raw image data will take up
			let s = (img_width * img_height).ceiling_div(8);

			if s < min_raw_size || (s == min_raw_size && r > rows) {
				min_raw_size = s;
				rows = r;
				per_line = img_width / width;
			}
		}

		let file = BufReader::new(File::open(file.path())?);
		let font = bdf_reader::Font::read(file)?;

		let metadata = FontMetadata {
			width,
			height,
			bold,
			per_line
		};
		fonts_with_metadata.push((font, metadata));
	}

	for i in 0..fonts_with_metadata.len() {
		let (font, metadata) = &fonts_with_metadata[i];
		let FontMetadata {
			width,
			height,
			bold,
			per_line
		} = *metadata;

		let mut bitmap = Bitmap::new((per_line * width) as _);
		let (mut lines, mut lines_double) = bitmap.init_lines(height);

		let glyphs: LinkedHashMap<char, GlyphData<'_>> = CHAR_RANGES
			.iter()
			.flat_map(|(start, end)| {
				(*start..=*end).map(|ch| {
					(
						ch,
						GlyphData::new(font.glyph(ch).expect("char not in font"), width, height).unwrap()
					)
				})
			})
			.collect();

		let mut idx = 0;
		for (_, g) in &glyphs {
			g.push_to_bitmap(&mut lines);
			g.push_to_bitmap_double(&mut lines_double);

			idx += 1;
			if idx >= per_line {
				idx = 0;
				bitmap.add_lines(lines, lines_double);

				// TODO this syntax is ugly but everything else is unstable
				let (lines0, lines0_double) = bitmap.init_lines(height);
				lines = lines0;
				lines_double = lines0_double;
			}
		}
		if idx > 0 {
			bitmap.add_lines(lines, lines_double);
		}

		let bmp = bitmap.bmp();
		let bmp_double = bitmap.bmp_double();
		let raw_width = bitmap.width();
		let raw_height = bitmap.height();
		let raw = bitmap.into_raw();

		let bitmap_len = raw.len();
		let bitmap_len_str = bitmap_len.to_formatted_string(&Locale::en_IE); // european english
		let font = Font {
			width,
			height,
			bold,
			bitmap: raw,
			bitmap_len,
			bitmap_len_str,
			img_width: raw_width,
			img_height: raw_height,
			bmp: base64::encode(bmp),
			bmp_double: base64::encode(bmp_double)
		};
		fonts.push(font);

		let font = VirtualFont {
			width,
			height,
			bold,
			glyphs: glyphs.clone(),
			pixels: 1
		};
		virtual_fonts.push(font);

		let font = VirtualFont {
			width: 2 * width,
			height: 2 * height,
			bold,
			glyphs,
			pixels: 2
		};
		virtual_fonts.push(font);
	}

	fonts.sort_unstable();
	virtual_fonts.sort_unstable();

	let mut rs = File::create("../src/tamzen.rs")?;
	writeln!(
		rs,
		"{}",
		RustSource {
			char_ranges: &char_ranges,
			fonts: &fonts
		}
		.render()?
	)?;

	let mut rs = File::create("../tests/tamzen.rs")?;
	writeln!(
		rs,
		"{}",
		RustTests {
			char_ranges: &char_ranges,
			fonts: &virtual_fonts
		}
		.render()?
	)?;

	Ok(())
}
