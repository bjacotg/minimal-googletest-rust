use googletest::{matcher::MatcherResult, prelude::*};
use std::marker::PhantomData;

#[derive(Debug)]
struct ArenaHolder<'a, T> {
    value: &'a T,
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

struct GetAFieldMatcher<Inner> {
    inner: Inner,
}

impl<'a, Inner> Matcher<ArenaHolder<'a, Strukt>> for GetAFieldMatcher<Inner>
where
    for<'b> Inner: Matcher<ArenaHolder<'b, i32>>,
{
    fn matches(&self, actual: & ArenaHolder<'a, Strukt>) -> MatcherResult
    {
        self.inner.matches(&actual.get_a_field())
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        todo!()
    }
}

struct HoldsMatcher<ExpectedT> {
    expected: ExpectedT,
}

impl<'a, ActualT: std::fmt::Debug, ExpectedT: PartialEq<ActualT>> Matcher<ArenaHolder<'a, ActualT>>
    for HoldsMatcher<ExpectedT>
{
    fn matches(&self, actual: & ArenaHolder<'a, ActualT>) -> MatcherResult
    {
        (&self.expected == actual.value).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        todo!()
    }
}

#[test]
fn does_it_work() -> Result<()> {
    let arena = vec![Strukt { a_field: 32 }];

    let holder = ArenaHolder { value: &arena[0] };

    verify_that!(holder, GetAFieldMatcher { inner: HoldsMatcher{expected: 32} })
}
