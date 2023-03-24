
static DECKS: &'static[&'static str] = &[
    "animals",
    "clothing",
    "geo_political",
    "hobbies",
    "musical_instruments",
    "occupations",
    "space",
    "transportation",
    "misc",
    ];

pub fn get_available_deck_names() -> &'static[&'static str] {
    DECKS
}