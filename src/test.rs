#[cfg(test)]
mod test {
	use crate::{ButtonLabel, TryFromSliceError};

	#[test]
	fn test_label() {
		let label: ButtonLabel = "(Y)es".try_into().unwrap();
		println!("label: {:?}", label);
		assert_eq!('y', label.control);
		assert_eq!("(Y)es", label.label);

		let label: ButtonLabel = "Can(c)el".try_into().unwrap();
		println!("label: {:?}", label);
		assert_eq!('c', label.control);
		assert_eq!("Can(c)el", label.label);

		let label: ButtonLabel = "No".try_into().unwrap();
		println!("label: {:?}", label);
		assert_eq!('n', label.control);
		assert_eq!("(N)o", label.label);

		let label: ButtonLabel = "N".try_into().unwrap();
		println!("label: {:?}", label);
		assert_eq!('n', label.control);
		assert_eq!("(N)", label.label);

		let label: ButtonLabel = "S(ì)".try_into().unwrap();
		println!("label: {:?}", label);
		assert_eq!('ì', label.control);
		assert_eq!("S(ì)", label.label);

		let label: Result<ButtonLabel, TryFromSliceError> = "".try_into();
		println!("label: {:?}", label);
		label.expect_err("Expected and error");
	}

	#[test]
	fn test_label_from() {
		let label: ButtonLabel = ButtonLabel::from("(Y)es").unwrap();
		println!("label: {:?}", label);
		assert_eq!('y', label.control);
		assert_eq!("(Y)es", label.label);
	}
}
