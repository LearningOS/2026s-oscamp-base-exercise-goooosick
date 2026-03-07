//! # Channel Communication
//!
//! In this exercise, you will use `std::sync::mpsc` channels to pass messages between threads.
//!
//! ## Concepts
//! - `mpsc::channel()` creates a multiple producer, single consumer channel
//! - `Sender::send()` sends a message
//! - `Receiver::recv()` receives a message
//! - Multiple producers can be created via `Sender::clone()`

use std::sync::mpsc;
use std::thread;

/// Create a producer thread that sends each element from items into the channel.
/// The main thread receives all messages and returns them.
pub fn simple_send_recv(items: Vec<String>) -> Vec<String> {
    let receiver = {
        let (sender, receiver) = mpsc::channel();
        items.into_iter().for_each(|item| {
            let s = sender.clone();
            thread::spawn(move || {
                let _ = s.send(item).unwrap();
            });
        });
        receiver
    };
    let mut vec = vec![];
    while let Ok(item) = receiver.recv() {
        vec.push(item);
    }
    vec
}

/// Create `n_producers` producer threads, each sending a message in format `"msg from {id}"`.
/// Collect all messages, sort them lexicographically, and return.
///
/// Hint: Use `tx.clone()` to create multiple senders. Note that the original tx must also be dropped.
pub fn multi_producer(n_producers: usize) -> Vec<String> {
    let receiver = {
        let (sender, receiver) = mpsc::channel();
        (0..n_producers).into_iter().for_each(|id| {
            let s = sender.clone();
            thread::spawn(move || {
                let _ = s.send(format!("msg from {id}")).unwrap();
            });
        });
        receiver
    };
    let mut vec = vec![];
    while let Ok(item) = receiver.recv() {
        vec.push(item);
    }
    vec.sort();
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_send_recv() {
        let items = vec!["hello".into(), "world".into(), "rust".into()];
        let result = simple_send_recv(items.clone());
        assert_eq!(result, items);
    }

    #[test]
    fn test_simple_empty() {
        let result = simple_send_recv(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multi_producer() {
        let result = multi_producer(3);
        assert_eq!(
            result,
            vec![
                "msg from 0".to_string(),
                "msg from 1".to_string(),
                "msg from 2".to_string(),
            ]
        );
    }

    #[test]
    fn test_multi_producer_single() {
        let result = multi_producer(1);
        assert_eq!(result, vec!["msg from 0".to_string()]);
    }
}
