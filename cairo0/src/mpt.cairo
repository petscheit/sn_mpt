%builtins pedersen range_check poseidon

from starkware.cairo.common.sponge_as_hash import SpongeHashBuiltin
from starkware.cairo.common.cairo_builtins import HashBuiltin, PoseidonBuiltin
from starkware.cairo.common.dict_access import DictAccess
from starkware.cairo.common.dict import dict_new, dict_update, dict_squash
from starkware.cairo.common.patricia import patricia_update, patricia_update_constants_new
from starkware.cairo.common.patricia_with_poseidon import patricia_update_using_update_constants
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

    alloc_locals;

    local n_updates: felt;

    %{
        # Set initial dictionary.
        n_updates = len(program_input["leaf_updates"])
        prev_root = int(program_input["pre_root"], 16)
        new_root = int(program_input["post_root"], 16)
        initial_dict = {int(entry["key"], 16): int(entry["pre_value"], 16) for entry in program_input["leaf_updates"]}
        ids.n_updates = n_updates
        preimage = {
            int(key, 16): tuple(int(value, 16) for value in values)
            for key, values in program_input["preimage"].items()
        }
    %}


    let (state_changes_start) = dict_new();
    let state_changes = state_changes_start;

    apply_dict_updates{
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
        vm_enter_scope(dict(
            preimage=preimage,
            n_updates=n_updates,
            prev_root=prev_root,
            new_root=new_root,
        ))
    %}

    verify_trie_update_poseidon{
        range_check_ptr=range_check_ptr,
        poseidon_ptr=poseidon_ptr,
        updated_state=squashed_dict_start
    }();

    // verify_trie_update_pedersen{
    //     range_check_ptr=range_check_ptr,
    //     pedersen_ptr=pedersen_ptr,
    //     updated_state=squashed_dict_start
    // }();
    %{ vm_exit_scope() %}

    return ();
}

func verify_trie_update_poseidon{
    range_check_ptr,
    poseidon_ptr: PoseidonBuiltin*,
    updated_state: DictAccess*
}() {
    alloc_locals;
    local height = 251;
    local n_updates: felt;
    local prev_root: felt;
    local new_root: felt;

    %{
        ids.n_updates = n_updates
        ids.prev_root = prev_root
        ids.new_root = new_root
    %}

    let (consts) = patricia_update_constants_new();
    patricia_update_using_update_constants{
        poseidon_ptr=poseidon_ptr,
        range_check_ptr=range_check_ptr,
    } (
        patricia_update_constants=consts,
        update_ptr=updated_state,
        n_updates=n_updates,
        height=height,
        prev_root=prev_root,
        new_root=new_root,
    );

    return ();
}

func verify_trie_update_pedersen{
    range_check_ptr,
    pedersen_ptr: HashBuiltin*,
    updated_state: DictAccess*
}() {
    alloc_locals;
    local height = 251;
    local n_updates: felt;
    local prev_root: felt;
    local new_root: felt;

    %{
        ids.n_updates = n_updates
        ids.prev_root = prev_root
        ids.new_root = new_root
    %}

    patricia_update{
        hash_ptr=pedersen_ptr,
        range_check_ptr=range_check_ptr,
    }(
        update_ptr=updated_state,
        n_updates=n_updates,
        height=height,
        prev_root=prev_root,
        new_root=new_root
    );

    return ();
}

func apply_dict_updates{
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
        entry = program_input["leaf_updates"][ids.i]
        ids.key = int(entry["key"], 16)
        ids.prev_value = int(entry["pre_value"], 16)
        ids.new_value = int(entry["post_value"], 16)
    %}

    dict_update{dict_ptr=dict_ptr}(
        key=key,
        prev_value=prev_value,
        new_value=new_value
    );

    return apply_dict_updates(n_updates=n_updates, i=i+1);
}