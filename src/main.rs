use sublime_fuzzy::best_match;

const WORDS: [&str; 17] = [
    "4 hengen A-hytti",
    "3 hengen A-hytti",
    "3 hengen A-hytti – handicap",
    "2 hengen A-hytti",
    "2 hengen A-hytti – handicap",
    "1 hengen A-hytti",
    "4 hengen Promenade-hytti",
    "4 hengen Promenade-hytti – allergia",
    "3 hengen Promenade-hytti – handicap",
    "2 hengen Promenade-hytti",
    "2 hengen Promenade-hytti – allergia",
    "4 hengen B-hytti",
    "2 hengen B-hytti",
    "2 hengen B-hytti – allergia",
    "4 hengen C-hytti",
    "3 hengen C-hytti",
    "2 hengen C-hytti",
];

const NEGATIVE_WORDS: [&str; 3] = [
    "allergia",
    "handicap",
    "inva",
];

const POSITIVE_WORDS: [&str; 3] = [
    "4 hengen",
    "Promenade",
    "A-hytti",
];

fn score_word(word: &str) -> isize {
    let mut positive_score = 0;
    let mut negative_score = 0;

    for positive_word in POSITIVE_WORDS.iter() {
        let score = match best_match(positive_word, word) {
            Some(m) => m.score(),
            None => 0,
        };

        positive_score += score;
    }

    for negative_word in NEGATIVE_WORDS.iter() {
        let score = match best_match(negative_word, word) {
            Some(m) => m.score(),
            None => 0,
        };

        negative_score += score;
    }

    positive_score - negative_score*10
}

fn main() {
    let mut scores: Vec<(String, isize)> = Vec::new();
    for word in WORDS.iter() {
        scores.push((word.to_string(), score_word(word)));
    }

    scores.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, score) in scores.iter() {
        println!("{}: {}", word, score);
    }
}
