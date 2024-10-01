#[starknet::contract]
mod TestContract {
    use starknet::ContractAddress;

    #[storage]
    struct Storage {
        supply: u256,
    }

    #[constructor]
    fn constructor(ref self: ContractState, initial_supply: u256) {
        self.supply.write(initial_supply);
    }
}