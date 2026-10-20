// stack_window.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The window of a program that shows its pages in a stack.

use adw::prelude::*;
use gtk4::{PolicyType, ScrolledWindow, Stack, StackTransitionType};

/// A window with a header bar and a [`gtk4::Stack`] for the pages.
///
/// The window has a fixed size: what does not fit scrolls, and a page can never make the
/// window larger (important for pages whose text depends on the font of the user).
///
/// # Examples
///
/// ```no_run
/// use adw::prelude::*;
/// use atlbase::ui_prelude::{StackWindow, show_stack_page};
///
/// let app = adw::Application::builder().application_id("org.example.Demo").build();
/// app.connect_activate(|app| {
///     let window = StackWindow::new(app, "Demo", 800, 600);
///     // window.header_bar.pack_end(&menu_button);
///     window.present(|stack| {
///         show_stack_page(stack, "home", false, || gtk4::Label::new(Some("Hello")).into());
///     });
/// });
/// app.run();
/// ```
pub struct StackWindow {
    /// The window.
    pub window: adw::ApplicationWindow,
    /// The header bar; add your own buttons before you call [`present`](Self::present).
    pub header_bar: adw::HeaderBar,
    /// The pages. Add them with [`show_stack_page`](super::stack_pages::show_stack_page).
    pub stack: Stack,
}

impl StackWindow {
    /// Creates the window (not shown yet) with `title` in the header bar and a content area
    /// of `width` × `height` pixels.
    pub fn new(app: &adw::Application, title: &str, width: i32, height: i32) -> Self {
        let header_bar = adw::HeaderBar::new();
        header_bar.add_css_class("flat");
        header_bar.set_title_widget(Some(&adw::WindowTitle::new(title, "")));

        let stack = Stack::builder()
            .transition_type(StackTransitionType::SlideLeftRight)
            .transition_duration(300)
            .hexpand(true)
            .vexpand(true)
            .build();
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .hexpand(true)
            .vexpand(true)
            .child(&stack)
            .build();

        let toolbar_view = adw::ToolbarView::new();
        toolbar_view.add_top_bar(&header_bar);
        toolbar_view.set_content(Some(&scrolled));
        toolbar_view.set_size_request(width, height);

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .content(&toolbar_view)
            .default_width(width)
            .default_height(height)
            .build();
        Self {
            window,
            header_bar,
            stack,
        }
    }

    /// Shows the window and calls `show_first` to put the first page into the stack.
    ///
    /// There is no slide animation while `show_first` runs: the first thing the user sees is
    /// the first page, not an animation to it.
    pub fn present(&self, show_first: impl FnOnce(&Stack)) {
        self.window.present();
        self.stack.set_transition_type(StackTransitionType::None);
        show_first(&self.stack);

        let stack = self.stack.clone();
        glib::idle_add_local_once(move || {
            stack.set_transition_type(StackTransitionType::SlideLeftRight)
        });
    }
}
