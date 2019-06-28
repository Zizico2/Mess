mod piece;
#[cfg(test)]
mod tests {
	pub trait Fdssf {
	fn ass(&self) -> String;
}

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let mut a = 2;
        let mut _b = vec![&mut a];

        *_b[0] += 1;

        println!("{:?}", a);


    }
}
