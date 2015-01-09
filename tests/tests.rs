extern crate ids_container;

use ids_container::IdsContainer;

#[test]
fn different_ids() {
    let mut container = IdsContainer::new();

    let id1 = container.insert(5);
    let id2 = container.insert(5);
    let id3 = container.insert(7);
    let id4 = container.insert(12);

    assert!(id1 != id2);
    assert!(id2 != id3);
    assert!(id3 != id4);
}

#[test]
fn get() {
    let mut container = IdsContainer::new();

    let id1 = container.insert(5);
    let id2 = container.insert(8);

    assert_eq!(container.get(&id1), Some(&5));
    assert_eq!(container.get(&id2), Some(&8));

    let id3 = container.insert(12);
    container.remove(id3.clone());
    assert_eq!(container.get(&id3), None);
}

#[test]
fn len() {
    let mut container = IdsContainer::new();
    assert_eq!(container.len(), 0);

    let id1 = container.insert(5);
    assert_eq!(container.len(), 1);

    let id2 = container.insert(5);
    assert_eq!(container.len(), 2);

    let id3 = container.insert(7);
    assert_eq!(container.len(), 3);

    let id4 = container.insert(12);
    assert_eq!(container.len(), 4);

    container.remove(id2);
    assert_eq!(container.len(), 3);

    container.remove(id1);
    assert_eq!(container.len(), 2);

    container.remove(id4);
    assert_eq!(container.len(), 1);

    container.remove(id3);
    assert_eq!(container.len(), 0);
}

#[test]
fn is_empty() {
    let mut container = IdsContainer::new();
    assert!(container.is_empty());

    let id1 = container.insert(5);
    assert!(!container.is_empty());

    let id2 = container.insert(5);
    assert!(!container.is_empty());

    let id3 = container.insert(7);
    assert!(!container.is_empty());

    let id4 = container.insert(12);
    assert!(!container.is_empty());

    container.remove(id2);
    assert!(!container.is_empty());

    container.remove(id1);
    assert!(!container.is_empty());

    container.remove(id4);
    assert!(!container.is_empty());

    container.remove(id3);
    assert!(container.is_empty());
}
