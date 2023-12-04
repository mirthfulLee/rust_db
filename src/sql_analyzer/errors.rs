use miette::Diagnostic;
use nom_supreme::error::{BaseErrorKind, ErrorTree, GenericErrorTree, StackContext};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Parse Error")]
pub struct FormattedError<'b> {
    // need 'b since Diagnostic derive uses 'a
    #[source_code]
    // marking what the passed string was
    src: &'b str,
    #[label("{kind}")]
    span: miette::SourceSpan,
    // tells miette to mark the provided span (the location in the
    // source code) with the error display from kind.
    kind: BaseErrorKind<&'b str, Box<dyn std::error::Error + Send + Sync + 'static>>,
    #[related]
    others: Vec<FormattedErrorContext<'b>>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Parse Error Context")]
pub struct FormattedErrorContext<'b> {
    #[source_code]
    src: &'b str,
    #[label("{context}")]
    span: miette::SourceSpan,
    context: StackContext<&'b str>,
}