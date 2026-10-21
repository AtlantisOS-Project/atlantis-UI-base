// renderer.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! How GTK draws the windows: on the processor or on the graphics card.
//!
//! GTK 4 chooses a renderer itself: the graphics card (`ngl`, `vulkan`) when it can, the
//! processor (`cairo`) otherwise. A program normally leaves that to GTK. The renderers differ in
//! memory: the driver of the graphics card (Mesa, shaders, textures) costs memory, so a small window
//! takes about 60 MB drawn on the processor (`cairo`), about 115 MB with the default and 160 to
//! 220 MB with `vulkan` or `ngl`. A program that wants to decide (a small program with forms
//! and lists looks the same in every renderer; one with large animations or video wants the
//! graphics card) calls [`set_renderer`].
//!
//! Call [`set_renderer`] first thing in `main`, before GTK is initialized (before
//! `Application::run`). A renderer the user has chosen with `GSK_RENDERER` is never overridden.

use std::ffi::OsStr;

/// The way the windows are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Renderer {
    /// On the processor (`cairo`): the least memory, no driver of the graphics card is loaded.
    Software,
    /// On the graphics card with OpenGL (`ngl`).
    Gpu,
    /// On the graphics card with Vulkan (`vulkan`).
    Vulkan,
    /// What GTK decides: the graphics card when there is one, the processor otherwise.
    Auto,
}

impl Renderer {
    /// The value for `GSK_RENDERER`, or `None` if GTK decides.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::ui_prelude::Renderer;
    ///
    /// assert_eq!(Renderer::Software.gsk_name(), Some("cairo"));
    /// assert_eq!(Renderer::Vulkan.gsk_name(), Some("vulkan"));
    /// assert_eq!(Renderer::Auto.gsk_name(), None);
    /// ```
    pub const fn gsk_name(self) -> Option<&'static str> {
        match self {
            Renderer::Software => Some("cairo"),
            Renderer::Gpu => Some("ngl"),
            Renderer::Vulkan => Some("vulkan"),
            Renderer::Auto => None,
        }
    }
}

/// What `GSK_RENDERER` should become: nothing if the user has set it, GTK decides, or the windows
/// are shown in a browser (Broadway has a renderer of its own, which no other one can replace).
fn value_to_set(
    current: Option<&OsStr>,
    backend: Option<&OsStr>,
    renderer: Renderer,
) -> Option<&'static str> {
    if current.is_some_and(|value| !value.is_empty()) {
        return None;
    }
    if backend.is_some_and(|backend| backend.to_string_lossy().contains("broadway")) {
        return None;
    }
    renderer.gsk_name()
}

/// Chooses how GTK draws the windows (see the [module documentation](self)).
///
/// The choice is made with the environment variable `GSK_RENDERER`; if the user has set it, the
/// user's choice stays and the function does nothing. It also does nothing with
/// `GDK_BACKEND=broadway` (the windows are shown in a browser, which needs the renderer of
/// Broadway). Call it once at the start of `main`,
/// before other threads exist (it changes the environment of the process) and before GTK is
/// initialized.
///
/// # Returns
///
/// `true` if the renderer was set by this call.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::{Renderer, set_renderer};
///
/// fn main() {
///     // draw on the processor and save the memory of the graphics driver
///     set_renderer(Renderer::Software);
///     // … create the application and run it
/// }
/// ```
pub fn set_renderer(renderer: Renderer) -> bool {
    let current = std::env::var_os("GSK_RENDERER");
    let backend = std::env::var_os("GDK_BACKEND");
    let Some(value) = value_to_set(current.as_deref(), backend.as_deref(), renderer) else {
        return false;
    };
    // SAFETY: documented to be called from `main` before any thread is started.
    unsafe { std::env::set_var("GSK_RENDERER", value) };
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_choice_of_the_user_wins() {
        assert_eq!(value_to_set(None, None, Renderer::Software), Some("cairo"));
        assert_eq!(
            value_to_set(Some("".as_ref()), None, Renderer::Gpu),
            Some("ngl")
        );
        assert_eq!(
            value_to_set(Some("vulkan".as_ref()), None, Renderer::Software),
            None
        );
        assert_eq!(value_to_set(None, None, Renderer::Auto), None);
    }

    #[test]
    fn broadway_keeps_its_own_renderer() {
        assert_eq!(
            value_to_set(None, Some("broadway".as_ref()), Renderer::Software),
            None
        );
        assert_eq!(
            value_to_set(None, Some("wayland,x11".as_ref()), Renderer::Software),
            Some("cairo")
        );
    }
}
