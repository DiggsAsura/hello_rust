struct Rectangle {
    width: u8,
    height: u8
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
}

fn give_two() -> i32 {
    2
}
    

// avoid to compile this code unless run carto test
#[cfg(test)]
mod dcode_tests {
    #[test]  // needed to test
    #[should_panic] // it SHOULD Panic
    fn test_basic() {
        assert!(1 == 1);  // ok
        panic!("Oh no!"); // panic!
    }

    #[test]
//    #[ignore]   // ignore this test (if its slow etc)
    fn test_equals() {
        assert_eq!(super::give_two(), 1 + 1);
        assert_ne!(super::give_two(), 1 + 2); // not equal
    }
    
    #[test]
//    #[should_panic]
    fn test_structs() {
        let r = super::Rectangle {
            width: 50,
            height: 50
        };

        assert!(r.is_square());
    }
}

