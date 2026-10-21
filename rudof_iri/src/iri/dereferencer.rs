//! The function [`IriS::dereference`] fetches IRIs with on wasm, where there
//! is no file system nor blocking HTTP client.
//!
//! [`IriS::dereference`] is synchronous, while the only way to make an HTTP
//! request from wasm is JavaScript's asynchronous `fetch`, so the platform
//! layer (e.g. `rudof_rdf`'s JSPI bridge) sets a function that waits for it
//! with [`set_dereferencer`].
//!
//! [`IriS::dereference`]: crate::IriS::dereference

use std::cell::Cell;
use url::Url;

/// Fetches `url` and returns the body of the response, or a description of
/// why it could not.
pub type Dereferencer = fn(&Url) -> Result<String, String>;

thread_local! {
    static DEREFERENCER: Cell<Option<Dereferencer>> = const { Cell::new(None) };
}

/// Sets the function used to dereference IRIs on wasm, or removes it with
/// `None`, after which dereferencing fails.
pub fn set_dereferencer(dereferencer: Option<Dereferencer>) {
    DEREFERENCER.set(dereferencer);
}

pub(crate) fn get() -> Option<Dereferencer> {
    DEREFERENCER.get()
}
