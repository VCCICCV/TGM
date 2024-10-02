
// use std::collections::HashMap;
// use std::sync::Arc;
// use std::sync::Mutex;
// pub struct SimpleEventBus {
//     subscribers: Arc<Mutex<HashMap<String, Vec<Box<dyn Fn(String) + Send + Sync>>>>>,
// }

// impl SimpleEventBus {
//     pub fn new() -> Self {
//         Self {
//             subscribers: Arc::new(Mutex::new(HashMap::new())),
//         }
//     }
// }

// impl EventBus for SimpleEventBus {
//     fn publish(&self, event: &str) {
//         if let Some(subs) = self.subscribers.lock().unwrap().get(event) {
//             for subscriber in subs {
//                 subscriber(event.to_string());
//             }
//         }
//         println!("Event published: {}", event);
//     }

//     fn subscribe(&mut self, event_type: String, subscriber: Box<dyn Fn(String) + Send + Sync>) {
//         self.subscribers
//             .lock()
//             .unwrap()
//             .entry(event_type.to_string())
//             .or_default()
//             .push(subscriber);
//     }
// }
