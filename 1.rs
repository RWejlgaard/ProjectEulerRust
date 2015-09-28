fn main() {
	let mut x = 3;
	let mut y = 5;
	let mut total = 0;

	while x <= 1000 {
		total += x;
		x += 3;
	}

	while y <= 1000 {
		total += y;
		y += 5;
	}

	println!("{:?}", total);
}