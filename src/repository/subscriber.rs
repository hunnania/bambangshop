use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::{self, Subscriber};

// Singleton of Database
lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
}