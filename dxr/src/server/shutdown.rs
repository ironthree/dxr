/// trait definition for server off switches that can be used to handle graceful shutdown
#[async_trait::async_trait]
pub trait ServerOffSwitch: Send + Sync {
    /// method for checking the state of the off switch
    fn state(&self) -> bool;

    /// method for flipping the off switch
    fn flip(&self);

    /// async method that sleeps for an arbitrary amount of time
    ///
    /// This determines how often the state of the off switch is checked.
    async fn sleep(&self);

    /// async method that checks the state of the server off switch
    ///
    /// This method only ever yields its future value once the state of the switch is flipped.
    async fn watch(&self) {
        loop {
            if self.state() {
                return;
            } else {
                self.sleep().await;
            }
        }
    }
}

#[cfg(feature = "tokio")]
mod tok_io {
    use std::sync::{Arc, RwLock};
    use std::time::Duration;

    use super::ServerOffSwitch;

    const DEFAULT_SLEEP: Duration = Duration::from_secs(5);

    /// implementation of [`ServerOffSwitch`] based on tokio
    #[derive(Clone, Debug)]
    pub struct TokioOffSwitch {
        state: Arc<RwLock<bool>>,
        sleep: Duration,
    }

    impl Default for TokioOffSwitch {
        fn default() -> Self {
            TokioOffSwitch::new()
        }
    }

    impl TokioOffSwitch {
        /// constructor for [`TokioOffSwitch`] with default settings (sleeping 5 seconds between
        /// checks of the "off switch state")
        pub fn new() -> TokioOffSwitch {
            TokioOffSwitch {
                state: Arc::new(RwLock::new(false)),
                sleep: DEFAULT_SLEEP,
            }
        }

        /// This method makes it possible to override the default sleep duration between checks of
        /// the "off switch state". Short durations will result in faster responses to server
        /// shutdown requests, but will have a higher performance impact.
        pub fn set_sleep_duration(&mut self, duration: Duration) {
            self.sleep = duration;
        }
    }

    #[async_trait::async_trait]
    impl ServerOffSwitch for TokioOffSwitch {
        fn state(&self) -> bool {
            *self.state.read().expect("Poisoned lock!")
        }

        fn flip(&self) {
            let mut state = self.state.write().expect("Poisoned lock!");
            *state = true;
        }

        async fn sleep(&self) {
            tokio::time::sleep(self.sleep).await
        }
    }
}

#[cfg(feature = "tokio")]
pub use tok_io::*;
