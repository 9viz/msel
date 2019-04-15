/*
 * -- lib.rs
 * msel
 * by viz (https://github.com/vizs)
 *
 * licensed under BSD 2-Clause "Simplified" License
 */
pub mod ui;

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
