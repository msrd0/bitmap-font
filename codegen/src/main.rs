use anyhow::{anyhow, bail};
use askama::Template;
use bdf::{Bitmap, Glyph};
use bit_vec::BitVec;
use linked_hash_map::LinkedHashMap;
use num_format::{Locale, ToFormattedString};
use std::{
	cmp::Ordering,
	collections::BTreeSet,
	fs::{self, File},
	io::Write
};

const CHARS: &str = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;

fn to_bmp(bitmap: &[u8], width: i32, height: i32) -> anyhow::Result<Vec<u8>> {
	let mut bmp: Vec<u8> = Vec::new();

	// file header
	bmp.write_all(&[0x42, 0x4D])?;
	bmp.write_all(&(14 + 40 + 8 + bitmap.len() as u32).to_le_bytes())?;
	bmp.write_all(&[0, 0, 0, 0])?;
	bmp.write_all(&(14 + 40 + 8 as u32).to_le_bytes())?;
	// info header
	bmp.write_all(&(40 as u32).to_le_bytes())?;
	bmp.write_all(&width.to_le_bytes())?;
	bmp.write_all(&(-height).to_le_bytes())?;
	bmp.write_all(&(1 as u16).to_le_bytes())?;
	bmp.write_all(&(1 as u16).to_le_bytes())?;
	bmp.write_all(&[0, 0, 0, 0, 0, 0, 0, 0, 0x01, 0, 0, 0, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])?;
	// color table
	bmp.write_all(&[0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0])?;
	// pixel data
	bmp.write_all(&bitmap)?;

	Ok(bmp)
}

#[derive(Template)]
#[template(path = "src.in", escape = "none")]
struct RustSource<'a> {
	fonts: &'a BTreeSet<Font>
}

#[derive(Template)]
#[template(path = "tests.in", escape = "none")]
struct RustTests<'a> {
	fonts: &'a BTreeSet<Font>
}

struct Font {
	width: u32,
	height: u32,
	glyphs: LinkedHashMap<char, GlyphData>,
	bitmap: Vec<u8>,
	bitmap_len: String,
	img_width: usize,
	bmp: String,
	bmp_double: String
}

impl PartialEq for Font {
	fn eq(&self, other: &Self) -> bool {
		self.width == other.width && self.height == other.height
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
		(self.width, self.height).cmp(&(other.width, other.height))
	}
}

struct GlyphData {
	bitmap: Bitmap,
	width: u32,
	height: u32
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
			height
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
		let mut line = String::new();
		for x in 0..self.width {
			line.push(self.bitmap.get(x, *y).then(|| '#').unwrap_or(' '));
		}
		line
	}

	fn mock_line_double(&self, y: &u32) -> String {
		let y = y / 2;
		let mut line = String::new();
		for x in 0..self.width {
			for _ in 0..2 {
				line.push(self.bitmap.get(x, y).then(|| '#').unwrap_or(' '));
			}
		}
		line
	}
}

fn main() -> anyhow::Result<()> {
	let mut fonts = BTreeSet::new();

	let dir = "tamzen-font/bdf";
	for file in fs::read_dir(dir)? {
		let file = file?;
		let path = file.path().display().to_string();
		println!("Inspecting file {}", path);
		let path = match path
			.strip_prefix("tamzen-font/bdf/Tamzen")
			.and_then(|path| path.strip_suffix("r.bdf"))
		{
			Some(path) => path,
			None => continue
		};

		let mut size = path.split('x');
		let (width, height) = match (
			size.next().and_then(|s| s.parse::<u32>().ok()),
			size.next().and_then(|s| s.parse::<u32>().ok())
		) {
			(Some(width), Some(height)) => (width, height),
			_ => continue
		};

		let min_width = CHARS.len() * width as usize;
		let img_width = if min_width % 32 == 0 {
			min_width
		} else {
			(min_width / 32 + 1) * 32
		};

		let min_width_double = min_width * 2;
		let img_width_double = if min_width_double % 32 == 0 {
			min_width_double
		} else {
			(min_width_double / 32 + 1) * 32
		};

		println!(" -> Found font {}x{}", width, height);
		let mut lines: Vec<BitVec> = Vec::new();
		let mut lines_double: Vec<BitVec> = Vec::new();
		for _ in 0..height {
			lines.push(BitVec::with_capacity(8 * img_width));
			lines_double.push(BitVec::with_capacity(8 * img_width_double));
		}
		let file = File::open(file.path())?;
		let font = bdf::read(file)?;
		let glyphs = font.glyphs();
		let glyphs: LinkedHashMap<char, GlyphData> = CHARS
			.chars()
			.map(|char| {
				let glyph = glyphs.get(&char).ok_or_else(|| anyhow!("char not in font")).unwrap();
				(char, GlyphData::new(glyph.clone(), width, height).unwrap())
			})
			.collect();
		for (_, g) in &glyphs {
			g.push_to_bitmap(&mut lines);
			g.push_to_bitmap_double(&mut lines_double);
		}

		let mut bitmap: Vec<u8> = Vec::new();
		for mut line in lines {
			for _ in min_width..img_width {
				line.push(false);
			}
			bitmap.extend(line.to_bytes());
		}

		let mut bitmap_double: Vec<u8> = Vec::new();
		for mut line in lines_double {
			for _ in min_width_double..img_width_double {
				line.push(false);
			}
			let bytes = line.to_bytes();
			bitmap_double.extend_from_slice(&bytes);
			bitmap_double.extend_from_slice(&bytes);
		}

		let bmp: Vec<u8> = to_bmp(&bitmap, img_width as _, height as _)?;
		let bmp_double: Vec<u8> = to_bmp(&&bitmap_double, img_width_double as _, 2 * height as i32)?;

		let bitmap_len = bitmap.len().to_formatted_string(&Locale::en_IE); // european english
		let font = Font {
			width,
			height,
			glyphs,
			bitmap,
			bitmap_len,
			img_width,
			bmp: base64::encode(&bmp),
			bmp_double: base64::encode(&bmp_double)
		};
		fonts.insert(font);
	}

	let mut rs = File::create("../src/generated.rs")?;
	writeln!(rs, "{}", RustSource { fonts: &fonts }.render()?)?;

	let mut rs = File::create("../tests/generated.rs")?;
	writeln!(rs, "{}", RustTests { fonts: &fonts }.render()?)?;

	Ok(())
}
