use core::pin::Pin;
use std::future::{Future, poll_fn};
use std::time::Duration;

use futures_core::ready;
use futures_core::task::{Context, Poll};
use tokio::time::{Sleep, sleep};

#[derive(Debug)]
pub struct OptionSleep {
    delay: Option<Pin<Box<Sleep>>>,
}

impl OptionSleep {
    pub async fn tick(&mut self) -> () {
        let instant = poll_fn(|cx| self.poll_tick(cx));
        instant.await
    }
    pub fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        match self.delay.as_mut() {
            Some(sleep) => {
                ready!(Pin::new(sleep).poll(cx));
            }
            None => { return Poll::Pending; }
        }

        self.delay = None;

        Poll::Ready(())
    }

    pub fn is_some(&self) -> bool { self.delay.is_some() }
    pub fn is_none(&self) -> bool { !self.is_some() }
}


pub fn option_sleep(duration: Option<Duration>) -> OptionSleep {
    OptionSleep { delay: duration.map(|d| Box::pin(sleep(d))) }
}