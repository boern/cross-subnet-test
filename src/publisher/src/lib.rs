use candid::{CandidType, Principal};
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;

type SubscriberStore = BTreeMap<Principal, Subscriber>;

thread_local! {
    static SUBSCRIBERS: RefCell<SubscriberStore> = RefCell::default();
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
fn subscribe(subscriber: Subscriber) {
    let subscriber_principal_id = ic_cdk::caller();
    SUBSCRIBERS.with(|subscribers| {
        subscribers
            .borrow_mut()
            .insert(subscriber_principal_id, subscriber)
    });
}

#[update]
async fn publish(counter: Counter) {
    SUBSCRIBERS.with(|subscribers| {
        for (k, v) in subscribers.borrow().iter() {
            if v.topic == counter.topic {
                let before_call = ic_cdk::api::time();
                // ic_cdk::println!("canister before inner call time:{:?}", before_call);
                let _call_result: Result<(), _> = ic_cdk::notify(*k, "update_count", (&counter,));
                // let _call_result: Result<(), _> =
                //     ic_cdk::call(*k, "update_count", (&counter,)).await;
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
        }
    });
}

#[query]
fn get_inner_call_time() -> CallTime {
    CALL_TIME.with(|c| c.get())
}
