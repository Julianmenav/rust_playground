pub fn run() {
    trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct Tweet {
        pub username: String,
        pub content: String
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        )
    };

    println!("1 new tweet: {}", tweet.summarize());
}
