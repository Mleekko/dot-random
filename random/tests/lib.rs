use random::Random;

#[test]
fn test_next_int32() {
    let random = Random::new(10);
    assert_eq!(11, random.next_int32());
}