use gtk4::glib::UnicodeType::TitlecaseLetter;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use std::io::Read;
use wl_clipboard_rs::paste::{ClipboardType, Error, MimeType, Seat, get_contents};

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
                let titleLabel = &Label::new(Some("<b>cliptools</b>"));
                titleLabel.set_markup("<b>cliptools</b>");
                container.append(titleLabel);
                let mut contents = vec![];
                if pipe.read_to_end(&mut contents).is_ok() {
                    let text = String::from_utf8_lossy(&contents).into_owned();
                    let mut text_type = "general";
                    // Figure out contents
                    if (text.len() >= 9 && text.len() <= 11)
                        && (text.starts_with("U") || text.starts_with("W"))
                    {
                        text_type = "slackUser"
                    } else if (text.len() >= 17 && text.len() <= 19) && text.parse::<f64>().is_ok()
                    {
                        text_type = "discordID"
                    } else if text.contains("@") {
                        text_type = "general-fedi;email"
                    } else if text.starts_with("http") {
                        text_type = "web-url"
                    } else if text.starts_with("tel:") {
                        text_type = "phone"
                    } else if text.contains("://") {
                        text_type = "uri"
                    } else if text.starts_with("/") {
                        text_type = "file-path"
                    } else if text.contains('=')
                        && text.chars().any(|c| c.is_ascii_digit())
                        && text
                            .chars()
                            .all(|c| c.is_ascii_digit() || "+-*/%^().= x, ".contains(c))
                    {
                        // this if statement was made by chatGPT
                        text_type = "equation";
                    } else if text.starts_with("0x") {
                        text_type = "hex"
                    } else if text.starts_with("git@") {
                        text_type = "git-ssh"
                    } else if text.ends_with(".git") {
                        text_type = "git-repo"
                    }

                    container.append(&Label::new(Some(&text)));
                    if text_type == "slackUser" {
                        let oisbtn = Button::with_label("Open In Slack");
                        oisbtn.connect_clicked(move |_| {
                            let url = format!("https://app.slack.com/team/{}", text);
                            let _ = webbrowser::open(&url);
                        });

                        container.append(&oisbtn);
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
