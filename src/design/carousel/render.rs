// render.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Turns the data structures of [`model`](super::model) into GTK widgets.
//!
//! [`page_widget`] and [`block_widget`] are public, so the pages can be used without a
//! [`GuideView`](super::GuideView) — for example in an `adw::NavigationPage` of your own.

use std::time::Duration;

use adw::prelude::*;
use gtk4::glib::{self, clone};
use gtk4::{gdk, gio, pango};

use super::model::{
    ActionButton, Block, Callout, CalloutKind, Image, ImageSource, Labels, Page, Step,
};
use super::style::ensure_default_style;
use super::view::GuideView;

/// Renders a page (header and all blocks) as a vertical box.
///
/// The box neither scrolls nor limits its width; that is up to the caller. The default style
/// of the guide widgets is registered on first use.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::{Labels, Page, page_widget};
/// # gtk4::init().unwrap();
///
/// let page = Page::new("Welcome").paragraph("Hello!");
/// let widget = page_widget(&page, &Labels::default());
/// # let _ = widget;
/// ```
pub fn page_widget(page: &Page, labels: &Labels) -> gtk4::Widget {
    ensure_default_style();
    let content = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(18)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(18)
        .margin_end(18)
        .build();

    content.append(&header(page));
    for block in &page.blocks {
        content.append(&block_widget(block, labels));
    }
    content.upcast()
}

/// Renders a single block.
///
/// `labels` supplies the texts that belong to the block itself (the tooltip of the copy
/// button of code blocks and the placeholder of missing images).
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::{Block, Labels, block_widget};
/// # gtk4::init().unwrap();
///
/// let widget = block_widget(&Block::Code("adb devices".into()), &Labels::default());
/// # let _ = widget;
/// ```
pub fn block_widget(block: &Block, labels: &Labels) -> gtk4::Widget {
    match block {
        Block::Heading(text) => gtk4::Label::builder()
            .label(text)
            .xalign(0.0)
            .wrap(true)
            .css_classes(["title-3"])
            .build()
            .upcast(),
        Block::Paragraph { text, markup } => text_label(text, *markup).upcast(),
        Block::Bullets(items) => bullets(items),
        Block::Steps { start, items } => steps(*start, items),
        Block::Code(text) => code(text, labels),
        Block::Image(image) => picture(image, labels),
        Block::Callout(callout) => callout_box(callout),
        Block::Button(button) => action_button(button),
    }
}

/// A page in a ScrolledWindow and a Clamp, as it is used in the carousel.
pub(crate) fn scrolled_page(page: &Page, labels: &Labels) -> gtk4::Widget {
    let clamp = adw::Clamp::builder()
        .maximum_size(640)
        .tightening_threshold(520)
        .child(&page_widget(page, labels))
        .build();
    // A viewport of our own with the "natural" height: otherwise the ScrolledWindow gives the
    // content only its *minimum* height, and shrinkable elements (images) are squeezed
    // instead of the page scrolling.
    let viewport = gtk4::Viewport::builder()
        .vscroll_policy(gtk4::ScrollablePolicy::Natural)
        .scroll_to_focus(false)
        .child(&clamp)
        .build();
    gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .hexpand(true)
        .vexpand(true)
        .child(&viewport)
        .build()
        .upcast()
}

fn header(page: &Page) -> gtk4::Widget {
    let row = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(14)
        .build();

    if let Some(icon) = &page.icon {
        row.append(
            &gtk4::Image::builder()
                .icon_name(icon)
                .pixel_size(48)
                .valign(gtk4::Align::Center)
                .build(),
        );
    }

    let titles = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(4)
        .hexpand(true)
        .valign(gtk4::Align::Center)
        .build();
    titles.append(
        &gtk4::Label::builder()
            .label(&page.title)
            .xalign(0.0)
            .wrap(true)
            .css_classes(["title-1"])
            .build(),
    );
    if let Some(subtitle) = &page.subtitle {
        titles.append(
            &gtk4::Label::builder()
                .label(subtitle)
                .xalign(0.0)
                .wrap(true)
                .css_classes(["dim-label"])
                .build(),
        );
    }
    row.append(&titles);
    row.upcast()
}

fn text_label(text: &str, markup: bool) -> gtk4::Label {
    gtk4::Label::builder()
        .label(text)
        .use_markup(markup)
        .xalign(0.0)
        .wrap(true)
        .wrap_mode(pango::WrapMode::WordChar)
        .build()
}

fn bullets(items: &[String]) -> gtk4::Widget {
    let list = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(6)
        .build();
    for item in items {
        let row = gtk4::Box::builder().spacing(10).build();
        row.append(
            &gtk4::Label::builder()
                .label("•")
                .valign(gtk4::Align::Start)
                .build(),
        );
        let label = text_label(item, false);
        label.set_hexpand(true);
        row.append(&label);
        list.append(&row);
    }
    list.upcast()
}

fn steps(start: u32, items: &[Step]) -> gtk4::Widget {
    let list = gtk4::ListBox::builder()
        .selection_mode(gtk4::SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();

    for (i, step) in items.iter().enumerate() {
        let number = gtk4::Label::builder()
            .label((start + i as u32).to_string())
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::Center)
            .css_classes(["guide-step-number"])
            .build();

        let row = adw::ActionRow::builder()
            .title(&step.title)
            .use_markup(false)
            .build();
        if let Some(detail) = &step.detail {
            row.set_subtitle(detail);
        }
        row.add_prefix(&number);
        list.append(&row);
    }
    list.upcast()
}

fn code(text: &str, labels: &Labels) -> gtk4::Widget {
    let root = gtk4::Box::builder()
        .spacing(6)
        .css_classes(["guide-code"])
        .build();

    let label = gtk4::Label::builder()
        .label(text)
        .xalign(0.0)
        .wrap(true)
        .wrap_mode(pango::WrapMode::WordChar)
        .selectable(true)
        .hexpand(true)
        .valign(gtk4::Align::Center)
        .css_classes(["monospace"])
        .build();
    // prevents the text from being selected automatically when it is shown
    label.set_focusable(false);

    let button = gtk4::Button::builder()
        .icon_name("edit-copy-symbolic")
        .valign(gtk4::Align::Center)
        .tooltip_text(&labels.copy)
        .css_classes(["flat"])
        .build();

    let text = text.to_owned();
    button.connect_clicked(move |button| {
        button.clipboard().set_text(&text);
        button.set_icon_name("object-select-symbolic");
        glib::timeout_add_local_once(
            Duration::from_millis(1500),
            clone!(
                #[weak]
                button,
                move || button.set_icon_name("edit-copy-symbolic")
            ),
        );
    });

    root.append(&label);
    root.append(&button);
    root.upcast()
}

/// A pill button. A click is passed on to the [`GuideView`] the button lives in (nothing
/// happens if the block is used without a `GuideView`).
fn action_button(button: &ActionButton) -> gtk4::Widget {
    let content = adw::ButtonContent::builder().label(&button.label).build();
    if let Some(icon) = &button.icon {
        content.set_icon_name(icon);
    }
    let widget = gtk4::Button::builder()
        .child(&content)
        .halign(gtk4::Align::Center)
        .css_classes(["pill"])
        .build();

    let action = button.action.clone();
    widget.connect_clicked(move |widget| {
        if let Some(view) = widget
            .ancestor(GuideView::static_type())
            .and_downcast::<GuideView>()
        {
            view.emit_by_name::<()>("action", &[&action]);
        }
    });
    widget.upcast()
}

fn callout_box(callout: &Callout) -> gtk4::Widget {
    let (icon, box_class, icon_class) = match callout.kind {
        CalloutKind::Info => (
            "dialog-information-symbolic",
            "guide-callout-info",
            "accent",
        ),
        CalloutKind::Tip => ("starred-symbolic", "guide-callout-tip", "success"),
        CalloutKind::Warning => (
            "dialog-warning-symbolic",
            "guide-callout-warning",
            "warning",
        ),
        CalloutKind::Danger => ("dialog-error-symbolic", "guide-callout-danger", "error"),
    };

    let root = gtk4::Box::builder()
        .spacing(12)
        .css_classes(["guide-callout", box_class])
        .build();
    root.append(
        &gtk4::Image::builder()
            .icon_name(icon)
            .valign(gtk4::Align::Start)
            .css_classes([icon_class])
            .build(),
    );

    let text = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(2)
        .hexpand(true)
        .build();
    if let Some(title) = &callout.title {
        text.append(
            &gtk4::Label::builder()
                .label(title)
                .xalign(0.0)
                .wrap(true)
                .css_classes(["heading"])
                .build(),
        );
    }
    text.append(&text_label(&callout.text, false));
    root.append(&text);
    root.upcast()
}

fn load_texture(source: &ImageSource) -> Result<gdk::Texture, String> {
    match source {
        ImageSource::File(path) => {
            gdk::Texture::from_filename(path).map_err(|e| format!("{}: {e}", path.display()))
        }
        ImageSource::Resource(path) => {
            let bytes = gio::resources_lookup_data(path, gio::ResourceLookupFlags::NONE)
                .map_err(|e| format!("{path}: {e}"))?;
            gdk::Texture::from_bytes(&bytes).map_err(|e| format!("{path}: {e}"))
        }
        ImageSource::Texture(texture) => Ok(texture.clone()),
    }
}

fn picture(image: &Image, labels: &Labels) -> gtk4::Widget {
    let outer = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(8)
        .build();

    match load_texture(&image.source) {
        Ok(texture) => {
            let picture = gtk4::Picture::builder()
                .paintable(&texture)
                .content_fit(gtk4::ContentFit::Fill)
                .can_shrink(true)
                .build();
            picture.set_alternative_text(image.alt.as_deref().or(image.caption.as_deref()));

            // The AspectFrame makes sure the height always matches the width exactly
            // (GtkPicture alone does not calculate this reliably).
            let aspect = texture.width() as f32 / texture.height().max(1) as f32;
            let frame = gtk4::AspectFrame::builder()
                .ratio(aspect)
                .obey_child(false)
                .overflow(gtk4::Overflow::Hidden)
                .css_classes(["card"])
                .child(&picture)
                .build();

            // Limit the width so the height does not exceed `max_height` and small images
            // do not grow beyond their original size.
            let max_width = ((image.max_height as f32 * aspect).round() as i32)
                .min(texture.width())
                .max(1);
            let clamp = adw::Clamp::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .maximum_size(max_width)
                .tightening_threshold(max_width)
                .child(&frame)
                .build();
            outer.append(&clamp);
        }
        Err(err) => {
            glib::g_warning!("atlbase", "image could not be loaded: {}", err);
            let missing = gtk4::Box::builder()
                .spacing(12)
                .halign(gtk4::Align::Center)
                .css_classes(["card", "guide-image-missing"])
                .build();
            missing.append(&gtk4::Image::from_icon_name("image-missing-symbolic"));
            missing.append(
                &gtk4::Label::builder()
                    .label(&labels.image_missing)
                    .css_classes(["dim-label"])
                    .build(),
            );
            outer.append(&missing);
        }
    }

    if let Some(caption) = &image.caption {
        outer.append(
            &gtk4::Label::builder()
                .label(caption)
                .wrap(true)
                .justify(gtk4::Justification::Center)
                .css_classes(["caption", "dim-label"])
                .build(),
        );
    }
    outer.upcast()
}
