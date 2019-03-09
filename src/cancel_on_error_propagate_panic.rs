use std::num::ParseIntError;

use crossbeam::{
    channel::{bounded, select, Receiver},
    thread::{scope, Scope},
};

fn source(scope: &Scope<'_>, done: Receiver<Void>, text: String) -> Receiver<String> {
    let (sender, receiver) = bounded(0);
    scope.spawn(move |_| {
        for word in text.split_whitespace() {
            let word = word.to_owned();
            select! {
                send(sender, word) -> res => res.unwrap(),
                recv(done) -> msg => match msg {
                    Err(_) => break,
                    Ok(void) => match void {},
                }
            }
        }
    });
    receiver
}

fn transform(
    scope: &Scope<'_>,
    in_ch: Receiver<String>,
) -> (Receiver<i32>, Receiver<(ParseIntError, Receiver<String>)>) {
    let (sender, out_ch) = bounded(0);
    let (err_sender, err_receiver) = bounded(1);
    scope.spawn(move |_| {
        while let Ok(word) = in_ch.recv() {
            match word.parse::<i32>() {
                Ok(int) => sender.send(int).unwrap(),
                Err(e) => {
                    err_sender.send((e, in_ch)).unwrap();
                    break;
                }
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

enum Void {}

pub fn run(text: &str) {
    let (cancel_sender, cancel_receiver) = bounded::<Void>(0);
    scope(|scope| {
        let words = source(scope, cancel_receiver.clone(), text.to_owned());
        let (numbers, errors) = transform(scope, words);
        sink(scope, numbers);
        match errors.recv() {
            Ok((e, ch)) => {
                println!("oups {} :(", e);
                drop(cancel_sender);
                for msg in ch {
                    println!("dropped message {:?}", msg)
                }
            }
            Err(_) => println!("ok"),
        }
    })
    .unwrap();
    println!("done")
}
