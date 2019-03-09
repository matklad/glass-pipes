use std::num::ParseIntError;

use crossbeam::{
    channel::{bounded, Receiver},
    thread::{scope, Scope},
};

fn source(scope: &Scope<'_>, text: String) -> Receiver<String> {
    let (sender, receiver) = bounded(0);
    scope.spawn(move |_| {
        for word in text.split_whitespace() {
            match sender.send(word.to_owned()) {
                Ok(()) => (),
                Err(_) => return,
            }
        }
    });
    receiver
}

fn transform(
    scope: &Scope<'_>,
    in_ch: Receiver<String>,
) -> (Receiver<i32>, Receiver<ParseIntError>) {
    let (sender, out_ch) = bounded(0);
    let (err_sender, err_receiver) = bounded(1);
    scope.spawn(move |_| {
        for word in in_ch {
            let int = match word.parse::<i32>() {
                Ok(int) => int,
                Err(e) => {
                    err_sender.send(e).unwrap();
                    return;
                }
            };
            match sender.send(int) {
                Ok(()) => (),
                Err(_) => return,
            }
        }
    });
    (out_ch, err_receiver)
}

fn sink(scope: &Scope<'_>, in_ch: Receiver<i32>) {
    scope.spawn(move |_| {
        for int in in_ch {
            if int == 666 {
                panic!("injected bug")
            }
            println!("{}", int);
        }
    });
}

pub fn run(text: &str) {
    scope(|scope| {
        let words = source(scope, text.to_owned());
        let (numbers, errors) = transform(scope, words);
        sink(scope, numbers);
        match errors.recv() {
            Ok(e) => {
                println!("oups {} :(", e);
            }
            Err(_) => println!("ok"),
        }
    })
    .unwrap();
    println!("done")
}
