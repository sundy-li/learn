use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

#[cfg(test)]
mod tests {
    use crate::{Entity, World};

    #[test]
    fn it_works() {
        let world = World(vec![Entity { x: 3, y: 4 }, Entity { x: 10, y: 20 }]);
        let encoded: Vec<u8> = bincode::serialize(&world).unwrap();

        // 8 bytes for the length of the vector, 4 bytes per float.
        assert_eq!(encoded.len(), 8 + 4 * 4);

        let decoded: World = bincode::deserialize(&encoded[..]).unwrap();

        assert_eq!(world, decoded);
    }

    #[test]
    fn it_works_option() {
        let mut world: Option<u32> = Some(3);
        let encoded: Vec<u8> = bincode::serialize(&world).unwrap();

        assert_eq!(encoded.len(), 4 + 1);

        world = None;
        let encoded: Vec<u8> = bincode::serialize(&world).unwrap();
        assert_eq!(encoded.len(), 1);
    }

    #[test]
    fn it_works_vec() {
        let world: Vec<u32> = vec![3, 4, 5];
        let encoded: Vec<u8> = bincode::serialize(&world).unwrap();

        assert_eq!(encoded.len(), 8 + 4 * 3);
    }
}
