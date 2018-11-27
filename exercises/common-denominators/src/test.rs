fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(convert_fracts(l), exp)
}

#[test]
fn basics_convert_fracts() {
    testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
}
