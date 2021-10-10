//! The 'structure' module defines the communication 'actors' that make up the
//! RTPS protocol.
//!
//! See [section 8.2](https://www.omg.org/spec/DDSI-RTPS/2.5/PDF#page=21) of the relevant specification document.

mod endpoint;
mod entity;
pub mod group;
mod guid;
mod history;
mod locator;
pub mod participant;
mod protocol_version;
mod vendor_id;

pub use endpoint::Endpoint;
pub use entity::Entity;
#[doc(inline)]
pub use group::{Group, Publisher, Subscriber};
pub use guid::Guid;
pub use locator::Locator;
#[doc(inline)]
pub use participant::Participant;
