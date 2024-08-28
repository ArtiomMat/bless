use bless::*;

fn main() {
    bless::set_raw_input(true);

    let mut scr = Screen::new();

    scr.write(|scr: &mut Screen, _: &()| {
        let mut x = Glyph::from('$');
        x.fl |= GLYPH_BLINK;
        x.bg = Color::DRed;
        
        scr.set(&x, 2,2);
    }, &()).unwrap();

    scr.flush().unwrap();

    let _ = bless::read_u8().unwrap();

    bless::show_cursor(true);
}

