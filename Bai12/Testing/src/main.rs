fn caps(input: &str) -> String {
    input.to_uppercase()
}

fn main() {}

#[cfg(test)] // configuration
mod test{
    use crate::*;

    #[test]
    fn check(){
        let result = caps("nguyen hong son");
        let expected = String::from("NGUYEN HONG SON");
        assert_eq!(result, expected, "String should be all uppercase");
    }

    fn check_uppercase(){
        let result = caps("nguyen hong son 2");
        let expected = String::from("NGUYEN HONG SON");
        assert_eq!(result, expected, "String should be all uppercase");
    }
}
