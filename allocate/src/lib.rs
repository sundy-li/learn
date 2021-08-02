pub trait GetState<'a, T> {
    fn state(place: usize) -> &'a mut T {
        unsafe { &mut *(place as *mut T) }
    }
}

struct Doggo {
    cuteness: u64,
    age: u8,
    scritches_required: bool,
}

impl<'a> GetState<'a, Doggo> for Doggo {}

#[cfg(test)]
mod tests {

    use super::*;
    use bumpalo::Bump;
    use std::{collections::HashMap, u64};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        // Create a new arena to bump allocate into.
        let bump = Bump::new();

        // Allocate values into the arena.
        let scooter = bump.alloc(Doggo {
            cuteness: u64::max_value(),
            age: 8,
            scritches_required: true,
        });

        scooter.age += 1;

        let mut ptr = (scooter as *mut Doggo) as usize;
        let mut scooter2 = unsafe { &*(ptr as *mut Doggo) };

        let mut map = HashMap::new();
        map.insert(3isize, ptr);
        map.insert(4isize, ptr);

        assert_eq!(scooter.age, 9);
        assert_eq!(scooter2.age, 9);

        let c = Doggo::state(ptr);
        assert_eq!(c.age, 9);
    }

    #[test]
    fn raw_ptr() {
        let rows = 8;
        let mut group_keys = Vec::with_capacity(rows);
        {
            group_keys.push(1u32);
            group_keys.push(2u32);
            group_keys.push(3u32);
            group_keys.push(3u32);
            group_keys.push(3u32);
            group_keys.push(3u32);
        }

        let ptr = group_keys.as_mut_ptr() as *mut u8;

        println!("{:?}", ptr as usize);

        let j = ptr as usize + 8;
        let p = unsafe { ptr.offset(2 * 4) };

        println!("j: {:?}  p: {:?}", j as *mut u8, p);

        let result = 100_u32;
        unsafe { std::ptr::copy_nonoverlapping(&result as *const u32 as *mut u8, p, 4) };

        println!("{:?}", group_keys);
    }
}
