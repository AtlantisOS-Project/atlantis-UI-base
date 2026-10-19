// view.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! [`GuideView`]: a ready-made widget that shows a [`Guide`] page by page.
//!
//! Structure:
//!
//! ```text
//! GuideView (adw::Bin)
//! └── Box (vertical)
//!     ├── adw::Carousel      ← one ScrolledWindow per page, swipeable
//!     └── CenterBox          ← [Back]  ● ● ○ ○  [Next]
//! ```

use super::model::{Guide, Labels};
use super::render;
use super::style::ensure_default_style;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use std::cell::{Cell, OnceCell, RefCell};
use std::sync::OnceLock;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct GuideView {
        pub(super) carousel: OnceCell<adw::Carousel>,
        pub(super) back: OnceCell<gtk4::Button>,
        pub(super) next: OnceCell<gtk4::Button>,
        pub(super) pages: RefCell<Vec<gtk4::Widget>>,
        pub(super) labels: RefCell<Labels>,
        pub(super) current: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GuideView {
        const NAME: &'static str = "AtlGuideView";
        type Type = super::GuideView;
        type ParentType = adw::Bin;
    }

    impl ObjectImpl for GuideView {
        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    glib::subclass::Signal::builder("finished").build(),
                    glib::subclass::Signal::builder("action")
                        .param_types([String::static_type()])
                        .build(),
                ]
            })
        }
    }

    impl WidgetImpl for GuideView {}
    impl BinImpl for GuideView {}
}

glib::wrapper! {
    /// Shows a [`Guide`] as a swipeable carousel with back/next navigation.
    ///
    /// # Signals
    ///
    /// * `finished` — emitted when the user presses the left button ("Exit") on the first
    ///   page or the right button ("Done") on the last page. Connect with
    ///   [`connect_finished`](Self::connect_finished).
    /// * `action` — emitted with the name of the action when the user clicks a button block
    ///   ([`ActionButton`](super::ActionButton)). Connect with
    ///   [`connect_action`](Self::connect_action).
    ///
    /// # Style
    ///
    /// The root widget has the CSS class `adw-guide`. See [`GUIDE_CSS`](super::GUIDE_CSS)
    /// for the classes you can override.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use atlbase::ui_prelude::{Guide, GuideView, Page};
    /// # gtk4::init().unwrap();
    /// # let stack = gtk4::Stack::new();
    ///
    /// let guide = Guide::new("Tour").page(Page::new("Welcome").paragraph("Hello!"));
    ///
    /// // the usual case: put the guide into a GtkStack and return to "home" afterwards
    /// GuideView::present_in_stack(&stack, "tour", &guide, "home");
    /// ```
    pub struct GuideView(ObjectSubclass<imp::GuideView>)
        @extends adw::Bin, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl GuideView {
    /// Creates the widget for a guide.
    ///
    /// The default style of the guide widgets is registered on first use.
    pub fn new(guide: &Guide) -> Self {
        ensure_default_style();
        let view: Self = glib::Object::new();
        view.add_css_class("adw-guide");
        view.setup(guide);
        view
    }

    /// Puts the guide into an existing [`gtk4::Stack`] and shows it. This is the usual case.
    ///
    /// * The guide becomes a new stack page called `name` and is shown. If a page with that
    ///   name exists already, it is replaced: the guide always starts on page 1 again, and
    ///   language and content are up to date.
    /// * On "Exit" (first page) or "Done" (last page) the stack switches to the page
    ///   `return_to`.
    ///
    /// If you need other behavior, use [`GuideView::new`] and
    /// [`connect_finished`](Self::connect_finished).
    pub fn present_in_stack(
        stack: &gtk4::Stack,
        name: &str,
        guide: &Guide,
        return_to: &str,
    ) -> Self {
        if let Some(old) = stack.child_by_name(name) {
            stack.remove(&old);
        }

        let view = Self::new(guide);
        let return_to = return_to.to_owned();
        view.connect_finished(clone!(
            #[weak]
            stack,
            move |_| stack.set_visible_child_name(&return_to)
        ));

        stack.add_named(&view, Some(name));
        stack.set_visible_child_name(name);
        view
    }

    /// Shows the guide in an [`adw::Dialog`] that closes itself when the guide is finished.
    ///
    /// A new [`GuideView`] is created for the dialog, so it cannot collide with one that is
    /// already built into the window.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use atlbase::ui_prelude::{Guide, GuideView, Page};
    /// # gtk4::init().unwrap();
    /// # let window = gtk4::Window::new();
    ///
    /// let guide = Guide::new("Tour").page(Page::new("Welcome").paragraph("Hello!"));
    /// GuideView::present_dialog(&guide, &window);
    /// ```
    pub fn present_dialog(guide: &Guide, parent: &impl IsA<gtk4::Widget>) -> adw::Dialog {
        let view = Self::new(guide);

        let header = adw::HeaderBar::new();
        header.set_title_widget(Some(&adw::WindowTitle::new(&guide.title, "")));
        let toolbar = adw::ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(&view));

        let dialog = adw::Dialog::builder()
            .title(&guide.title)
            .content_width(560)
            .content_height(680)
            .child(&toolbar)
            .build();
        view.connect_finished(clone!(
            #[weak]
            dialog,
            move |_| {
                dialog.close();
            }
        ));
        dialog.present(Some(parent));
        dialog
    }

    /// Connects a handler to the `finished` signal: the guide was left at the beginning or
    /// at the end.
    pub fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("finished", false, move |args| {
            let view = args[0]
                .get::<GuideView>()
                .expect("argument 0 of the signal is the GuideView");
            f(&view);
            None
        })
    }

    /// Connects a handler to the `action` signal: the user clicked a button block
    /// ([`ActionButton`](super::ActionButton)). The handler gets the view and the name of the
    /// action.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use atlbase::ui_prelude::{Guide, GuideView, Page};
    /// # gtk4::init().unwrap();
    ///
    /// let guide = Guide::new("Setup")
    ///     .page(Page::new("Folder").button("choose-folder", "Choose folder"));
    /// let view = GuideView::new(&guide);
    /// view.connect_action(|view, action| {
    ///     if action == "choose-folder" {
    ///         println!("the user wants to choose a folder (page {})", view.page());
    ///     }
    /// });
    /// ```
    pub fn connect_action<F: Fn(&Self, &str) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("action", false, move |args| {
            let view = args[0]
                .get::<GuideView>()
                .expect("argument 0 of the signal is the GuideView");
            let action = args[1]
                .get::<String>()
                .expect("argument 1 of the signal is the name of the action");
            f(&view, &action);
            None
        })
    }

    /// The index of the page that is currently shown (starting at 0).
    pub fn page(&self) -> u32 {
        self.imp().current.get()
    }

    /// The number of pages.
    pub fn n_pages(&self) -> u32 {
        self.imp().pages.borrow().len() as u32
    }

    /// Jumps to a page; the index is limited to the valid range.
    pub fn set_page(&self, index: u32, animate: bool) {
        let imp = self.imp();
        let target = {
            let pages = imp.pages.borrow();
            let Some(last) = pages.len().checked_sub(1) else {
                return;
            };
            pages[(index as usize).min(last)].clone()
        };
        let index = index.min(self.n_pages() - 1);
        imp.current.set(index);
        self.update_navigation();
        if let Some(carousel) = imp.carousel.get() {
            carousel.scroll_to(&target, animate);
        }
    }

    fn setup(&self, guide: &Guide) {
        let imp = self.imp();
        *imp.labels.borrow_mut() = guide.labels.clone();

        let carousel = adw::Carousel::builder()
            .hexpand(true)
            .vexpand(true)
            // The mouse wheel would collide with the vertical scrolling of the pages, and
            // dragging with the mouse with the selection of text. Swiping by touch stays on.
            .allow_scroll_wheel(false)
            .allow_mouse_drag(false)
            .build();
        for page in &guide.pages {
            let widget = render::scrolled_page(page, &guide.labels);
            carousel.append(&widget);
            imp.pages.borrow_mut().push(widget);
        }
        carousel.connect_page_changed(clone!(
            #[weak(rename_to = view)]
            self,
            move |_, index| {
                view.imp().current.set(index);
                view.update_navigation();
            }
        ));

        let dots = adw::CarouselIndicatorDots::builder()
            .carousel(&carousel)
            .build();

        let back = gtk4::Button::builder().css_classes(["flat"]).build();
        back.connect_clicked(clone!(
            #[weak(rename_to = view)]
            self,
            move |_| {
                match view.page() {
                    0 => view.emit_by_name::<()>("finished", &[]),
                    n => view.set_page(n - 1, true),
                }
            }
        ));

        let next = gtk4::Button::builder()
            .css_classes(["suggested-action"])
            .build();
        next.connect_clicked(clone!(
            #[weak(rename_to = view)]
            self,
            move |_| {
                let page = view.page();
                if page + 1 >= view.n_pages() {
                    view.emit_by_name::<()>("finished", &[]);
                } else {
                    view.set_page(page + 1, true);
                }
            }
        ));

        let navigation = gtk4::CenterBox::builder()
            .start_widget(&back)
            .center_widget(&dots)
            .end_widget(&next)
            .margin_top(6)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        let root = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .build();
        root.append(&carousel);
        root.append(&navigation);
        self.set_child(Some(&root));

        imp.carousel.set(carousel).expect("setup runs only once");
        imp.back.set(back).expect("setup runs only once");
        imp.next.set(next).expect("setup runs only once");
        self.update_navigation();
    }

    fn update_navigation(&self) {
        let imp = self.imp();
        let (Some(back), Some(next)) = (imp.back.get(), imp.next.get()) else {
            return;
        };
        let labels = imp.labels.borrow();
        let first = imp.current.get() == 0;
        let last = imp.current.get() + 1 >= self.n_pages();

        back.set_label(if first { &labels.exit } else { &labels.back });
        next.set_label(if last { &labels.done } else { &labels.next });
    }
}
