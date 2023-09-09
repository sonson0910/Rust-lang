
// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }

// fn try_login() -> Result<i32, String>{
//     Ok(1),
//     Err(String::from("Account denied")),
// }

// authentication
// authorization

struct Employee{
    position: Position,
    status: Status,

}

enum Position{
    CEO,
    CTO,
    IT,
    Manager, 
    Marketer,
}

enum Status{
    Active,
    Denied,
}

fn try_access(employee: &Employee) -> Result<(), String>{
    match employee.status{
        Status::Denied => return Err("Access denied".to_owned()),
        _ => (),
    }
    match employee.position{
        Position::CEO => Ok(()),
        Position::CTO => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("invalid position".to_string()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String>{
    let access = try_access(employee)?;
    println!("access");
    Ok(())
}

fn main() {
    // panic!("Hello, world!");    Bao loi
    let manager = Employee { 
        position: Position::Manager, 
        status: Status::Active,
    };
    let IT = Employee { 
        position: Position::IT, 
        status: Status::Denied,
    };
    // let access = try_access(&manager);
    // let access = try_access(&manager);

    print_access(&manager);
    print_access(&IT);
}
