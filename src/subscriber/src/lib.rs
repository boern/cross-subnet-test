use candid::{CandidType, Principal};
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::Cell;

thread_local! {
    static COUNTER: Cell<u64> = Cell::new(0);
    static CALL_TIME: Cell<CallTime> = Cell::new(CallTime::default());
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Counter {
    topic: String,
    value: u64,
}

#[derive(Clone, Copy, Debug, CandidType, Deserialize, Default)]
struct CallTime {
    before_call: u64,
    after_call: u64,
    delay: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Subscriber {
    topic: String,
}

#[update]
async fn setup_subscribe(publisher_id: Principal, topic: String) {
    let subscriber = Subscriber { topic };
    let before_call = ic_cdk::api::time();
    // ic_cdk::println!("canister before inner call time:{:?}", before_call);
    let _call_result: Result<(), _> = ic_cdk::call(publisher_id, "subscribe", (subscriber,)).await;
    let after_call = ic_cdk::api::time();
    // ic_cdk::println!("canister after inner call time:{:?}", after_call);
    let delay = after_call - before_call;
  
    let call_time = CallTime {
        before_call,
        after_call,
        delay: delay,
    };
    
    ic_cdk::println!("canister inner call time:{:?}", call_time);

    CALL_TIME.with(|c| {
        c.set(call_time);
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
fn get_inner_call_time() -> CallTime {
    CALL_TIME.with(|c| c.get())
}
