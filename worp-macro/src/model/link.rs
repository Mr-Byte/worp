use super::error::DocumentError;
use crate::{next_pair, parser::Rule};
use pest::iterators::Pairs;
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub struct Link {
    label: String,
    target: LinkTarget,
}

impl TryFrom<Pairs<'_, Rule>> for Link {
    type Error = DocumentError;

    fn try_from(mut link_pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let label = next_pair!(link_pairs => Rule::macro_link_label)?.as_str().to_string();
        let target = next_pair!(link_pairs => Rule::macro_link_target)?.into_inner().try_into()?;
        let link = Link { label, target };

        Ok(link)
    }
}

#[derive(Debug)]
pub enum LinkTarget {
    Target(String),
    TargetSet(Vec<LabeledTarget>),
}

impl TryFrom<Pairs<'_, Rule>> for LinkTarget {
    type Error = DocumentError;

    fn try_from(mut link_target_pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
        let rule = link_target_pairs
            .peek()
            .map(|pair| pair.as_rule())
            .ok_or_else(DocumentError::malformed("Unexpected end of document."))?;

        let target = match rule {
            Rule::macro_name => {
                let macro_name = next_pair!(link_target_pairs => Rule::macro_name)?.as_str().to_owned();
                LinkTarget::Target(macro_name)
            }
            Rule::macro_link_target_set => {
                let link_target_set_pairs = next_pair!(link_target_pairs => Rule::macro_link_target_set)?.into_inner();
                let mut labeled_targets = vec![];

                for link_target_set_pair in link_target_set_pairs {
                    if link_target_set_pair.as_rule() == Rule::macro_link_target_with_label {
                        let mut link_target_with_label_pairs = link_target_set_pair.into_inner();
                        let label = next_pair!(link_target_with_label_pairs => Rule::macro_link_target_label)?
                            .as_str()
                            .to_owned();
                        let target = next_pair!(link_target_with_label_pairs => Rule::macro_name)?.as_str().to_owned();

                        labeled_targets.push(LabeledTarget { label, target });
                    } else {
                        unreachable!()
                    }
                }

                LinkTarget::TargetSet(labeled_targets)
            }
            _ => unreachable!(),
        };

        Ok(target)
    }
}

#[derive(Debug)]
pub struct LabeledTarget {
    label: String,
    target: String,
}
