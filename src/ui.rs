use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, ButtonsType, ComboBoxText, Entry,
    FileChooserAction, FileChooserDialog, Label, MessageDialog, MessageType, Notebook, Orientation,
    ResponseType, ScrolledWindow, TextView,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use crate::config::Config;
use crate::patcher::Patcher;
use crate::proton::ProtonLauncher;

pub fn build_ui(app: &Application) {
    let config = Rc::new(RefCell::new(Config::load()));

    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GYeet - VotV Patcher")
        .default_width(1000)
        .default_height(800)
        .build();

    // Apply CSS styling
    apply_css();

    // Main container
    let main_box = GtkBox::new(Orientation::Vertical, 10);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    // Title
    let title = Label::new(Some("GYeet - VotV Patcher"));
    title.add_css_class("title");
    main_box.append(&title);

    // Subtitle
    let subtitle = Label::new(Some("Developed by ModSauce"));
    subtitle.add_css_class("subtitle");
    main_box.append(&subtitle);

    // Create notebook (tabs)
    let notebook = Notebook::new();
    notebook.set_vexpand(true);

    // Console output (shared across tabs)
    let console_scroll = ScrolledWindow::new();
    console_scroll.set_min_content_height(200);
    let console = TextView::new();
    console.set_editable(false);
    console.add_css_class("console");
    console_scroll.set_child(Some(&console));

    // Create tabs
    let patch_tab = create_patch_tab(config.clone(), console.clone(), window.clone());
    let install_tab = create_install_tab(config.clone(), console.clone(), window.clone());
    let launch_tab = create_launch_tab(config.clone(), console.clone(), window.clone());
    let settings_tab = create_settings_tab(config.clone(), console.clone(), window.clone());

    notebook.append_page(&patch_tab, Some(&Label::new(Some("Patch/Update"))));
    notebook.append_page(&install_tab, Some(&Label::new(Some("Install"))));
    notebook.append_page(&launch_tab, Some(&Label::new(Some("Launch Game"))));
    notebook.append_page(&settings_tab, Some(&Label::new(Some("Settings"))));

    main_box.append(&notebook);

    // Console section
    let console_label = Label::new(Some("Console Output"));
    console_label.set_halign(gtk4::Align::Start);
    console_label.add_css_class("section-title");
    main_box.append(&console_label);
    main_box.append(&console_scroll);

    window.set_child(Some(&main_box));
    window.present();
}

fn create_patch_tab(
    config: Rc<RefCell<Config>>,
    console: TextView,
    window: ApplicationWindow,
) -> GtkBox {
    let vbox = GtkBox::new(Orientation::Vertical, 15);
    vbox.set_margin_top(15);
    vbox.set_margin_bottom(15);
    vbox.set_margin_start(15);
    vbox.set_margin_end(15);

    // VotV.exe path selection
    let path_label = Label::new(Some("Game Location (VotV.exe)"));
    path_label.set_halign(gtk4::Align::Start);
    vbox.append(&path_label);

    let path_box = GtkBox::new(Orientation::Horizontal, 10);
    let votv_path_entry = Entry::new();
    votv_path_entry.set_placeholder_text(Some("Path to VotV.exe..."));
    votv_path_entry.set_text(&config.borrow().votv_exe_path);
    votv_path_entry.set_hexpand(true);
    path_box.append(&votv_path_entry);

    let browse_btn = Button::with_label("Browse...");
    let window_clone = window.clone();
    let entry_clone = votv_path_entry.clone();
    let config_clone = config.clone();
    browse_btn.connect_clicked(move |_| {
        browse_file(&window_clone, &entry_clone, "Select VotV.exe", "*.exe");
        let new_path = entry_clone.text().to_string();
        config_clone.borrow_mut().votv_exe_path = new_path;
        if let Err(e) = config_clone.borrow().save() {
            eprintln!("Failed to save config: {}", e);
        }
    });
    path_box.append(&browse_btn);
    vbox.append(&path_box);

    // Also save when text is changed manually
    let config_clone = config.clone();
    let entry_clone = votv_path_entry.clone();
    votv_path_entry.connect_changed(move |_| {
        let new_path = entry_clone.text().to_string();
        config_clone.borrow_mut().votv_exe_path = new_path;
        if let Err(e) = config_clone.borrow().save() {
            eprintln!("Failed to save config: {}", e);
        }
    });

    // Patch button
    let patch_btn = Button::with_label("Check for Updates & Patch");
    patch_btn.add_css_class("primary-button");
    patch_btn.set_margin_top(20);

    let _config_clone = config.clone();
    let console_clone = console.clone();
    let window_clone = window.clone();
    let entry_clone = votv_path_entry.clone();
    let patch_btn_clone = patch_btn.clone();
    patch_btn.connect_clicked(move |_| {
        let votv_path = entry_clone.text().to_string();
        if votv_path.is_empty() {
            show_error(&window_clone, "Please select VotV.exe first!");
            return;
        }

        log_to_console(&console_clone, "Starting patch process...");
        patch_btn_clone.set_sensitive(false);

        let (tx, rx) = mpsc::channel::<String>();
        setup_progress_receiver(rx, console_clone.clone(), Some(patch_btn_clone.clone()));

        let votv_path_clone = votv_path.clone();
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            let patcher = Patcher::new();
            let result = patcher.run_update(&votv_path_clone, |line| {
                if tx_clone.send(line.to_string()).is_err() {
                    // Stop if receiver has been dropped
                    return;
                }
            });

            let msg = match result {
                Ok(0) => "Patch completed successfully!".to_string(),
                Ok(code) => format!("Patch exited with code: {}", code),
                Err(e) => format!("Patch failed: {}", e),
            };
            let _ = tx.send(msg);
            let _ = tx.send("DONE".to_string());
        });
    });
    vbox.append(&patch_btn);

    vbox
}

fn create_install_tab(
    config: Rc<RefCell<Config>>,
    console: TextView,
    window: ApplicationWindow,
) -> GtkBox {
    let vbox = GtkBox::new(Orientation::Vertical, 15);
    vbox.set_margin_top(15);
    vbox.set_margin_bottom(15);
    vbox.set_margin_start(15);
    vbox.set_margin_end(15);

    // Install directory selection
    let dir_label = Label::new(Some("Installation Directory"));
    dir_label.set_halign(gtk4::Align::Start);
    vbox.append(&dir_label);

    let dir_box = GtkBox::new(Orientation::Horizontal, 10);
    let install_dir_entry = Entry::new();
    install_dir_entry.set_placeholder_text(Some("Where to install VotV..."));
    install_dir_entry.set_text(&config.borrow().install_dir);
    install_dir_entry.set_hexpand(true);
    dir_box.append(&install_dir_entry);

    let browse_dir_btn = Button::with_label("Browse...");
    let window_clone = window.clone();
    let entry_clone = install_dir_entry.clone();
    let config_clone = config.clone();
    browse_dir_btn.connect_clicked(move |_| {
        browse_folder(&window_clone, &entry_clone, "Select Installation Directory");
        config_clone.borrow_mut().install_dir = entry_clone.text().to_string();
    });
    dir_box.append(&browse_dir_btn);
    vbox.append(&dir_box);

    // Version selection
    let version_label = Label::new(Some("Game Version"));
    version_label.set_halign(gtk4::Align::Start);
    version_label.set_margin_top(10);
    vbox.append(&version_label);

    let version_box = GtkBox::new(Orientation::Horizontal, 10);
    let version_combo = ComboBoxText::new();
    version_combo.append_text("Click Refresh to load versions");
    version_combo.set_active(Some(0));
    version_combo.set_hexpand(true);
    version_box.append(&version_combo);

    // Store versions in a shared RefCell
    let available_versions: Rc<RefCell<Vec<crate::patcher::GameVersion>>> =
        Rc::new(RefCell::new(Vec::new()));

    let refresh_btn = Button::with_label("🔄 Refresh");
    let combo_clone = version_combo.clone();
    let console_clone2 = console.clone();
    let versions_rc = available_versions.clone();
    refresh_btn.connect_clicked(move |_| {
        log_to_console(&console_clone2, "Fetching available versions...");
        combo_clone.remove_all();
        combo_clone.append_text("Loading...");
        combo_clone.set_active(Some(0));

        let (tx, rx) = mpsc::channel::<Result<Vec<crate::patcher::GameVersion>, String>>();
        let combo = combo_clone.clone();
        let console = console_clone2.clone();
        let versions = versions_rc.clone();

        // Set up receiver
        glib::idle_add_local(move || {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(vers) => {
                        combo.remove_all();
                        for version in &vers {
                            combo.append_text(&version.name);
                        }
                        if !vers.is_empty() {
                            combo.set_active(Some(0));
                        }
                        *versions.borrow_mut() = vers.clone();
                        log_to_console(&console, &format!("✅ Found {} versions", vers.len()));
                    }
                    Err(e) => {
                        combo.remove_all();
                        combo.append_text("Error - try again");
                        combo.set_active(Some(0));
                        log_to_console(&console, &format!("❌ Failed to fetch versions: {}", e));
                    }
                }
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });

        std::thread::spawn(move || {
            let result = crate::patcher::fetch_game_versions();
            let _ = tx.send(result);
        });
    });
    version_box.append(&refresh_btn);
    vbox.append(&version_box);

    // Install button
    let install_btn = Button::with_label("Install VotV");
    install_btn.add_css_class("primary-button");
    install_btn.set_margin_top(20);

    let console_clone = console.clone();
    let window_clone = window.clone();
    let entry_clone = install_dir_entry.clone();
    let combo_clone = version_combo.clone();
    let versions_clone = available_versions.clone();
    let install_btn_clone = install_btn.clone();
    install_btn.connect_clicked(move |_| {
        let install_dir = entry_clone.text().to_string();
        if install_dir.is_empty() {
            show_error(
                &window_clone,
                "Please select an installation directory first!",
            );
            return;
        }

        // Get selected version
        let selected_idx = match combo_clone.active() {
            Some(idx) => idx as usize,
            None => {
                show_error(&window_clone, "Please select a version first!");
                return;
            }
        };

        let versions = versions_clone.borrow();
        if versions.is_empty() {
            show_error(
                &window_clone,
                "Please click Refresh to load versions first!",
            );
            return;
        }

        if selected_idx >= versions.len() {
            show_error(&window_clone, "Invalid version selection!");
            return;
        }

        let selected_version = versions[selected_idx].clone();
        drop(versions); // Release the borrow

        log_to_console(&console_clone, "Starting install process...");
        install_btn_clone.set_sensitive(false);

        let (tx, rx) = mpsc::channel::<String>();
        setup_progress_receiver(rx, console_clone.clone(), Some(install_btn_clone.clone()));

        let install_dir_clone = install_dir.clone();
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            let patcher = Patcher::new();
            let result = patcher.run_install(&install_dir_clone, &selected_version, move |line| {
                if tx_clone.send(line.to_string()).is_err() {
                    // Stop if receiver has been dropped
                    return;
                }
            });

            let msg = match result {
                Ok(0) => "Install completed successfully!".to_string(),
                Ok(code) => format!("Install exited with code: {}", code),
                Err(e) => format!("Install failed: {}", e),
            };
            let _ = tx.send(msg);
            let _ = tx.send("DONE".to_string());
        });
    });
    vbox.append(&install_btn);

    vbox
}

fn create_launch_tab(
    config: Rc<RefCell<Config>>,
    console: TextView,
    window: ApplicationWindow,
) -> GtkBox {
    let vbox = GtkBox::new(Orientation::Vertical, 15);
    vbox.set_margin_top(15);
    vbox.set_margin_bottom(15);
    vbox.set_margin_start(15);
    vbox.set_margin_end(15);

    // Info label
    let info = Label::new(Some("Launch VotV using Proton (Steam compatibility layer)"));
    info.set_wrap(true);
    info.set_halign(gtk4::Align::Start);
    vbox.append(&info);

    // Proton version selection
    let proton_label = Label::new(Some("Proton Version:"));
    proton_label.set_halign(gtk4::Align::Start);
    proton_label.set_margin_top(15);
    vbox.append(&proton_label);

    let proton_combo = ComboBoxText::new();
    proton_combo.append_text("Auto-detect");
    proton_combo.append_text("Proton 9.0");
    proton_combo.append_text("Proton 8.0");
    proton_combo.append_text("Proton Experimental");
    proton_combo.set_active(Some(0));
    vbox.append(&proton_combo);

    // Detect Proton button
    let detect_btn = Button::with_label("Detect Proton Installations");
    let config_clone = config.clone();
    let console_clone = console.clone();
    let combo_clone = proton_combo.clone();
    let window_clone = window.clone();
    detect_btn.connect_clicked(move |_| {
        let steam_path = config_clone.borrow().steam_path.clone();
        if steam_path.is_empty() {
            show_error(&window_clone, "Please set Steam path in Settings first!");
            return;
        }

        log_to_console(&console_clone, "Detecting Proton installations...");
        let launcher = ProtonLauncher::new(steam_path);
        let versions = launcher.detect_proton_versions();

        if versions.is_empty() {
            log_to_console(&console_clone, "No Proton installations found!");
        } else {
            log_to_console(
                &console_clone,
                &format!("Found {} Proton version(s)", versions.len()),
            );

            // Clear and repopulate combo box
            combo_clone.remove_all();
            combo_clone.append_text("Auto-detect");

            for version in &versions {
                log_to_console(&console_clone, &format!("  - {}", version));
                combo_clone.append_text(version);
            }
            combo_clone.set_active(Some(0));
        }
    });
    vbox.append(&detect_btn);

    // Launch button
    let launch_btn = Button::with_label("Launch VotV with Proton");
    launch_btn.add_css_class("primary-button");
    launch_btn.set_margin_top(20);

    let config_clone = config.clone();
    let console_clone = console.clone();
    let window_clone = window.clone();
    let combo_clone = proton_combo.clone();
    launch_btn.connect_clicked(move |_| {
        let votv_path = config_clone.borrow().votv_exe_path.clone();
        if votv_path.is_empty() {
            show_error(
                &window_clone,
                "Please select VotV.exe in the Patch/Update tab first!",
            );
            return;
        }

        let steam_path = config_clone.borrow().steam_path.clone();
        if steam_path.is_empty() {
            show_error(&window_clone, "Please set Steam path in Settings first!");
            return;
        }

        let proton_version = combo_clone
            .active_text()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Auto-detect".to_string());

        log_to_console(
            &console_clone,
            &format!("Launching VotV with Proton ({})...", proton_version),
        );

        let launcher = ProtonLauncher::new(steam_path);
        let console_clone2 = console_clone.clone();

        match launcher.launch_votv(&votv_path, &proton_version, |line| {
            log_to_console(&console_clone2, &line);
        }) {
            Ok(_) => log_to_console(&console_clone, "✅ Game launched successfully!"),
            Err(e) => log_to_console(&console_clone, &format!("❌ Failed to launch game: {}", e)),
        }
    });
    vbox.append(&launch_btn);

    vbox
}

fn setup_progress_receiver(rx: mpsc::Receiver<String>, console: TextView, button: Option<Button>) {
    glib::idle_add_local(move || match rx.try_recv() {
        Ok(msg) => {
            if msg == "DONE" {
                if let Some(button) = &button {
                    button.set_sensitive(true);
                }
                return glib::ControlFlow::Break;
            } else {
                log_to_console(&console, &msg);
            }
            glib::ControlFlow::Continue
        }
        Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
        Err(mpsc::TryRecvError::Disconnected) => {
            if let Some(button) = &button {
                button.set_sensitive(true);
            }
            glib::ControlFlow::Break
        }
    });
}

fn create_settings_tab(
    config: Rc<RefCell<Config>>,
    console: TextView,
    window: ApplicationWindow,
) -> GtkBox {
    let vbox = GtkBox::new(Orientation::Vertical, 15);
    vbox.set_margin_top(15);
    vbox.set_margin_bottom(15);
    vbox.set_margin_start(15);
    vbox.set_margin_end(15);

    // Steam path
    let steam_label = Label::new(Some("Steam Installation"));
    steam_label.set_halign(gtk4::Align::Start);
    steam_label.set_margin_top(15);
    vbox.append(&steam_label);

    let steam_box = GtkBox::new(Orientation::Horizontal, 10);
    let steam_entry = Entry::new();
    steam_entry.set_placeholder_text(Some("Path to Steam directory..."));
    steam_entry.set_text(&config.borrow().steam_path);
    steam_entry.set_hexpand(true);
    steam_box.append(&steam_entry);

    let browse_steam_btn = Button::with_label("Browse...");
    let window_clone = window.clone();
    let entry_clone = steam_entry.clone();
    let config_clone = config.clone();
    browse_steam_btn.connect_clicked(move |_| {
        browse_folder(&window_clone, &entry_clone, "Select Steam Directory");
        config_clone.borrow_mut().steam_path = entry_clone.text().to_string();
    });
    steam_box.append(&browse_steam_btn);
    vbox.append(&steam_box);

    // Save settings button
    let save_btn = Button::with_label("Save Settings");
    save_btn.add_css_class("primary-button");
    save_btn.set_margin_top(20);

    let config_clone = config.clone();
    let console_clone = console.clone();
    let window_clone = window.clone();
    let steam_clone = steam_entry.clone();
    save_btn.connect_clicked(move |_| {
        let mut cfg = config_clone.borrow_mut();
        cfg.steam_path = steam_clone.text().to_string();

        match cfg.save() {
            Ok(_) => {
                log_to_console(&console_clone, "✅ Settings saved successfully!");
                show_info(&window_clone, "Settings saved successfully!");
            }
            Err(e) => {
                log_to_console(
                    &console_clone,
                    &format!("❌ Failed to save settings: {}", e),
                );
                show_error(&window_clone, &format!("Failed to save settings: {}", e));
            }
        }
    });
    vbox.append(&save_btn);

    vbox
}

// Helper functions
fn browse_file(window: &ApplicationWindow, entry: &Entry, title: &str, _pattern: &str) {
    let dialog = FileChooserDialog::builder()
        .title(title)
        .transient_for(window)
        .action(FileChooserAction::Open)
        .build();

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Open", ResponseType::Accept);

    let entry = entry.clone();
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            if let Some(file) = dialog.file() {
                if let Some(path) = file.path() {
                    entry.set_text(&path.to_string_lossy());
                }
            }
        }
        dialog.close();
    });

    dialog.show();
}

fn browse_folder(window: &ApplicationWindow, entry: &Entry, title: &str) {
    let dialog = FileChooserDialog::builder()
        .title(title)
        .transient_for(window)
        .action(FileChooserAction::SelectFolder)
        .build();

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Select", ResponseType::Accept);

    let entry = entry.clone();
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            if let Some(file) = dialog.file() {
                if let Some(path) = file.path() {
                    entry.set_text(&path.to_string_lossy());
                }
            }
        }
        dialog.close();
    });

    dialog.show();
}

fn log_to_console(console: &TextView, message: &str) {
    let buffer = console.buffer();
    let mut end_iter = buffer.end_iter();
    buffer.insert(&mut end_iter, &format!("{}\n", message));

    // Auto-scroll to bottom
    let mark = buffer.create_mark(None, &buffer.end_iter(), false);
    console.scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);
}

fn show_error(window: &ApplicationWindow, message: &str) {
    let dialog = MessageDialog::new(
        Some(window),
        gtk4::DialogFlags::MODAL,
        MessageType::Error,
        ButtonsType::Ok,
        message,
    );
    dialog.connect_response(|dialog, _| dialog.close());
    dialog.show();
}

fn show_info(window: &ApplicationWindow, message: &str) {
    let dialog = MessageDialog::new(
        Some(window),
        gtk4::DialogFlags::MODAL,
        MessageType::Info,
        ButtonsType::Ok,
        message,
    );
    dialog.connect_response(|dialog, _| dialog.close());
    dialog.show();
}

fn apply_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        r#"
        .title {
            font-size: 26pt;
            font-weight: bold;
            margin-bottom: 10px;
            color: #b794f4;
        }

        .subtitle {
            font-size: 10pt;
            color: #9f7aea;
            margin-bottom: 15px;
        }

        .section-title {
            font-size: 12pt;
            font-weight: bold;
            margin-top: 10px;
            color: #d6bcfa;
        }

        .primary-button {
            min-height: 50px;
            font-size: 12pt;
            font-weight: bold;
            background: linear-gradient(135deg, #7c3aed 0%, #a855f7 100%);
            color: white;
            border-radius: 8px;
            border: none;
            padding: 12px 24px;
        }

        .primary-button:hover {
            background: linear-gradient(135deg, #8b5cf6 0%, #c084fc 100%);
        }

        .secondary-button {
            background: #4c1d95;
            color: #e9d5ff;
            border-radius: 6px;
            border: 1px solid #6d28d9;
        }

        .secondary-button:hover {
            background: #5b21b6;
        }

        .console {
            font-family: 'Fira Code', 'Consolas', monospace;
            background-color: #1a1a2e;
            color: #e9d5ff;
            padding: 10px;
            border-radius: 6px;
        }

        progressbar {
            min-height: 24px;
            border-radius: 4px;
        }

        progressbar progress {
            background: linear-gradient(90deg, #7c3aed 0%, #a855f7 100%);
            border-radius: 4px;
        }

        progressbar trough {
            background-color: #2d2d44;
            border-radius: 4px;
        }

        entry {
            background-color: #2d2d44;
            color: #e9d5ff;
            border: 1px solid #4c1d95;
            border-radius: 4px;
            padding: 8px;
        }

        entry:focus {
            border-color: #7c3aed;
        }

        combobox button {
            background-color: #2d2d44;
            color: #e9d5ff;
            border: 1px solid #4c1d95;
            border-radius: 4px;
        }

        notebook {
            background-color: #16213e;
        }

        notebook header {
            background-color: #1a1a2e;
        }

        notebook tab {
            background-color: #2d2d44;
            color: #c4b5fd;
            border-radius: 6px 6px 0 0;
            padding: 10px 20px;
            margin: 2px;
        }

        notebook tab:checked {
            background: linear-gradient(180deg, #7c3aed 0%, #6d28d9 100%);
            color: white;
        }

        window {
            background-color: #16213e;
        }

        label {
            color: #e9d5ff;
        }
        "#,
    );

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
