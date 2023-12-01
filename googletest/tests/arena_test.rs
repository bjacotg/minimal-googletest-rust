use std::fmt::Debug;
use std::marker::PhantomData;

use googletest::matcher::MatcherResult;
use googletest::prelude::*;

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

struct GetAFieldMatcher<InnerMatcher> {
    inner: InnerMatcher,
}

impl<'arena, InnerMatcher> Matcher<'arena> for GetAFieldMatcher<InnerMatcher>
where
    InnerMatcher:  Matcher<'arena, ActualT = ArenaHolder<'arena, i32>>,
{
    type ActualT = ArenaHolder<'arena, Strukt>;

    fn matches<'b>(&self, actual: &'b Self::ActualT) -> googletest::matcher::MatcherResult
    where
        'arena: 'b
    {
        let a = actual.get_a_field();
        self.inner.matches(&a)
    }

    fn describe(&self, _matcher_result: googletest::matcher::MatcherResult) -> String {
        todo!()
    }
}

#[test]
fn test_arena_holder_anything() -> Result<()> {
    let arena = vec![Strukt { a_field: 32 }];

    let holder = ArenaHolder { value: &arena[0] };
    verify_that!(holder, GetAFieldMatcher { inner: anything() })
}

struct IsEncodedStringMatcher<ActualT, InnerMatcherT> {
    inner: InnerMatcherT,
    phantom: PhantomData<ActualT>,
}

impl<'a, ActualT: AsRef<[u8]> + Debug + 'a, InnerMatcherT> Matcher<'a>
    for IsEncodedStringMatcher<ActualT, InnerMatcherT>
where
    InnerMatcherT: Matcher<'static, ActualT = str>,
{
    type ActualT = ActualT;

    fn matches<'b>(&self, actual: &'b Self::ActualT) -> MatcherResult
    where
        'a: 'b,
    {
        std::str::from_utf8(actual.as_ref())
            .map(|s| self.inner.matches(&s))
            .unwrap_or(MatcherResult::NoMatch)
    }

    fn explain_match<'b>(&self, actual: &'b Self::ActualT) -> String
    where
        'a: 'b,
    {
        match std::str::from_utf8(actual.as_ref()) {
            Ok(s) => format!("which is a UTF-8 encoded string {}", self.inner.explain_match(&s)),
            Err(_) => "which is not a UTF-8 encoded string".into(),
        }
    }

    fn describe(&self, matcher_result: googletest::matcher::MatcherResult) -> String {
        todo!()
    }
}

#[test]
fn test_is_encoded_anything() -> Result<()> {
    verify_that!(
        "holder".as_bytes(),
        IsEncodedStringMatcher { inner: anything(), phantom: PhantomData }
    )
}
