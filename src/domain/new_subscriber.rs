//! src/domain/new_subscriber.rs

use crate::domain::SubscriberName;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
