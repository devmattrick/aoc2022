// These don't actually test anything but provide a convenient way of running against my actual inputs
#[macro_export]
macro_rules! input_tests {
    (
        $day:literal,
        $($test_part:ident),*
     ) => {
        #[cfg(test)]
        mod tests_input {
            use anyhow::Result;

            use crate::util;

            $(
                #[test]
                fn $test_part() -> Result<()> {
                    let input = util::read_input($day)?;

                    let actual = super::$test_part(&input)?;

                    println!("{}", actual);

                    Ok(())
                }
            )*
        }
    };
}
