use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    current_key: String
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    const ROUND_ONE_WORDS: [&'static str; 100] = [
        "sheen", "hues", "nets", "hate", "stoat", "tuns", "hen", "that", "house", "see", "taste",
        "ton", "tease", "tea", "hue", "eta", "shoe", "asset", "suet", "tents", "toot", "tats",
        "sets", "aunt", "anus", "shuts", "too", "teats", "seat", "hes", "est", "state", "teens",
        "suns", "shush", "sheet", "ant", "tutu", "stunt", "nests", "shot", "sot", "son", "sea",
        "sane", "ashen", "onus", "shahs", "ants", "noose", "asst", "shuns", "south", "sue",
        "oaten", "oho", "tones", "neon", "heats", "aunts", "heath", "unset", "hath", "toots",
        "oat", "nouns", "sash", "auto", "hash", "the", "soon", "thees", "east", "heat", "shoot",
        "hoses", "nous", "thus", "tost", "shoes", "ten", "eat", "shut", "snoot", "she", "thee",
        "eats", "nuts", "tee", "tot", "haunt", "oaths", "tuna", "stent", "toss", "aeons", "noses",
        "ashes", "teeth", "nosh",
    ];
    const ROUND_TWO_WORDS: [&'static str; 100] = [
        "hoodies",
        "esthetes",
        "toast",
        "oasis",
        "sandiness",
        "indents",
        "dhoti",
        "tediousness",
        "detonation",
        "stead",
        "shone",
        "inundates",
        "sensations",
        "deadens",
        "desist",
        "thinnest",
        "disunited",
        "senate",
        "nonsense",
        "addends",
        "thine",
        "nineties",
        "attenuated",
        "assessed",
        "touted",
        "anode",
        "intuit",
        "unison",
        "shush",
        "tenets",
        "taste",
        "tints",
        "thesis",
        "sodded",
        "asses",
        "stints",
        "sheathed",
        "handstands",
        "stent",
        "siestas",
        "destitute",
        "ostentatious",
        "situate",
        "sunniest",
        "diets",
        "nosiness",
        "tones",
        "tanned",
        "intends",
        "tissues",
        "seasons",
        "neath",
        "anaesthetists",
        "outside",
        "intent",
        "stunts",
        "douses",
        "snouts",
        "detains",
        "addend",
        "tidied",
        "sunshine",
        "hotshot",
        "hisses",
        "sessions",
        "untidiest",
        "undaunted",
        "sassiest",
        "dandies",
        "heeded",
        "nineteenth",
        "entente",
        "stoutness",
        "nastiest",
        "saunas",
        "attitudes",
        "intensest",
        "sainthood",
        "adenoids",
        "sedated",
        "idiots",
        "donuts",
        "soundest",
        "statuettes",
        "antidote",
        "hiatuses",
        "unions",
        "insetted",
        "antenna",
        "stinted",
        "anion",
        "tested",
        "unseat",
        "sadists",
        "distant",
        "unease",
        "tease",
        "onuses",
        "tinniest",
        "sunsets",
    ];
    const ROUND_THREE_WORDS: [&'static str; 100] = [
        "refutation",
        "talkative",
        "sculls",
        "flirted",
        "viruses",
        "crafted",
        "interpose",
        "strip",
        "folio",
        "householders",
        "merit",
        "luminaries",
        "meters",
        "metamorphosed",
        "perpetuals",
        "divination",
        "revenues",
        "enunciation",
        "spenders",
        "divert",
        "harelip",
        "reverences",
        "emanate",
        "peppiest",
        "mantlepieces",
        "thiamine",
        "chooses",
        "hairdos",
        "restroom",
        "snappish",
        "tankers",
        "careers",
        "dispersal",
        "opponent",
        "adulterated",
        "sunroof",
        "dissects",
        "demur",
        "sanatorium",
        "authenticated",
        "rotated",
        "platens",
        "unseemliness",
        "cramped",
        "overestimates",
        "fathoms",
        "mediums",
        "temped",
        "pistillate",
        "overshare",
        "smokier",
        "delineations",
        "hardcovers",
        "pharmacopoeias",
        "scarceness",
        "sleeve",
        "inverses",
        "strove",
        "helper",
        "foothills",
        "stuck",
        "forehands",
        "sillier",
        "nominate",
        "tackle",
        "futile",
        "ascendents",
        "electrode",
        "flipper",
        "multitask",
        "imperatives",
        "advertisers",
        "administer",
        "meteoroid",
        "snaked",
        "immoralities",
        "honorariums",
        "succored",
        "slummed",
        "callous",
        "manifests",
        "islands",
        "craftsmanship",
        "cretin",
        "interdepartmental",
        "survivor",
        "tailspin",
        "scatters",
        "replacements",
        "shallots",
        "trifle",
        "fruitcake",
        "preteens",
        "pastors",
        "commands",
        "shiftless",
        "lithest",
        "scenes",
        "spitfires",
        "fondle",
    ];
    const ROUND_FOUR_WORDS: [&'static str; 100] = [
        "goons",
        "daubs",
        "weedy",
        "outings",
        "seesaw",
        "businesses",
        "sanitize",
        "tugboats",
        "geeing",
        "quahogs",
        "downsizing",
        "tawniest",
        "unquestioning",
        "gowns",
        "untwisted",
        "dissenting",
        "tattooing",
        "aweigh",
        "wigged",
        "goats",
        "jigsaw",
        "toasty",
        "sandy",
        "hideaways",
        "swines",
        "ageings",
        "questing",
        "unguents",
        "waxiest",
        "swoon",
        "tangy",
        "buyouts",
        "bandannas",
        "stodgiest",
        "quash",
        "joint",
        "abuts",
        "heightens",
        "nexus",
        "yogis",
        "subjugated",
        "obtained",
        "squeegees",
        "indexing",
        "attitudinizing",
        "wings",
        "beauteous",
        "byway",
        "judged",
        "inhibited",
        "suggestions",
        "shagging",
        "towhead",
        "downsize",
        "downs",
        "desensitize",
        "bayonet",
        "weeing",
        "waxwings",
        "destining",
        "inning",
        "newbie",
        "ninny",
        "tightwads",
        "totting",
        "highest",
        "innings",
        "tinny",
        "negating",
        "tough",
        "denying",
        "exude",
        "betides",
        "disabuse",
        "bests",
        "abnegates",
        "jousting",
        "snows",
        "hesitating",
        "enjoying",
        "goosed",
        "queuing",
        "geishas",
        "disengaged",
        "dinghies",
        "eighteenth",
        "twinged",
        "soundings",
        "seething",
        "bating",
        "swains",
        "downy",
        "unhinging",
        "unbinding",
        "adjoins",
        "squeegee",
        "tibia",
        "gayness",
        "gabbing",
        "instituting",
    ];
    const ROUND_FIVE_WORDS: [&'static str; 100] = [
        "soundtrack",
        "chagrinning",
        "regatta",
        "sweetheart",
        "reshuffle",
        "bumming",
        "girdling",
        "tailgate",
        "reveille",
        "steadfast",
        "locus",
        "endear",
        "extradite",
        "greying",
        "conscience",
        "cussing",
        "dumbwaiters",
        "pyorrhea",
        "breakup",
        "downsizes",
        "grindstones",
        "sketched",
        "washerwomen",
        "squeegeed",
        "nonesuch",
        "ringed",
        "criterions",
        "sobriquet",
        "batty",
        "masterminding",
        "planetary",
        "lassie",
        "maidservant",
        "denouncement",
        "grumpiest",
        "seedier",
        "interception",
        "ecclesiastic",
        "dinkies",
        "apostrophes",
        "couches",
        "bicepses",
        "assemblywomen",
        "pincers",
        "traveller",
        "clingier",
        "displayable",
        "achievements",
        "devilry",
        "claims",
        "immuring",
        "rural",
        "bedspreads",
        "regaining",
        "laboratory",
        "defeating",
        "ninny",
        "stains",
        "captivating",
        "raring",
        "blenches",
        "snore",
        "siroccos",
        "pharmacopoeias",
        "grads",
        "franks",
        "mechanism",
        "elucidates",
        "lapidary",
        "cracked",
        "proceeded",
        "nominally",
        "dimple",
        "ritzy",
        "efface",
        "echelon",
        "volleyball",
        "metabolic",
        "ululates",
        "combats",
        "undetectable",
        "salads",
        "accessibly",
        "microaggression",
        "pitiless",
        "daylight",
        "workstations",
        "noted",
        "pawpaws",
        "vouchsafe",
        "fluoridated",
        "anticipatory",
        "stultifies",
        "prearranged",
        "dongle",
        "forgetting",
        "wiretapping",
        "shrunken",
        "basketballs",
        "surgical",
    ];

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn set_current_key(&mut self, key_event: KeyEvent) {
        self.current_key = key_event.code.to_string();
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }

            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => self.set_current_key(key_event)
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.current_key.clone().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

/*
fn check_input_with_current_word(&mut self, key_event: KeyEvent) {
    print!("{}", key_event);
}*/

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
