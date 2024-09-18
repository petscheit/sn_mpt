%builtins pedersen range_check poseidon
from starkware.cairo.common.alloc import alloc

from starkware.cairo.common.sponge_as_hash import SpongeHashBuiltin
from starkware.cairo.common.cairo_builtins import HashBuiltin, PoseidonBuiltin
from starkware.cairo.common.dict_access import DictAccess
from starkware.cairo.common.dict import dict_new, dict_update, dict_squash
from src.patricia import patricia_update, patricia_update_constants_new
// from starkware.cairo.common.patricia import patricia_update, patricia_update_constants_new
from starkware.cairo.common.builtin_poseidon.poseidon import (
    poseidon_hash_single,
    poseidon_hash,
    poseidon_hash_many,
)


func main{
    pedersen_ptr: HashBuiltin*,
    range_check_ptr,
    poseidon_ptr: PoseidonBuiltin*,
}() {

    let contract_state_hash = verify_contract_state();
    let state_root = verify_contract(contract_state_hash);
    assert state_root = 0x34e41ac48df28204189050de68200d53a035219260dec46824d009b225866d2;

    return ();
}

func verify_contract{
    pedersen_ptr: HashBuiltin*,
    range_check_ptr,
    poseidon_ptr: PoseidonBuiltin*,
}(contract_state_hash: felt) -> felt {
    alloc_locals;
    
    local class_commitment: felt;
    local contract_root: felt;
    local contract_address: felt;

    %{
        ids.contract_root = int(program_input["contract_root"], 16)
        ids.class_commitment = int(program_input["class_commitment"], 16)
        ids.contract_address = int(program_input["contract_address"], 16)
        initial_dict = {ids.contract_address: ids.contract_state_hash}
        preimage = {
            int(key, 16): tuple(int(value, 16) for value in values)
            for key, values in program_input["contract_proof"].items()
        }
    %}

    let (state_changes_start) = dict_new();
    let state_changes = state_changes_start;

    dict_update{dict_ptr=state_changes}(
        key=contract_address,
        prev_value=contract_state_hash,
        new_value=contract_state_hash
    );

    let (squashed_dict_start, squashed_dict_end) = dict_squash{
        range_check_ptr=range_check_ptr
    }(state_changes_start, state_changes);


    patricia_update{
        hash_ptr=pedersen_ptr,
        range_check_ptr=range_check_ptr,
    } (
        update_ptr=squashed_dict_start,
        n_updates=1,
        height=251,
        prev_root=contract_root,
        new_root=contract_root,
    );

    %{ print("Validated contract_root") %}

    %{
        print("contract_root: ", ids.contract_root)
        print("class_commitment: ", ids.class_commitment)
    %}

    let (hash_chain: felt*) = alloc();
    assert hash_chain[0] = 28355430774503553497671514844211693180464; //STARKNET_STATE_V0
    assert hash_chain[1] = contract_root;
    assert hash_chain[2] = class_commitment;
    
    let (state_root) = poseidon_hash_many(3, hash_chain);

    %{ print("state_root: ", ids.state_root) %}

    return state_root;

}

func verify_contract_state{
    pedersen_ptr: HashBuiltin*,
    range_check_ptr,
}() -> felt {

    alloc_locals;
    local root: felt;
    local height = 251;
    local n_updates: felt;

    %{ 
        ids.n_updates = len(program_input["contract_data"]["slots"]) 
        ids.root = int(program_input["contract_data"]["root"], 16)

        initial_dict = {int(entry["key"], 16): int(entry["value"], 16) for entry in program_input["contract_data"]["slots"]}
        hex_preimage = {}

    %}

    let (state_changes_start) = dict_new();
    let state_changes = state_changes_start;

    apply_contract_state_read_updates{
        range_check_ptr=range_check_ptr,
        dict_ptr=state_changes,
    }(
        n_updates=n_updates,
        i=0
    );

    let (squashed_dict_start, squashed_dict_end) = dict_squash{
        range_check_ptr=range_check_ptr
    }(state_changes_start, state_changes);


    %{
        preimage = {
            int(key, 16): tuple(int(value, 16) for value in values)
            for key, values in hex_preimage.items()
        }
    %}

    patricia_update{
        hash_ptr=pedersen_ptr,
        range_check_ptr=range_check_ptr,
    } (
        update_ptr=squashed_dict_start,
        n_updates=1,
        height=height,
        prev_root=root,
        new_root=root,
    );

    // should be computed via pedersen(class_hash, root, nonce, contract_state_hash_version)
    let contract_state_hash = 0x41C2A6012DA4203B4D5A948ECEA20669183CD3255E934093C62C72D7C8A430F;

    return contract_state_hash;
}

func apply_contract_state_read_updates{
    range_check_ptr,
    dict_ptr: DictAccess*,
} (
    n_updates: felt,
    i: felt
) {
    alloc_locals;

    if (i == n_updates) {
        return ();
    }

    local key: felt;
    local prev_value: felt;
    local new_value: felt;

    %{
        entry = program_input["contract_data"]["slots"][ids.i]
        ids.key = int(entry["key"], 16)
        ids.prev_value = int(entry["value"], 16)
        ids.new_value = int(entry["value"], 16)
        slot_preimage = entry["preimage"]
        hex_preimage = hex_preimage | slot_preimage # add the slot preimage to the preimage
        
    %}

    dict_update{dict_ptr=dict_ptr}(
        key=key,
        prev_value=prev_value,
        new_value=new_value
    );

    return apply_contract_state_read_updates(n_updates=n_updates, i=i+1);
}