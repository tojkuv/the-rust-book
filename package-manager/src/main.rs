use package_manager_example_6923::art;
use package_manager_example_6923::art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    art::mix(red, yellow);
}
