// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::matcher::{Matcher, MatcherResult};
use std::{fmt::Debug, marker::PhantomData};

/// Matches an iterable type whose elements contain a value matched by `inner`.
///
/// By default, this matches a container with any number of elements matched
/// by `inner`. Use the method [`ContainsMatcher::times`] to constrain the
/// matched containers to a specific number of matching elements.
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(["Some value"], contains(eq("Some value")))?;  // Passes
/// verify_that!(vec!["Some value"], contains(eq("Some value")))?;  // Passes
/// #     Ok(())
/// # }
/// # fn should_fail_1() -> Result<()> {
/// verify_that!([] as [String; 0], contains(eq("Some value")))?;   // Fails
/// #     Ok(())
/// # }
/// # fn should_fail_2() -> Result<()> {
/// verify_that!(["Some value"], contains(eq("Some other value")))?;   // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail_1().unwrap_err();
/// # should_fail_2().unwrap_err();
/// ```
pub fn contains<T, InnerMatcherT>(
    inner: InnerMatcherT,
) -> ContainsMatcher<T, InnerMatcherT, NoCountMatcher> {
    ContainsMatcher { inner, count: NoCountMatcher, phantom: Default::default() }
}

// TODO maybe remove
pub struct NoCountMatcher;

/// A matcher which matches a container containing one or more elements a given
/// inner [`Matcher`] matches.
pub struct ContainsMatcher<T, InnerMatcherT, CountMatcher> {
    inner: InnerMatcherT,
    count: CountMatcher,
    phantom: PhantomData<T>,
}

impl<T, InnerMatcherT> ContainsMatcher<T, InnerMatcherT, NoCountMatcher> {
    /// Configures this instance to match containers which contain a number of
    /// matching items matched by `count`.
    ///
    /// For example, to assert that exactly three matching items must be
    /// present, use:
    ///
    /// ```ignore
    /// contains(...).times(eq(3))
    /// ```
    ///
    /// One can also use `times(eq(0))` to test for the *absence* of an item
    /// matching the expected value.
    pub fn times<CountMatcher>(
        self,
        count: CountMatcher,
    ) -> ContainsMatcher<T, InnerMatcherT, CountMatcher> {
        ContainsMatcher { inner: self.inner, count, phantom: self.phantom }
    }
}

// TODO(hovinen): Revisit the trait bounds to see whether this can be made more
//  flexible. Namely, the following doesn't compile currently:
//
//      let matcher = contains(eq(&42));
//      let val = 42;
//      let _ = matcher.matches(&vec![&val]);
//
//  because val is dropped before matcher but the trait bound requires that
//  the argument to matches outlive the matcher. It works fine if one defines
//  val before matcher.
impl<'a, T: Debug + 'a, InnerMatcherT: Matcher<'a, ActualT = T>, ContainerT: Debug + 'a>
    Matcher<'a> for ContainsMatcher<ContainerT, InnerMatcherT, NoCountMatcher>
where
    for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
{
    type ActualT = ContainerT;

    fn matches<'b>(&self, actual: &'b Self::ActualT) -> MatcherResult where 'a: 'b  {

            for v in actual.into_iter() {
                if self.inner.matches(v).into() {
                    return MatcherResult::Match;
                }
            }
            MatcherResult::NoMatch
    }

    fn explain_match<'b>(&self, actual: &'b Self::ActualT) -> String  where 'a: 'b {
        let count = self.count_matches(actual);
        match (count, &self.count) {
            (0, _) => "which does not contain a matching element".to_string(),
            (_, _) => "which contains a matching element".to_string(),
        }
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match (matcher_result, &self.count) {

            (MatcherResult::Match, _) => format!(
                "contains at least one element which {}",
                self.inner.describe(MatcherResult::Match)
            ),
            (MatcherResult::NoMatch, _) => {
                format!("contains no element which {}", self.inner.describe(MatcherResult::Match))
            }
        }
    }
}

impl<'a, T: Debug + 'a, InnerMatcherT: Matcher<'a, ActualT = T>, ContainerT: Debug + 'a, CountMatcher>
    Matcher<'a> for ContainsMatcher<ContainerT, InnerMatcherT, CountMatcher>
where
    for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
    for<'c> CountMatcher: Matcher<'c, ActualT = usize>
{
    type ActualT = ContainerT;

    fn matches<'b>(&self, actual: &'b Self::ActualT) -> MatcherResult where 'a: 'b  {
            self.count.matches(&self.count_matches(actual))
    }

    fn explain_match<'b>(&self, actual: &'b Self::ActualT) -> String  where 'a: 'b {
        let count = self.count_matches(actual);
        format!("which contains {} matching elements", count)
       
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match (matcher_result, &self.count) {
            (MatcherResult::Match, count) => format!(
                "contains n elements which {}\n  where n {}",
                self.inner.describe(MatcherResult::Match),
                count.describe(MatcherResult::Match)
            ),
            (MatcherResult::NoMatch, count) => format!(
                "doesn't contain n elements which {}\n  where n {}",
                self.inner.describe(MatcherResult::Match),
                count.describe(MatcherResult::Match)
            ),
        }
    }

    fn and<Right: Matcher<'a, ActualT = Self::ActualT>>(
        self,
        right: Right,
    ) -> crate::prelude::__internal_unstable_do_not_depend_on_these::ConjunctionMatcher<Self, Right>
    where
        Self: Sized,
    {
        crate::prelude::__internal_unstable_do_not_depend_on_these::ConjunctionMatcher::new(self, right)
    }
}

impl<'a, ActualT, InnerMatcherT, CountMatcher> ContainsMatcher<ActualT, InnerMatcherT, CountMatcher> {
    fn count_matches<T: Debug + 'a, ContainerT: 'a>(&self, actual: &ContainerT) -> usize
    where
        for<'b> &'b ContainerT: IntoIterator<Item = &'b T>,
        InnerMatcherT: Matcher<'a, ActualT = T>,
    {
        let mut count = 0;
        for v in actual.into_iter() {
            if self.inner.matches(v).into() {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::{contains, ContainsMatcher};
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;

    #[test]
    fn contains_matches_singleton_slice_with_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&vec![1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_matches_singleton_vec_with_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&vec![1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_matches_two_element_slice_with_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&[0, 1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_does_not_match_singleton_slice_with_wrong_value() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&[0]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_does_not_match_empty_slice() -> Result<()> {
        let matcher = contains(eq(1));

        let result = matcher.matches(&[]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_matches_slice_with_repeated_value() -> Result<()> {
        let matcher = contains(eq(1)).times(eq(2));

        let result = matcher.matches(&[1, 1]);

        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn contains_does_not_match_slice_with_too_few_of_value() -> Result<()> {
        let matcher = contains(eq(1)).times(eq(2));

        let result = matcher.matches(&[0, 1]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_does_not_match_slice_with_too_many_of_value() -> Result<()> {
        let matcher = contains(eq(1)).times(eq(1));

        let result = matcher.matches(&[1, 1]);

        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn contains_formats_without_multiplicity_by_default() -> Result<()> {
        let matcher: ContainsMatcher<Vec<i32>, _, _> = contains(eq(1));

        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            eq("contains at least one element which is equal to 1")
        )
    }

    #[test]
    fn contains_formats_with_multiplicity_when_specified() -> Result<()> {
        let matcher: ContainsMatcher<Vec<i32>, _, _> = contains(eq(1)).times(eq(2));

        verify_that!(
            Matcher::describe(&matcher, MatcherResult::Match),
            eq("contains n elements which is equal to 1\n  where n is equal to 2")
        )
    }

    #[test]
    fn contains_mismatch_shows_number_of_times_element_was_found() -> Result<()> {
        verify_that!(
            contains(eq(3)).times(eq(1)).explain_match(&vec![1, 2, 3, 3]),
            displays_as(eq("which contains 2 matching elements"))
        )
    }

    #[test]
    fn contains_mismatch_shows_when_matches() -> Result<()> {
        verify_that!(
            contains(eq(3)).explain_match(&vec![1, 2, 3, 3]),
            displays_as(eq("which contains a matching element"))
        )
    }

    #[test]
    fn contains_mismatch_shows_when_no_matches() -> Result<()> {
        verify_that!(
            contains(eq(3)).explain_match(&vec![1, 2]),
            displays_as(eq("which does not contain a matching element"))
        )
    }
}
