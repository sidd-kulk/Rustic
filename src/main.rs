fn main(){
    let person = Person { name: "My Name".to_string(),
                          address: Address {street: 1,house_identifier: "".to_string(), pincode: "".to_string()} };

    println!("{:?}", person)

}

#[derive(Debug)]
struct Person{
    name: String,
    address: Address
}

#[derive(Debug)]
struct Address{
    street: u16,
    house_identifier: String,
    pincode: String
}