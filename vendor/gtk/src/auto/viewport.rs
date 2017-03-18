// This file was generated by gir (c3b4020) from gir-files (71d73f0)
// DO NOT EDIT

use Adjustment;
use Bin;
use Container;
use Scrollable;
use ShadowType;
use Widget;
use ffi;
use gdk;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Viewport(Object<ffi::GtkViewport>): Bin, Container, Widget, Scrollable;

    match fn {
        get_type => || ffi::gtk_viewport_get_type(),
    }
}

impl Viewport {
    pub fn new(hadjustment: Option<&Adjustment>, vadjustment: Option<&Adjustment>) -> Viewport {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_viewport_new(hadjustment.to_glib_none().0, vadjustment.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_bin_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_viewport_get_bin_window(self.to_glib_none().0))
        }
    }

    pub fn get_shadow_type(&self) -> ShadowType {
        unsafe {
            from_glib(ffi::gtk_viewport_get_shadow_type(self.to_glib_none().0))
        }
    }

    pub fn get_view_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_viewport_get_view_window(self.to_glib_none().0))
        }
    }

    pub fn set_shadow_type(&self, type_: ShadowType) {
        unsafe {
            ffi::gtk_viewport_set_shadow_type(self.to_glib_none().0, type_.to_glib());
        }
    }
}