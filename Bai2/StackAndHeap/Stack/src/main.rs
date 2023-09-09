fn main() {
    fn _x(){
        let mut _a = "hello".to_string();
        let _b = 100;
        _a.push_str("!");
        _y();
    }

    fn _y(){
        let mut _a = String::from("World");
        _a.push_str("!");
    }
}
