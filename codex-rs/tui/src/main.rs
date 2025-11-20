use clap::Parser;
use codex_arg0::arg0_dispatch_or_else;
use codex_common::CliConfigOverrides;
use codex_tui::Cli;
use codex_tui::run_main;

#[derive(Parser, Debug)]
struct TopCli {
    #[clap(flatten)]
    config_overrides: CliConfigOverrides,

    #[clap(flatten)]
    inner: Cli,
}

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|codex_linux_sandbox_exe| async move {
        let top_cli = TopCli::parse();
        let mut inner = top_cli.inner;
        inner
            .config_overrides
            .raw_overrides
            .splice(0..0, top_cli.config_overrides.raw_overrides);
        let exit_info = run_main(inner, codex_linux_sandbox_exe).await?;

        // Execute update if requested
        if let Some(action) = exit_info.update_action {
            let (cmd, args) = action.command_args();
            println!("\nUpdating Codex via `{} {}`...\n", cmd, args.join(" "));

            match std::process::Command::new(cmd).args(args).status() {
                Ok(status) if status.success() => {
                    println!("\n🎉 Update ran successfully! Please restart Codex.\n");
                }
                Ok(status) => {
                    eprintln!("\nUpdate failed with exit code: {:?}\n", status.code());
                }
                Err(err) => {
                    eprintln!("\nUpdate failed to execute: {}\n", err);
                }
            }
            return Ok(());
        }

        let token_usage = exit_info.token_usage;
        if !token_usage.is_zero() {
            println!("{}", codex_core::protocol::FinalOutput::from(token_usage),);
        }
        Ok(())
    })
}
