fn main() {
    let numbers = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twefth"];
    let strings = ["My good friends brought to me,",
                   "All their good wishes,",
                   "Gifts for one and all,",
                   "Some mistletoe,",
                   "A guardian angel,",
                   "Gold and silver stringsnsel,",
                   "Candles a-glowing,",
                   "Little silver bells",
                   "A shining star",
                   "Four colored lights,",
                   "Three boughs of holly,",
                   "Two candy canes,"];

    println!("On the {} day of Christmas", numbers[0]);
    
    println!("{}",strings[0]);

    println!("And a song for the Christmas tree.");
    println!();

    for verse in (1..12) {
        println!("On the {} day of Christmas", numbers[verse]);

        for string in (0..verse+1) {
            println!("{}",strings[string]);
        }

        println!("And a song for the Christmas tree.");
        println!();
    }
}
