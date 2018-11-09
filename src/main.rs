fn main(){
    println!("Hello World!!");

    let string = "272";
    let integer = string.parse::<u16>().unwrap();
    println!("{:?}", integer)

}