pub fn write_json(res: &serde_json::Value, name: &str) -> anyhow::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(&format!("results/{}.json", name))?;
    write!(file, "{}", serde_json::to_string(res)?)?;
    file.flush()?;
    Ok(())
}
