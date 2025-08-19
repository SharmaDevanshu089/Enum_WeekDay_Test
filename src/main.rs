enum Weekday {
    Sunday,
    Monday,
}
fn main() {
    let day = Weekday::Monday;
    match day {
        Weekday::Monday => print!("Weekend"),
        Weekday::Sunday => print!("Go to Office")
    }
}
