#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Vacant,
    Deleted,
    Occupied { key: u8, value: u8 },
}

#[derive(Debug)]
pub struct CustomHashMap {
    entries: Vec<Slot>,
    size: usize,
    capacity: usize,
}

impl Default for CustomHashMap {
    fn default() -> Self {
        let default_capacity = 8;
        Self::with_capacity(default_capacity)
    }
}

impl CustomHashMap {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: vec![Slot::Vacant; capacity],
            size: 0,
            capacity,
        }
    }

    fn hash(&self, key: u8) -> usize {
        (key as usize) % self.capacity
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert(&mut self, key: u8, value: u8) -> Option<u8> {
        let mut current_index = 0;
        while current_index < self.capacity {
            let current_hash = (self.hash(key) + current_index) % self.capacity;
            let current_slot = self.entries[current_hash];
            match current_slot {
                Slot::Vacant => {
                    self.entries[current_hash] = Slot::Occupied { key, value };
                    self.size += 1;
                    return None;
                }
                Slot::Deleted => {
                    self.entries[current_hash] = Slot::Occupied { key, value };
                    self.size += 1;
                    return None;
                }
                Slot::Occupied {
                    key: current_key,
                    value: current_value,
                } => {
                    if current_key == key {
                        self.entries[current_hash] = Slot::Occupied { key, value };
                        return Some(current_value);
                    } else {
                        current_index += 1;
                    }
                }
            }
        }
        None
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: u8) -> Option<&u8> {
        let mut current_index = 0;
        while current_index < self.capacity {
            let current_hash = (self.hash(key) + current_index) % self.capacity;
            let current_slot = &self.entries[current_hash];
            match current_slot {
                Slot::Vacant => return None,
                Slot::Deleted => current_index += 1,
                Slot::Occupied {
                    key: current_key,
                    value: current_value,
                } => {
                    if *current_key == key {
                        return Some(current_value);
                    }
                    current_index += 1;
                }
            }
        }
        None
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    pub fn remove(&mut self, key: u8) -> Option<u8> {
        let mut current_index = 0;
        while current_index < self.capacity {
            let current_hash = (self.hash(key) + current_index) % self.capacity;
            let current_slot = self.entries[current_hash];
            match current_slot {
                Slot::Vacant => return None,
                Slot::Deleted => current_index += 1,
                Slot::Occupied {
                    key: current_key,
                    value,
                } => {
                    if current_key == key {
                        self.entries[current_hash] = Slot::Deleted;
                        self.size -= 1;
                        return Some(value);
                    }
                    current_index += 1;
                }
            }
        }
        None
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_new_should_be_empty() {
        let map = CustomHashMap::default();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn when_insert_new_key_should_return_none() {
        let mut map = CustomHashMap::default();
        assert_eq!(map.insert(1, 10), None);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn when_get_existing_key_should_return_value() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10);
        assert_eq!(map.get(1), Some(&10));
    }

    #[test]
    fn when_get_nonexistent_key_should_return_none() {
        let map = CustomHashMap::default();
        assert_eq!(map.get(1), None);
    }

    #[test]
    fn when_insert_existing_key_should_update_and_return_old() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10);
        assert_eq!(map.insert(1, 20), Some(10));
        assert_eq!(map.get(1), Some(&20));
    }

    #[test]
    fn when_hash_collision_should_probe_to_next_slot() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10); // hash: 1
        map.insert(9, 90); // hash: 1, should probe to 2
        assert_eq!(map.get(1), Some(&10));
        assert_eq!(map.get(9), Some(&90));
    }

    #[test]
    fn when_remove_existing_should_return_value() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10);
        assert_eq!(map.remove(1), Some(10));
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn when_remove_nonexistent_should_return_none() {
        let mut map = CustomHashMap::default();
        assert_eq!(map.remove(1), None);
    }

    #[test]
    fn when_get_through_deleted_should_find_value() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10);
        map.insert(9, 90);
        map.remove(1);
        assert_eq!(map.get(9), Some(&90));
    }

    #[test]
    fn when_insert_after_delete_should_reuse_slot() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10);
        map.remove(1);
        assert_eq!(map.insert(1, 20), None); // treated as new insert
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn when_collision_should_not_update_existing() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10);
        map.insert(9, 90); // collides with 1
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(1), Some(&10)); // unchanged
        assert_eq!(map.get(9), Some(&90)); // probed
    }

    #[test]
    fn when_collision_insert_should_probe_linearly() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10); // hash: 1
        map.insert(9, 90); // hash: 1, goes to 2
        map.insert(17, 170); // hash: 1, goes to 3

        assert_eq!(map.get(1), Some(&10));
        assert_eq!(map.get(9), Some(&90));
        assert_eq!(map.get(17), Some(&170));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn when_collision_remove_middle_should_keep_probe_chain() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10); // hash: 1
        map.insert(9, 90); // hash: 1, goes to 2
        map.insert(17, 170); // hash: 1, goes to 3

        map.remove(9); // middle of chain
        assert_eq!(map.get(17), Some(&170)); // should still find this
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn when_collision_remove_first_should_keep_probe_chain() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10); // hash: 1
        map.insert(9, 90); // hash: 1, goes to 2

        map.remove(1); // first in chain
        assert_eq!(map.get(9), Some(&90)); // should still find this
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn when_collision_insert_after_remove_should_reuse_slot() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10); // hash: 1
        map.insert(9, 90); // hash: 1, goes to 2
        map.remove(1);

        map.insert(17, 170); // hash: 1, should use slot 1
        assert_eq!(map.get(17), Some(&170));
        assert_eq!(map.get(9), Some(&90));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn when_collision_update_should_not_affect_probe_chain() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10); // hash: 1
        map.insert(9, 90); // hash: 1, goes to 2

        assert_eq!(map.insert(1, 100), Some(10)); // update first
        assert_eq!(map.get(9), Some(&90)); // chain intact
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn when_collision_remove_and_reinsert_should_reuse_first_deleted() {
        let mut map = CustomHashMap::default();
        map.insert(1, 10);
        map.insert(9, 90);
        map.insert(17, 170);
        // [Empty, 1, 9, 17]

        map.remove(1);
        // [Empty, Deleted, 9, 17]
        map.remove(9);
        // [Empty, Deleted, Deleted, 17]

        map.insert(25, 250);
        // [Empty, 25, Deleted, 17]

        assert_eq!(map.get(25), Some(&250));
        assert_eq!(map.get(17), Some(&170));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn when_map_full_should_handle_gracefully() {
        let mut map = CustomHashMap::default(); // capacity is 8
        for i in 0..8 {
            map.insert(i, i * 10);
        }
        assert_eq!(map.len(), 8);
        assert_eq!(map.get(0), Some(&0));
        assert_eq!(map.get(7), Some(&70));
    }

    #[test]
    fn when_probe_wraps_around_capacity_should_continue_search() {
        let mut map = CustomHashMap::default();
        map.insert(7, 70); // hash: 7
        map.insert(15, 150); // hash: 7, wraps to 0
        assert_eq!(map.get(15), Some(&150));
    }

    #[test]
    fn when_all_slots_deleted_and_get_nonexistent_should_terminate() {
        let mut map = CustomHashMap::default(); // capacity is 8
                                                // Fill the entire map
        for i in 0..8 {
            map.insert(i, i * 10);
        }
        // Delete all entries
        for i in 0..8 {
            map.remove(i);
        }
        // Now all slots are Deleted (no Vacant slots)
        // Try to get a key that was never in the map
        assert_eq!(map.get(100), None);
    }

    // Advanced
    #[test]
    fn when_insert_delete_insert_same_hash_sequence_should_work() {
        let mut map = CustomHashMap::default();
        // Fill slots 0,1,2
        map.insert(0, 0); // hash: 0
        map.insert(8, 8); // hash: 0, probes to 1
        map.insert(16, 16); // hash: 0, probes to 2

        // Remove middle element
        map.remove(8);
        // Remove first element
        map.remove(0);
        // Insert new element with same hash
        map.insert(24, 24); // hash: 0, should reuse first deleted

        assert_eq!(map.get(16), Some(&16)); // Last original still there
        assert_eq!(map.get(24), Some(&24)); // New insert worked
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn when_wrap_around_with_deletions_should_find_elements() {
        let mut map = CustomHashMap::default(); // capacity 8
        map.insert(7, 7); // hash: 7
        map.insert(15, 15); // hash: 7, wraps to 0
        map.insert(23, 23); // hash: 7, wraps to 1

        map.remove(15); // Delete middle element
        assert_eq!(map.get(23), Some(&23)); // Should still find last element
    }

    #[test]
    fn when_multiple_hash_collisions_with_interleaved_deletions() {
        let mut map = CustomHashMap::default();
        // All these hash to 0
        map.insert(0, 0); // slot 0
        map.insert(8, 8); // slot 1
        map.insert(16, 16); // slot 2
        map.insert(24, 24); // slot 3

        map.remove(8); // Delete from slot 1
        map.remove(16); // Delete from slot 2

        assert_eq!(map.get(24), Some(&24)); // Should still find last element

        map.insert(32, 32); // Should reuse first deleted slot (1)
        assert_eq!(map.get(32), Some(&32));
    }

    #[test]
    fn when_insert_at_capacity_boundary() {
        let mut map = CustomHashMap::default(); // capacity 8
                                                // Fill up to capacity - 1
        for i in 0..7 {
            map.insert(i, i);
        }
        // Insert at last slot
        map.insert(7, 7);
        assert_eq!(map.get(7), Some(&7));

        // Try one more (should handle gracefully even if not optimal)
        map.insert(8, 8);
    }

    #[test]
    fn when_delete_and_reinsert_at_capacity_boundary() {
        let mut map = CustomHashMap::default();
        // Fill completely
        for i in 0..8 {
            map.insert(i, i);
        }
        // Remove last element
        map.remove(7);
        // Insert new element that would hash to last slot
        map.insert(15, 15); // hash: 7
        assert_eq!(map.get(15), Some(&15));
    }

    #[test]
    fn when_long_probe_sequence_with_deletions() {
        let mut map = CustomHashMap::default();
        // Create a long probe sequence
        map.insert(0, 0); // slot 0
        map.insert(8, 8); // slot 1
        map.insert(16, 16); // slot 2
        map.insert(24, 24); // slot 3
        map.insert(32, 32); // slot 4

        // Delete some middle elements
        map.remove(8);
        map.remove(24);

        // Should still find element at end of probe sequence
        assert_eq!(map.get(32), Some(&32));

        // Insert new element that hashes to 0
        map.insert(40, 40);
        assert_eq!(map.get(40), Some(&40));
    }

    #[test]
    fn when_remove_all_and_refill_different_order() {
        let mut map = CustomHashMap::default();
        // First fill
        for i in 0..8 {
            map.insert(i, i);
        }
        // Remove all
        for i in 0..8 {
            map.remove(i);
        }
        // Refill in reverse order
        for i in (0..8).rev() {
            map.insert(i, i * 10);
        }

        assert_eq!(map.len(), 8);
        // Check all values
        for i in 0..8 {
            assert_eq!(map.get(i), Some(&(i * 10)));
        }
    }

    #[test]
    fn when_remove_with_all_slots_deleted_should_terminate() {
        let mut map = CustomHashMap::default();
        // Fill map
        for i in 0..8 {
            map.insert(i, i * 10);
        }
        // Delete all but one
        for i in 0..7 {
            map.remove(i);
        }
        // Try to remove a non-existent key
        assert_eq!(map.remove(100), None);
    }
}
