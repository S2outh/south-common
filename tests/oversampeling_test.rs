use south_common::utils::Oversampeling;

#[test]
pub fn test_smt() {
    let mut ov = Oversampeling::new(11, 0i64);

    for i in (0..20).step_by(2) {
        println!("{}", i);
        if let Some(_) = ov.insert(i as i16) {
            panic!("too early");
        }
    }

    if let Some(v) = ov.insert(20 as i16) {
        assert_eq!(v, 10)
    } else {
        panic!("not enough values")
    }
}
