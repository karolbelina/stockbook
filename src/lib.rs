//! Stockbook embeds 1-bit raster images in your code at compile time.
//!
//! Designed primarily for `#![no_std]` usage, in embedded or other
//! program-memory-constrained environments. Compatible with
//! [`avr-progmem`](https://crates.io/crates/avr_progmem).
//!
//! The main functionality of Stockbook is the [`stamp!`] macro, which lets you
//! include data similarly to how [`include_bytes!`] does, but from an image,
//! specifically a 1-bit black and white image. The macro returns a [`Stamp`]
//! type, which just holds the image's width, height, and a static reference to the
//! pixel data. The pixel data is represented internally as an array of bytes, in
//! which individual bits correspond to individual pixels.
//!
//! ## Example
//!
//! File `assets/star.png` (scaled x8 for preview, originally 12x12 px):
//!
//! ![Star](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAABgCAYAAADimHc4AAAAAXNSR0IArs4c6QAAAT5JREFUeJzt3FEKwyAUAEEtvf+V0yP4YWUSsnMCy/Igxtg5xrjGjV3X3vLmnH9ayRkfvYC3KwBWAKwAWAGwAmAFwAqAFQArAFYArABYAbACYAXA5sDnAbvv+3fp84ImACsAVgCsAFgBsAJgBcAKgBUAKwBWAKwAWAGwAmAFwOalX8i/XBOAFQArAFYArABYAbACYMvvgtom7Fl9d9QEYAXACoAVACsAVgCsANj2/YC37xN27xc0AVgBsAJgBcAKgBUAKwB2/J7w0/cJp+8RNwFYAbACYAXACoAVACsA1nnAps4DHq4AWAGwAmAFwAqAFQD76gWs7D5n332f0gRgBcAKgBUAKwBWAKwAGL8nrP+/X/++JgArAFYArABYAbACYAXAjp8H6Of8ldX6Tu8TmgCsAFgBsAJgBcAKgBUA+wFZhym1RhU7SwAAAABJRU5ErkJggg==)
//!
//! File `src/lib.rs`:
//!
//! ```rust
//! use stockbook::{stamp, Color, Stamp};
//!
//! # const STAR_DATA: [u8; 18] = [
//! #     0b00000110, 0b00000000, 0b01100000, 0b00001111, 0b00000000, 0b11110000,
//! #     0b11111111, 0b11110111, 0b11111110, 0b00111111, 0b11000001, 0b11111000,
//! #     0b00111111, 0b11000011, 0b10011100, 0b01110000, 0b11100110, 0b00000110,
//! # ];
//! #
//! # const EXPECTED_PIXELS: &[(usize, usize)] = &[
//! #     (5, 0), (6, 0), (5, 1), (6, 1), (4, 2), (5, 2), (6, 2), (7, 2),
//! #     (4, 3), (5, 3), (6, 3), (7, 3), (0, 4), (1, 4), (2, 4), (3, 4),
//! #     (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), (9, 4), (10, 4), (11, 4),
//! #     (1, 5), (2, 5), (3, 5), (4, 5), (5, 5), (6, 5), (7, 5), (8, 5),
//! #     (9, 5), (10, 5), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6), (7, 6),
//! #     (8, 6), (9, 6), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7), (8, 7),
//! #     (2, 8), (3, 8), (4, 8), (5, 8), (6, 8), (7, 8), (8, 8), (9, 8),
//! #     (2, 9), (3, 9), (4, 9), (7, 9), (8, 9), (9, 9), (1, 10), (2, 10),
//! #     (3, 10), (8, 10), (9, 10), (10, 10), (1, 11), (2, 11), (9, 11),
//! #     (10, 11),
//! # ];
//! #
//! # static mut ACTUAL_PIXELS: Vec<(usize, usize)> = Vec::new();
//! #
//! # macro_rules! stamp {
//! #     ($path:literal) => { unsafe { Stamp::from_raw(12, 12, STAR_DATA.as_ptr()) } };
//! # }
//! static STAR_SPRITE: Stamp = stamp!("assets/star.png");
//!
//! pub fn draw_star() {
//!     for (x, y, color) in STAR_SPRITE.pixels() {
//!         match color {
//!             Color::Black => {}, // Treat as transparent
//!             Color::White => draw_pixel_at(x, y),
//!         }
//!     }
//! }
//!
//! fn draw_pixel_at(x: usize, y: usize) {
//!     /* ... */
//!     # unsafe { ACTUAL_PIXELS.push((x, y)); }
//! }
//! # draw_star();
//! # assert_eq!(unsafe { ACTUAL_PIXELS.as_slice() }, EXPECTED_PIXELS);
//! ```
//!
//! ## Supported formats
//!
//! Stockbook uses the [image](https://docs.rs/image) crate under the hood. See its
//! own [list of supported formats](https://docs.rs/image/latest/image/codecs/index.html#supported-formats)
//! for more details.
//!
//! ## Feature flags
//!
//! - **`progmem`** &mdash; wraps all pixel data of `Stamp`s in
//!   [`avr_progmem::wrapper::ProgMem`](https://docs.rs/avr-progmem/latest/avr_progmem/wrapper/struct.ProgMem.html)s.
//!   Combined with the `avr` target architecture, this allows you to keep most of
//!   the data in program memory without the need to copy it to RAM. A no-op for
//!   non-`avr` target architectures.
//!
//! ## Unstable features
//!
//! Although this library works on `stable`, any changes to images referenced by the
//! [`stamp!`] macro might not be detected because of caching. Therefore, until
//! [`track_path` API](https://doc.rust-lang.org/stable/proc_macro/tracked_path/fn.path.html)
//! ([Tracking Issue](https://github.com/rust-lang/rust/issues/99515)) stabilizes,
//! it is recommended to use the `nightly` toolchain, however functionality behind
//! this feature is unstable and may change or stop compiling at any time.

#![no_std]
#![warn(missing_docs)]

mod data;
mod iter;

use data::*;
use iter::*;

pub use stockbook_stamp_macro::stamp;

/// Rectangular, 1-bit, raster image.
///
/// A stamp is defined by its width, height, and the color of its pixels, of which
/// there are two: [`Black`](Color::Black) and [`White`](Color::White). Coordinate
/// _(0, 0)_ is the top-left corner of the stamp.
///
/// Stamp's pixel colors are represented internally as an array of bytes, in which
/// individual bits correspond to individual pixels.
#[derive(Debug, Clone)]
pub struct Stamp {
    width: usize,
    height: usize,
    data: Data,
}

impl Stamp {
    /// Size of the stamp in pixels &mdash; width and height, or columns and rows.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { unsafe { Stamp::from_raw(3, 2, [0b000_000_00].as_ptr()) } };
    /// # }
    /// static IMAGE: Stamp = stamp!("image_3x2.png");
    ///
    /// assert_eq!(IMAGE.size(), [3, 2]);
    /// ```
    #[inline]
    pub fn size(&self) -> [usize; 2] {
        [self.width, self.height]
    }

    /// Width of the stamp in pixels.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { unsafe { Stamp::from_raw(3, 2, [0b000_000_00].as_ptr()) } };
    /// # }
    /// static IMAGE: Stamp = stamp!("image_3x2.png");
    ///
    /// assert_eq!(IMAGE.width(), 3);
    /// ```
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Height of the stamp in pixels.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #    ($path:literal) => { unsafe { Stamp::from_raw(3, 2, [0b000_000_00].as_ptr()) } };
    /// # }
    /// static IMAGE: Stamp = stamp!("image_3x2.png");
    ///
    /// assert_eq!(IMAGE.height(), 2);
    /// ```
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Number of pixels in the stamp.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { unsafe { Stamp::from_raw(3, 2, [0b000_000_00].as_ptr()) } };
    /// # }
    /// static IMAGE: Stamp = stamp!("image_3x2.png");
    ///
    /// assert_eq!(IMAGE.pixel_count(), 6);
    /// ```
    #[inline]
    pub fn pixel_count(&self) -> usize {
        self.width * self.height
    }

    /// Checks if a given coordinate is within the bounds of the image.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Color, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { unsafe { Stamp::from_raw(5, 4, [0b00000000, 0b00000000, 0b0000_0000].as_ptr()) } };
    /// # }
    /// static IMAGE: Stamp = stamp!("image_5x4.png");
    ///
    /// assert!(IMAGE.is_within_bounds(0, 0));
    /// assert!(IMAGE.is_within_bounds(4, 3));
    /// assert!(!IMAGE.is_within_bounds(5, 3));
    /// assert!(!IMAGE.is_within_bounds(4, 4));
    /// ```
    pub fn is_within_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Returns an iterator over all pixels of a [`Stamp`]. The iteration order is
    /// _x_ from 0 to _width_, then _y_ from 0 to _height_. A pixel is a
    /// _(x, y, color)_ tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use stockbook::{stamp, Color, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { unsafe { Stamp::from_raw(3, 3, [0b101_010_10, 0b1_0000000].as_ptr()) } };
    /// # }
    /// static IMAGE: Stamp = stamp!("checkerboard_3x3.png");
    ///
    /// let mut pixels = IMAGE.pixels();
    ///
    /// assert_eq!(pixels.next(), Some((0, 0, Color::White)));
    /// assert_eq!(pixels.next(), Some((1, 0, Color::Black)));
    /// assert_eq!(pixels.next(), Some((2, 0, Color::White)));
    /// assert_eq!(pixels.next(), Some((0, 1, Color::Black)));
    /// # for _ in 0..4 {
    /// #     pixels.next();
    /// # }
    /// /* ... */
    /// assert_eq!(pixels.next(), Some((2, 2, Color::White)));
    /// assert_eq!(pixels.next(), None);
    /// ```
    pub fn pixels(&self) -> Pixels<'_> {
        Pixels::new(self)
    }

    /// Yields the color of the stamp at the provided coordinate. Panicking version of
    /// [`get_color_checked`](Stamp::get_color_checked).
    ///
    /// # Panics
    ///
    /// This method panics if the coordinate is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Color, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { unsafe { Stamp::from_raw(3, 3, [0b101_010_10, 0b1_0000000].as_ptr()) } };
    /// # }
    /// static IMAGE: Stamp = stamp!("checkerboard_3x3.png");
    ///
    /// assert_eq!(IMAGE.get_color(0, 0), Color::White);
    /// assert_eq!(IMAGE.get_color(1, 0), Color::Black);
    /// assert_eq!(IMAGE.get_color(0, 1), Color::Black);
    /// ```
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.get_color_checked(x, y).expect("")
    }

    /// Yields the color of the stamp at the provided coordinate. Returns [`None`] if
    /// the coordinate is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Color, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { unsafe { Stamp::from_raw(3, 3, [0b101_010_10, 0b1_0000000].as_ptr()) } };
    /// # }
    /// static IMAGE: Stamp = stamp!("checkerboard_3x3.png");
    ///
    /// assert_eq!(IMAGE.get_color_checked(0, 0), Some(Color::White));
    /// assert_eq!(IMAGE.get_color_checked(1, 0), Some(Color::Black));
    /// assert_eq!(IMAGE.get_color_checked(3, 0), None);
    /// assert_eq!(IMAGE.get_color_checked(0, 3), None);
    /// ```
    pub fn get_color_checked(&self, x: usize, y: usize) -> Option<Color> {
        if !self.is_within_bounds(x, y) {
            return None;
        }

        // SAFETY: we just checked the coordinates are within the bounds of the stamp
        let color = unsafe { self.get_color_unchecked(x, y) };
        Some(color)
    }

    /// Yields the color of the stamp at the provided coordinate, without doing bounds
    /// checking.
    ///
    /// For a safe alternative see [`get_color`](Stamp::get_color) or
    /// [`get_color_checked`](Stamp::get_color_checked).
    ///
    /// # Safety
    ///
    /// Callers must ensure that the provided coordinate is within the bounds of the stamp.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Color, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { unsafe { Stamp::from_raw(3, 3, [0b101_010_10, 0b1_0000000].as_ptr()) } };
    /// # }
    /// static IMAGE: Stamp = stamp!("checkerboard_3x3.png");
    ///
    /// // SAFETY: provided coordinates are guaranteed to be within the bounds
    /// // of the stamp
    /// assert_eq!(unsafe { IMAGE.get_color_unchecked(0, 0) }, Color::White);
    /// assert_eq!(unsafe { IMAGE.get_color_unchecked(1, 0) }, Color::Black);
    /// assert_eq!(unsafe { IMAGE.get_color_unchecked(0, 1) }, Color::Black);
    /// ```
    pub unsafe fn get_color_unchecked(&self, x: usize, y: usize) -> Color {
        let idx = y * self.width + x;
        let byte = self.data.get_unchecked(idx / 8);
        let mask = 0b10000000 >> (idx % 8);

        if byte & mask != 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    /// Constructs a new stamp.
    ///
    /// You should not need to call this function directly. It is recommended to use the
    /// [`stamp!`] macro instead, which calls this constructor for you, while enforcing
    /// its contract.
    ///
    /// # Safety
    ///
    /// `data` must point to an array of bytes with at least `(width * height) / 8`
    /// elements rounding up to the nearest integer. Bits after the `width * height`-th
    /// one are ignored. Also general Rust pointer dereferencing constraints apply, i.e.
    /// it must not be dangling.
    ///
    /// For example, here the dimensions of the stamp are 3x3, so 9 pixels in total, and
    /// so `data` must contain at least 9 bits (2 bytes rounding up), which it does:
    ///
    /// ```rust
    /// use stockbook::Stamp;
    ///
    /// let stamp = unsafe { Stamp::from_raw(3, 3, [0b11111111, 0b1_0000000].as_ptr()) };
    /// ```
    ///
    /// Here, only 8 bits are provided, so this is undefined behavior:
    ///
    /// ```rust,no_run
    /// # use stockbook::Stamp;
    /// let stamp = unsafe { Stamp::from_raw(3, 3, [0b11111111].as_ptr()) }; // Undefined behavior
    /// ```
    ///
    /// Similarly here, but in a const context:
    ///
    /// ```rust,no_run
    /// # use stockbook::Stamp;
    /// static STAMP: Stamp = unsafe { Stamp::from_raw(3, 3, [0b11111111].as_ptr()) }; // Undefined behavior
    /// ```
    ///
    /// If the `"progmem"` feature is enabled, `data` must point to a valid byte array
    /// that is stored in the program memory domain. The array must be initialized,
    /// readable, and immutable (i.e. it must not be changed). Also the pointer must be
    /// valid for the `'static` lifetime.
    pub const unsafe fn from_raw(width: usize, height: usize, data: *const u8) -> Self {
        Self {
            width,
            height,
            data: Data::from_raw(data),
        }
    }
}

/// Color of a pixel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Black (`#000000ff` or `rgba(0, 0, 0, 255)`)
    Black,
    /// White (`#ffffffff` or `rgba(255, 255, 255, 255)`)
    White,
}
