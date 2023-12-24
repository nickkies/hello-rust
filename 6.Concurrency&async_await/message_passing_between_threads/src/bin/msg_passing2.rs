use std::{sync::mpsc, thread};

fn main() {
    let sentences = [
    "!tpircs llehs a eb ot detnaw eh esuaceB ?tsuR nrael barC eht sirreF did yhW".to_string(),
        "!sgel sih fo thgie lla htiw tsuR ni edoc nac eh - reksat-itlum etamitlu eht si barC eht sirreF".to_string()
    ];
    let (tx, rx) = mpsc::channel();

    for sentence in sentences {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let reversed = sentence.chars().rev().collect::<String>();
            tx_clone.send(reversed).unwrap();
        });
    }
    drop(tx);

    let printer = thread::spawn(|| {
        println!("Reversed sentences:");
        for sentence in rx {
            println!("{sentence}");
        }
    });
    printer.join().unwrap();
}
