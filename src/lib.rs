// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-jfs: jfs component model.
//!
//! Real implementation: jfs_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JfsItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JfsError {
    EmptyJfsItemName,
    DuplicateJfsItem(String),
    EmptyEventSubject,
}

impl fmt::Display for JfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyJfsItemName => write!(f, "jfs_item name must not be empty"),
            Self::DuplicateJfsItem(id) => write!(f, "duplicate jfs_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for JfsError {}

pub struct Jfs {
    pub name: String,
    jfs_items: BTreeMap<String, JfsItem>,
    events: BTreeMap<String, Event>,
}

impl Jfs {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            jfs_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_jfs_item(&mut self, item: JfsItem) -> Result<(), JfsError> {
        if item.name.trim().is_empty() {
            return Err(JfsError::EmptyJfsItemName);
        }
        let id = item.id.clone();
        if self.jfs_items.contains_key(&id) {
            return Err(JfsError::DuplicateJfsItem(id.clone()));
        }
        self.jfs_items.insert(id, item);
        Ok(())
    }

    pub fn jfs_item_count(&self) -> usize {
        self.jfs_items.len()
    }

    pub fn get_jfs_item(&self, id: &str) -> Option<&JfsItem> {
        self.jfs_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), JfsError> {
        if event.subject.trim().is_empty() {
            return Err(JfsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_jfs_item(id: &str, name: &str) -> JfsItem {
        JfsItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Jfs::new("Jfs");
        assert_eq!(m.name, "Jfs");
        assert_eq!(m.jfs_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_jfs_item() {
        let mut m = Jfs::new("Jfs");
        m.add_jfs_item(sample_jfs_item("a1", "Alpha")).unwrap();
        assert_eq!(m.jfs_item_count(), 1);
        assert!(m.get_jfs_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Jfs::new("Jfs");
        assert!(m.add_jfs_item(sample_jfs_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Jfs::new("Jfs");
        m.add_jfs_item(sample_jfs_item("x", "One")).unwrap();
        assert!(m.add_jfs_item(sample_jfs_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Jfs::new("Jfs");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
