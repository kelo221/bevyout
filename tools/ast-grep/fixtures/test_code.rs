#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    #[test]
    fn test_system_shape() {
        let values = Vec::new();
        assert!(values.is_empty());
    }

    fn fixture_system(query: Query<&Transform>) {
        println!("fixture {}", query.is_empty());
    }
}
