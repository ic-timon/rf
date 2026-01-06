//! Container module tests

use rf_container::{Ring, ThreadSafeRing, Queue, Set};

#[test]
fn test_ring_basic() {
    let mut ring = Ring::new(3);
    
    // Test push and len
    assert_eq!(ring.len(), 0);
    ring.push(1);
    assert_eq!(ring.len(), 1);
    ring.push(2);
    ring.push(3);
    assert_eq!(ring.len(), 3);
    
    // Test overflow (should overwrite oldest)
    ring.push(4);
    assert_eq!(ring.len(), 3);
    
    // Test pop
    assert_eq!(ring.pop(), Some(2)); // 1 was overwritten
    assert_eq!(ring.pop(), Some(3));
    assert_eq!(ring.pop(), Some(4));
    assert_eq!(ring.pop(), None);
}

#[test]
fn test_ring_empty() {
    let mut ring: Ring<i32> = Ring::new(5);
    assert_eq!(ring.len(), 0);
    assert_eq!(ring.pop(), None);
    assert!(ring.is_empty());
}

#[test]
fn test_ring_full() {
    let mut ring = Ring::new(2);
    ring.push(1);
    ring.push(2);
    assert!(ring.is_full());
    ring.push(3); // Should overwrite 1
    assert!(ring.is_full());
    assert_eq!(ring.pop(), Some(2));
}

#[test]
fn test_thread_safe_ring() {
    let ring = ThreadSafeRing::new(3);
    
    // Test push
    ring.push(1);
    ring.push(2);
    assert_eq!(ring.len(), 2);
    
    // Test pop
    assert_eq!(ring.pop(), Some(1));
    assert_eq!(ring.pop(), Some(2));
    assert_eq!(ring.pop(), None);
}

#[test]
fn test_queue_basic() {
    let queue: Queue<i32> = Queue::new();
    
    // Test push and pop
    queue.push(1);
    queue.push(2);
    queue.push(3);
    
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.pop(), None);
}

#[test]
fn test_queue_empty() {
    let queue: Queue<i32> = Queue::new();
    assert_eq!(queue.pop(), None);
    assert!(queue.is_empty());
}

#[test]
fn test_queue_clone() {
    let queue1: Queue<i32> = Queue::new();
    queue1.push(1);
    queue1.push(2);
    
    let queue2 = queue1.clone();
    
    // Clone shares the same underlying queue (Arc)
    // So both should see the same elements
    assert_eq!(queue1.pop(), Some(1));
    assert_eq!(queue2.pop(), Some(2)); // Next element from shared queue
}

#[test]
fn test_set_basic() {
    let mut set = Set::new();
    
    // Test insert
    assert!(set.insert(1));
    assert!(set.insert(2));
    assert!(!set.insert(1)); // Duplicate
    
    assert_eq!(set.len(), 2);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
}

#[test]
fn test_set_remove() {
    let mut set = Set::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    assert!(set.remove(&2));
    assert_eq!(set.len(), 2);
    assert!(!set.contains(&2));
    assert!(!set.remove(&2)); // Already removed
}

#[test]
fn test_set_clear() {
    let mut set = Set::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    set.clear();
    assert_eq!(set.len(), 0);
    assert!(set.is_empty());
}

#[test]
fn test_set_iter() {
    let mut set = Set::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    // Test that all elements are present
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
    assert_eq!(set.len(), 3);
}

