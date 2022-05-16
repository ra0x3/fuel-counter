contract;

abi Counter {
    fn init_counter(value: u64) -> u64;
    fn get_count() -> u64;
    fn increment_counter(value: u64) -> u64;
}

storage {
    counter: u64,
}

impl Counter for Contract {
    fn init_counter(value: u64) -> u64 {
        storage.counter = value;
        value
    }

    fn get_count() -> u64 {
        storage.counter
    }

    fn increment_counter(amount: u64) -> u64 {
        let incremented = storage.counter + amount;
        storage.counter = incremented;
        incremented
    }
}
