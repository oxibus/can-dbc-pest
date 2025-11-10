#![allow(clippy::result_large_err)]

use can_dbc_pest::{DbcParser, Parser as _, Rule};
use pest::error::Error;
use pest::iterators::Pairs;

fn parse(input: &str, rule: Rule) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    DbcParser::parse(rule, input)
}

#[test]
fn signal_prefixed_with_extra_lines() -> Result<(), Error<Rule>> {
    parse(
        "\n SG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS",
        Rule::signal,
    )?;
    parse(
        "\n\n SG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS",
        Rule::signal,
    )?;
    parse(
        "\n//comment \n SG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS",
        Rule::signal,
    )?;
    parse(
        "\n  \n  //comment \n  \n SG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS",
        Rule::signal,
    )?;
    parse(
        "\n  SG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS\n\n  //comment\n  \n ",
        Rule::signal,
    )?;

    Ok(())
}
