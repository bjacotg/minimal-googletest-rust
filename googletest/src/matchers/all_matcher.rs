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

// There are no visible documentation elements in this module; the declarative
// macro is documented at the top level.
#![doc(hidden)]

/// Matches a value which all of the given matchers match.
///
/// Each argument is a [`Matcher`][crate::matcher::Matcher] which matches
/// against the actual value.
///
/// For example:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!("A string", all!(starts_with("A"), ends_with("string")))?; // Passes
/// #     Ok(())
/// # }
/// # fn should_fail() -> Result<()> {
/// verify_that!("A string", all!(starts_with("A"), ends_with("not a string")))?; // Fails
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// # should_fail().unwrap_err();
/// ```
///
/// Using this macro is equivalent to using the
/// [`and`][crate::matchers::conjunction_matcher::AndMatcherExt::and] extension
/// method:
///
/// ```
/// # use googletest::prelude::*;
/// # fn should_pass() -> Result<()> {
/// verify_that!(10, gt(9).and(lt(11)))?; // Also passes
/// #     Ok(())
/// # }
/// # should_pass().unwrap();
/// ```
///
/// Assertion failure messages are not guaranteed to be identical, however.
#[macro_export]
macro_rules! all {
    ($($matcher:expr),* $(,)?) => {{
        use $crate::matchers::all_matcher::internal::AllMatcher;
        AllMatcher::new([$(&$matcher),*])
    }}
}

/// Functionality needed by the [`all`] macro.
///
/// For internal use only. API stablility is not guaranteed!
#[doc(hidden)]
pub mod internal {
    use crate::matcher::{MatchExplanation, Matcher, MatcherResult};
    use crate::matcher_support::description::Description;
    use crate::matchers::anything;
    use std::fmt::Debug;

    /// A matcher which matches an input value matched by all matchers in the
    /// array `components`.
    ///
    /// For internal use only. API stablility is not guaranteed!
    #[doc(hidden)]
    pub struct AllMatcher<'a, T: Debug + ?Sized, const N: usize> {
        components: [&'a dyn Matcher<ActualT = T>; N],
    }

    impl<'a, T: Debug + ?Sized, const N: usize> AllMatcher<'a, T, N> {
        /// Constructs an [`AllMatcher`] with the given component matchers.
        ///
        /// Intended for use only by the [`all`] macro.
        pub fn new(components: [&'a dyn Matcher<ActualT = T>; N]) -> Self {
            Self { components }
        }
    }

    impl<'a, T: Debug + ?Sized, const N: usize> Matcher for AllMatcher<'a, T, N> {
        type ActualT = T;

        fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
            for component in self.components {
                match component.matches(actual) {
                    MatcherResult::DoesNotMatch => {
                        return MatcherResult::DoesNotMatch;
                    }
                    MatcherResult::Matches => {}
                }
            }
            MatcherResult::Matches
        }

        fn explain_match(&self, actual: &Self::ActualT) -> MatchExplanation {
            match N {
                0 => anything::<T>().explain_match(actual),
                1 => self.components[0].explain_match(actual),
                _ => {
                    let failures = self
                        .components
                        .iter()
                        .filter(|component| !component.matches(actual).into_bool())
                        .map(|component| format!("{}", component.explain_match(actual)))
                        .collect::<Description>();
                    if failures.len() == 1 {
                        MatchExplanation::create(format!("{}", failures))
                    } else {
                        MatchExplanation::create(format!("\n{}", failures.bullet_list().indent()))
                    }
                }
            }
        }

        fn describe(&self, matcher_result: MatcherResult) -> String {
            match N {
                0 => anything::<T>().describe(matcher_result),
                1 => self.components[0].describe(matcher_result),
                _ => {
                    let properties = self
                        .components
                        .iter()
                        .map(|m| m.describe(matcher_result))
                        .collect::<Description>()
                        .bullet_list()
                        .indent();
                    format!(
                        "{}:\n{properties}",
                        if matcher_result.into() {
                            "has all the following properties"
                        } else {
                            "has at least one of the following properties"
                        }
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::internal;
    use crate::matcher::{Matcher, MatcherResult};
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn description_shows_more_than_one_matcher() -> Result<()> {
        let first_matcher = starts_with("A");
        let second_matcher = ends_with("string");
        let matcher: internal::AllMatcher<String, 2> = all!(first_matcher, second_matcher);

        verify_that!(
            matcher.describe(MatcherResult::Matches),
            eq(indoc!(
                "
                has all the following properties:
                  * starts with prefix \"A\"
                  * ends with suffix \"string\""
            ))
        )
    }

    #[test]
    fn description_shows_one_matcher_directly() -> Result<()> {
        let first_matcher = starts_with("A");
        let matcher: internal::AllMatcher<String, 1> = all!(first_matcher);

        verify_that!(matcher.describe(MatcherResult::Matches), eq("starts with prefix \"A\""))
    }

    #[test]
    fn mismatch_description_shows_which_matcher_failed_if_more_than_one_constituent() -> Result<()>
    {
        let first_matcher = starts_with("Another");
        let second_matcher = ends_with("string");
        let matcher: internal::AllMatcher<str, 2> = all!(first_matcher, second_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }

    #[test]
    fn mismatch_description_is_simple_when_only_one_consistuent() -> Result<()> {
        let first_matcher = starts_with("Another");
        let matcher: internal::AllMatcher<str, 1> = all!(first_matcher);

        verify_that!(
            matcher.explain_match("A string"),
            displays_as(eq("which does not start with \"Another\""))
        )
    }
}
