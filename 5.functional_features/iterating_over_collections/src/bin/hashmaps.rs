use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::from([("Harry", 40.0), ("Hermoine", 50.0), ("Ron", 35.5)]);
    for (_, mark) in &mut marks {
        *mark = (*mark * 100.0) / 50.0;
    }
    marks
        .iter()
        .for_each(|(name, mark)| println!("{name} scored {mark}%"));
}
