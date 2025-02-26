// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, Toggle};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwToggleGroup")]
    pub struct ToggleGroup(Object<ffi::AdwToggleGroup, ffi::AdwToggleGroupClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;

    match fn {
        type_ => || ffi::adw_toggle_group_get_type(),
    }
}

impl ToggleGroup {
    #[doc(alias = "adw_toggle_group_new")]
    pub fn new() -> ToggleGroup {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_toggle_group_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ToggleGroup`] objects.
    ///
    /// This method returns an instance of [`ToggleGroupBuilder`](crate::builders::ToggleGroupBuilder) which can be used to create [`ToggleGroup`] objects.
    pub fn builder() -> ToggleGroupBuilder {
        ToggleGroupBuilder::new()
    }

    #[doc(alias = "adw_toggle_group_add")]
    pub fn add(&self, toggle: Toggle) {
        unsafe {
            ffi::adw_toggle_group_add(self.to_glib_none().0, toggle.into_glib_ptr());
        }
    }

    #[doc(alias = "adw_toggle_group_get_active")]
    #[doc(alias = "get_active")]
    pub fn active(&self) -> u32 {
        unsafe { ffi::adw_toggle_group_get_active(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_toggle_group_get_active_name")]
    #[doc(alias = "get_active_name")]
    #[doc(alias = "active-name")]
    pub fn active_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_toggle_group_get_active_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_group_get_can_shrink")]
    #[doc(alias = "get_can_shrink")]
    #[doc(alias = "can-shrink")]
    pub fn can_shrink(&self) -> bool {
        unsafe { from_glib(ffi::adw_toggle_group_get_can_shrink(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_group_get_homogeneous")]
    #[doc(alias = "get_homogeneous")]
    #[doc(alias = "homogeneous")]
    pub fn is_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::adw_toggle_group_get_homogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_group_get_n_toggles")]
    #[doc(alias = "get_n_toggles")]
    #[doc(alias = "n-toggles")]
    pub fn n_toggles(&self) -> u32 {
        unsafe { ffi::adw_toggle_group_get_n_toggles(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_toggle_group_get_toggle")]
    #[doc(alias = "get_toggle")]
    pub fn toggle(&self, index: u32) -> Option<Toggle> {
        unsafe {
            from_glib_none(ffi::adw_toggle_group_get_toggle(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    #[doc(alias = "adw_toggle_group_get_toggle_by_name")]
    #[doc(alias = "get_toggle_by_name")]
    pub fn toggle_by_name(&self, name: &str) -> Option<Toggle> {
        unsafe {
            from_glib_none(ffi::adw_toggle_group_get_toggle_by_name(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_toggle_group_get_toggles")]
    #[doc(alias = "get_toggles")]
    pub fn toggles(&self) -> gtk::SelectionModel {
        unsafe { from_glib_full(ffi::adw_toggle_group_get_toggles(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_group_remove")]
    pub fn remove(&self, toggle: &Toggle) {
        unsafe {
            ffi::adw_toggle_group_remove(self.to_glib_none().0, toggle.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toggle_group_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::adw_toggle_group_remove_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toggle_group_set_active")]
    #[doc(alias = "active")]
    pub fn set_active(&self, active: u32) {
        unsafe {
            ffi::adw_toggle_group_set_active(self.to_glib_none().0, active);
        }
    }

    #[doc(alias = "adw_toggle_group_set_active_name")]
    #[doc(alias = "active-name")]
    pub fn set_active_name(&self, name: Option<&str>) {
        unsafe {
            ffi::adw_toggle_group_set_active_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toggle_group_set_can_shrink")]
    #[doc(alias = "can-shrink")]
    pub fn set_can_shrink(&self, can_shrink: bool) {
        unsafe {
            ffi::adw_toggle_group_set_can_shrink(self.to_glib_none().0, can_shrink.into_glib());
        }
    }

    #[doc(alias = "adw_toggle_group_set_homogeneous")]
    #[doc(alias = "homogeneous")]
    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::adw_toggle_group_set_homogeneous(self.to_glib_none().0, homogeneous.into_glib());
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "active")]
    pub fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<F: Fn(&ToggleGroup) + 'static>(
            this: *mut ffi::AdwToggleGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "active-name")]
    pub fn connect_active_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_name_trampoline<F: Fn(&ToggleGroup) + 'static>(
            this: *mut ffi::AdwToggleGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_active_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "can-shrink")]
    pub fn connect_can_shrink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_shrink_trampoline<F: Fn(&ToggleGroup) + 'static>(
            this: *mut ffi::AdwToggleGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-shrink\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_can_shrink_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "homogeneous")]
    pub fn connect_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<F: Fn(&ToggleGroup) + 'static>(
            this: *mut ffi::AdwToggleGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::homogeneous\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_homogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "n-toggles")]
    pub fn connect_n_toggles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_toggles_trampoline<F: Fn(&ToggleGroup) + 'static>(
            this: *mut ffi::AdwToggleGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-toggles\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_n_toggles_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "toggles")]
    pub fn connect_toggles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_toggles_trampoline<F: Fn(&ToggleGroup) + 'static>(
            this: *mut ffi::AdwToggleGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::toggles\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_toggles_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v1_7")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
impl Default for ToggleGroup {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ToggleGroup`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ToggleGroupBuilder {
    builder: glib::object::ObjectBuilder<'static, ToggleGroup>,
}

impl ToggleGroupBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn active(self, active: u32) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn active_name(self, active_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("active-name", active_name.into()),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn can_shrink(self, can_shrink: bool) -> Self {
        Self {
            builder: self.builder.property("can-shrink", can_shrink),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn homogeneous(self, homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("homogeneous", homogeneous),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: gtk::AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn orientation(self, orientation: gtk::Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ToggleGroup`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ToggleGroup {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
