use nom_locate::LocatedSpan;
// Using tag_no_case from nom_supreme since its error is nicer
// ParserExt is mostly for adding `.context` on calls to identifier to say what kind of identifier we want
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while, take_while1},
    character::complete::{alphanumeric1, char, multispace0, multispace1, one_of},
    combinator::{cut, map, opt, value},
    error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    multi::{separated_list0, separated_list1},
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    AsChar, Err, IResult, InputIter, InputLength, InputTakeAtPosition, Parser,
};
use nom_supreme::{tag::complete::tag_no_case, ParserExt};
use serde::{Deserialize, Serialize};

// Use nom_locate's LocatedSpan as a wrapper around a string input
pub type Span<'a> = LocatedSpan<&'a str>;
// the result for all of our parsers, they will have our span type as input and can have any output
// this will use a default error type but we will change that latter
pub type ParseResult<'a, T> = IResult<Span<'a>, T>;

/// Parse a unquoted sql identifier
pub(crate) fn identifier(i: Span) -> ParseResult<String> {
    map(take_while1(|c: char| c.is_alphanumeric()), |s: Span| {
        s.fragment().to_string()
    })(i)
}

/// Implement the parse function to more easily convert a span into a sql
/// command
pub trait Parse<'a>: Sized {
    /// Parse the given span into self
    fn parse(input: Span<'a>) -> ParseResult<'a, Self>;
    /// Helper method for tests to convert a str into a raw span and parse
    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }
}

// many other imports omitted
/// A colum's type
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlTypeInfo {
    // these are basic for now. Will add more + size max later on
    // TODO: add more types
    String,
    Int,
}

// parses "string | int"
impl<'a> Parse<'a> for SqlTypeInfo {
    fn parse(input: Span<'a>) -> ParseResult<'a, Self> {
        // context will help give better error messages later on
        context(
            "Column Type",
            // alt will try each passed parser and return what ever succeeds
            alt((
                map(tag_no_case("string"), |_| Self::String),
                map(tag_no_case("int"), |_| Self::Int),
            )),
        )(input)
    }
}

/// A column's name + type
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub type_info: SqlTypeInfo,
}

/// parses "<colName> <colType>"
impl<'a> Parse<'a> for Column {
    fn parse(input: Span<'a>) -> ParseResult<'a, Self> {
        context(
            "Create Column",
            map(
                separated_pair(
                    identifier.context("Column Name"),
                    multispace1,
                    SqlTypeInfo::parse,
                ),
                |(name, type_info)| Self { name, type_info },
            ),
        )(input)
    }
}

/// The table and its columns to create
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CreateStatement {
    pub table: String,
    pub columns: Vec<Column>,
}

// parses a comma seperated list of column definitions contained in parens
fn column_definitions(input: Span<'_>) -> ParseResult<'_, Vec<Column>> {
    context(
        "Column Definitions",
        map(
            tuple((
                char('('),
                separated_list1(tuple((multispace0, char(','), multispace0)), Column::parse),
                char(')'),
            )),
            |(_, cols, _)| cols,
        ),
    )(input)
}

// parses "CREATE TABLE <table name> <column defs>
impl<'a> Parse<'a> for CreateStatement {
    fn parse(input: Span<'a>) -> ParseResult<'a, Self> {
        map(
            separated_pair(
                // table name
                preceded(
                    tuple((
                        tag_no_case("create"),
                        multispace1,
                        tag_no_case("table"),
                        multispace1,
                    )),
                    identifier.context("Table Name"),
                ),
                multispace1,
                // column defs
                column_definitions,
            )
            .context("Create Table"),
            |(table, columns)| Self { table, columns },
        )(input)
    }
}

// I was a test hater earlier but may as well cover the basics...
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create() {
        let expected = CreateStatement {
            table: "foo".into(),
            columns: vec![
                Column {
                    name: "col1".into(),
                    type_info: SqlTypeInfo::Int,
                },
                Column {
                    name: "col2".into(),
                    type_info: SqlTypeInfo::String,
                },
                Column {
                    name: "col3".into(),
                    type_info: SqlTypeInfo::String,
                },
            ],
        };
        assert_eq!(
            CreateStatement::parse_from_raw(
                "CREATE TABLE foo (col1 int, col2 string, col3 string)"
            )
            .unwrap()
            .1,
            expected
        )
    }
}
