use crate::shard::Shard;
use std::sync::atomic::{AtomicU32, Ordering};
use lru::LruCache;
use log::{info, warn, error, debug};

pub struct ResourceManager {
    cpu: AtomicU32,
    memory: AtomicU32,
    bandwidth: AtomicU32,
    resource_cache: LruCache<String, u32>,
}

impl ResourceManager {
    /// Creates a new ResourceManager with predefined resources and a cache.
    pub fn new(cpu: u32, memory: u32, bandwidth: u32) -> Self {
        let resource_cache = LruCache::new(100);
        ResourceManager {
            cpu: AtomicU32::new(cpu),
            memory: AtomicU32::new(memory),
            bandwidth: AtomicU32::new(bandwidth),
            resource_cache,
        }
    }

    /// Attempts to allocate resources to a shard, retrying if necessary.
    pub fn allocate_resources(&self, shard: &mut Shard) -> Result<(), &'static str> {
        let (cpu_required, memory_required, bandwidth_required) = shard.get_resource_requirements();

        if self.try_allocate_resources(cpu_required, memory_required, bandwidth_required).is_ok() {
            shard.allocate_resources(cpu_required, memory_required, bandwidth_required);
            info!("Resources successfully allocated to shard");
            Ok(())
        } else {
            warn!("Insufficient resources to allocate to shard");
            Err("Insufficient resources")
        }
    }

    /// Tries to allocate resources to a shard based on its requirements.
    fn try_allocate_resources(&self, cpu_required: u32, memory_required: u32, bandwidth_required: u32) -> Result<(), &'static str> {
        let mut current_cpu = self.cpu.load(Ordering::Acquire);
        let mut current_memory = self.memory.load(Ordering::Acquire);
        let mut current_bandwidth = self.bandwidth.load(Ordering::Acquire);

        while cpu_required <= current_cpu && memory_required <= current_memory && bandwidth_required <= current_bandwidth {
            if self.cpu.compare_exchange(current_cpu, current_cpu - cpu_required, Ordering::AcqRel, Ordering::Relaxed).is_ok()
                && self.memory.compare_exchange(current_memory, current_memory - memory_required, Ordering::AcqRel, Ordering::Relaxed).is_ok()
                && self.bandwidth.compare_exchange(current_bandwidth, current_bandwidth - bandwidth_required, Ordering::AcqRel, Ordering::Relaxed).is_ok() {

                self.resource_cache.put("CPU".to_string(), current_cpu - cpu_required);
                self.resource_cache.put("Memory".to_string(), current_memory - memory_required);
                self.resource_cache.put("Bandwidth".to_string(), current_bandwidth - bandwidth_required);
                debug!("Resource cache updated after allocation");
                return Ok(());
            }

            // Update current values for the next iteration
            current_cpu = self.cpu.load(Ordering::Acquire);
            current_memory = self.memory.load(Ordering::Acquire);
            current_bandwidth = self.bandwidth.load(Ordering::Acquire);
        }

        Err("Failed to allocate resources")
    }

    /// Releases resources back to the manager after they are no longer in use.
    pub fn release_resources(&self, cpu: u32, memory: u32, bandwidth: u32) {
        self.cpu.fetch_add(cpu, Ordering::AcqRel);
        self.memory.fetch_add(memory, Ordering::AcqRel);
        self.bandwidth.fetch_add(bandwidth, Ordering::AcqRel);
        info!("Resources released: CPU={}, Memory={}, Bandwidth={}", cpu, memory, bandwidth);
    }

    /// Optimize the overall resource usage of the system.
    pub fn optimize_resource_usage(&self) {
        // Example optimization logic: reclaim resources from the cache if they are not used
        if let Some((key, value)) = self.resource_cache.pop_lru() {
            match key.as_str() {
                "CPU" => self.cpu.fetch_add(value, Ordering::AcqRel),
                "Memory" => self.memory.fetch_add(value, Ordering::AcqRel),
                "Bandwidth" => self.bandwidth.fetch_add(value, Ordering::AcqRel),
                _ => error!("Unknown resource key in cache: {}", key),
            }
            info!("Optimized resources by reclaiming from cache: {}={}", key, value);
        }
    }

    /// Logs the current state of resources for monitoring and auditing.
    pub fn log_resource_state(&self) {
        let cpu = self.cpu.load(Ordering::Acquire);
        let memory = self.memory.load(Ordering::Acquire);
        let bandwidth = self.bandwidth.load(Ordering::Acquire);

        info!("Current resource state: CPU={}, Memory={}, Bandwidth={}", cpu, memory, bandwidth);
    }
}
