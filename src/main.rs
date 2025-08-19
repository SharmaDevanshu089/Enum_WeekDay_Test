enum Weekday {
    Sunday,
    Monday,
}
fn main() {
    let day = Weekday::Monday;
    match day {
        Weekday::Monday => println!("Office Day"),
        Weekday::Sunday => println!("Holiday"),
        _ => println!("Error Occured")
    }
}
