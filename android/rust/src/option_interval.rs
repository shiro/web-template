use core::pin::Pin;
use std::future::poll_fn;
use std::time::Duration;

use futures_core::task::{Context, Poll};
use tokio::time::{Interval, interval};

#[derive(Debug)]
pub struct OptionInterval {
    interval: Option<Pin<Box<Interval>>>,
}

impl OptionInterval {
    pub async fn tick(&mut self) -> () {
        let instant = poll_fn(|cx|
            if let Some(interval) = &mut self.interval {
                interval.poll_tick(cx)
            } else {
                Poll::Pending
            }
        );
        instant.await;
    }
    pub fn is_some(&self) -> bool { self.interval.is_some() }
    pub fn is_none(&self) -> bool { !self.is_some() }
}

impl From<Option<Duration>> for OptionInterval {
    fn from(value: Option<Duration>) -> Self {
        option_interval(value)
    }
}

impl From<Duration> for OptionInterval {
    fn from(value: Duration) -> Self {
        option_interval(Some(value))
    }
}

pub fn option_interval(duration: Option<Duration>) -> OptionInterval {
    OptionInterval { interval: duration.map(|d| Box::pin(interval(d))) }
}