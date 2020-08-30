### iterate over struct type


```rust
extern crate iter_struct;

use iter_struct::IterStruct;

#[derive(Debug, IterStruct)]
struct Person {
    fist_name: String,
    last_name: String,
    age: u32,
}

#[test]
fn its_work() {
    
    let person = Person {
        fist_name: "Ali".to_string(),
        last_name: "GholiKhani".to_string(),
        age: 23,
    };

    let expect = vec![
        ("fist_name".to_string(), "Ali".to_string()),
        ("last_name".to_string(), "GholiKhani".to_string()),
        ("age".to_string(), "23".to_string()),
    ];

    assert_eq!(expect, person.iter_struct());
}
```