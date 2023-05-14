fn main() {
    twelve_days();
}

fn twelve_days() {
    let stuff = ["partridge in a pear tree", "turtle doves", "french hens", "calling birds", "gold rings", "geese laying", "swans swimming", "maids milking", "drummers drumming", "pipers piping", "ladies dancing", "lords leaping"];
    for i in 1..13 {
        let song = format!("On the {i} day of Christmas my true love sent to me");
        let mut rest = if i > 1 {format!("and {} {}", i, stuff[i-1])} else {format!("")};
        for (j, item) in stuff.split_last().unwrap().1.iter().enumerate().rev() {
            rest = format!("{} {}, {}", j+1, item, rest);
        }
        println!("{song} {rest}");
    }
}
