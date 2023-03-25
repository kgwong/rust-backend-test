
static DECKS: &'static[&'static str] = &[
    "animals",
    "clothing",
    "geo-political",
    "hobbies",
    "musical-instruments",
    "occupations",
    "space",
    "transportation",
    "misc",
    ];

pub fn get_available_deck_names() -> &'static[&'static str] {
    DECKS
}