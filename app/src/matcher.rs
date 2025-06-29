use crate::config::CaseSensitivity;
use crate::item::Item;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub fn new_matcher(case: CaseSensitivity) -> SkimMatcherV2 {
    let matcher = SkimMatcherV2::default();
    match case {
        CaseSensitivity::Smart => matcher.smart_case(),
        CaseSensitivity::Respect => matcher.respect_case(),
        CaseSensitivity::Ignore => matcher.ignore_case(),
    }
}

pub fn update_scores(query: &str, matcher: &SkimMatcherV2, items: &mut [Item]) {
    items
        .iter_mut()
        .for_each(|item| match matcher.fuzzy_indices(&item.key, query) {
            Some((score, match_indices)) => {
                item.score = Some(score as u32);
                item.match_indices = Some(match_indices);
            }
            None => {
                item.score = None;
                item.match_indices = None;
            }
        });
}
