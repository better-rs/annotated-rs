#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::io::{self, BufReader, Read, Write};
use std::time::Duration;

use rocket::http::ContentType;
use rocket::response::{Content, Stream};
use rocket_contrib::serve::StaticFiles;

const BUF_SIZE: usize = 4096;

struct Counter {
    n: usize,
    state: State,
}

enum State { Flush, Sleep, Write }

impl Read for Counter {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        // ping/pong between sleep and flush, and implicit sleep -> write
        match self.state {
            State::Flush => {
                self.state = State::Sleep;
                Err(io::ErrorKind::WouldBlock)?;
            },
            State::Sleep => std::thread::sleep(Duration::from_millis(500)),
            State::Write => { /* fall through to `State::Write` */ },
        }

        self.n += 1;
        self.state = State::Flush;

        // `BufReader` won't call us unless its buffer is empty, and then buf
        // will be the whole of the buffer, ie of size BUF_SIZE (due to the
        // `with_capacity` call).  So `data` is definitely going to fit.
        let data = format!("data: {}\n\n", self.n);
        buf.write_all(data.as_bytes())?;
        Ok(data.len())
    }
}

type CounterStream = Stream<BufReader<Counter>>;

#[get("/updates")]
fn updates() -> Content<CounterStream> {
    let reader = BufReader::with_capacity(BUF_SIZE, Counter { n: 0, state: State::Write });
    let ct = ContentType::with_params("text", "event-stream", ("charset", "utf-8"));
    Content(ct, Stream::from(reader))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![updates])
        .mount("/", StaticFiles::from("static"))
        .launch();
}
