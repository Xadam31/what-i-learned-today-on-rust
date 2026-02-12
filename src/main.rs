fn main() {
    let day: i32= 5;
    match day{
        1 => println!("sunday"),
        2 => println!("monday"),
        3=>println!("tuesday"),
        4=>println!("wednesday"),
        5=>println!("thursday"),
        6=>println!("friday"),
        7=>println!("saturday"),
        _=>println!("like thats a real day")
    }
    let weekend: bool = day>5;
    if weekend == true {
        println!("weekend,yes");
    }
    else {
        println!("weekend no");
    }
}
