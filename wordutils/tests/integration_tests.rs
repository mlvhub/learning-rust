extern crate wordutils;

use wordutils::initials;

#[test]
fn do_initials() {
    assert_eq!(initials("j. alfred prufrock"), "JAP");
}

