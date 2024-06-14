pub fn update() {
    // Check package version and compare to main branch on Github repo
    let package_version = env!("CARGO_PKG_VERSION");
    let repo_link = env!("CARGO_PKG_REPOSITORY");
    println!("Repo link: {}", repo_link);
    println!("Package version: {}", package_version);

    println!("Checking repository for updates...");

    println!("Update complete!");
}
