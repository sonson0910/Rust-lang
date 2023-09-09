#[derive(Debug)]
enum IpAddressKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct _IpAddress{
    kind: IpAddressKind,
    address: String,
}

impl _IpAddress{
    fn some_function(){
        println!("Blockchain Development");
    }
}

fn main() {

    let localhost = IpAddressKind::V4(127,0,0,1);
    println!("local host = {:#?}", localhost);
}

fn route(ip_kind: IpAddressKind){}
