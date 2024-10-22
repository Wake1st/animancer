
// Generate `Cargo.lock`.
system::command("cargo", ["update", "--package", variable::get("project-name")]);