fn main() -> anyhow::Result<()> {
    unc_workspaces::unc_abi_client::Generator::new("src/gen".into())
        .file("res/adder.json")
        .generate()?;
    Ok(())
}
