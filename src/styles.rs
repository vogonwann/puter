use gtk::{self, gdk};

pub fn init_styles() -> Option<()> {
    let display = gdk::Display::default()?;
    
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        ".bought-item {
            background-color: alpha(@success_color, 0.15);
            border: 1px solid alpha(@success_color, 0.3);
            border-radius: 6px;
            padding: 6px;
            margin: 3px;
        }"
    );

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    Some(())
}