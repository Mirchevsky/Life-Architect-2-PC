/// Daily Quote Engine — ported from the Android DailyQuoteEngine.kt
///
/// Rotation strategy (mirrors Android):
///  - 22 historical figures, each with 9-12 quotes.
///  - Cycles through all people in order, advancing one quote per person per day.
///  - Once all quotes for a person are exhausted, that person is marked exhausted.
///  - Once all people are exhausted, the entire cycle resets (all indexes back to 0).
///  - The selected quote is stable for the entire calendar day (stored in the DB).
///  - On first launch (no state), a fresh cycle is started from person 0, quote 0.

use chrono::Local;
use rusqlite::{Connection, Result};

// ── Public types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DailyQuote {
    pub person: String,
    pub quote: String,
}

// ── Quote data ───────────────────────────────────────────────────────────────

/// Returns the full quote source: ordered list of (person, quotes[]).
/// Order is preserved — it determines the rotation cycle order.
fn quote_source() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("Brothers Grimm", vec![
            "Folklore is the poetry of the people.",
            "Where there is kindness, magic often follows.",
            "No matter how dark the woods, there is always a way out. (Hansel and Gretel)",
            "A nation's soul is found in its stories.",
            "What is written in books is only half of what is learned from them.",
            "A hero is not the strongest or fastest, but the one who does not turn back.",
            "A true storyteller listens before he speaks.",
            "A clever mind will always outmatch brute strength.",
            "Not all witches are wicked, and not all kings are wise. (The Twelve Brothers)",
        ]),
        ("Moses", vec![
            "Do not let fear silence your calling.",
            "A ruler must serve before he can lead.",
            "Justice must be swift, but it must also be fair.",
            "The wilderness is the price of freedom.",
            "Even I am not allowed into the Promised Land.",
            "Honor your father and mother.",
            "The journey is long, but faith makes it bearable.",
            "The burden of guiding a nation is heavy, but it must be carried.",
            "Love the stranger.",
        ]),
        ("Julius Caesar", vec![
            "No one is so brave that he is not disturbed by something unexpected.",
            "We will either find a way or make one.",
            "An army marches on its stomach.",
            "The envy of others is the price of ambition.",
            "Victory is sweetest when it is hardest won.",
            "It is better to create than to learn.",
            "Live so that your actions inspire others long after you are gone.",
            "Experience is the teacher of all things.",
            "Great leaders are often born in the fires of adversity.",
            "I came, I saw, I conquered.",
        ]),
        ("Charles Darwin", vec![
            "The more one thinks, the less one feels.",
            "I am like a gambler, and love a wild experiment.",
            "Nothing exists for itself alone, but only in relation to other forms of life.",
            "The highest possible stage in moral culture is when we recognize that we ought to control our thoughts.",
            "The power of observation is not given to all.",
            "It is no use trying to sum people up.",
            "A mathematician without an imagination is a mere machine.",
            "The tree of life should perhaps be called the coral of life, for its base grows steadily, while its branches decay.",
            "Life is about evolution, not perfection.",
            "To understand the complexity of life, one must be humble enough to embrace the mystery.",
            "It is not the strongest of the species that survive, nor the most intelligent, but the one most responsive to change.",
        ]),
        ("Bob Marley", vec![
            "You can't run away from yourself.",
            "If something can corrupt you, you're corrupted already.",
            "Love is my religion.",
            "Judge not before you judge yourself.",
            "My music will go on forever. Maybe it's a fool who says that, but when me know facts, me can say facts. My music will go on forever.",
            "You can't find peace by avoiding life.",
            "You never know how strong you are until being strong is your only choice.",
            "The richness of life is not in its possessions but in the moments that leave you breathless.",
            "Be yourself. No one can be you better than you.",
            "If you can dream it, you can achieve it.",
            "Possession makes you rich? I don't have that type of richness. My richness is life.",
            "Don't worry about a thing, 'cause every little thing is gonna be all right.",
        ]),
        ("Napoleon", vec![
            "Imagination rules the world.",
            "A leader is a dealer in hope.",
            "Ambition never rests.",
            "If you want a thing done well, do it yourself.",
            "Music is the voice that tells us the human race is greater than it knows.",
            "Great ambition is the passion of a great character.",
            "Nothing is more difficult than to know precisely what we want.",
            "An order that can be misunderstood will be misunderstood.",
            "Ambition is like love, impatient both of delays and rivals.",
            "There are no such things as obstacles, there are only challenges.",
            "Loyalty cannot be demanded, it must be inspired.",
        ]),
        ("Christopher Columbus", vec![
            "You can never cross the ocean until you have the courage to lose sight of the shore.",
            "The journey is often the reward.",
            "Faith is a means by which we discover the unseen.",
            "Leadership means not only guiding the ship but also ensuring its purpose is noble.",
            "The courage to explore comes not from fearlessness but from overcoming it.",
            "This is a world full of wonder and opportunity.",
            "Let my deeds speak louder than my words.",
            "It is easy to discover what another has discovered before.",
            "Every step we take forward brings us closer to the stars.",
        ]),
        ("William Shakespeare", vec![
            "Love all, trust a few, do wrong to none. (All's Well That Ends Well)",
            "The robbed that smiles steals something from the thief. (Othello)",
            "Better three hours too soon than a minute too late. (The Merry Wives of Windsor)",
            "What's done cannot be undone. (Macbeth)",
            "Suspicion always haunts the guilty mind. (Henry VI, Part 3)",
            "All that glisters is not gold. (The Merchant of Venice)",
            "Strong reasons make strong actions. (King John)",
            "The empty vessel makes the loudest sound. (Henry V)",
            "One touch of nature makes the whole world kin. (Troilus and Cressida)",
            "If music be the food of love, play on.",
            "Parting is such sweet sorrow, that I shall say good night till it be morrow. (Romeo and Juliet)",
        ]),
        ("Galileo Galilei", vec![
            "The book of nature is written in the language of mathematics.",
            "Wine is sunlight, held together by water.",
            "The greatest enemy of knowledge is not ignorance, it is the illusion of knowledge.",
            "Genius is simply patience applied to great ideas.",
            "Every star has its place in the symphony of the cosmos.",
            "Innovation distinguishes between a leader and a follower.",
            "Do not be afraid of being called a rebel when you are following the path of reason.",
            "True teaching is not about filling minds, but inspiring them.",
            "Measure what is measurable and make measurable what is not so.",
            "The sun, with all those planets revolving around it and dependent on it, can still ripen a bunch of grapes as if it had nothing else in the universe to do.",
        ]),
        ("Mahatma Gandhi", vec![
            "An eye for an eye will leave the whole world blind.",
            "In a gentle way, you can shake the world.",
            "You must be the change you wish to see in the world.",
            "It is unwise to be sure of one's own wisdom.",
            "The weak can never forgive. Forgiveness is the attribute of the strong.",
            "Truth is one, paths are many.",
            "The best way to predict the future is to create it.",
            "No culture can live if it attempts to be exclusive.",
            "It is health that is real wealth and not pieces of gold and silver.",
            "In doing something, do it with love or never do it at all.",
            "The best way to find yourself is to lose yourself.",
        ]),
        ("Bruce Lee", vec![
            "Knowing is not enough, we must apply. Willing is not enough, we must do.",
            "Adapt what is useful, reject what is useless, and add what is specifically your own.",
            "The possession of anything begins in the mind.",
            "The greatest mistake is to anticipate the outcome of the engagement; you ought not to be thinking of whether it ends in victory or defeat.",
            "It's not the daily increase but daily decrease. Hack away at the unessential.",
            "Martial arts are ultimately self-knowledge.",
            "Defeat is a state of mind; no one is ever defeated until defeat has been accepted as a reality.",
            "Do not deny the classical approach, simply as a reaction, or you will have created another pattern and trapped yourself there.",
            "Obey the principles without being bound by them.",
            "Simplicity is the key to brilliance.",
        ]),
        ("Mozart", vec![
            "The music is not in the notes, but in the silence between.",
            "In order to win others over, one must write things that are simple.",
            "Music and even harmony itself must never offend the ear.",
            "Without travel, one's talent becomes caged.",
            "There is no glory in being a mere imitator.",
            "An artist must never be too self-satisfied.",
            "It is a mistake to think that the practice of my art has become easy to me.",
            "Hardest and most important thing in music is the tempo.",
            "I would rather write ten thousand notes than a single letter of the alphabet.",
            "Love, love, love — that is the soul of genius.",
            "I believe that I am entitled to some credit for originality.",
        ]),
        ("Martin Luther King Jr", vec![
            "The time is always right to do what is right.",
            "Injustice anywhere is a threat to justice everywhere.",
            "Life's most persistent and urgent question is, 'What are you doing for others?'",
            "If I cannot do great things, I can do small things in a great way.",
            "We are not makers of history. We are made by history.",
            "Faith is taking the first step even when you don't see the whole staircase.",
            "Justice too long delayed is justice denied.",
            "Freedom is never free.",
            "We will remember not the words of our enemies, but the silence of our friends.",
            "I have a dream that one day this nation will rise up and live out the true meaning of its creed.",
        ]),
        ("King David", vec![
            "When pride comes, then comes disgrace, but with humility comes wisdom.",
            "Fools find no pleasure in understanding, but delight in airing their own opinions.",
            "The words of the reckless pierce like swords, but the tongue of the wise brings healing.",
            "The righteous show mercy and give freely.",
            "The plans of the diligent lead to profit as surely as haste leads to poverty.",
            "A person's wisdom yields patience; it is to one's glory to overlook an offense.",
            "Those who guard their mouths and their tongues keep themselves from calamity.",
            "A person who refuses to admit their mistakes can never be successful.",
            "Do not boast about tomorrow, for you do not know what a day may bring.",
            "Let justice roll on like a river, righteousness like a never-failing stream!",
        ]),
        ("Marilyn Monroe", vec![
            "Imperfection is beauty, madness is genius, and it's better to be absolutely ridiculous than absolutely boring.",
            "A career is born in public, talent in privacy.",
            "Sometimes good things fall apart so that better things can fall together.",
            "I don't consider myself an intellectual. But I admire intellectual people.",
            "I live to succeed, not to please you or anyone else.",
            "If I'd observed all the rules, I'd never have gotten anywhere.",
            "Fear is stupid. So are regrets.",
            "Just because you fail once doesn't mean you're gonna fail at everything.",
            "Always believe in yourself. Because if you don't, then who will, sweetie?",
        ]),
        ("Nikola Tesla", vec![
            "Every living being is an engine geared to the wheelwork of the universe.",
            "The scientists of today think deeply instead of clearly.",
            "Instinct is something that transcends knowledge.",
            "The history of science shows that theories are perishable.",
            "Our entire biological system, the brain, and the Earth itself, work on the same frequencies.",
            "There is a difference between progress and technology.",
            "With ideas, it is like with dizzy heights: one must grasp them in time, or they are lost.",
            "We all make mistakes, and it is better to make them while pursuing a noble goal.",
            "The hard work of the future will be pushing buttons.",
            "I don't care that they stole my idea... I care that they don't have any of their own.",
        ]),
        ("Sigmund Freud", vec![
            "Out of your vulnerabilities will come your strength.",
            "In the small matters, trust the mind; in the large ones, the heart.",
            "The only person with whom you have to compare yourself is you in the past.",
            "The mind is like an iceberg, it floats with one-seventh of its bulk above water.",
            "Dreams are the guardians of sleep and not its disturbers.",
            "To be completely honest with oneself is the best exercise.",
            "Where does a thought go when it's forgotten?",
            "The voice of the intellect is a soft one, but it does not rest until it has gained a hearing.",
            "When inspiration does not come to me, I go halfway to meet it.",
            "One day, in retrospect, the years of struggle will strike you as the most beautiful.",
        ]),
        ("Leonardo da Vinci", vec![
            "A painter should begin every canvas with a wash of black, because all things in nature are dark except where exposed by the light.",
            "Learning never exhausts the mind.",
            "Nothing can be loved or hated unless it is first understood.",
            "Nature never breaks her own laws.",
            "The knowledge of all things is possible.",
            "The greatest geniuses sometimes accomplish more when they work less.",
            "It had long since come to my attention that people of accomplishment rarely sat back and let things happen to them. They went out and happened to things.",
            "Every now and then go away, have a little relaxation, for when you come back to your work your judgment will be surer.",
            "Simplicity is the ultimate sophistication.",
            "Nothing strengthens authority so much as silence.",
            "Time stays long enough for anyone who will use it.",
            "Art is never finished, only abandoned.",
        ]),
        ("Alexander the Great", vec![
            "An army of sheep led by a lion is better than an army of lions led by a sheep.",
            "In the way of conquest, there are no rest periods.",
            "Upon the conduct of each depends the fate of all.",
            "Fortune favors the bold.",
            "We are the storm upon the horizon.",
            "Life is worth nothing if not lived with honor and courage.",
            "An empire does not reside in size, but in greatness of spirit.",
            "I feel no fear; I see no danger; I foresee no difficulties.",
            "I shall not evade the judgment of the people.",
            "Whatever possession we gain by our sword cannot be sure or lasting, but the love gained by kindness and moderation is certain.",
            "Without Knowledge, Skill cannot be focused. Without Skill, Strength cannot be brought to bear and without Strength, Knowledge may not be applied.",
            "The world will know my name and it will remember it forever.",
        ]),
        ("Isaac Newton", vec![
            "If I have seen further, it is by standing on the shoulders of giants.",
            "What we know is a drop, what we don't know is an ocean.",
            "To every action there is always opposed an equal reaction.",
            "If others would think as hard as I did, then they would get similar results.",
            "No great discovery was ever made without a bold guess.",
            "Truth is the offspring of silence and meditation.",
            "I feign no hypotheses.",
            "If I have done the public any service, it is due to my patient thought.",
            "It is the weight, not numbers of experiments that is to be regarded.",
            "Live your life as an exclamation rather than an explanation.",
            "Whence arises all that order and beauty we see in the world?",
            "Tact is the art of making a point without making an enemy.",
        ]),
        ("Albert Einstein", vec![
            "Life is like riding a bicycle. To keep your balance, you must keep moving.",
            "I have no special talent. I am only passionately curious.",
            "In the middle of difficulty lies opportunity.",
            "Logic will get you from A to B. Imagination will take you everywhere.",
            "If you can't explain it simply, you don't understand it well enough.",
            "It is not that I'm so smart. But I stay with the questions much longer.",
            "The measure of intelligence is the ability to change.",
            "Best way to predict the future is to invent it.",
            "Everything should be made as simple as possible, but not simpler.",
            "We cannot solve our problems with the same thinking we used when we created them.",
            "The more I learn, the more I realize how much I don't know.",
            "Everything is energy and that's all there is to it.",
        ]),
    ]
}

// ── Persistence helpers ──────────────────────────────────────────────────────

/// Ensures the `daily_quote_state` table exists.
pub fn ensure_table(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS daily_quote_state (
            id              INTEGER PRIMARY KEY CHECK (id = 1),
            last_date       TEXT,
            last_person     TEXT,
            last_quote      TEXT,
            cycle_position  INTEGER NOT NULL DEFAULT 0,
            quote_indexes   TEXT NOT NULL DEFAULT '{}'
        );
        INSERT OR IGNORE INTO daily_quote_state
            (id, last_date, last_person, last_quote, cycle_position, quote_indexes)
            VALUES (1, NULL, NULL, NULL, 0, '{}');",
    )
}

/// Loads the persisted rotation state from the DB.
fn load_state(conn: &Connection) -> (Option<String>, Option<String>, Option<String>, usize, Vec<usize>) {
    let source = quote_source();
    let person_count = source.len();

    let result = conn.query_row(
        "SELECT last_date, last_person, last_quote, cycle_position, quote_indexes
         FROM daily_quote_state WHERE id = 1",
        [],
        |row| {
            let last_date: Option<String> = row.get(0)?;
            let last_person: Option<String> = row.get(1)?;
            let last_quote: Option<String> = row.get(2)?;
            let cycle_pos: usize = row.get::<_, i64>(3)? as usize;
            let indexes_json: String = row.get(4)?;
            Ok((last_date, last_person, last_quote, cycle_pos, indexes_json))
        },
    );

    match result {
        Ok((date, person, quote, pos, json)) => {
            let indexes = parse_indexes(&json, person_count);
            (date, person, quote, pos, indexes)
        }
        Err(_) => (None, None, None, 0, vec![0usize; person_count]),
    }
}

/// Saves the rotation state back to the DB.
fn save_state(
    conn: &Connection,
    today: &str,
    person: &str,
    quote: &str,
    cycle_pos: usize,
    indexes: &[usize],
) {
    let json = serialize_indexes(indexes);
    let _ = conn.execute(
        "UPDATE daily_quote_state
         SET last_date = ?1, last_person = ?2, last_quote = ?3,
             cycle_position = ?4, quote_indexes = ?5
         WHERE id = 1",
        rusqlite::params![today, person, quote, cycle_pos as i64, json],
    );
}

/// Serializes the per-person quote index array as a compact JSON array string.
fn serialize_indexes(indexes: &[usize]) -> String {
    let parts: Vec<String> = indexes.iter().map(|i| i.to_string()).collect();
    format!("[{}]", parts.join(","))
}

/// Parses a JSON array string back into a Vec<usize>.
/// Falls back to all-zeros if the string is malformed or the wrong length.
fn parse_indexes(json: &str, expected_len: usize) -> Vec<usize> {
    let trimmed = json.trim();
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        let inner = &trimmed[1..trimmed.len() - 1];
        if inner.trim().is_empty() {
            return vec![0usize; expected_len];
        }
        let parsed: Vec<usize> = inner
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();
        if parsed.len() == expected_len {
            return parsed;
        }
    }
    vec![0usize; expected_len]
}

// ── Public API ───────────────────────────────────────────────────────────────

/// Returns today's quote. If the stored date matches today, returns the cached
/// quote unchanged. Otherwise advances the rotation and persists the new state.
pub fn get_daily_quote(conn: &Connection) -> DailyQuote {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let source = quote_source();
    let person_count = source.len();

    let (last_date, last_person, last_quote, mut cycle_pos, mut indexes) = load_state(conn);

    // Validate loaded state dimensions
    if indexes.len() != person_count {
        indexes = vec![0usize; person_count];
        cycle_pos = 0;
    }

    // Return cached quote if it was already selected today
    if let (Some(ref date), Some(ref person), Some(ref quote)) =
        (last_date, last_person, last_quote)
    {
        if date == &today {
            return DailyQuote {
                person: person.clone(),
                quote: quote.clone(),
            };
        }
    }

    // Check if the entire cycle is exhausted — reset if so
    let all_exhausted = indexes
        .iter()
        .enumerate()
        .all(|(i, &idx)| idx >= source[i].1.len());
    if all_exhausted {
        indexes = vec![0usize; person_count];
        cycle_pos = 0;
    }

    // Advance through the cycle to find the next person with remaining quotes
    let mut selected_person_idx = cycle_pos % person_count;
    for offset in 0..person_count {
        let candidate = (cycle_pos + offset) % person_count;
        if indexes[candidate] < source[candidate].1.len() {
            selected_person_idx = candidate;
            break;
        }
    }

    let person_name = source[selected_person_idx].0;
    let quote_idx = indexes[selected_person_idx];
    let quote_text = source[selected_person_idx].1[quote_idx];

    // Advance this person's quote index
    indexes[selected_person_idx] += 1;

    // Advance cycle position to the next person
    let next_cycle_pos = (selected_person_idx + 1) % person_count;

    save_state(conn, &today, person_name, quote_text, next_cycle_pos, &indexes);

    DailyQuote {
        person: person_name.to_string(),
        quote: quote_text.to_string(),
    }
}
