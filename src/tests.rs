use crate::deserialiser::{DeserialisedResult, Deserialiser};

#[test]
fn deser_test_file() {
	let mut deserialiser = Deserialiser::default();
	deserialiser.digest(include_bytes!("../test_files/mini_cube_b.bgcode"));

	loop {
		let r = deserialiser.deserialise().unwrap();
		match r {
			DeserialisedResult::MoreBytesRequired(_) => {
				break;
			}
			_ => (),
		}
	}
}

// #[test]
// fn ser_test_file
