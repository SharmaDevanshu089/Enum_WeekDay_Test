enum Weekday {
    Sunday,
    Monday,
}
fn main() {
    let day = Weekday::Monday;
    match day {
        Weekday::Monday => print!("Office Day"),
        Weekday::Sunday => print!("Holiday"),
        _ => println!("Error Occured")
    }
}
