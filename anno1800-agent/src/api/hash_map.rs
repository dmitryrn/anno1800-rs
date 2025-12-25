use super::AnnoPtr;

pub struct HashMapPtr {
    pub address: u64,
}

impl AnnoPtr for HashMapPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

impl HashMapPtr {
    pub fn get_entry(&self, key: u32) -> u64 {
        let hash = fnv1a_32(key);
        let bucket_address = self.get_data() + 16 * (hash & self.get_bitmask());
        let hashmap_end_marker = self.get_last();
        unsafe {
            let bucket = HashMapBucketPtr::new(bucket_address);
            let bucket_end_marker = bucket.get_last();
            let mut container = HashMapContainerPtr::new(bucket.get_first());
            loop {
                if container.address == hashmap_end_marker {
                    return 0;
                }
                if container.get_key() == key {
                    return container.get_data();
                }

                container = HashMapContainerPtr::new(container.get_next());

                if container.address == bucket_end_marker {
                    return 0;
                }
            }
        }
    }

    fn get_last(&self) -> u64 {
        self.get(0x0000)
    }

    fn get_data(&self) -> u64 {
        self.get(0x0010)
    }

    fn get_bitmask(&self) -> u64 {
        self.get(0x0028)
    }
}

pub struct HashMapBucketPtr {
    pub address: u64,
}

impl HashMapBucketPtr {
    fn get_last(&self) -> u64 {
        self.get(0x0000)
    }

    fn get_first(&self) -> u64 {
        self.get(0x0008)
    }
}

impl AnnoPtr for HashMapBucketPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

pub struct HashMapContainerPtr {
    pub address: u64,
}

impl HashMapContainerPtr {
    fn get_next(&self) -> u64 {
        self.get(0x0008)
    }

    fn get_key(&self) -> u32 {
        self.get(0x0010)
    }

    fn get_data(&self) -> u64 {
        self.get(0x0018)
    }
}

impl AnnoPtr for HashMapContainerPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

/// https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
fn fnv1a_32(input: u32) -> u64 {
    let bytes = input.to_le_bytes();
    let mut hash = 0xCBF29CE4_84222325u64;

    for byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }

    hash
}

fn fnv1a_16(input: u16) -> u64 {
    let bytes = input.to_le_bytes();
    let mut hash = 0xCBF29CE4_84222325u64;

    for byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }

    hash
}

#[cfg(test)]
mod test {
    use crate::api::{hash_map::fnv1a_16, hash_map::fnv1a_32, ware_type::STEEL_BEAMS};

    #[test]
    fn test_fnv1a_32_steel_beams() {
        assert_eq!(fnv1a_32(STEEL_BEAMS.0) & 0x1ff, 0x12a);
    }

    #[test]
    fn test_fnv1a_16_island_1() {
        assert_eq!(fnv1a_16(0x21c2) & 0x3f, 0x2c);
    }

    #[test]
    fn test_fnv1a_16_island_2() {
        assert_eq!(fnv1a_16(0x2442) & 0x3f, 0x13);
    }
}
