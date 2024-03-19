use nom::IResult;

pub fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_do_nothing_parser() -> Result<(), Box<dyn Error>> {
        let input = "some_input";
        let (remaining_input, output) = do_nothing_parser(input)?;
        assert_eq!(remaining_input, input);
        assert_eq!(output, "");
        Ok(())
    }
}
