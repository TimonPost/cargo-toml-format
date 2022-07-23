use std::collections::BTreeMap;

use toml_edit::{Table, Item, Document};

use cargo_fmt::package_order::TomlSection;

/// Each `Matcher` field when matched to a heading or key token
/// will be matched with `.contains()`.
pub struct Matcher<'a> {
    /// Toml headings with braces `[heading]`.
    pub heading: &'a [&'a str],
    /// Toml heading with braces `[heading]` and the key
    /// of the array to sort.
    pub heading_key: &'a [(&'a str, &'a str)],
}

pub const MATCHER: Matcher<'_> = Matcher {
    heading: &["dependencies", "dev-dependencies", "build-dependencies"],
    heading_key: &[("workspace", "members"), ("workspace", "exclude")],
};

/// A state machine to track collection of headings.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Heading {
    /// After collecting heading segments we recurse into another table.
    Next(Vec<String>),
    /// We have found a completed heading.
    ///
    /// The the heading we are processing has key value pairs.
    Complete(Vec<String>),
}

/// Returns a sorted toml `Document`.
pub fn sort_toml(
    toml: &mut Document,
) {
    let mut ordering = TomlSection::manifest_spec();
    let mut ordering_1 = TomlSection::manifest_spec();
     
    let ordering_ref = ordering_1.iter().map(|k| k.as_str()).collect::<Vec<_>>();
    let matcher = Matcher {
        heading: ordering_ref.as_slice(),
        heading_key: &[("workspace", "members"), ("workspace", "exclude")],
    };

    let mut first_table = None;
    let mut heading_order: BTreeMap<(usize, String), Vec<Heading>> = BTreeMap::new();
   
    for (idx, (head, item)) in toml.iter_mut().enumerate() {
        if !matcher.heading.contains(&head.get()) {
            if !ordering.contains(&head.get().to_string()) && !ordering.is_empty() {
                ordering.push(head.to_owned());
            }
            continue;
        }

        match item {
            Item::Table(table) => {
                if first_table.is_none() {
                    // The root table is always index 0 which we ignore so add 1
                    first_table = Some(idx + 1);
                }

                let headings = heading_order.entry((idx, head.to_string())).or_default();
               
                // Push a `Heading::Complete` here incase the tables are ordered
                // [heading.segs]
                // [heading]
                // It will just be ignored if not the case
                headings.push(Heading::Complete(vec![head.to_string()]));

                gather_headings(table, headings, 1);

                headings.sort();

                table.sort_values();
            }
            Item::None => continue,
            _ => unreachable!("Top level toml must be tables"),
        }
    }

    sort_by_ordering(&ordering, &heading_order, toml);
}

fn sort_by_ordering(
    ordering: &[String],
    heading_order: &BTreeMap<(usize, String), Vec<Heading>>,
    toml: &mut Document,
) {
    let mut idx = 0;
    for heading in ordering {
        if let Some((_, to_sort_headings)) =
            heading_order.iter().find(|((_, key), _)| key == heading)
        {
            for h in to_sort_headings {
                if let Heading::Complete(segs) = h {
                    let mut table = Some(toml.as_table_mut());
                    for seg in segs {
                        table = table.and_then(|t| t[seg].as_table_mut());
                    }
                    // Do not reorder the unsegmented tables
                    if let Some(table) = table {
                        table.set_position(idx);
                        idx += 1;
                    }
                }
            }
        } else if let Some(tab) = toml.as_table_mut()[heading].as_table_mut() {
            tab.set_position(idx);
            idx += 1;
            walk_tables_set_position(tab, &mut idx)
        } else if let Some(arrtab) = toml.as_table_mut()[heading].as_array_of_tables_mut()
        {
            for tab in arrtab.iter_mut() {
                tab.set_position(idx);
                idx += 1;
                walk_tables_set_position(tab, &mut idx);
            }
        }
    }
}

fn walk_tables_set_position(table: &mut Table, idx: &mut usize) {
    for (_, item) in table.iter_mut() {
        match item {
            Item::Table(tab) => {
                tab.set_position(*idx);
                *idx += 1;
                walk_tables_set_position(tab, idx)
            }
            Item::ArrayOfTables(arr) => {
                for tab in arr.iter_mut() {
                    tab.set_position(*idx);
                    *idx += 1;
                    walk_tables_set_position(tab, idx)
                }
            }
            _ => {}
        }
    }
}

fn gather_headings(table: &Table, keys: &mut Vec<Heading>, depth: usize) {
    if table.is_empty() && !table.is_implicit() {
        let next = match keys.pop().unwrap() {
            Heading::Next(segs) => Heading::Complete(segs),
            comp => comp,
        };
        keys.push(next);
    }

    for (head, item) in table.iter() {
        match item {
            Item::Value(_) => {
                if keys.last().map_or(false, |h| matches!(h, Heading::Complete(_))) {
                    continue;
                }

                let next = match keys.pop().unwrap() {
                    Heading::Next(segs) => Heading::Complete(segs),
                    _complete => unreachable!("the above if check prevents this"),
                };
                keys.push(next);
                continue;
            }
            Item::Table(table) => {
                let next = match keys.pop().unwrap() {
                    Heading::Next(mut segs) => {
                        segs.push(head.into());
                        Heading::Next(segs)
                    }
                    // This happens when
                    //
                    // [heading]       // transitioning from here to
                    // [heading.segs]  // here
                    Heading::Complete(segs) => {
                        let take = depth.max(1);
                        let mut next = segs[..take].to_vec();
                        next.push(head.into());
                        keys.push(Heading::Complete(segs));
                        Heading::Next(next)
                    }
                };
                keys.push(next);
                gather_headings(table, keys, depth + 1);
            }
            Item::ArrayOfTables(_arr) => unreachable!("no [[heading]] are sorted"),
            Item::None => unreachable!("an empty table will not be sorted"),
        }
    }
}