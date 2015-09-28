fn main() {
    let mut vec = vec![];

	for x in 100..1000 {
		for y in 100..1000 {
			let z = (x * y).to_string();
            if z == z && !vec.contains(z){
                vec.push(z);
            }
		}
	}

    for i in vec {
        println!("{:?}", i);
    }
}