use std::marker::PhantomData;

struct ArenaHolder<'a, T> {
    value: &'a T,
}

impl<'a, T: PartialEq<T>> PartialEq<T> for ArenaHolder<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.value == other
    }
}

struct Strukt {
    a_field: i32,
}

impl<'a> ArenaHolder<'a, Strukt> {
    fn get_a_field(&self) -> ArenaHolder<'_, i32> {
        ArenaHolder {
            value: &self.value.a_field,
        }
    }
}


