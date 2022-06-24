use rocket::local::blocking::Client;
use rocket::http::Status;

#[test]
fn test_count() {
    let client = Client::tracked(super::rocket()).unwrap();

    fn get_count(client: &Client) -> usize {
        let response = client.get("/count").dispatch().into_string().unwrap();
        let count = response.split(" ").last().unwrap();
        count.parse().unwrap()
    }

    // Count starts at 0; our hit is the first.
    for i in 1..128 {
        assert_eq!(get_count(&client), i);
    }
}

// Cargo runs each test in parallel on different threads. We use all of these
// tests below to show (and assert) that state is managed per-Rocket instance.
#[test] fn test_count_parallel() { test_count() }
#[test] fn test_count_parallel_2() { test_count() }
#[test] fn test_count_parallel_3() { test_count() }
#[test] fn test_count_parallel_4() { test_count() }
#[test] fn test_count_parallel_5() { test_count() }
#[test] fn test_count_parallel_6() { test_count() }
#[test] fn test_count_parallel_7() { test_count() }
#[test] fn test_count_parallel_8() { test_count() }
#[test] fn test_count_parallel_9() { test_count() }

#[test]
fn test_queue_push_pop() {
    let client = Client::tracked(super::rocket()).unwrap();

    let response = client.put("/queue/push?event=test1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let response = client.get("/queue/pop").dispatch();
    assert_eq!(response.into_string().unwrap(), "test1");

    client.put("/queue/push?event=POP!%20...goes+").dispatch();
    client.put("/queue/push?event=the+weasel").dispatch();
    let r1 = client.get("/queue/pop").dispatch().into_string().unwrap();
    let r2 = client.get("/queue/pop").dispatch().into_string().unwrap();
    assert_eq!(r1 + &r2, "POP! ...goes the weasel");
}

#[test]
fn test_request_local_state() {
    use super::request_local::Atomics;
    use std::sync::atomic::Ordering;

    let client = Client::tracked(super::rocket()).unwrap();

    client.get("/req-local/1-2").dispatch();
    let atomics = client.rocket().state::<Atomics>().unwrap();
    assert_eq!(atomics.uncached.load(Ordering::Relaxed), 2);
    assert_eq!(atomics.cached.load(Ordering::Relaxed), 1);

    client.get("/req-local/1-2").dispatch();
    let atomics = client.rocket().state::<Atomics>().unwrap();
    assert_eq!(atomics.uncached.load(Ordering::Relaxed), 4);
    assert_eq!(atomics.cached.load(Ordering::Relaxed), 2);

    client.get("/req-local/1-2-3-4").dispatch();
    let atomics = client.rocket().state::<Atomics>().unwrap();
    assert_eq!(atomics.uncached.load(Ordering::Relaxed), 8);
    assert_eq!(atomics.cached.load(Ordering::Relaxed), 3);
}
