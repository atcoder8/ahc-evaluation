use std::process::Command;

use anyhow::{ensure, Context};

use crate::config::Config;

/// Builds the submission code.
pub fn build_submission(config: &Config) -> anyhow::Result<()> {
    let cmd_args = &config.command.build.submission;

    // Skips build if build command for submission code is empty.
    let Some(program) = cmd_args.first() else {
        return Ok(());
    };

    let process_handle = Command::new(program)
        .args(&cmd_args[1..])
        .spawn()
        .with_context(|| {
            format!(
                "
Failed to start the child process that builds the submission code.
List of arguments: {:?}
",
                cmd_args
            )
        })?;

    let output = process_handle
        .wait_with_output()
        .with_context(|| "Failed to building the submission code.")?;

    ensure!(
        output.status.success(),
        BuildCommandError {
            cmd_args: config.command.build.submission.clone(),
            output
        }
    );

    Ok(())
}

/// Builds the local tester.
pub fn build_tester(config: &Config) -> anyhow::Result<()> {
    let cmd_args = &config.command.build.tester;

    // Skips build if build command for local tester is empty.
    let Some(program) = cmd_args.first() else {
        return Ok(());
    };

    let process_handle = Command::new(program)
        .args(&cmd_args[1..])
        .spawn()
        .with_context(|| {
            format!(
                "
Failed to start the child process that builds the local tester.
List of arguments: {:?}
",
                cmd_args
            )
        })?;

    let output = process_handle
        .wait_with_output()
        .with_context(|| "Failed to build the local tester.")?;

    ensure!(
        output.status.success(),
        BuildCommandError {
            cmd_args: config.command.build.tester.clone(),
            output
        }
    );

    Ok(())
}

#[derive(Debug)]
pub struct BuildCommandError {
    pub cmd_args: Vec<String>,
    pub output: std::process::Output,
}

impl std::fmt::Display for BuildCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout = String::from_utf8(self.output.stdout.clone()).unwrap();
        let stderr = String::from_utf8(self.output.stderr.clone()).unwrap();

        write!(
            f,
            "
Failed to execute command.

Exit code: {}

Command line arguments: {:?},

------------------------------- Standard Output --------------------------------
{}
--------------------------------------------------------------------------------

---------------------------- Standard Error Output -----------------------------
{}
--------------------------------------------------------------------------------
",
            self.output.status, self.cmd_args, stdout, stderr,
        )
    }
}

impl std::error::Error for BuildCommandError {}
