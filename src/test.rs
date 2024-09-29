use super::*;

#[test]
fn test_search() {
    let games: Vec<Game> = search("Elden Ring".to_string());
    let expected: [Game; 3] = [
        Game {
            image: String::from("https://howlongtobeat.com/games/68151_Elden_Ring.jpg".to_owned()),
            title: String::from("Elden Ring"),
            main: String::from("59h 35m"),
            extra: String::from("100h 5m"),
            completionist: String::from("132h 56m"),
        },
        Game {
            image: String::from(
                "https://howlongtobeat.com/games/139385_Elden_Ring_Shadow_of_the_Erdtree.jpg",
            ),
            title: String::from("Elden Ring: Shadow of the Erdtree"),
            main: String::from("25h 43m"),
            extra: String::from("38h 19m"),
            completionist: String::from("50h 0m"),
        },
        Game {
            image: String::from("https://howlongtobeat.com/games/108888_Elden_Ring_GB.jpg"),
            title: String::from("Elden Ring GB"),
            main: String::from("0h 23m"),
            extra: String::from("0h 29m"),
            completionist: String::from("0h 31m"),
        },
    ];

    assert_eq!(games, expected);
}
