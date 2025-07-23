trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// Fixed signature
fn compare_license_types<T: Licensed, U: Licensed>(software1: &T, software2: &U) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // Example usage
    let some_software = SomeSoftware;
    let other_software = OtherSoftware;
    println!("{}", compare_license_types(&some_software, &other_software)); // Prints: true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(&SomeSoftware, &OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(&OtherSoftware, &SomeSoftware));
    }
}
