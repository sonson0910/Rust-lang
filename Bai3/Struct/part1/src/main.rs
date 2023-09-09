#[derive(Debug)]
struct Member{
    username: String,
    email: String,
    age: u64,
    active: bool,
}

fn main() {
    let mut member1 = Member{
        email: String::from("sonlearn155@gmail.com"),
        username: String::from("Son"),
        age: 20,
        active:true,
    };

    // let name = member1.username;
    // println!("name = {}", name);
    // member1.username = String::from("Nguyen Hong Son");
    // println!("member1: {}", member1.username);

    let member2 = create_new_member(
        String::from("Son"), String::from("sonlearn155@gmail.com"), 
        21,
    );

    println!("member2 = {:#?}", member2);

    let member3 = Member{
        username: String::from("josh"),
        ..member2 // Lay toan bo thong tin member 2
    };

    println!("member3 = {:#?}", member3);
}

fn create_new_member (username: String, email: String, age: u64) -> Member{
    Member { 
        username: username, 
        email: email, 
        age: age, 
        active: true, 
    }
}