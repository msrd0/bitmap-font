use anyhow::{anyhow, bail};
use askama::Template;
use bdf::Glyph;
use bit_vec::BitVec;
use linked_hash_map::LinkedHashMap;
use num_format::{Locale, ToFormattedString};
use std::{
	cmp::Ordering,
	collections::BTreeSet,
	convert::TryInto,
	fs::{self, File},
	io::Write
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
const MIN_ROWS: u32 = 4;
const MAX_ROWS: u32 = 20;

mod filters {
	pub fn chunks<'a>(src: &'a [u8], size: &usize) -> askama::Result<Vec<&'a [u8]>> {
		Ok(src.chunks(*size).collect())
	}
}

#[derive(Template)]
#[template(path = "src.in", escape = "none")]
struct RustSource<'a> {
	char_ranges: &'a Vec<CharRange>,
	fonts: &'a BTreeSet<Font>
}

#[derive(Template)]
#[template(path = "tests.in", escape = "none")]
struct RustTests<'a> {
	char_ranges: &'a Vec<CharRange>,
	fonts: &'a BTreeSet<VirtualFont>
}

struct CharRange {
	start: char,
	mid: char,
	end: char,
	skip: u32
}

struct Font {
	width: u32,
	height: u32,
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

struct VirtualFont {
	width: u32,
	height: u32,
	bold: bool,
	glyphs: LinkedHashMap<char, GlyphData>,
	pixels: u32
}

impl VirtualFont {
	fn glyph(&self, c: &char) -> GlyphData {
		let mut glyph = self.glyphs.get(c).expect(&format!("No glyph found for char '{}'", c)).clone();
		glyph.pixels = self.pixels;
		glyph
	}
}

impl PartialEq for VirtualFont {
	fn eq(&self, other: &Self) -> bool {
		self.width == other.width && self.height == other.height && self.bold == other.bold
	}
}

impl Eq for VirtualFont {}

impl PartialOrd for VirtualFont {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for VirtualFont {
	fn cmp(&self, other: &Self) -> Ordering {
		(self.width, self.height, self.bold).cmp(&(other.width, other.height, other.bold))
	}
}

#[derive(Clone)]
struct GlyphData {
	bitmap: bdf::Bitmap,
	width: u32,
	height: u32,
	pixels: u32
}

impl GlyphData {
	fn new(glyph: &Glyph, width: u32, height: u32) -> anyhow::Result<Self> {
		if glyph.width() != width as u32 || glyph.height() != height as u32 {
			let msg = format!(
				"Font is not a monospace font (width={}, height={}, glyph={}x{})",
				width,
				height,
				glyph.width(),
				glyph.height()
			);
			if glyph.width() >= width as u32 && glyph.height() >= height as u32 {
				println!("[WARN] {} - clipping", msg);
			} else {
				bail!("{}", msg);
			}
		}

		Ok(Self {
			bitmap: glyph.map().clone(),
			width,
			height,
			pixels: 1
		})
	}

	fn push_to_bitmap(&self, lines: &mut Vec<BitVec>) {
		for y in 0..self.height {
			for x in 0..self.width {
				lines[y as usize].push(self.bitmap.get(x, y));
			}
		}
	}

	fn push_to_bitmap_double(&self, lines_double: &mut Vec<BitVec>) {
		for y in 0..self.height {
			for x in 0..self.width {
				for _ in 0..2 {
					lines_double[y as usize].push(self.bitmap.get(x, y));
				}
			}
		}
	}

	// y is a reference because askama is stupid
	fn mock_line(&self, y: &u32) -> String {
		let y = y / self.pixels;
		let mut line = String::new();
		for x in 0..self.width {
			for _ in 0..self.pixels {
				line.push(self.bitmap.get(x, y).then(|| '#').unwrap_or(' '));
			}
		}
		line
	}
}

fn main() -> anyhow::Result<()> {
	let mut fonts = BTreeSet::new();
	let mut virtual_fonts = BTreeSet::new();

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
		let bold = if path.ends_with("b") {
			true
		} else if path.ends_with("r") {
			false
		} else {
			continue;
		};
		let path = &path[..path.len() - 1];

		let mut size = path.split('x');
		let (width, height) = match (
			size.next().and_then(|s| s.parse::<u32>().ok()),
			size.next().and_then(|s| s.parse::<u32>().ok())
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
		let mut per_line = char_count as u32;
		let mut min_raw_size = u32::MAX;
		for r in MIN_ROWS..=MAX_ROWS {
			// characters that can be displayed per line
			let pl = (char_count as u32).ceiling_div(r);
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

		let mut bitmap = Bitmap::new((per_line * width) as _);
		let (mut lines, mut lines_double) = bitmap.init_lines(height);
		let file = File::open(file.path())?;
		let font = bdf::read(file)?;
		let glyphs = font.glyphs();
		let glyphs: LinkedHashMap<char, GlyphData> = CHAR_RANGES
			.iter()
			.flat_map(|(start, end)| {
				(*start..=*end).map(|char| {
					let glyph = glyphs.get(&char).ok_or_else(|| anyhow!("char not in font")).unwrap();
					(char, GlyphData::new(glyph, width, height).unwrap())
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
			bmp: base64::encode(&bmp),
			bmp_double: base64::encode(&bmp_double)
		};
		fonts.insert(font);

		let font = VirtualFont {
			width,
			height,
			bold,
			glyphs: glyphs.clone(),
			pixels: 1
		};
		virtual_fonts.insert(font);

		let font = VirtualFont {
			width: 2 * width,
			height: 2 * height,
			bold,
			glyphs,
			pixels: 2
		};
		virtual_fonts.insert(font);
	}

	let mut rs = File::create("../src/generated.rs")?;
	writeln!(
		rs,
		"{}",
		RustSource {
			char_ranges: &char_ranges,
			fonts: &fonts
		}
		.render()?
	)?;

	let mut rs = File::create("../tests/generated.rs")?;
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
