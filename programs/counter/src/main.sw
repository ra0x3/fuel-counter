contract;

abi Counter {
    fn init_counter(value: u32) -> u32;
    fn get_count() -> u32;
    fn increment_counter(value: u32) -> u32;
}

storage {
    counter: u32,
}

impl Counter for Contract {
    fn init_counter(value: u32) -> u32 {
        storage.counter = value;
        value
    }

    fn get_count() -> u32 {
        storage.counter
    }

    fn increment_counter(amount: u32) -> u32 {
        let incremented = storage.counter + amount;
        storage.counter = incremented;
        incremented
    }
}
