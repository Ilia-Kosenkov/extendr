use std::error::Error;

use xshell::{cmd, Shell};

use crate::extendrtests::with_absolute_path::{swap_extendr_api_path, R_FOLDER_PATH};

pub(crate) fn run(shell: &Shell) -> Result<(), Box<dyn Error>> {
    let _document_handle = swap_extendr_api_path(shell)?;

    run_tests(shell)?;

    Ok(())
}

fn run_tests(shell: &Shell) -> Result<(), Box<dyn Error>> {
    let _r_path = shell.push_dir(R_FOLDER_PATH);
    cmd!(shell, "Rscript -e devtools::test()").run()?;

    Ok(())
}
