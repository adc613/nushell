use crate::commands::from_delimited_data::from_delimited_data;
use crate::commands::WholeStreamCommand;
use crate::prelude::*;
use nu_errors::ShellError;
use nu_protocol::Signature;

pub struct FromTSV;

#[derive(Deserialize)]
pub struct FromTSVArgs {
    headerless: bool,
}

#[async_trait]
impl WholeStreamCommand for FromTSV {
    fn name(&self) -> &str {
        "from tsv"
    }

    fn signature(&self) -> Signature {
        Signature::build("from tsv").switch(
            "headerless",
            "don't treat the first row as column names",
            None,
        )
    }

    fn usage(&self) -> &str {
        "Parse text as .tsv and create table."
    }

    async fn run(&self, args: CommandArgs) -> Result<OutputStream, ShellError> {
        from_tsv(args).await
    }
}

async fn from_tsv(args: CommandArgs) -> Result<OutputStream, ShellError> {
    let name = args.call_info.name_tag.clone();
    let (FromTSVArgs { headerless }, input) = args.process().await?;

    from_delimited_data(headerless, '\t', "TSV", input, name).await
}

#[cfg(test)]
mod tests {
    use super::FromTSV;
    use super::ShellError;

    #[test]
    fn examples_work_as_expected() -> Result<(), ShellError> {
        use crate::examples::test as test_examples;

        Ok(test_examples(FromTSV {})?)
    }
}
