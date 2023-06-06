#![cfg(test)]

use crate::routes::root::service::#Name#Service;

#[test]
pub fn example() {
    let _service = #Name#Service::new();

    assert_eq!(10, 10);
}
