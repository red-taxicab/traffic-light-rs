#[rustfmt::skip]
use std::{
    error,
    result,
};

#[traffic_light::main]
async fn main() -> result::Result<(), Box<dyn error::Error>> {
    Ok(())
}
