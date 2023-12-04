use std::str::FromStr;

use nom_locate::LocatedSpan;
// Using tag_no_case from nom_supreme since its error is nicer
// ParserExt is mostly for adding `.context` on calls to identifier to say what kind of identifier we want
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while, take_while1},
    character::complete::{alphanumeric1, char, i32, multispace0, multispace1, one_of},
    combinator::{cut, map, opt, value},
    error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    multi::{separated_list0, separated_list1},
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    AsChar, Err, IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Parser,
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

pub fn comma_sep<'a, O, F>(f: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, Vec<O>>
where
    F: FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    separated_list1(tuple((multispace0, char(','), multispace0)), f)
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
pub enum SqlType {
    // these are basic for now. Will add more + size max later on
    // TODO: add more types
    String,
    Int,
}

// parses "string | int"
impl<'a> Parse<'a> for SqlType {
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
    pub type_info: SqlType,
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
                    SqlType::parse,
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
            tuple((char('('), comma_sep(Column::parse), char(')'))),
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlValue {
    String(String),
    Int(i32),
}

impl<'a> Parse<'a> for SqlValue {
    fn parse(input: Span<'a>) -> ParseResult<'a, Self> {
        alt((
            map(alphanumeric1, |s: Span<'a>| {
                Self::Int(s.to_string().parse::<i32>().unwrap())
            }),
            map(alphanumeric1, |s: Span<'a>| Self::String(s.to_string())),
        ))(input)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct RowValue {
    pub values: Vec<SqlValue>,
}

impl<'a> Parse<'a> for RowValue {
    fn parse(input: Span<'a>) -> ParseResult<'a, Self> {
        // context will help give better error messages later on
        map(
            context(
                "Value of Insert Row",
                // alt will try each passed parser and return what ever succeeds
                delimited(
                    tuple((multispace0, char('('), multispace0)),
                    comma_sep(SqlValue::parse),
                    tuple((multispace0, char(')'), multispace0)),
                ),
            ),
            |values| Self { values },
        )(input)
    }
}

/// The table and its columns to create
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InsertStatement {
    pub table: String,
    pub columns: Option<Vec<String>>,
    pub values: RowValue,
}

impl<'a> Parse<'a> for InsertStatement {
    fn parse(input: Span<'a>) -> ParseResult<'a, Self> {
        map(
            tuple((
                delimited(
                    // table name
                    tuple((
                        multispace1,
                        tag_no_case("insert"),
                        multispace1,
                        tag_no_case("into"),
                    )),
                    identifier.context("Table Name"),
                    tuple((multispace1, tag_no_case("values"))),
                ),
                opt(comma_sep(identifier)),
                RowValue::parse,
            ))
            .context("Create Table"),
            |(table, columns, values)| Self {
                table,
                columns,
                values,
            },
        )(input)
    }
}
// I was a test hater earlier but may as well cover the basics...
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_stmt() {
        let expected = CreateStatement {
            table: "foo".into(),
            columns: vec![
                Column {
                    name: "col1".into(),
                    type_info: SqlType::Int,
                },
                Column {
                    name: "col2".into(),
                    type_info: SqlType::String,
                },
                Column {
                    name: "col3".into(),
                    type_info: SqlType::String,
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

    #[test]
    fn test_insert_stmt() {
        let expected = InsertStatement {
            table: String::from("foo"),
            columns: None,
            values: RowValue {
                values: vec![
                    SqlValue::String(String::from("abc")),
                    SqlValue::Int(123),
                    SqlValue::String(String::from("def")),
                ],
            },
        };
        let parse_result =
            InsertStatement::parse_from_raw("INSERT INTO foo values ('abc', 123, 'def')")
                .unwrap()
                .1;
        print!("{:?}", parse_result);
        assert_eq!(parse_result, expected)
    }
}
