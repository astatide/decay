// needs to implement Bondable
struct Molecule {
    atoms: Vec<String>,
    neighbors: HashMap<String, Vec<String>>,
}
