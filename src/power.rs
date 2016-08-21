pub struct Power(pub [i8; 3]);

use std::cmp::min;

impl Power {

	fn spend(&mut self, x: i8) {
		self.transfer(x, 2, 0);
	}

	fn gain(&mut self, x: i8) {
		let zero_to_one = min(x, self[0]);
		self.transfer(zero_to_one, 0, 1);

		let one_to_two = min(x - zero_to_one, self[1]);
		self.transfer(one_to_two, 1, 2);
	}

	fn burn(&mut self, x: i8) {
		self[1] -= x;
		self.transfer(x, 1, 2)
	}

	fn transfer(&mut self, x: i8, a: usize, b: usize) {
		self[a] -= x;
		self[b] += x;
	}
}

use std::ops::{Index, IndexMut};

impl Index<usize> for Power {
	type Output = i8;

	fn index(&self, i: usize) -> &i8 {
		&self.0[i]
	}
}

impl IndexMut<usize> for Power {
	fn index_mut(&mut self, i: usize) -> &mut i8 {
		&mut self.0[i]
	}
}