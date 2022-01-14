use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use rand::{prelude::SliceRandom, thread_rng};

static OSHI_LIST: [&str; 7] = [
    "湊あくあ",
    "ロボコさん",
    "星街すいせい",
    "しあんにゃん",
    "白上フブキ",
    "雪花ラミィ",
    "宝鐘マリン",
];

pub fn authorize() -> bool {
    let mut authorized = false;
    let mut oshi_list_shuffled = OSHI_LIST.to_vec();
    oshi_list_shuffled.shuffle(&mut thread_rng());

    let chosen_oshi = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("おしてだれですか？")
        // .default(0)
        .items(&oshi_list_shuffled)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();

    if oshi_list_shuffled[chosen_oshi] == OSHI_LIST[3] {
        authorized = true;
    }

    authorized
}
