mod view;
mod model;

use std::{thread, time};
use view::display;
use model::{step};

fn main() {
	let delay: time::Duration = time::Duration::from_secs(1);
	let mut board = vec![
		vec![false, true, false, false, false, false, false],
		vec![false, false, true, false, false, false, false],
		vec![true, true, true, false, false, false, false],
		vec![false, false, false, false, false, false, false],
		vec![false, false, false, false, false, false, false],
		vec![false, false, false, false, false, false, false],
		vec![false, false, false, false, false, false, false],
	];
	loop {
		display(&board);
		thread::sleep(delay);
		board = step(&board);
	}
}
