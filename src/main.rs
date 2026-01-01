mod modules;

fn main() {
    modules::display::show_logo();

    if modules::display::get_user_confirmation() {
        println!("Proceeding...");
    } else {
        println!("Cancelled.");
        std::process::exit(0);
    }
}
