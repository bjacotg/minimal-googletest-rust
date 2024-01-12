use googletest::{matcher::MatcherResult, prelude::*};
use std::marker::PhantomData;

#[derive(Debug)]
struct ArenaHolder<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T: PartialEq> PartialEq<T> for ArenaHolder<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.value == other
    }
}

impl<'a, T: PartialEq + ?Sized> PartialEq<&T> for ArenaHolder<'a, T> {
    fn eq(&self, other: &&T) -> bool {
        self.value == *other
    }
}

#[derive(Debug, PartialEq)]
struct Strukt {
    a_field: i32,
    a_string: String,
}

impl<'a> ArenaHolder<'a, Strukt> {
    fn get_a_field(&self) -> ArenaHolder<'_, i32> {
        ArenaHolder { value: &self.value.a_field }
    }

    fn get_a_string(&self) -> ArenaHolder<'_, str> {
        ArenaHolder { value: &self.value.a_string }
    }
}

fn has_a_field<'a, Inner>(inner: Inner) -> impl Matcher<ArenaHolder<'a, Strukt>>
where
    Inner: for<'b> Matcher<ArenaHolder<'b, i32>>,
{
    struct AFieldMatcher<Inner> {
        inner: Inner,
    }

    impl<'b, Inner> Matcher<ArenaHolder<'b, Strukt>> for AFieldMatcher<Inner>
    where
        Inner: for<'c> Matcher<ArenaHolder<'c, i32>>,
    {
        fn matches(&self, actual: &ArenaHolder<'b, Strukt>) -> MatcherResult {
            self.inner.matches(&actual.get_a_field())
        }

        fn describe(&self, matcher_result: MatcherResult) -> googletest::description::Description {
            todo!()
        }
    }

    AFieldMatcher { inner }
}

#[test]
fn check() -> Result<()> {
    let arena = vec![Strukt { a_field: 33, a_string: "something".to_string() }];
    let holder = ArenaHolder { value: &arena[0] };

    verify_that!(holder.get_a_field(), eq(33))?;
    verify_that!(holder.get_a_string(), eq("something"))?;
    verify_that!(holder, has_a_field(eq(33)))?;
    Ok(())
}


