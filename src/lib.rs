//! Why curses when you have bless :)
//! A library for creating graphical applications within the terminal, with the fastest performance
//! I can squeeze.

mod error;
mod term;

pub use crate::error::Error;
pub use crate::term::*;

pub const GLYPH_BOLD: u8 = 0b1;
/// Gives the light version of the font, if available.
pub const GLYPH_LIGHT: u8 = 0b10;
/// Not supported everywhere.
pub const GLYPH_ITALIC: u8 = 0b100;
/// Underline.
pub const GLYPH_LINE: u8 = 0b1000;
/// Strike-through.
pub const GLYPH_STRIKE: u8 = 0b10000;
/// Blink a library defined time.
pub const GLYPH_BLINK: u8 = 0b100000;

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    Default,

    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,

    DBlack,
    DRed,
    DGreen,
    DYellow,
    DBlue,
    DPurple,
    DCyan,
    DWhite,
    
    /// Full RGB, may not be supported always. You want to check with `can_rgb()`.
    RGB(u8,u8,u8),
}

#[derive(Copy, Clone)]
pub struct Glyph {
    /// can be null-terminator, fg bg and fl will be ignored.
    pub c: char,
    pub fl: u8,
    pub fg: Color,
    pub bg: Color,
}

impl From<char> for Glyph {
    fn from(c: char) -> Self {
        Self {
            c,
            fl: 0,
            fg: Color::Default,
            bg: Color::Default,
        }
    }
}

/// While technically possible, it shouldn't be a thing to have multiple context. Though they wont
/// interfere with each other, there will be duplicate information stored like size of terminal.
pub struct Screen {
    s: String,
    glyphs: Vec<Glyph>,
    size: [usize; 2],
}

impl Screen {
    /// Will automatically hide cursor because the cursor looks like an artifact when flushing
    pub fn new() -> Self {
        show_cursor(false);
        Self {
            s: String::new(),
            glyphs: Vec::new(),
            size: [0, 0],
        }
    }
    
    pub fn size(&self) -> &[usize; 2] {
        return &self.size;
    }

    pub fn set(&mut self, g: &Glyph, x: usize, y: usize) {
        self.glyphs[x + y * self.size[0]] = *g;
    }
    pub fn get(&mut self, x: usize, y: usize) -> &Glyph {
        &self.glyphs[x + y * self.size[0]]
    }

    /// The writer function is free to set individual glyphs
    pub fn write<T>(&mut self, writer: fn(&mut Self, &T), data: &T) -> Result<(), Error> {
        let size = term::get_size()?;

        // Resize the whole thingy mcgib
        if size != self.size {
            self.size = size;
            self.glyphs.resize_with(size[0] * size[1], || Glyph::from('\0'));
        }

        self.s.clear();
        // Put cursor at 0,0
        self.s.push_str("\x1B[H");
        writer(self, data); 

        Ok(())
    }

    /// After calling the final write(), use this to flush the buffer on the terminal.
    pub fn flush(&mut self) -> Result<(), Error> {
        for g in &self.glyphs {
            // An invalid ASCII character
            if g.c == '\0' {
                self.s.push_str("\x1B[0m ");
                continue;
            }
            // Background color
            self.s.push_str("\x1B[0");
            
            // Background color
            match g.bg {
                Color::Black => { self.s.push_str(";100"); },
                Color::Red => { self.s.push_str(";101"); },
                Color::Green => { self.s.push_str(";102"); },
                Color::Yellow => { self.s.push_str(";103"); },
                Color::Blue => { self.s.push_str(";104"); },
                Color::Purple => { self.s.push_str(";105"); },
                Color::Cyan => { self.s.push_str(";106"); },
                Color::White => { self.s.push_str(";107"); },
    
                Color::DBlack => { self.s.push_str(";40"); },
                Color::DRed => { self.s.push_str(";41"); },
                Color::DGreen => { self.s.push_str(";42"); },
                Color::DYellow => { self.s.push_str(";43"); },
                Color::DBlue => { self.s.push_str(";44"); },
                Color::DPurple => { self.s.push_str(";45"); },
                Color::DCyan => { self.s.push_str(";46"); },
                Color::DWhite => { self.s.push_str(";47"); },

                Color::RGB(r,g,b) => {
                    self.s.push_str(
                        format!(";48;2;{};{};{}", r,g,b).as_str()
                    );
                },

                _ => {}
            }
            // Foreground color
            match g.fg {
                Color::Black => { self.s.push_str(";90"); },
                Color::Red => { self.s.push_str(";91"); },
                Color::Green => { self.s.push_str(";92"); },
                Color::Yellow => { self.s.push_str(";93"); },
                Color::Blue => { self.s.push_str(";94"); },
                Color::Purple => { self.s.push_str(";95"); },
                Color::Cyan => { self.s.push_str(";96"); },
                Color::White => { self.s.push_str(";97"); },

                Color::DBlack => { self.s.push_str(";30"); },
                Color::DRed => { self.s.push_str(";31"); },
                Color::DGreen => { self.s.push_str(";32"); },
                Color::DYellow => { self.s.push_str(";33"); },
                Color::DBlue => { self.s.push_str(";34"); },
                Color::DPurple => { self.s.push_str(";35"); },
                Color::DCyan => { self.s.push_str(";36"); },
                Color::DWhite => { self.s.push_str(";37"); },

                Color::RGB(r,g,b) => {
                    self.s.push_str(
                        format!(";38;2;{};{};{}", r,g,b).as_str()
                    );
                },

                _ => {}
            }
            // Flags
            if g.fl & GLYPH_BOLD != 0 {
                self.s.push_str(";1");
            }
            if g.fl & GLYPH_LIGHT != 0 {
                self.s.push_str(";2");
            }
            if g.fl & GLYPH_ITALIC != 0 {
                self.s.push_str(";3");
            }
            if g.fl & GLYPH_LINE != 0 {
                self.s.push_str(";4");
            }
            if g.fl & GLYPH_BLINK != 0 {
                self.s.push_str(";5");
            }
            if g.fl & GLYPH_STRIKE != 0 {
                self.s.push_str(";9");
            }

            self.s.push('m');
            self.s.push(g.c);
        }

        term::write(self.s.as_str())?;
        Ok(())
    }
}

// Inform the terminal if we want the cursor to be shown
pub fn show_cursor(yes: bool) {
    // Ignore because it's not life-threatening if it fails...
    if yes {
        _ = write("\x1B[?25h");
    } else {
        _ = write("\x1B[?25l");
    }
}
