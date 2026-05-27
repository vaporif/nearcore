use crate::logic::mocks::mock_external::MockAction;
use crate::logic::tests::vm_logic_builder::VMLogicBuilder;
use crate::logic::{HostError, VMLogicError};
use near_primitives_core::hash::CryptoHash;

#[test]
fn promise_yield_resume_status_returns_zero_for_absent_row() {
    let mut builder = VMLogicBuilder::free();
    let mut logic = builder.build();

    let data_id = CryptoHash::hash_bytes(b"never-yielded");
    logic.wrapped_internal_write_register(0, &data_id.0).unwrap();

    let result =
        logic.promise_yield_resume_status(u64::MAX, 0).expect("promise_yield_resume_status");

    assert_eq!(result, 0);
}

#[test]
fn promise_yield_resume_status_returns_one_for_yielded() {
    let data_id = CryptoHash::hash_bytes(b"yielded-id");
    let mut builder = VMLogicBuilder::free();
    builder
        .ext
        .action_log
        .push(MockAction::YieldCreate { data_id, receiver_id: "alice.near".parse().unwrap() });
    let mut logic = builder.build();

    logic.wrapped_internal_write_register(0, &data_id.0).unwrap();

    let result =
        logic.promise_yield_resume_status(u64::MAX, 0).expect("promise_yield_resume_status");

    assert_eq!(result, 1);
}

#[test]
fn promise_yield_resume_status_returns_two_after_resume() {
    let data_id = CryptoHash::hash_bytes(b"resumed-id");
    let mut builder = VMLogicBuilder::free();
    builder
        .ext
        .action_log
        .push(MockAction::YieldCreate { data_id, receiver_id: "alice.near".parse().unwrap() });
    builder.ext.action_log.push(MockAction::YieldResume { data_id, data: vec![] });
    let mut logic = builder.build();

    logic.wrapped_internal_write_register(0, &data_id.0).unwrap();

    let result =
        logic.promise_yield_resume_status(u64::MAX, 0).expect("promise_yield_resume_status");

    assert_eq!(result, 2);
}

#[test]
fn promise_yield_resume_status_rejects_malformed_data_id() {
    let mut builder = VMLogicBuilder::free();
    let mut logic = builder.build();

    let bad = [1u8; 16];
    logic.wrapped_internal_write_register(0, &bad).unwrap();

    let err =
        logic.promise_yield_resume_status(u64::MAX, 0).expect_err("must reject malformed data_id");

    assert!(matches!(err, VMLogicError::HostError(HostError::DataIdMalformed)));
}

#[test]
fn promise_yield_resume_status_allowed_in_view_call() {
    let data_id = CryptoHash::hash_bytes(b"view-call-id");
    let mut builder = VMLogicBuilder::view();
    let mut logic = builder.build();

    logic.wrapped_internal_write_register(0, &data_id.0).unwrap();

    let result = logic
        .promise_yield_resume_status(u64::MAX, 0)
        .expect("view call must not error with ProhibitedInView");

    assert_eq!(result, 0);
}
