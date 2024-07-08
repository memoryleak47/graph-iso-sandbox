use super::Boolvec;

#[test]
fn test1() {
	let mut b = Boolvec::zeros(35);
	b.set(12, true);
	assert_eq!(b.get(11), false);
	assert_eq!(b.get(12), true);
	assert_eq!(b.get(13), false);

	b.set(33, true);
	assert_eq!(b.get(32), false);
	assert_eq!(b.get(33), true);
	assert_eq!(b.get(34), false);

	assert_eq!(b.to_string(35), String::from("00000000000010000000000000000000010"));
}
