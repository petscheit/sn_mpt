%builtins pedersen range_check bitwise poseidon
from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.hash import hash2
from starkware.cairo.common.sponge_as_hash import SpongeHashBuiltin
from starkware.cairo.common.registers import get_label_location
from starkware.cairo.common.bitwise import bitwise_and
from starkware.cairo.common.cairo_builtins import HashBuiltin, PoseidonBuiltin, BitwiseBuiltin
from starkware.cairo.common.dict_access import DictAccess
from starkware.cairo.common.dict import dict_new, dict_update, dict_squash
from starkware.cairo.common.builtin_poseidon.poseidon import (
    poseidon_hash_single,
    poseidon_hash,
    poseidon_hash_many,
)

func main{
    pedersen_ptr: HashBuiltin*,
    range_check_ptr,
    bitwise_ptr: BitwiseBuiltin*,
    poseidon_ptr: PoseidonBuiltin*,
}() {

    alloc_locals;
    let pow2_array: felt* = pow2alloc252();

    with pow2_array {
        let value = verify_proof(0x34e41ac48df28204189050de68200d53a035219260dec46824d009b225866d2, 0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d, 0x3b92238d3ae057fa455c2e182a43661b99f45f4499473d977ec1009a9e805ed);
    }

    %{ print("value:", ids.value) %}

    return ();
}

func verify_proof{
    pedersen_ptr: HashBuiltin*,
    bitwise_ptr: BitwiseBuiltin*,
    poseidon_ptr: PoseidonBuiltin*,
    pow2_array: felt*,
}(state_commitment: felt, contract_address: felt, storage_address: felt) -> felt {
    alloc_locals;
   
    // Compute contract_root
    %{ 
        nodes = program_input["contract_data"]["storage_proof"][::-1]
        #print("nodes: ", nodes)
        vm_enter_scope(dict(nodes=nodes)) 
    %}
    let (contract_state_nodes, contract_state_nodes_len) = load_nodes();
    let (contract_root, value) = traverse(contract_state_nodes, contract_state_nodes_len, storage_address);
    %{ vm_exit_scope() %}

    return 1;
}

func traverse{
    pedersen_ptr: HashBuiltin*,
    bitwise_ptr: BitwiseBuiltin*,
    pow2_array: felt*,
}(nodes: felt**, n_nodes: felt, expected_path: felt) -> (root: felt, value: felt) {

    let leaf = [nodes];
    let leaf_hash = hash_edge_node(leaf);

    let path = leaf[1];
    let path_length_pow2 = pow2_array[leaf[2]];

    
    let (root, path) = traverse_inner(nodes + 1, n_nodes, expected_path, leaf_hash, path, path_length_pow2);

    assert path = expected_path;
    return (root=root, value=leaf[0]);
}

func traverse_inner{
    pedersen_ptr: HashBuiltin*,
    bitwise_ptr: BitwiseBuiltin*,
    pow2_array: felt*,
    
}(nodes: felt**, n_nodes: felt, expected_path: felt, hash_value: felt, path: felt, path_length_pow2: felt) -> (root: felt, path: felt) {
    alloc_locals;

    tempvar path_length = path_length_pow2;
    tempvar hash = hash_value;
    tempvar current_path = path;
    tempvar counter = 0;


    loop:
    let i = [ap - 1];
    let path = [ap - 2];
    let hash = [ap - 3];
    let path_length = [ap - 4];

    %{ memory[ap] = 1 if ids.i == ids.n_nodes - 1 else 0 %}
    jmp end_loop if [ap] != 0, ap++;

    %{ memory[ap] = nodes_types[ids.i + 1] %}
    jmp edge_node if [ap] != 0, ap++;

    %{
        print("i: ", ids.i)
        print("hash: ", ids.hash)
        print("left: ", memory[memory[ids.nodes + ids.i]])
        print("right: ", memory[memory[ids.nodes + ids.i] + 1])
    %}

    assert bitwise_ptr[i].x = expected_path;
    assert bitwise_ptr[i].y = current_path;
    let bitwise_res = bitwise_ptr[i].x_and_y;
    if(bitwise_res == 0) {
        assert hash = [[nodes + i] + 1];
        [ap] = path, ap++;
    } else {
        assert hash = [[nodes + i]];
        [ap] = path + path_length_pow2, ap++;
    }
    let next_path = [ap - 1];

    assert pedersen_ptr.x = [[nodes + i]];
    assert pedersen_ptr.y = [[nodes + i] + 1];
    let next_hash = pedersen_ptr.result;
    let next_path_length = path_length * 2;
    tempvar pedersen_ptr = pedersen_ptr + HashBuiltin.SIZE;
    [ap] = next_path_length, ap++;
    [ap] = next_hash, ap++;
    [ap] = next_path, ap++;
    [ap] = i + 1, ap++;
    jmp loop; 

    edge_node:
    %{ print("found edge node") %}
    assert hash = [[nodes + i]];
    // let next_path = [[nodes + i] + 1] * path_length;
    // let next_path_length = path_length * pow2_array[[[nodes + i] + 2]];
    // assert pedersen_ptr[i].x = [[nodes + i]];
    // assert pedersen_ptr[i].y = [[nodes + i] + 1];
    // let next_hash = [pedersen_ptr + i].result + [[nodes + i] + 2];


    [ap] = path_length, ap++;
    [ap] = hash, ap++;
    [ap] = path, ap++;
    [ap] = i + 1, ap++;
    jmp loop;

    end_loop:

    let pedersen_ptr = pedersen_ptr + n_nodes * HashBuiltin.SIZE;
    let bitwise_ptr = bitwise_ptr + n_nodes * BitwiseBuiltin.SIZE;
    return (root=hash, path=current_path);

}

func hash_binary_node{
    pedersen_ptr: HashBuiltin*,
}(node: felt*) -> felt {
    let (node_hash) = hash2{hash_ptr=pedersen_ptr}(node[0], node[1]);
    return node_hash;
}

func hash_edge_node{
    pedersen_ptr: HashBuiltin*,
}(node: felt*) -> felt {
    let (node_hash) = hash2{hash_ptr=pedersen_ptr}(node[0], node[1]);
    return node_hash + node[2];
}

func load_nodes() -> (nodes: felt**, len: felt) {
    alloc_locals;
    let (nodes: felt**) = alloc();
    local len: felt;
   %{ 
        nodes_types = []
        ids.len = len(nodes)
        for i in range(len(nodes)):
            nodes_types.append(len(nodes[i]) % 2) # 0 for binary, 1 for edge
            for j in range(len(nodes[i])):
                nodes[i][j] = int(nodes[i][j],16)
        segments.write_arg(ids.nodes, nodes)
    %}
    return (nodes=nodes, len=len);
}

// func traverse_inner_loop{
//     pedersen_ptr: HashBuiltin*,
//     bitwise_ptr: BitwiseBuiltin*,
//     pow2_array: felt*,
// }(nodes: felt**, n_nodes: felt, expected_path: felt, hash_value: felt, path: felt, path_length_pow2: felt) -> (root: felt, path: felt) {
//     alloc_locals;
//     tempvar hash = hash_value;
//     tempvar current_path = path;
//     tempvar path_length = path_length_pow2;
//     tempvar i = n_nodes;

//     loop:
//     let i = [ap - 1];
//     let path_length_pow2 = [ap - 2];
//     let current_path = [ap - 3];
//     let hash = [ap - 4];

//     %{ memory[ap] = 1 if ids.i == 0 else 0 %}
//     jmp end_loop if [ap] != 0, ap++;

//     %{ memory[ap] = node_types[ids.i] %}
//     jmp edge_node if [ap] != 0, ap++;

//     // binary_node:
//     assert bitwise_ptr[n_nodes - i].x = expected_path;
//     assert bitwise_ptr[n_nodes - i].y = current_path;
//     let result = bitwise_ptr[n_nodes - i].x_and_y;
//     %{
//         memory[ap] = nodes[ids.i][0]
//         memory[ap+1] = nodes[ids.i][1]
//     %}
//     ap += 2;

//     let left = [ap - 2];
//     let right = [ap - 1];

//     if(result == 0) {
//         assert hash = left;
//         [ap] = path, ap++;
//     } else {
//         assert hash = right;
//         [ap] = path + path_length, ap++;
//     }

//     let new_path = [ap - 1];
//     let idx = n_nodes - i;
//     %{ print("idx: ", ids.idx) %}
//     assert pedersen_ptr[n_nodes - i].x = left;
//     assert pedersen_ptr[n_nodes - i].y = right;
//     let res = pedersen_ptr[n_nodes - i].result;
//     [ap] = res, ap++;
//     [ap] = new_path, ap++;
//     [ap] = path_length * 2, ap++;
//     [ap] = i - 1, ap++;

//     jmp loop;

//     edge_node:
//      %{
//         memory[ap] = nodes[ids.i][0]
//         memory[ap+1] = nodes[ids.i][1]
//         memory[ap+2] = nodes[ids.i][2]
//     %}
//     ap += 3;

//     let len = [ap - 1];
//     let edge_path = [ap - 2];
//     let child = [ap - 3];
//     assert pedersen_ptr[n_nodes - i].x = child;
//     assert pedersen_ptr[n_nodes - i].y = edge_path;
//     assert hash = pedersen_ptr[n_nodes - i].result + len;
//     [ap] = hash, ap++;
//     [ap] = edge_path * path_length, ap++;
//     [ap] = path_length * pow2_array[len], ap++;
//     [ap] = i - 1, ap++;
//     jmp loop;

//     end_loop:
//     assert 1 = 1;

//     let bitwise_ptr = bitwise_ptr + n_nodes * BitwiseBuiltin.SIZE;
//     let pedersen_ptr = pedersen_ptr + n_nodes * HashBuiltin.SIZE;

//     return (root=hash, path=current_path);
   
//     // let node = nodes[n_nodes - 1];
//     // %{ memory[ap] = nodes_types[ids.n_nodes - 1] %}
//     // jmp edge_node if [ap] != 0, ap++;

//     // // binary_node:
//     // let (result) = bitwise_and(expected_path, path_length_pow2);
//     // local new_path: felt;
//     // if(result == 0) {
//     //     assert hash_value = node[0];
//     //     new_path = path;
//     // } else {
//     //     assert hash_value = node[1];
//     //     new_path = path + path_length_pow2;
//     // }
//     // let next_path_length_pow2 = path_length_pow2 * 2;
//     // let next_hash = hash_binary_node(node);
    
//     // return traverse_inner(n_nodes - 1, expected_path, next_hash, new_path, next_path_length_pow2);

//     // edge_node:
//     // assert hash_value = node[0];
//     // let next_path = node[1] * path_length_pow2;
//     // let next_path_length_pow2 = path_length_pow2 * pow2_array[node[2]];
//     // let next_hash = hash_edge_node(node);

//     // return traverse_inner(n_nodes - 1, expected_path, next_hash, next_path, next_path_length_pow2);
// }


// func main{
//     pedersen_ptr: HashBuiltin*,
//     range_check_ptr,
//     bitwise_ptr: BitwiseBuiltin*,
//     poseidon_ptr: PoseidonBuiltin*,
// }() {
//     alloc_locals;

//     let (values: felt**) = alloc();
//     local values_len: felt;

//     %{
//         values = [
//             [
//                 0x68d0f40b5b03dbdc143f62b7b919e860fcae6c40f56d40aee0596cdb96ba92b,
//                 0x36f0b3cc543026b655f3d8f8deeb693e92411bf7aa15dcb0cf9e4fad87e53f1
//             ],
//             [
//                 0x5ad7108ab389e100f777137526a8f89eb9f285fee621cc793b029a218667c77,
//                 0x88f9bbe380a1712c1c94a268b043cff810c04f16e63fb72ca4cdfc0ce4eee8
//             ],
//             [
//                 0x13abf13f9bdacc7cbf67452c8de93a0a05a5e7956ec747ccd8c35f1dcffc655,
//                 0x45a42aff6707b02e813d3aab7758807a1e8cf31b14d4c3ae0856fcfbe385bf3
//             ],
//             [
//                 0x1f9c26f99e60bb05b2a377571b2236978615a37f843177e44f83bddc9abc5d4,
//                 0x48dd1c8ca70901b45d8acea63bb65f3145eb327d68ea7ce781bc0141c4fae35
//             ],
//             [
//                 0x2a047a8e06554a17c69c152b2dfec4152bc695504fc247c6566ca6e0ca4900c,
//                 0x3c0346567477cf8d8a96244adc306ba45b55f3f5f279525c3d49f207513887c
//             ],
//             [
//                 0x18542e9c9a649816d7949d7339c05865cd627eeec5206880328d412b47f9622,
//                 0x3251e889d8b9ca67950aea52a14b78f6900c8613354729146931bcd3a880a2b
//             ],
//             [
//                 0x61b4c1d6280ead308c921ad947114b750fd8ac04eb270d4c6fbc01bd6cb9a99,
//                 0x684558eb585a140d0e60654f5b82cc2df225b70156957b93f95f89338ae8c3f
//             ],
//             [
//                 0x5a42cd403c852c0633c0c290df0e7e354831225737c168e043e79f9935c5e7b,
//                 0x5822e9f306071f9cb465162c2f7f8c6b52f3d4582374c4d975d76b70d24db67
//             ],
//             [
//                 0x452909d162b1ca26042c9045a118cf5aafc16f1ba5b2021d1b2d60396e4cfe7,
//                 0x3dad9c07dd4e5c07d09ab5c1e375565c678a1c509d8304bda7e4c384ec98a39
//             ],
//             [
//                 0x6119a822cc0120ffe7d25dfb17f73372adafcd5acc92160e5faa7054e9fbaf1,
//                 0x5a3299b2eac3e67c83a5c80ae19da07513f34c44916b87e73652044e730ee5d
//             ],
//             [
//                 0x4c05464bb417d3f770eb1438a0baed6fe3b87d2eae365032fbdab60361440b2,
//                 0x17e11e61095a8d794fa647b7470e60bd641e59bdaa465ed3d6853303f4e04f1
//             ],
//             [
//                 0x4218d6204d2c0533cbdc533c6fddc637b523d766c55d1c042812e36565e88f6,
//                 0x628911e37c0af7ecd9dde97327bc606b5f02f4324cb6a6b3a23b404f9af14ed
//             ],
//             [
//                 0x51998c2e8a53e52a8dad3d392d68583bb0ea6bfede23a75d6755979fba54cba,
//                 0x469d050f9bb99c161671a9877075441c86c82e04e7b3d5cf9157d1149d490f8
//             ],
//             [
//                 0x1e09c7004d3f2a036374e7688892d9f9903d547a53e233e22373f172f2ae150,
//                 0x18d4577c2903b6c966b8adbaeaea540b5ce804697441a9f64958004c56c89a6
//             ],
//             [
//                 0x427aa9891cb7eb3bd5b0d117741a262d9a220e90c4b33c54e5dba8f72824186,
//                 0x5cf734f807c6a284e1a29562dde710c551d2b085825582335c15d13a96681d8
//             ],
//             [
//                 0x5d366c38109a2cab92dbba744935de39617164e7884666e3a88367901401178,
//                 0x26b487ce8efe519d27049c3851a573d1fc16ff8352186f35a6e9934e6810fa5
//             ],
//             [
//                 0x76a014b75e045e1af6a9a98860f4f0f98d222403e77c008eeedd21109a55e54,
//                 0x39faf31366a101af96276ff0a626c92d26e08c5f767b15fea5609ae47950986
//             ],
//             [
//                 0x599ef137c74c7226636f07dd6154525b59c4499813e5deb96fb7c0151a3c2f5,
//                 0x48e6e159b6c64dbcbac33e480d32327ad1f10a58e37b290fc3a8d0d01d78543
//             ],
//             [
//                 0x6933e582797103e657bb4a81f107aba20761dcec7b8eb7a6f9b6616384362ea,
//                 0x15e52094f073718f4c45ede0dc02e327a3a738aa154f5629132a037481b3efe
//             ],
//             [
//                 0x62635d2e3810f9b1f646953de7e1f28e70a32143d77a07abbd7501f4e067f5,
//                 0x311a46d3b1c5c69f5245545a9c38dcf7f9b130a15160e89d8afc18b595e8ebb
//             ],
//             [
//                 0x43c33c1937564800000,
//                 0x38d3ae057fa455c2e182a43661b99f45f4499473d977ec1009a9e805ed,
//                 0xe7
//             ]
//         ]
//         values = list(reversed(values))

//         ids.values_len = 3
//     %}

    // local expected_path = 0x3b92238d3ae057fa455c2e182a43661b99f45f4499473d977ec1009a9e805ed;
    // tempvar path = 0x38d3ae057fa455c2e182a43661b99f45f4499473d977ec1009a9e805ed;
    // tempvar i = 0;

    // loop:
    // let i = [ap - 1];

    // %{ memory[ap] = 1 if ids.values_len == ids.i else 0 %}
    // jmp end_loop if [ap] != 0, ap++;

    // %{ memory[ap] = node_types[ids.i] %}
    // jmp edge_node if [ap] != 0, ap++;

    // assert bitwise_ptr[i].x = x;
    // assert bitwise_ptr[i].y = y;
    // let result = bitwise_ptr[i].x_and_y;
    // %{
    //     print("found binary node")
    //     memory[ap] = values[ids.i][0]
    //     memory[ap+1] = values[ids.i][1]
    // %}

    // ap += 2;

    // let x = [ap - 2];
    // let y = [ap - 1];
    
    // assert pedersen_ptr[i].x = x;
    // assert pedersen_ptr[i].y = y;
    // let result = pedersen_ptr[i].result;

    // [ap] = i + 1, ap++;
    // jmp loop; 

    // edge_node:

    // %{ print("found edge node") %}


    // [ap] = i + 1, ap++;
    // jmp loop;

    // end_loop:

    // let pedersen_ptr = pedersen_ptr + values_len * HashBuiltin.SIZE;
    // return ();

    

// }

func pow2alloc252() -> (array: felt*) {
    let (data_address) = get_label_location(data);
    return (data_address,);

    data:
    dw 0x1;
    dw 0x2;
    dw 0x4;
    dw 0x8;
    dw 0x10;
    dw 0x20;
    dw 0x40;
    dw 0x80;
    dw 0x100;
    dw 0x200;
    dw 0x400;
    dw 0x800;
    dw 0x1000;
    dw 0x2000;
    dw 0x4000;
    dw 0x8000;
    dw 0x10000;
    dw 0x20000;
    dw 0x40000;
    dw 0x80000;
    dw 0x100000;
    dw 0x200000;
    dw 0x400000;
    dw 0x800000;
    dw 0x1000000;
    dw 0x2000000;
    dw 0x4000000;
    dw 0x8000000;
    dw 0x10000000;
    dw 0x20000000;
    dw 0x40000000;
    dw 0x80000000;
    dw 0x100000000;
    dw 0x200000000;
    dw 0x400000000;
    dw 0x800000000;
    dw 0x1000000000;
    dw 0x2000000000;
    dw 0x4000000000;
    dw 0x8000000000;
    dw 0x10000000000;
    dw 0x20000000000;
    dw 0x40000000000;
    dw 0x80000000000;
    dw 0x100000000000;
    dw 0x200000000000;
    dw 0x400000000000;
    dw 0x800000000000;
    dw 0x1000000000000;
    dw 0x2000000000000;
    dw 0x4000000000000;
    dw 0x8000000000000;
    dw 0x10000000000000;
    dw 0x20000000000000;
    dw 0x40000000000000;
    dw 0x80000000000000;
    dw 0x100000000000000;
    dw 0x200000000000000;
    dw 0x400000000000000;
    dw 0x800000000000000;
    dw 0x1000000000000000;
    dw 0x2000000000000000;
    dw 0x4000000000000000;
    dw 0x8000000000000000;
    dw 0x10000000000000000;
    dw 0x20000000000000000;
    dw 0x40000000000000000;
    dw 0x80000000000000000;
    dw 0x100000000000000000;
    dw 0x200000000000000000;
    dw 0x400000000000000000;
    dw 0x800000000000000000;
    dw 0x1000000000000000000;
    dw 0x2000000000000000000;
    dw 0x4000000000000000000;
    dw 0x8000000000000000000;
    dw 0x10000000000000000000;
    dw 0x20000000000000000000;
    dw 0x40000000000000000000;
    dw 0x80000000000000000000;
    dw 0x100000000000000000000;
    dw 0x200000000000000000000;
    dw 0x400000000000000000000;
    dw 0x800000000000000000000;
    dw 0x1000000000000000000000;
    dw 0x2000000000000000000000;
    dw 0x4000000000000000000000;
    dw 0x8000000000000000000000;
    dw 0x10000000000000000000000;
    dw 0x20000000000000000000000;
    dw 0x40000000000000000000000;
    dw 0x80000000000000000000000;
    dw 0x100000000000000000000000;
    dw 0x200000000000000000000000;
    dw 0x400000000000000000000000;
    dw 0x800000000000000000000000;
    dw 0x1000000000000000000000000;
    dw 0x2000000000000000000000000;
    dw 0x4000000000000000000000000;
    dw 0x8000000000000000000000000;
    dw 0x10000000000000000000000000;
    dw 0x20000000000000000000000000;
    dw 0x40000000000000000000000000;
    dw 0x80000000000000000000000000;
    dw 0x100000000000000000000000000;
    dw 0x200000000000000000000000000;
    dw 0x400000000000000000000000000;
    dw 0x800000000000000000000000000;
    dw 0x1000000000000000000000000000;
    dw 0x2000000000000000000000000000;
    dw 0x4000000000000000000000000000;
    dw 0x8000000000000000000000000000;
    dw 0x10000000000000000000000000000;
    dw 0x20000000000000000000000000000;
    dw 0x40000000000000000000000000000;
    dw 0x80000000000000000000000000000;
    dw 0x100000000000000000000000000000;
    dw 0x200000000000000000000000000000;
    dw 0x400000000000000000000000000000;
    dw 0x800000000000000000000000000000;
    dw 0x1000000000000000000000000000000;
    dw 0x2000000000000000000000000000000;
    dw 0x4000000000000000000000000000000;
    dw 0x8000000000000000000000000000000;
    dw 0x10000000000000000000000000000000;
    dw 0x20000000000000000000000000000000;
    dw 0x40000000000000000000000000000000;
    dw 0x80000000000000000000000000000000;
    dw 0x100000000000000000000000000000000;
    dw 0x200000000000000000000000000000000;
    dw 0x400000000000000000000000000000000;
    dw 0x800000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000;
    dw 0x2000000000000000000000000000000000;
    dw 0x4000000000000000000000000000000000;
    dw 0x8000000000000000000000000000000000;
    dw 0x10000000000000000000000000000000000;
    dw 0x20000000000000000000000000000000000;
    dw 0x40000000000000000000000000000000000;
    dw 0x80000000000000000000000000000000000;
    dw 0x100000000000000000000000000000000000;
    dw 0x200000000000000000000000000000000000;
    dw 0x400000000000000000000000000000000000;
    dw 0x800000000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000000;
    dw 0x2000000000000000000000000000000000000;
    dw 0x4000000000000000000000000000000000000;
    dw 0x8000000000000000000000000000000000000;
    dw 0x10000000000000000000000000000000000000;
    dw 0x20000000000000000000000000000000000000;
    dw 0x40000000000000000000000000000000000000;
    dw 0x80000000000000000000000000000000000000;
    dw 0x100000000000000000000000000000000000000;
    dw 0x200000000000000000000000000000000000000;
    dw 0x400000000000000000000000000000000000000;
    dw 0x800000000000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000000000;
    dw 0x2000000000000000000000000000000000000000;
    dw 0x4000000000000000000000000000000000000000;
    dw 0x8000000000000000000000000000000000000000;
    dw 0x10000000000000000000000000000000000000000;
    dw 0x20000000000000000000000000000000000000000;
    dw 0x40000000000000000000000000000000000000000;
    dw 0x80000000000000000000000000000000000000000;
    dw 0x100000000000000000000000000000000000000000;
    dw 0x200000000000000000000000000000000000000000;
    dw 0x400000000000000000000000000000000000000000;
    dw 0x800000000000000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000000000000;
    dw 0x2000000000000000000000000000000000000000000;
    dw 0x4000000000000000000000000000000000000000000;
    dw 0x8000000000000000000000000000000000000000000;
    dw 0x10000000000000000000000000000000000000000000;
    dw 0x20000000000000000000000000000000000000000000;
    dw 0x40000000000000000000000000000000000000000000;
    dw 0x80000000000000000000000000000000000000000000;
    dw 0x100000000000000000000000000000000000000000000;
    dw 0x200000000000000000000000000000000000000000000;
    dw 0x400000000000000000000000000000000000000000000;
    dw 0x800000000000000000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000000000000000;
    dw 0x2000000000000000000000000000000000000000000000;
    dw 0x4000000000000000000000000000000000000000000000;
    dw 0x8000000000000000000000000000000000000000000000;
    dw 0x10000000000000000000000000000000000000000000000;
    dw 0x20000000000000000000000000000000000000000000000;
    dw 0x40000000000000000000000000000000000000000000000;
    dw 0x80000000000000000000000000000000000000000000000;
    dw 0x100000000000000000000000000000000000000000000000;
    dw 0x200000000000000000000000000000000000000000000000;
    dw 0x400000000000000000000000000000000000000000000000;
    dw 0x800000000000000000000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000000000000000000;
    dw 0x2000000000000000000000000000000000000000000000000;
    dw 0x4000000000000000000000000000000000000000000000000;
    dw 0x8000000000000000000000000000000000000000000000000;
    dw 0x10000000000000000000000000000000000000000000000000;
    dw 0x20000000000000000000000000000000000000000000000000;
    dw 0x40000000000000000000000000000000000000000000000000;
    dw 0x80000000000000000000000000000000000000000000000000;
    dw 0x100000000000000000000000000000000000000000000000000;
    dw 0x200000000000000000000000000000000000000000000000000;
    dw 0x400000000000000000000000000000000000000000000000000;
    dw 0x800000000000000000000000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000000000000000000000;
    dw 0x2000000000000000000000000000000000000000000000000000;
    dw 0x4000000000000000000000000000000000000000000000000000;
    dw 0x8000000000000000000000000000000000000000000000000000;
    dw 0x10000000000000000000000000000000000000000000000000000;
    dw 0x20000000000000000000000000000000000000000000000000000;
    dw 0x40000000000000000000000000000000000000000000000000000;
    dw 0x80000000000000000000000000000000000000000000000000000;
    dw 0x100000000000000000000000000000000000000000000000000000;
    dw 0x200000000000000000000000000000000000000000000000000000;
    dw 0x400000000000000000000000000000000000000000000000000000;
    dw 0x800000000000000000000000000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000000000000000000000000;
    dw 0x2000000000000000000000000000000000000000000000000000000;
    dw 0x4000000000000000000000000000000000000000000000000000000;
    dw 0x8000000000000000000000000000000000000000000000000000000;
    dw 0x10000000000000000000000000000000000000000000000000000000;
    dw 0x20000000000000000000000000000000000000000000000000000000;
    dw 0x40000000000000000000000000000000000000000000000000000000;
    dw 0x80000000000000000000000000000000000000000000000000000000;
    dw 0x100000000000000000000000000000000000000000000000000000000;
    dw 0x200000000000000000000000000000000000000000000000000000000;
    dw 0x400000000000000000000000000000000000000000000000000000000;
    dw 0x800000000000000000000000000000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000000000000000000000000000;
    dw 0x2000000000000000000000000000000000000000000000000000000000;
    dw 0x4000000000000000000000000000000000000000000000000000000000;
    dw 0x8000000000000000000000000000000000000000000000000000000000;
    dw 0x10000000000000000000000000000000000000000000000000000000000;
    dw 0x20000000000000000000000000000000000000000000000000000000000;
    dw 0x40000000000000000000000000000000000000000000000000000000000;
    dw 0x80000000000000000000000000000000000000000000000000000000000;
    dw 0x100000000000000000000000000000000000000000000000000000000000;
    dw 0x200000000000000000000000000000000000000000000000000000000000;
    dw 0x400000000000000000000000000000000000000000000000000000000000;
    dw 0x800000000000000000000000000000000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000000000000000000000000000000;
    dw 0x2000000000000000000000000000000000000000000000000000000000000;
    dw 0x4000000000000000000000000000000000000000000000000000000000000;
    dw 0x8000000000000000000000000000000000000000000000000000000000000;
    dw 0x10000000000000000000000000000000000000000000000000000000000000;
    dw 0x20000000000000000000000000000000000000000000000000000000000000;
    dw 0x40000000000000000000000000000000000000000000000000000000000000;
    dw 0x80000000000000000000000000000000000000000000000000000000000000;
    dw 0x100000000000000000000000000000000000000000000000000000000000000;
    dw 0x200000000000000000000000000000000000000000000000000000000000000;
    dw 0x400000000000000000000000000000000000000000000000000000000000000;
    dw 0x800000000000000000000000000000000000000000000000000000000000000;
    dw 0x1000000000000000000000000000000000000000000000000000000000000000;
}