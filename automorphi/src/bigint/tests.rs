use super::BigInt;

#[test]
fn equality() {
	assert_eq!(BigInt::new(0), BigInt::new(0));

	let b11 = BigInt::new(11);
	let mut b12 = b11; b12.add(&BigInt::new(1));

	assert_eq!(b12, BigInt::new(12));

	let mut bmax12 = b12; bmax12.add(&BigInt::new(u32::MAX));

	let mut bmax12_2 = BigInt::new(u32::MAX-1);
	bmax12_2.add(&BigInt::new(13));

	assert_eq!(bmax12, bmax12_2);

	let mut bmax13 = BigInt::new(u32::MAX-2);
	bmax13.add(&BigInt::new(2+13));

	assert_ne!(bmax13, bmax12);
}
