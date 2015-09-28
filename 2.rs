fn main() {
	let mut x = 1;
	let mut y = 2;
	let mut z = 0;
	let mut s = 2;

	while x < 4000000 && y < 4000000 {
		if z == 0{
			x = x + y;
			if (x % 2) == 0{
				s += x;
			}
			z = 1;
		}
		if z == 1{
			y = x + y;
			if (y % 2) == 0{
				s += y;
			}
			z = 0;
		}
	}

	println!("{:?}", s);
}