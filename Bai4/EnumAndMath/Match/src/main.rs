fn main() {
    decimal(Coin::Bitcoin(Balance::Shark));
}

#[derive(Debug)]
enum Balance{
    Small,
    Intermediate,
    Fish,
    Shark,
}

#[derive(Debug)]
enum Coin{
    Cardano,
    Bitcoin(Balance),
    Ethereum,
}

fn decimal(coin: Coin) -> u8 {
    match coin{
        Coin::Cardano => {
            println!("cardano Match");
            1
        },
        Coin::Ethereum => 10,
        Coin::Bitcoin(bala) => {
            println!("I am a {:#?}", bala);
            100
        }
    }
}