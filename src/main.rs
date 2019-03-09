/// Here error leads to a "clean" panic which tears down all channels.
pub mod panic_on_error;

/// This is based on
/// https://medium.com/statuscode/pipeline-patterns-in-go-a37bb3a7e61d Here,
/// each pipeline "owns" it's sink, so it can never panic on `send`. It can,
/// however, enter deadlock.
pub mod deadlock_on_error;

/// Full solution from the above Go blog post, with cancellation. Similarly to
/// the blog post, this correctly handles forseen errors, but deadlocks on
/// panics. The in-progress messages are dropped.
///
/// Note that only the first participant in the pipeline really needs to care
/// about cancellation, everyone else gets effectively cancelled by exhausting
/// the source.
pub mod cancel_on_error_deadlock_on_panic;

/// The somewhat elaborate solution which has the following properties:
/// - expected errors cancel the pipeline in front-to-back fashion
/// - intermediate members of the pipeline don't have do decide what to do with
///   in-progress messages
/// - unexpected panics propagate backwards
///
/// The idea is, upon encountering an error, send it to the supervisor,
/// *together with in-progress messages*. Supervisor than gets to decide what to
/// do with them.
pub mod cancel_on_error_propagate_panic;

/// Solutions which relies on failable select, and propagates both panics and
/// legit errors backwards.
pub mod epipe;

fn main() {
    panic_on_error::run("1 2 3 4 5 6 7 8 9");
    // panic
    // panic_on_error::run("1 2 3 nope 5 6 7 8 9");
    // panic
    // panic_on_error::run("1 2 3 666 5 6 7 8 9");
    println!();

    deadlock_on_error::run("1 2 3 4 5 6 7 8 9");
    // deadlock
    // deadlock_on_error::run("1 2 3 nope 5 6 7 8 9");
    // panic
    // deadlock_on_error::run("1 2 3 666 5 6 7 8 9");
    println!();

    cancel_on_error_deadlock_on_panic::run("1 2 3 4 5 6 7 8 9");
    // cancellation
    cancel_on_error_deadlock_on_panic::run("1 2 3 nope 5 6 7 8 9");
    // deadlock
    // cancel_on_error_deadlock_on_panic::run("1 2 3 666 5 6 7 8 9");
    println!();

    cancel_on_error_propagate_panic::run("1 2 3 4 5 6 7 8 9");
    // cancellation
    cancel_on_error_propagate_panic::run("1 2 3 nope 5 6 7 8 9");
    // panic
    // cancel_on_error_propagate_panic::run("1 2 3 666 5 6 7 8 9");
    println!();

    epipe::run("1 2 3 4 5 6 7 8 9");
    // cancellation
    epipe::run("1 2 3 nope 5 6 7 8 9");
    // cancellation
    // epipe::run("1 2 3 666 5 6 7 8 9");
    println!();
}
