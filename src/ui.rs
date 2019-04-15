extern crate termion;

use std::io::{Write, stdin};
use crate::Items;
use termion::{input::TermRead, event::Key, raw::IntoRawMode};
use termion::{style, cursor, clear};

/*
 * the main ui run function
 * adapted from
 * https://github.com/Munksgaard/inquirer-rs/blob/master/src/list.rs
 *
 * takes in the items struct and adds elements to
 * the sel_items vector
 */
pub fn run(items: &mut Items) {
    let mut screen = termion::get_tty().unwrap()
                                       .into_raw_mode()
                                       .unwrap();
    let stdin  = stdin();
    let aitems = items.get_items();
    let nitms  = aitems.len() - 1;

    write!(screen, "\n\r{}", cursor::Hide).unwrap();

    for _ in 0..nitms {
        write!(screen, "\n").unwrap();
    }

    /* the cursor position variable */
    let mut cur: usize = 0;

    let mut input = stdin.keys();

    /* the main loop */
    loop {
        /* delete the previous text */
        write!(screen, "\r\x1b[K").unwrap();
        /* move up back to the start */
        write!(screen, "{}", cursor::Up((nitms + 1) as u16)).unwrap();

        for (n, item) in aitems.iter().enumerate() {
            write!(screen, "\n\r{}", clear::CurrentLine).unwrap();

            let cur_item = aitems.get(n)
                                 .unwrap();

            write!(screen, "\t\t").unwrap();

            if items.sel_items.contains(cur_item) {
                write!(screen, "{}", style::Invert).unwrap();
            }
            if cur == n {
                /* highlighted item */
                write!(screen, ">").unwrap();
            }
                write!(screen, "{}{}", item, style::Reset).unwrap();
        }

        screen.flush()
              .unwrap();

        /*  get input */
        let inp = input.next()
                       .unwrap();

        /* see what the user has input-ed */
        match inp.unwrap() {
            /* down */
            Key::Char('j')|Key::Ctrl('n') => {
                /* if in bottom item */
                if cur == nitms { cur = 0; }
                else { cur += 1; }
            }
            /* up */
            Key::Char('k')|Key::Ctrl('p') => {
                /* if in top item */
                if cur == 0 { cur = nitms; }
                else { cur -= 1; }
            }
            /* exit loop */
            Key::Esc|Key::Ctrl('c')|Key::Ctrl('q') => {
                break;
            }
            /* space for selecting */
            Key::Char(' ') => {
                items.add_rm_sel(aitems.get(cur).unwrap());
            }
            /* first element */
            Key::Char('g')|Key::Alt('<') => {
                cur = 0;
            }
            /* last element */
            Key::Char('G')|Key::Alt('>') => {
                cur = nitms;
            }
            /* something else */
            _ => {}
        }
    }
    write!(screen, "\n\r{}", cursor::Show).unwrap();
}
