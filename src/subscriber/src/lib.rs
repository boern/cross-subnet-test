use candid::{CandidType, Principal};
use ic_cdk::{query, update};
use serde::Deserialize;
use std::cell::Cell;

thread_local! {
    static COUNTER: Cell<u64> = Cell::new(0);
    static CALL_TIME: Cell<u64> = Cell::new(0);
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Counter {
    topic: String,
    value: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Subscriber {
    topic: String,
}

#[update]
async fn setup_subscribe(publisher_id: Principal, topic: String) {
    let subscriber = Subscriber { topic };
    let before_call = ic_cdk::api::time();
    ic_cdk::println!("canister before inner call time:{:?}", before_call);
    let _call_result: Result<(), _> = ic_cdk::call(publisher_id, "subscribe", (subscriber,)).await;
    let after_call = ic_cdk::api::time();
    ic_cdk::println!("canister after inner call time:{:?}", after_call);
    let delay = after_call - before_call;

    ic_cdk::println!("canister inner call time:{:?}", delay);

    CALL_TIME.with(|c| {
        c.set(delay);
    });
}

#[update]
fn update_count(counter: Counter) {
    COUNTER.with(|c| {
        c.set(c.get() + counter.value);
    });
}

#[query]
fn get_count() -> u64 {
    COUNTER.with(|c| c.get())
}

#[query]
fn get_inner_call_time() -> u64 {
    CALL_TIME.with(|c| c.get())
}
