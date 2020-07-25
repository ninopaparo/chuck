pub mod utils {
    use dialoguer::{theme::ColorfulTheme, Select};
    pub fn show_info() {
        println!(
            "{}\nVersion: {}\n",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
    }
    pub fn choose_category() -> String {
        let selections = &[
            "Animal",
            "Career",
            "Celebrity",
            "Dev",
            "Fashion",
            "Food",
            "History",
            "Money",
            "Movie",
            "Music",
            "Science",
            "Sport",
            "Travel",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select a category")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
        let category = selections[selection];
        String::from(category)
    }
}
