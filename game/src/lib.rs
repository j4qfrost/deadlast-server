#![feature(vec_remove_item)]
mod card;
mod card_container;
mod action;

use card::*;
use card_container::*;

pub fn start_game() {
	unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
