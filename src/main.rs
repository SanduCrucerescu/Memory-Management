use assigment_1_rust::file_api;
use assigment_1_rust::memory_management;
fn main() {
    let mut frag = Default::default();
    let path = std::env::args().nth(1).expect("No path provided");
    let file_api = file_api::FileApi {
        filename: String::from(path),
        out: 0,
    };
    file_api.clear_file();
    let (maxbytes, operations) = file_api.read_file();
    let management = memory_management::MemoryManagement {
        max_bytes: maxbytes,
        operations: operations,
        blocks_vec: vec![],
        file_api: file_api.clone(),
        errors: vec![],
    };

    let mut m = management.clone();
    m.first_fit();
    let (all, free) = m.print_block();
    frag = m.fragmentation();
    file_api.write_file(false, "First Fit", frag, all, free, m.errors);

    let mut m1 = management.clone();
    m1.best_fit();
    let (all, free) = m1.print_block();
    frag = m1.fragmentation();
    file_api.write_file(false, "\nBest Fit", frag, all, free, m1.errors);

    let mut m2 = management.clone();
    m2.worst_fit();
    let (all, free) = m2.print_block();
    frag = m2.fragmentation();
    file_api.write_file(false, "\nWorst Fit", frag, all, free, m2.errors);
}
