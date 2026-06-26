use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use std::io::Read;
use wl_clipboard_rs::paste::{ClipboardType, Error, MimeType, Seat, get_contents};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let application = Application::builder()
        .application_id("online.sometgirl.cliptools")
        .build();

    application.connect_activate(|app| {
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
                let mut contents = vec![];
                if pipe.read_to_end(&mut contents).is_ok() {
                    let text = String::from_utf8_lossy(&contents);
                    let mut text_type = "general";
                    // Figure out contents
                    if text.len() >= 9 && text.len() <= 11 {
                        if text.starts_with("U") || text.starts_with("W") {
                            text_type = "slackUser"
                        }
                    }
                    container.append(&Label::new(Some(&format!("Pasted: {}", text))));
                    if text_type == "slackUser" {
                        let oisbtn = Button::with_label("Open In Slack");
                        oisbtn.connect_clicked(move |_| {
                            let url = format!("https://app.slack.com/team/{}", text);
                            let _ = webbrowser::open(&url);
                        });

                        container.append(&oisbtn);
                    }
                    container.append(&Button::with_label("Meow3"));
                } else {
                    container.append(&Label::new(Some("Failed to read clipboard data pipe.")));
                    container.append(&Button::with_label("Close"));
                }
            }
            Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {
                container.append(&Label::new(Some(
                    "Clipboard is empty or does not contain text.",
                )));
                container.append(&Button::with_label("Close"));
            }
            Err(_) => {
                container.append(&Label::new(Some(
                    "Error occurred while fetching contents from the clipboard.",
                )));
                container.append(&Button::with_label("Close"));
            }
        };

        // Set the container as the single child of the window
        window.set_child(Some(&container));
        window.present();
    });

    application.run();

    Ok(())
}
