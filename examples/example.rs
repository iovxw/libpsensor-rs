extern crate futures;
extern crate tokio_core;

extern crate libpsensor;

use std::time::Duration;

use futures::Stream;
use tokio_core::reactor::Core;

fn main() {
    let mut lp = Core::new().unwrap();
    let (sensors, stream) = libpsensor::new(Duration::from_secs(1), &lp.handle());
    println!("{:?}", sensors);
    lp.run(stream.for_each(|(sensor, value)| {
                                 println!("sensor: {}, type: {:?}, value: {}",
                                          sensor.name,
                                          sensor.sensor,
                                          value);
                                 Ok(())
                             }))
        .unwrap();
}
