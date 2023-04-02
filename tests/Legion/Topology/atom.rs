#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_builder() {
        let atom = AtomBuilder::new()
        .element(Elements::H(0))
        .build();
            assert_eq!(atom.get_element()?, Elements::H(0));
    }
}