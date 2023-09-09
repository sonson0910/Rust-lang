use std::ops::Index;

enum Vinfast{
    VF6,
    VF7,
    VF8,
    VF9,
}

struct Price{
    price_vf6: i64,
    price_vf7: i64,
    price_vf8: i64,
    price_vf9: i64,
}

impl Index<Vinfast> for Price{
    type Output = i64;

    fn index(&self, brand: Vinfast) -> &Self::Output{
        match brand{
            Vinfast::VF6 => &self.price_vf6,
            Vinfast::VF7 => &self.price_vf7,
            Vinfast::VF8 => &self.price_vf8,
            Vinfast::VF9 => &self.price_vf9,
        }
    }
}

fn main() {
    let price = Price{
        price_vf6: 40000,
        price_vf7: 45000,
        price_vf8: 48000,
        price_vf9: 52000,
    };
    let find = price[Vinfast::VF7];
    println!("price of VF7: {}", find);
}
