/*
 * -- lib.rs
 * msel
 * by viz (https://github.com/vizs)
 *
 * licensed under BSD 2-Clause "Simplified" License
 */
pub struct Items {
    pub all_items: Vec<String>,
    pub sel_items: Vec<String>
}

impl Items {
    pub fn new(all_item: &Vec<String>) -> Items {
        Items {
                all_items: all_item.to_vec(),
                sel_items: vec![],
              }
    }

    /*
     * add or delete item from sel_items vector
     * if item is already in the vector, it is
     * removed
     */
    #[allow(unused_assignments)]
    pub fn add_rm_sel(&mut self, new_item: &String) {
        if self.sel_items.contains(new_item) {
            let item_ind = self.sel_items.iter()
                                         .position(|x| x == new_item);
            let mut ind: usize = 0;

            match item_ind.ok_or(0) {
                Ok(n) => { ind = n; },
                _ => { return (); },
            }
            self.sel_items.remove(ind);
        } else {
            self.sel_items.push(new_item.to_string());
        }
    }

    /*
     * return all items
     */
    pub fn get_items(&self) -> Vec<String> {
        self.all_items.to_vec()
    }
}

pub mod ui {
    extern crate termion;

    use std::io::{stdout, Write, stdin};
    use super::Items;
    use termion::{input::TermRead, event::Key,raw::IntoRawMode};
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
        let mut stdout = stdout().into_raw_mode()
                                 .unwrap();
        let stdin  = stdin();
        let aitems = items.get_items();
        let nitms  = aitems.len() - 1;

        /* disable cursor */
        write!(stdout, "\n\r{}", cursor::Hide).unwrap();

        for _ in 0..nitms {
            write!(stdout, "\n").unwrap();
        }

        /* the cursor position variable */
        let mut cur: usize = 0;

        let mut input = stdin.keys();

        /* the main loop */
        loop {
            print!("{}", cursor::Up((nitms + 1) as u16));

            for (n, item) in aitems.iter().enumerate() {
                write!(stdout, "\n\r{}", clear::CurrentLine).unwrap();

                let cur_item = aitems.get(n)
                                     .unwrap();
                /* highlighted item */
                write!(stdout, "\t\t").unwrap();
                if items.sel_items.contains(cur_item) {
                    write!(stdout, "{}", style::Bold).unwrap();
                }

                if cur == n {
                    write!(stdout, ">{}{}", item, style::Reset).unwrap();
                } else {
                    write!(stdout, "{}{}", item, style::Reset).unwrap();
                }
            }

            stdout.lock()
                  .flush()
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
                Key::Esc|Key::Ctrl('c') => {
                    write!(stdout, "\n\r{}", cursor::Show).unwrap();
                    break;
                }
                /* space for selecting */
                Key::Char(' ') => {
                    items.add_rm_sel(aitems.get(cur).unwrap());
                }
                /* first element */
                Key::Char('g') => {
                    cur = 0;
                }
                /* last element */
                Key::Char('G') => {
                    cur = nitms;
                }
                /* something else */
                _ => {}
            }
        }

        write!(stdout, "\n\r{}", cursor::Show).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui() {
        let mut items = Items::new(&vec![String::from("lmao"), String::from("something")]);

        ui::run(&mut items);

        println!("{:?}", items.sel_items);
    }
}
