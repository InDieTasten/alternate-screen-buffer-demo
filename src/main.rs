use std::io::{stdin, stdout, Read, Write};

fn main() {
    pause();
    enter_alternate_screen_buffer_mode();
    set_cursor_pos_top_left();
    println!("We are now in the alternate screen buffer! 🎉");
    pause();
    leave_alternate_screen_buffer_mode();
    println!(
        "We are now back in the normal buffer. You should see your previous terminal output above."
    );
}

fn enter_alternate_screen_buffer_mode() {
    // ANSI CSI - Set Mode command: https://terminalguide.namepad.de/seq/csi_sh__p/
    // \x1b    ESC
    // [       Control Sequence Introducer
    // ?1049   Mode specifier: alternate screen buffer (https://terminalguide.namepad.de/mode/p1049/)
    // h       Set mode (https://terminalguide.namepad.de/seq/csi_sh__p/)
    print!("\x1b[?1049h");
}

fn leave_alternate_screen_buffer_mode() {
    // ANSI CSI - Reset mode command: https://terminalguide.namepad.de/seq/csi_sl__p/
    // \x1b    ESC
    // [       Control Sequence Introducer
    // ?1049   Mode specifier: alternate screen buffer (https://terminalguide.namepad.de/mode/p1049/)
    // l       Reset mode (https://terminalguide.namepad.de/seq/csi_sh__p/)
    println!("\x1b[?1049l");
}

fn set_cursor_pos_top_left() {
    // ANSI CSI - Set Cursor Position: (https://terminalguide.namepad.de/seq/csi_ch/)
    // \x1b    is the ESC sequence character
    // [       Control Sequence Introducer
    // 1       x coordindate (first column)
    // ;       delimiter for coordinate dimensions
    // 1       y coordinate (first row)
    // H       Set Cursor Pos
    print!("\x1b[1;1H");
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
