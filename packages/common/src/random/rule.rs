use cosmwasm_std::Timestamp;

/// Get nano part of a timestamp
pub fn nano_part(ts: &Timestamp) -> u32 {
    let nanos = ts.nanos();
    let secs = ts.seconds();
    println!("nanos is: {:?}", nanos);
    println!("second is: {:?}", secs);

    (nanos - secs * 1_000_000_000) as u32
}

/// Choose a random number between 0 and `count` by the timestamp's nanos part
pub fn choose_idx_by_nano(ts: &Timestamp, count: u32) -> u32 {
    nano_part(ts) % count
}

#[test]
fn test_nanos() {
    assert_eq!(0, nano_part(&Timestamp::from_seconds(0)));

    let ts = &Timestamp::from_nanos(1_232_332_234);
    assert_eq!(232_332_234, nano_part(ts));

    let idx = choose_idx_by_nano(ts, 7);
    let expected = 232332234 % 7;
    println!("expected is: {}", expected);
    assert_eq!(expected, idx);

    let idx = choose_idx_by_nano(ts, 6534);
    let expected = 232332234 % 6534;
    println!("expected is: {}", expected);
    assert_eq!(expected, idx);
}
