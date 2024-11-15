use std::collections::HashMap;
use std::time::Instant;

use rust_open_addresing_linear_probing::CustomHashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let use_custom = args.iter().any(|arg| arg == "--custom");
    let capacity = args
        .iter()
        .position(|arg| arg == "--capacity")
        .and_then(|i| args.get(i + 1))
        .and_then(|n| n.parse::<usize>().ok())
        .expect("Capacity is required");

    if use_custom {
        let mut map = CustomHashMap::with_capacity(capacity);
        bench("custom", &mut map, capacity);
    } else {
        let mut map = HashMap::with_capacity(capacity);
        bench("std", &mut map, capacity);
    }
}

trait Map<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

impl Map<u8, u8> for HashMap<u8, u8> {
    fn insert(&mut self, key: u8, value: u8) -> Option<u8> {
        self.insert(key, value)
    }
    fn get(&self, key: &u8) -> Option<&u8> {
        self.get(key)
    }
    fn remove(&mut self, key: &u8) -> Option<u8> {
        self.remove(key)
    }
}

impl Map<u8, u8> for CustomHashMap {
    fn insert(&mut self, key: u8, value: u8) -> Option<u8> {
        self.insert(key, value)
    }
    fn get(&self, key: &u8) -> Option<&u8> {
        self.get(*key)
    }
    fn remove(&mut self, key: &u8) -> Option<u8> {
        self.remove(*key)
    }
}

fn bench<M: Map<u8, u8>>(name: &str, map: &mut M, capacity: usize) {
    println!(
        "=== {} HashMap Benchmark (capacity: {}) ===",
        name, capacity
    );
    assert!(capacity > 1000, "Capacity must be greater than 1000");
    let mut total_ops = 0;
    let mut checksum = 0;
    let start = Instant::now();
    let mut operation_timings = Vec::new();

    // Initial insertions
    let t0 = Instant::now();
    let fill_size = (255_usize).min((capacity as f64 * 0.8) as usize);
    for i in 0..fill_size {
        assert!(map.insert(i as u8, ((i * 10) % 255) as u8).is_none());
        assert_eq!(map.get(&(i as u8)), Some(&(((i * 10) % 255) as u8)));
        total_ops += 2;
    }
    operation_timings.push(("Initial insertions", t0.elapsed()));

    // Update existing keys (50% of inserted)
    for i in 0..fill_size / 2 {
        let old = map.insert(i as u8, ((i * 20) % 255) as u8);
        assert!(old.is_some());
        total_ops += 1;
    }

    // Collision handling (keys that hash to same slot)
    for i in (0..fill_size).step_by(8) {
        let key = i as u8;
        let value = ((i * 30) % 255) as u8;
        map.insert(key, value);
        assert_eq!(map.get(&key), Some(&value));
        total_ops += 2;
    }

    // Deletions and probe chain maintenance
    for i in (0..fill_size).step_by(2) {
        let removed = map.remove(&(i as u8));
        assert!(removed.is_some());
        total_ops += 1;
    }

    // Verify probe chains still work after deletions
    for i in (1..fill_size).step_by(2) {
        if let Some(v) = map.get(&(i as u8)) {
            checksum += *v as u32;
        }
        total_ops += 1;
    }

    // Reinsert into deleted slots
    for i in (0..200).step_by(2) {
        map.insert(i as u8, ((i * 40) % 255) as u8);
        total_ops += 1;
    }

    // Long probe sequence with interleaved operations
    let probe_keys = [8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96];
    for &k in &probe_keys {
        map.insert(k, (k as usize * 5 % 255) as u8);
        total_ops += 1;
    }

    // Remove scattered elements
    for &k in &[16, 32, 48, 64, 80, 96] {
        map.remove(&k);
        total_ops += 1;
    }

    // Verify probe chain integrity
    for &k in &[8, 24, 40, 56, 72, 88] {
        assert!(map.get(&k).is_some());
        total_ops += 1;
    }

    // Final stress test: rapid insert/remove cycles
    let mut last_inserted = Vec::new();
    for i in 0..100 {
        let k = (i * 7) % 255;
        map.insert(k as u8, ((i * 50) % 255) as u8);
        last_inserted.push(k as u8);
        if last_inserted.len() > 10 {
            last_inserted.remove(0); // Keep only last 10 inserted keys
        }
        // Verify one of our recently inserted keys still exists
        assert!(last_inserted.iter().any(|k| map.get(k).is_some()));
        total_ops += 2;
    }

    // Calculate final checksum
    for i in 0..255 {
        if let Some(v) = map.get(&(i as u8)) {
            checksum += *v as u32;
        }
        total_ops += 1;
    }

    let total_time = start.elapsed();

    // Print detailed statistics
    println!("\nDetailed Statistics:");
    println!("Total time: {:?}", total_time);
    println!(
        "Operations per second: {:.2}",
        total_ops as f64 / total_time.as_secs_f64()
    );
    println!(
        "Average time per operation: {:?}",
        total_time / total_ops as u32
    );

    println!("\nOperation Breakdown:");
    for (op, time) in operation_timings {
        println!("{}: {:?}", op, time);
    }

    println!("\nSummary:");
    println!("Total operations: {}", total_ops);
    println!("Final checksum: {}", checksum);
    println!("=====================================");
}
