#[rustfmt::skip]
use std::{
    error,
    result,
};

#[rustfmt::skip]
#[traffic_light_macros::test]
async fn block_on_result_ok() {
    let x: result::Result<(), Box<dyn error::Error>> = async {
        Ok(())
    }.await;

    assert!(x.is_ok())
}

#[rustfmt::skip]
#[traffic_light_macros::test]
async fn block_on_result_err() {
    let x: result::Result<(), &str> = async {
        Err("")
    }.await;

    assert!(x.is_err())
}
