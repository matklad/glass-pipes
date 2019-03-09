use crossbeam::{
    channel::{bounded, Receiver},
    thread::{scope, Scope},
};

fn source(scope: &Scope<'_>, text: String) -> Receiver<String> {
    let (sender, receiver) = bounded(0);
    scope.spawn({
        // Here and below, we make sure that pipeline stage itself owns the
        // channel.
        let receiver = receiver.clone();
        move |_| {
            for word in text.split_whitespace() {
                sender.send(word.to_owned()).unwrap()
            }
            drop(receiver)
        }
    });
    receiver
}

fn transform(scope: &Scope<'_>, in_ch: Receiver<String>) -> Receiver<i32> {
    let (sender, out_ch) = bounded(0);
    scope.spawn({
        let out_ch = out_ch.clone();
        move |_| {
            for word in in_ch {
                match word.parse::<i32>() {
                    Ok(int) => sender.send(int).unwrap(),
                    Err(e) => {
                        println!("oups {} :(", e);
                        return;
                    }
                }
            }
            drop(out_ch);
        }
    });
    out_ch
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
        let numbers = transform(scope, words);
        sink(scope, numbers);
    })
    .unwrap();
    println!("done")
}
