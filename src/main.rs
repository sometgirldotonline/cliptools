use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use open;
use phf::Map;
use phf::phf_map;
use regex::Regex;
use reqwest::blocking;
use std::io::Read;
use std::rc::Rc;
use wl_clipboard_rs::paste::{ClipboardType, Error, MimeType, Seat, get_contents};
use xdg_utils;

static AREA_CODES: phf::Map<&'static str, i32> = phf::phf_map! {
    "AF" => 93,
    "AL" => 355,
    "DZ" => 213,
    "AS" => 1684,
    "AD" => 376,
    "AO" => 244,
    "AI" => 1264,
    "AQ" => 672,
    "AG" => 1268,
    "AR" => 54,
    "AM" => 374,
    "AW" => 297,
    "AU" => 61,
    "AT" => 43,
    "AZ" => 994,
    "BS" => 1242,
    "BH" => 973,
    "BD" => 880,
    "BB" => 1246,
    "BY" => 375,
    "BE" => 32,
    "BZ" => 501,
    "BJ" => 229,
    "BM" => 1441,
    "BT" => 975,
    "BO" => 591,
    "BA" => 387,
    "BW" => 267,
    "BR" => 55,
    "IO" => 246,
    "VG" => 1284,
    "BN" => 673,
    "BG" => 359,
    "BF" => 226,
    "BI" => 257,
    "KH" => 855,
    "CM" => 237,
    "CA" => 1,
    "CV" => 238,
    "KY" => 1345,
    "CF" => 236,
    "TD" => 235,
    "CL" => 56,
    "CN" => 86,
    "CX" => 61,
    "CC" => 61,
    "CO" => 57,
    "KM" => 269,
    "CK" => 682,
    "CR" => 506,
    "HR" => 385,
    "CU" => 53,
    "CW" => 599,
    "CY" => 357,
    "CZ" => 420,
    "CD" => 243,
    "DK" => 45,
    "DJ" => 253,
    "DM" => 1767,
    "DO" => 1809,
    "TL" => 670,
    "EC" => 593,
    "EG" => 20,
    "SV" => 503,
    "GQ" => 240,
    "ER" => 291,
    "EE" => 372,
    "ET" => 251,
    "FK" => 500,
    "FO" => 298,
    "FJ" => 679,
    "FI" => 358,
    "FR" => 33,
    "PF" => 689,
    "GA" => 241,
    "GM" => 220,
    "GE" => 995,
    "DE" => 49,
    "GH" => 233,
    "GI" => 350,
    "GR" => 30,
    "GL" => 299,
    "GD" => 1473,
    "GU" => 1671,
    "GT" => 502,
    "GG" => 441481,
    "GN" => 224,
    "GW" => 245,
    "GY" => 592,
    "HT" => 509,
    "HN" => 504,
    "HK" => 852,
    "HU" => 36,
    "IS" => 354,
    "IN" => 91,
    "ID" => 62,
    "IR" => 98,
    "IQ" => 964,
    "IE" => 353,
    "IM" => 441624,
    "IL" => 972,
    "IT" => 39,
    "CI" => 225,
    "JM" => 1876,
    "JP" => 81,
    "JE" => 441534,
    "JO" => 962,
    "KZ" => 7,
    "KE" => 254,
    "KI" => 686,
    "XK" => 383,
    "KW" => 965,
    "KG" => 996,
    "LA" => 856,
    "LV" => 371,
    "LB" => 961,
    "LS" => 266,
    "LR" => 231,
    "LY" => 218,
    "LI" => 423,
    "LT" => 370,
    "LU" => 352,
    "MO" => 853,
    "MK" => 389,
    "MG" => 261,
    "MW" => 265,
    "MY" => 60,
    "MV" => 960,
    "ML" => 223,
    "MT" => 356,
    "MH" => 692,
    "MR" => 222,
    "MU" => 230,
    "YT" => 262,
    "MX" => 52,
    "FM" => 691,
    "MD" => 373,
    "MC" => 377,
    "MN" => 976,
    "ME" => 382,
    "MS" => 1664,
    "MA" => 212,
    "MZ" => 258,
    "MM" => 95,
    "NA" => 264,
    "NR" => 674,
    "NP" => 977,
    "NL" => 31,
    "AN" => 599,
    "NC" => 687,
    "NZ" => 64,
    "NI" => 505,
    "NE" => 227,
    "NG" => 234,
    "NU" => 683,
    "KP" => 850,
    "MP" => 1670,
    "NO" => 47,
    "OM" => 968,
    "PK" => 92,
    "PW" => 680,
    "PS" => 970,
    "PA" => 507,
    "PG" => 675,
    "PY" => 595,
    "PE" => 51,
    "PH" => 63,
    "PN" => 64,
    "PL" => 48,
    "PT" => 351,
    "PR" => 1787,
    "QA" => 974,
    "CG" => 242,
    "RE" => 262,
    "RO" => 40,
    "RU" => 7,
    "RW" => 250,
    "BL" => 590,
    "SH" => 290,
    "KN" => 1869,
    "LC" => 1758,
    "MF" => 590,
    "PM" => 508,
    "VC" => 1784,
    "WS" => 685,
    "SM" => 378,
    "ST" => 239,
    "SA" => 966,
    "SN" => 221,
    "RS" => 381,
    "SC" => 248,
    "SL" => 232,
    "SG" => 65,
    "SX" => 1721,
    "SK" => 421,
    "SI" => 386,
    "SB" => 677,
    "SO" => 252,
    "ZA" => 27,
    "KR" => 82,
    "SS" => 211,
    "ES" => 34,
    "LK" => 94,
    "SD" => 249,
    "SR" => 597,
    "SJ" => 47,
    "SZ" => 268,
    "SE" => 46,
    "CH" => 41,
    "SY" => 963,
    "TW" => 886,
    "TJ" => 992,
    "TZ" => 255,
    "TH" => 66,
    "TG" => 228,
    "TK" => 690,
    "TO" => 676,
    "TT" => 1868,
    "TN" => 216,
    "TR" => 90,
    "TM" => 993,
    "TC" => 1649,
    "TV" => 688,
    "VI" => 1340,
    "UG" => 256,
    "UA" => 380,
    "AE" => 971,
    "GB" => 44,
    "US" => 1,
    "UY" => 598,
    "UZ" => 998,
    "VU" => 678,
    "VA" => 379,
    "VE" => 58,
    "VN" => 84,
    "WF" => 681,
    "EH" => 212,
    "YE" => 967,
    "ZM" => 260,
    "ZW" => 263,
};

#[derive(Debug, PartialEq)]
enum ClipboardAction {
    OpenSlackUser,
    OpenDiscordUser,
    OpenEmail,
    OpenBluesky,
    OpenFedi,
    OpenMatrix,
    OpenPhone,
    OpenWhatsapp,
    OpenTelegram,
    OpenURI,
    OpenFile,
    OpenGit,
    General(GeneralAction),
}
#[derive(Debug, PartialEq)]
enum GeneralAction {
    Spellcheck,
    AskOllama,
    SearchWeb,
    CopyClean,
    Translate,
    QR,
    Encode,
    Case,
    ClearBlankLines,
    FindReplace,
}

#[derive(Debug, PartialEq)]
enum ClipboardInfo {
    MathExpression,
    HexColour,
    HexNumber,
    TextInfo, // char,word etc count
}

// i wrote this function myself but was not sure how to properly implement the syntax regarding the ? so i did ask chatGPT.
fn get_user_areacode() -> Result<i32, reqwest::Error> {
    let request = reqwest::blocking::get("https://ipinfo.io/country")?;

    if request.status().is_success() {
        let cc = request.text()?.trim().to_string();
        Ok(AREA_CODES.get(&cc).cloned().unwrap_or(-1))
    } else {
        Ok(-1)
    }
}

// vibecoded func to check if an app has a uri bound
fn is_handler_bound(target: &str) -> bool {
    // 1. Normalize the target. If it looks like a URI scheme (no slash, no x-scheme prefix), convert it.
    let mime_query = if !target.contains('/') && !target.starts_with("x-scheme-handler/") {
        // Strip trailing colon or slashes if user passed "https://" instead of "https"
        let clean_scheme = target.trim_end_matches(':').trim_end_matches('/');
        format!("x-scheme-handler/{}", clean_scheme)
    } else {
        target.to_string()
    };

    // 2. Query the default application handler
    match xdg_utils::query_default_app(&mime_query) {
        Ok(desktop_file) => !desktop_file.trim().is_empty(),
        Err(_) => false,
    }
}

fn open_uri(url: &str) {
    match open::that(url) {
        Ok(_) => println!("Successfully requested system to open: {}", url),
        Err(e) => eprintln!("Failed to open URI: {}", e),
    }
}

fn clean_if_num(input: &str, area_code: i32) -> String {
    let mut input = input.replace("tel:", "");
    if Regex::new(r"(?im)^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$")
                        .unwrap()
                        .is_match(&input){
                            return parse_num(&input, area_code)
                        }
                        else{
                            return input.to_string()
                        }
}


fn parse_num(num: &str, area_code: i32) -> String{
    let has_ac = num.starts_with("+");
    let nums_only = num.chars().filter(|c| c.is_ascii_digit()).collect();
    if has_ac {
        nums_only
    } else {
        format!("{}{}", area_code, nums_only)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let area_code = get_user_areacode()?;
    println!("User area code is: {}", area_code);
    let application = Application::builder()
        .application_id("online.sometgirl.cliptools")
        .build();
    let appr_clone = application.clone();

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Cliptools")
            .resizable(false)
            .build();

        window.set_size_request(400, 250);

        let container = GtkBox::new(Orientation::Vertical, 10);
        container.set_margin_top(20);
        container.set_margin_bottom(20);
        container.set_margin_start(20);
        container.set_margin_end(20);

        let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);

        match result {
            Ok((mut pipe, _)) => {
                let title_label = &Label::new(Some("<b>cliptools</b>"));
                title_label.set_markup("<b>cliptools</b>");
                container.append(title_label);
                let mut contents = vec![];
                if pipe.read_to_end(&mut contents).is_ok() {
                    let text = Rc::new(String::from_utf8_lossy(&contents).into_owned()).trim().to_string();
                    let mut actions: Vec<ClipboardAction> = Vec::new();
                    let mut info: Vec<ClipboardInfo> = Vec::new();
                    // Figure out contents
                    if (text.len() >= 9 && text.len() <= 11)
                        && (text.starts_with("U") || text.starts_with("W"))
                    {
                        actions.push(ClipboardAction::OpenSlackUser)
                    }
                    if (text.len() >= 17 && text.len() <= 19) && text.parse::<f64>().is_ok() {
                        actions.push(ClipboardAction::OpenDiscordUser)
                    }
                    if text.contains("@") && !text.starts_with("@") && !text.contains("://") {
                        actions.push(ClipboardAction::OpenEmail);
                    }
                    if text.starts_with("@") && text.chars().filter(|c| *c == '@').count() == 2 {
                        actions.push(ClipboardAction::OpenFedi);
                    }
                    if text.starts_with("@")
                        && text.chars().filter(|c| *c == '@').count() == 1
                        && text.contains(".")
                    {
                        actions.push(ClipboardAction::OpenBluesky);
                    }
                    if Regex::new(r"^(?=.{1,255}$)[@#!][a-z0-9._=\-\/]+:(?:[a-zA-Z0-9](?:[a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}(?::\d{1,5})?$").unwrap().is_match(&text){
                        actions.push(ClipboardAction::OpenMatrix)
                    }
                    if Regex::new(r"^[a-zA-Z0-9](?:[a-zA-Z0-9]|-(?!-)){0,38}[a-zA-Z0-9]\/[a-zA-Z0-9](?:[a-zA-Z0-9._-]*[a-zA-Z0-9])?$").unwrap().is_match(&text){
                        actions.push(OpenGit)
                    }
                    if Regex::new(r"^@[a-zA-Z][a-zA-Z0-9_]{4,31}$")
                        .unwrap()
                        .is_match(&text)
                    {
                        actions.push(ClipboardAction::OpenTelegram);
                    }
                    let mut has_phn_match = false;
                    for mat in Regex::new(r"(?im)^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$")
                        .unwrap()
                        .find_iter(&text) {
                        has_phn_match = true;
                    }
                    if has_phn_match
                    {
                        actions.push(ClipboardAction::OpenPhone);
                        actions.push(ClipboardAction::OpenWhatsapp);
                        actions.push(ClipboardAction::OpenTelegram);
                    }
                    if text.starts_with("tel:") {
                        actions.push(ClipboardAction::OpenPhone);
                        actions.push(ClipboardAction::OpenWhatsapp);
                        actions.push(ClipboardAction::OpenTelegram);
                    }
                    if text.contains("://") {
                        actions.push(ClipboardAction::OpenURI);
                    }
                    if text.starts_with("/") {
                        actions.push(ClipboardAction::OpenFile);
                    }
                    if text.contains('=')
                        && text.chars().any(|c| c.is_ascii_digit())
                        && text
                            .chars()
                            .all(|c| c.is_ascii_digit() || "+-*/%^().= x, ".contains(c))
                    {
                        // this if statement was made by chatGPT
                        info.push(ClipboardInfo::MathExpression);
                    }
                    if text.starts_with("0x") {
                        info.push(ClipboardInfo::HexNumber);
                    }
                    info.push(ClipboardInfo::TextInfo);
                    // add the general actions the lazy way
                    actions.push(ClipboardAction::General(GeneralAction::AskOllama));
                    actions.push(ClipboardAction::General(GeneralAction::Case));
                    actions.push(ClipboardAction::General(GeneralAction::ClearBlankLines));
                    actions.push(ClipboardAction::General(GeneralAction::CopyClean));
                    actions.push(ClipboardAction::General(GeneralAction::Encode));
                    actions.push(ClipboardAction::General(GeneralAction::FindReplace));
                    actions.push(ClipboardAction::General(GeneralAction::QR));
                    actions.push(ClipboardAction::General(GeneralAction::SearchWeb));
                    actions.push(ClipboardAction::General(GeneralAction::Spellcheck));
                    actions.push(ClipboardAction::General(GeneralAction::Translate));
                    container.append(&Label::new(Some(&text)));
                    for action in actions {
                        let button = Button::with_label(match action {
                            ClipboardAction::OpenSlackUser => "Open In Slack",
                            ClipboardAction::OpenDiscordUser => "Open In Discord",
                            ClipboardAction::OpenEmail => "Open Email",
                            ClipboardAction::OpenURI => "Open URI",
                            ClipboardAction::OpenWhatsapp => "Open in Whatsapp",
                            ClipboardAction::OpenTelegram => "Open in Telegram",
                            ClipboardAction::OpenFedi => "Open Fediverse Profile",
                            ClipboardAction::OpenBluesky => "Open Bluesky Profile",
                            ClipboardAction::OpenFile => "Open File",
                            ClipboardAction::OpenPhone => "Open In Telephony App",
                            ClipboardAction::OpenMatrix => "Open in Matrix",
                            ClipboardAction::OpenGit => "Open Github Repository",
                            ClipboardAction::General(GeneralAction::AskOllama) => "Ask to Ollama Model",
                            ClipboardAction::General(GeneralAction::Case) => "Adjust Case",
                            ClipboardAction::General(GeneralAction::CopyClean) => "Clean and Copy",
                            ClipboardAction::General(GeneralAction::ClearBlankLines) => "Clear Blank Lines",
                            ClipboardAction::General(GeneralAction::Encode) => "Encode",
                            ClipboardAction::General(GeneralAction::FindReplace) => "Find/Replace",
                            ClipboardAction::General(GeneralAction::QR) => "Make QR Code",
                            ClipboardAction::General(GeneralAction::SearchWeb) => "Search Web",
                            ClipboardAction::General(GeneralAction::Spellcheck) => "Check Spelling",
                            ClipboardAction::General(GeneralAction::Translate) => "Translate",
                            _ => todo!(),
                        });
                        container.append(&button);
                        let text = text.clone();
                        let appq_clone = app.clone();
                        button.connect_clicked(move |_| {
                            match action {
                                ClipboardAction::OpenSlackUser => {
                                    let url = format!("https://app.slack.com/team/{}", text);
                                    open_uri(&url);
                                }
                                ClipboardAction::OpenDiscordUser => {
                                    if is_handler_bound("discord://") {
                                        let url = format!("discord://-/users/{}", text);
                                        open_uri(&url);
                                    } else {
                                        let url = format!("https://discord.com/users/{}", text);
                                        open_uri(&url);
                                    }
                                }
                                ClipboardAction::OpenEmail => {
                                    let url = format!("mailto:{}", text);
                                    open_uri(&url);
                                }
                                ClipboardAction::OpenURI => {
                                    open_uri(&text);
                                }
                                ClipboardAction::OpenTelegram => {
                                    let url = format!("https://t.me/{}", clean_if_num(&text, area_code));
                                    open_uri(&url);
                                }
                                ClipboardAction::OpenPhone => {
                                    let url = format!("tel:{}", clean_if_num(&text, area_code));
                                    open_uri(&url);
                                }
                                ClipboardAction::OpenWhatsapp => {
                                    let url = format!("https://wa.me/{}", clean_if_num(&text, area_code));
                                    open_uri(&url);
                                },
                                ClipboardAction::OpenGit => {
                                    let url = format!("https://github.com/{}", text);
                                    open_uri(&url);
                                }
                                _ => todo!(),
                            }
                            appq_clone.quit();
                        });
                    }
                } else {
                    container.append(&Label::new(Some("Failed to read clipboard data pipe.")));
                }
            }
            Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {
                container.append(&Label::new(Some(
                    "Clipboard is empty or does not contain text.",
                )));
            }
            Err(_) => {
                container.append(&Label::new(Some(
                    "Error occurred while fetching contents from the clipboard.",
                )));
            }
        };
        let close_btn = &Button::with_label("Close");
        container.append(close_btn);
        let appq_clone = app.clone();
        close_btn.connect_clicked(move |_| {
            appq_clone.quit();
        });

        // Set the container as the single child of the window
        window.set_child(Some(&container));
        window.present();
    });

    appr_clone.run();

    Ok(())
}
