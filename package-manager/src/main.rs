use package_manager::art;
use package_manager::art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    art::mix(red, yellow);
}
