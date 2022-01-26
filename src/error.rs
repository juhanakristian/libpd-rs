use thiserror::Error;

#[derive(Error, Debug)]
pub enum InitializationError {
    #[error("Pure Data is already initialized.")]
    AlreadyInitialized,
    #[error("An unknown error occurred in Pure Data initialization.")]
    InitializationFailed,
    #[error("An unknown error occurred in Pure Data audio initialization.")]
    AudioInitializationFailed,
}

#[derive(Error, Debug)]
pub enum IoError {
    #[error("Failed to open patch for unknown reason.")]
    FailedToOpenPatch,
    // Add more errors here..
}

#[derive(Error, Debug)]
pub enum SendError {
    #[error("No destination found for receiver: `{0}` in loaded pd patch.")]
    MissingDestination(String),
    // Add more errors here..
}

#[derive(Error, Debug)]
pub enum SubscriptionError {
    #[error("Failed to subscribe to sender: `{0}` in loaded pd patch.")]
    FailedToSubscribeToSender(String),
    // Add more errors here..
}
