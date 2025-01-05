use emojis::Group;

pub const APP_ID: &str = "org.sparklet.windot";
// pub const EMOJIS_PER_ROW: i32 = 10;

pub const fn group_display_name(group: Group) -> &'static str {
    match group {
        Group::SmileysAndEmotion => "😄 Smileys & Emotion",
        Group::PeopleAndBody => "🧑 People & Body",
        Group::AnimalsAndNature => "🐷 Animals & Nature",
        Group::Activities => "⚽ Activities",
        Group::FoodAndDrink => "🍕 Food & Drink",
        Group::Objects => "🧦 Objects",
        Group::TravelAndPlaces => "✈️ Travel & Places",
        Group::Symbols => "☢️ Symbols",
        Group::Flags => "🏳️‍⚧️ Flags",
    }
}

pub const GROUPS: &[Group] = &[
    Group::SmileysAndEmotion,
    Group::PeopleAndBody,
    Group::AnimalsAndNature,
    Group::Activities,
    Group::FoodAndDrink,
    Group::Objects,
    Group::TravelAndPlaces,
    Group::Symbols,
    Group::Flags,
];
