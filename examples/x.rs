use bless::*;

fn main() {
    bless::set_raw_input(true);

    let mut scr = Screen::new();

    scr.write(|scr: &mut Screen, _: &()| {
        let mut x = Glyph::from('$');
        x.fl |= GLYPH_ITALIC;
        x.bg = Color::DRed;
        x.fg = Color::RGB(128, 255, 0);
        
        scr.set(&x, 2,2);
        
        scr.set(&x, 3,2);
    }, &()).unwrap();

    scr.flush().unwrap();

    let _ = bless::read_u8().unwrap();

    bless::show_cursor(true);
}

