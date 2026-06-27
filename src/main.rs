use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use std::io::Read;
use wl_clipboard_rs::paste::{ClipboardType, Error, MimeType, Seat, get_contents};
use open;
use xdg_utils;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum ClipboardAction {
    OpenSlackUser,
    OpenDiscordUser,
    OpenEmail,
    OpenBluesky,
    OpenFedi,
    OpenMatrix,
    CallPhone,
    OpenWhatsapp,
    OpenTelegram,
    OpenURI,
    OpenFile,
    OpenGit,
    CloneGitRepo,
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
    DedupeLines,
    FindReplace,
}

#[derive(Debug, PartialEq)]
enum ClipboardInfo {
    MathExpression,
    HexColour,
    HexNumber,
    TextInfo, // char,word etc count
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


fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                    let text = Rc::new(String::from_utf8_lossy(&contents).into_owned());
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
                    if text.contains("@") && !text.starts_with("@")&&!text.contains("://") {
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
                    if text.starts_with("tel:") {
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
                    if text.starts_with("git@") {
                        actions.push(ClipboardAction::CloneGitRepo);
                    }
                    if text.ends_with(".git") {
                        actions.push(ClipboardAction::CloneGitRepo);
                    }
                    info.push(ClipboardInfo::TextInfo);
                    container.append(&Label::new(Some(&text)));
                    for action in actions {
                        let button = Button::with_label(match action {
                            ClipboardAction::OpenSlackUser => "Open In Slack",
                            ClipboardAction::OpenDiscordUser => "Open In Discord",
                            ClipboardAction::OpenEmail => "Open Email",
                            ClipboardAction::OpenURI => "Open URI",
                            _ => todo!(),
                        });
                        container.append(&button);
                        let text = Rc::clone(&text);
                        let appq_clone = app.clone();
                        button.connect_clicked(move |_|{
                            match action{
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
