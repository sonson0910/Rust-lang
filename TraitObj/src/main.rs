trait Clicky{
    fn click(&self) -> String;
}

struct Keyboard;
impl Clicky for Keyboard{
    fn click(&self) -> String {
        "Keyboard Input".to_owned()
    }
}

struct Mouse;
impl Clicky for Mouse{
    fn click(&self) -> String {
        "Mouse click".to_owned()
    }
}

fn main() {
    // let x = Keyboard;
    // let x: &dyn Clicky = &Keyboard;
    // x.click();
    // let y = Mouse;
    // y.click();

    let x: Box<dyn Clicky> = Box::new(Keyboard);
    let y: Box<dyn Clicky> = Box::new(Mouse);


    let clickers = vec![x, y];
    for i in clickers{
        // println!("Click is: {}", i.click());
        if i.click() == "Mouse click".to_owned(){
            println!("this is mouse")
        }else{
            println!("not mouse")
        }
    }
}
