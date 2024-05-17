mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        // Call the private function within the module
        let recipe = get_secret_recipe();
        println!("Making sausage with secret recipe: {}", recipe);
        println!("sausage!");
    }
}

fn main() {
    // Call the public function from the module
    sausage_factory::make_sausage();
}
