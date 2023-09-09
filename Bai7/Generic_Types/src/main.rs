fn main() {
    let number_list = vec![20,1245,6235,124,112,41];

    let mut largest = number_list[0];
    
    let mut largest = get_largest(number_list);

    println!("the largest number is {}", largest);


    //check
    let number_list = vec!['c', 'q', 't', 'y', 'i'];

    let mut largest = number_list[0];

    let mut largest = get_largest(number_list);

    println!("the largest number is {}", largest);
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T
{
    let mut largest = number_list[0];

    for number in number_list{
        if number > largest{
            largest = number
        }
    }
    largest
}