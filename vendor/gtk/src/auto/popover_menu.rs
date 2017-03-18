// This file was generated by gir (c3b4020) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use Popover;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct PopoverMenu(Object<ffi::GtkPopoverMenu>): Popover, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_popover_menu_get_type(),
    }
}

impl PopoverMenu {
    #[cfg(feature = "v3_16")]
    pub fn new() -> PopoverMenu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_menu_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn open_submenu(&self, name: &str) {
        unsafe {
            ffi::gtk_popover_menu_open_submenu(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn get_property_visible_submenu(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "visible-submenu".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_visible_submenu(&self, visible_submenu: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "visible-submenu".to_glib_none().0, Value::from(visible_submenu).to_glib_none().0);
        }
    }

    pub fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, Value::from(&position).to_glib_none().0);
        }
    }

    pub fn get_child_submenu<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "submenu".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_child_submenu<T: IsA<Widget>>(&self, item: &T, submenu: Option<&str>) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "submenu".to_glib_none().0, Value::from(submenu).to_glib_none().0);
        }
    }
}