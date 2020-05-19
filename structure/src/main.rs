use std::time::Instant;

use stories::{ModerationStatus::*, Story};
use geometry::{Triangle, Point};

pub mod stories {
    use std::time;
    use std::time::Instant;

    use url::Url;

    #[derive(Debug)]
    pub enum ModerationStatus {
        NotModeratedYet,
        Accepted,
        Rejected,
    }

    #[derive(Debug)]
    pub struct Story {
        pub id: u64,

        pub author: String,
        pub header: String,
        pub target_link: Url,
        pub images_count: u64,

        pub moderation_status: ModerationStatus,
        pub dc: time::Instant,
    }

    impl Story {
        pub fn new(id: u64, author: &str, header: &str, target_link: &str) -> Story {
            Story {
                id,
                author: author.to_string(),
                header: header.to_string(),
                target_link: target_link.parse().unwrap(),
                images_count: 0,
                moderation_status: ModerationStatus::NotModeratedYet,
                dc: Instant::now(),
            }
        }
    }

    impl PartialEq for Story {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
}

pub mod geometry {
    #[derive(Debug)]
    pub struct Point(pub u64, pub u64);

    #[derive(Debug)]
    pub struct Triangle(pub Point, pub Point, pub Point);
}

fn main() {
    let ss = [
        Story {
            id: 1,
            author: "Lena Katina".to_string(),
            header: "Summertime".to_string(),
            target_link: "https://127.0.0.1/stories/1".parse().unwrap(),
            images_count: 1,
            moderation_status: Accepted,
            dc: Instant::now(),
        },
        Story {
            id: 2,
            author: "Daria Nicolodi".to_string(),
            header: "Spring".to_string(),
            target_link: "https://127.0.0.1/stories/2".parse().unwrap(),
            images_count: 2,
            moderation_status: Accepted,
            dc: Instant::now(),
        },
        Story {
            id: 3,
            author: "Dominika Szadkowska".to_string(),
            header: "Wintertime".to_string(),
            target_link: "https://127.0.0.1/stories/3".parse().unwrap(),
            images_count: 3,
            moderation_status: Accepted,
            dc: Instant::now(),
        }
    ];

    print_stories(&ss);

    let mut story = Story::new(
        4,
        "Wiktoria Liptai",
        "Autumn",
        "https://127.0.0.1/stories/4",
    );

    println!();
    println!("story: {:#?}", story);

    story = Story{
        header: "Sunny".to_string(),
        moderation_status: Rejected,
        ..story
    };

    println!();
    println!("updated story: {:#?}", story);

    let triangle = Triangle(Point(1, 2), Point(3, 4), Point(5, 6));

    println!();
    println!("triangle: {:#?}", triangle);
}

fn print_stories(ss: &[Story]) {
    ss.iter()
        .enumerate()
        .for_each(|(i, s)|
            println!("story #{}: {:#?}", i, s)
        );
}
