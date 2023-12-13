// For the `cargo_crate_version!` macro
#[macro_use]
extern crate self_update;

fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    let mut rel_builder = self_update::backends::github::ReleaseList::configure();

    rel_builder.repo_owner("toschoo");

    let releases = rel_builder.repo_name("hello-update").build()?.fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);

    let mut status_builder = self_update::backends::github::Update::configure();

    status_builder
        .repo_owner("toschoo");

    let status = status_builder
        .repo_name("hello-update")
        .bin_name("hello-update")
        .show_download_progress(true)
        //.target_version_tag("v9.9.10")
        //.show_output(false)
        //.no_confirm(true)
        //
        // For private repos, you will need to provide a GitHub auth token
        // **Make sure not to bake the token into your app**; it is recommended
        // you obtain it via another mechanism, such as environment variables
        // or prompting the user for input
        // .auth_token(env!("SELF_UPDATE_AUTH_TOKEN"))
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}


fn main() {
    if let Err(e) = run() {
        println!("[ERROR] {}", e);
        ::std::process::exit(1);
    }

    println!("Hello, world!");
}
