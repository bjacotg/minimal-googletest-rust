use googletest::matcher::Matcher;
use googletest::prelude::*;
use std::marker::PhantomData;

#[derive(Debug)]
struct ArenaHolder<'a, T> {
    value: &'a T,
}

impl<'a, T: PartialEq<T>> PartialEq<T> for ArenaHolder<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.value == other
    }
}

#[derive(Debug)]
struct Strukt {
    a_field: i32,
}

impl<'a> ArenaHolder<'a, Strukt> {
    fn get_a_field(&self) -> ArenaHolder<'_, i32> {
        ArenaHolder { value: &self.value.a_field }
    }
}

struct GetAFieldMatcher<Inner, ActualT> {
    inner: Inner,
    _phantom: PhantomData<ActualT>,
}

impl<'a, Inner> Matcher
    for GetAFieldMatcher<Inner, ArenaHolder<'a, Strukt>> where
    // error[E0582]: binding for associated type `ActualT` references lifetime `'b`, which does not appear in the trait input types
    Inner: for<'b> Matcher<ActualT = ArenaHolder<'b, i32>>
{
    type ActualT = ArenaHolder<'a, Strukt>;

    fn matches<'b>(&self, actual: &'b Self::ActualT) -> googletest::matcher::MatcherResult
    where
        Self::ActualT: 'b,
    {
        self.inner.matches(&actual.get_a_field())
    }

    fn describe(&self, matcher_result: googletest::matcher::MatcherResult) -> String {
        todo!()
    }
}

#[test]
fn does_not_match_value_when_list_is_empty() -> Result<()> {
    //verify_that!((), not(any!()))
    Ok(())
}
