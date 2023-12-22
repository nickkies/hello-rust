use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    let sentense = [
        "!dlroW olleH".to_string(),
        ".tsurT eW tsuR nI".to_string(),
        "!ytsuR teG s'teL".to_string(),
        "!tsuB ro tsuR".to_string(),
    ];

    for s in sentense {
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let reversed: String = s.chars().rev().collect();
            tx_clone.send(reversed).unwrap();
        });
    }

    drop(tx);

    for sentense in rx {
        println!("{sentense}");
    }
}
