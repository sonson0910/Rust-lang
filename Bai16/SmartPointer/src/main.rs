use std::rc::Rc;
#[derive(Debug)]

struct Car {
    number: String
}   

struct Door{
    vehicle: Rc<Car>, 
}

fn main() {
    let car = Rc::new(Car {
        number: "30A - 521.124".to_owned(),
    });

    let left_door = Door {
        vehicle: Rc::clone(&car),
    };
    let right_door = Door {
        vehicle: Rc::clone(&car),
    };
    drop(car);
    println!("left door: {:?}", left_door.vehicle);
    println!("right door: {:?}", right_door.vehicle);
}
