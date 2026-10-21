//! The classification of an API response and the bound on a 429's re-queue.
//!
//! Rulings: `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.

use crate::daemon::*;

fn rate() -> Value {
    serde_json::json!({ "retry-after": "1" })
}

#[test]
fn bounded_429_requeue_keeps_the_existing_two_retry_contract() {
    assert!(may_requeue_429(0, false));
    assert!(may_requeue_429(1, false));
    assert!(!may_requeue_429(2, false));
    assert!(!may_requeue_429(0, true));
}

#[test]
fn acceptable_429_is_the_only_body_outcome_that_requeues() {
    let error = classify_api_body(
        reqwest::StatusCode::TOO_MANY_REQUESTS,
        &RetryAfter::Acceptable { seconds: 1 },
        "/character",
        rate(),
        Ok("slow down".into()),
    )
    .unwrap_err();
    assert!(matches!(error, ApiError::RateLimited(_)));

    let terminal = [
        RetryAfter::Missing,
        RetryAfter::Malformed { raw: "soon".into() },
        RetryAfter::Negative { raw: "-1".into() },
        RetryAfter::OverCap { raw: "901".into() },
    ];
    for retry_after in terminal {
        let error = classify_api_body(
            reqwest::StatusCode::TOO_MANY_REQUESTS,
            &retry_after,
            "/character",
            rate(),
            Ok("slow down".into()),
        )
        .unwrap_err();
        let ApiError::Other(message) = error else {
            panic!("{retry_after} entered the retryable arm")
        };
        assert!(message.contains("terminal rate limit"));
        assert!(message.contains("NOT retrying"));
    }
}

#[test]
fn body_read_transport_failure_is_not_recast_as_bad_json() {
    let error = classify_api_body(
        reqwest::StatusCode::OK,
        &RetryAfter::Missing,
        "/character",
        Value::Null,
        Err("connection closed before message completed".into()),
    )
    .unwrap_err();
    let ApiError::Other(message) = error else {
        panic!("body transport failure used the wrong error kind")
    };
    assert!(message.contains("body read transport failure"));
    assert!(message.contains("connection closed before message completed"));
    assert!(!message.contains("bad JSON"));
}

#[test]
fn http_status_keeps_precedence_over_error_body_transport_failure() {
    let error = classify_api_body(
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        &RetryAfter::Missing,
        "/character",
        Value::Null,
        Err("body reset".into()),
    )
    .unwrap_err();
    let ApiError::Other(message) = error else {
        panic!("HTTP failure used the wrong error kind")
    };
    assert!(message.contains("returned 500 Internal Server Error"));
    assert!(message.contains("body read transport failure: body reset"));
}

#[test]
fn malformed_success_body_remains_a_json_failure() {
    let error = classify_api_body(
        reqwest::StatusCode::OK,
        &RetryAfter::Missing,
        "/character",
        Value::Null,
        Ok("not json".into()),
    )
    .unwrap_err();
    let ApiError::Other(message) = error else {
        panic!("JSON failure used the wrong error kind")
    };
    assert!(message.contains("bad JSON"));
}
