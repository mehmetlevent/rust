// #[derive(Debug)]
/* struct Rectangle {
    width: u32,
    height: u32,

}
impl Rectangle{
   fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    } 
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value <1  {  // if value < 1 || value > 100 
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        else if value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        
        

        Guess { value }
    }
}
*/






/* pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn greeting(name: &str) -> String{
    format!("Hello {}!", name)
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

/*
    #[test]
    #[should_panic (expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100() {
        //Guess::new(200);
        Guess::new(-2);
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting ("Carl");
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was {}", result);
    }
    #[test]
    fn larger_can_hold_smaller(){
        let larger  = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller  = Rectangle {
            width: 5,
            height: 1,
        };
       // assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&smaller));
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn failing_test(){
        panic!("This test is failing");
    } */
}