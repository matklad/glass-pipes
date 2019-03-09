use std::num::ParseIntError;

use crossbeam::{
    channel::{bounded, select, Receiver},
    thread::{scope, Scope},
};

fn source(scope: &Scope<'_>, done: Receiver<Void>, text: String) -> Receiver<String> {
    let (sender, receiver) = bounded(0);
    scope.spawn({
        // Here and below, we make sure that pipeline stage itself owns the
        // channel.
        let receiver = receiver.clone();
        move |_| {
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
            drop(receiver)
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
    scope.spawn({
        let out_ch = out_ch.clone();
        move |_| {
            for word in in_ch {
                match word.parse::<i32>() {
                    Ok(int) => sender.send(int).unwrap(),
                    Err(e) => {
                        err_sender.send(e).unwrap();
                        return;
                    }
                }
            }
            drop(out_ch);
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
        let words = source(scope, cancel_receiver, text.to_owned());
        let (numbers, errors) = transform(scope, words);
        sink(scope, numbers);
        match errors.recv() {
            Ok(e) => {
                println!("oups {} :(", e);
                drop(cancel_sender);
            }
            Err(_) => println!("ok"),
        }
    })
    .unwrap();
    println!("done")
}
