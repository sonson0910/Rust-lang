use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s1 = String::from("A string");
    // let s2 = String::new();
    // let s3 = "A string";

    // let s4 = s3.to_string();
    let s2 = String::from("Toi ten la Nguyen Hong Son");
    let s3 = String::from("Okw");

    // let s5 = s2 + " test " + &s3;
    // println!("{}", s5);

        // Bytes
    for i in s3.chars(){
        println!("{}", i)
    }

    // Scalar values
    for i in s3.bytes(){
        println!("{}", i)
    }

    //Grapheme Cluesters: in chu hoan chinh
    for i in s3.graphemes(true){
        println!("{}", i)
    }

}
