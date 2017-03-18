// This file was generated by gir (c3b4020) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Box;
use Container;
use Object;
use Widget;
use Window;
use ffi;
#[cfg(feature = "v3_12")]
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
#[cfg(feature = "v3_12")]
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Dialog(Object<ffi::GtkDialog>): Window, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_dialog_get_type(),
    }
}

impl Dialog {
    pub fn new() -> Dialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_dialog_new()).downcast_unchecked()
        }
    }

    //pub fn new_with_buttons<'a, 'b, T: Into<Option<&'a str>>, U: IsA<Window>, V: Into<Option<&'b str>>>(title: T, parent: Option<&U>, flags: DialogFlags, first_button_text: V, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Dialog {
    //    unsafe { TODO: call ffi::gtk_dialog_new_with_buttons() }
    //}
}

pub trait DialogExt {
    fn add_action_widget<T: IsA<Widget>>(&self, child: &T, response_id: i32);

    fn add_button(&self, button_text: &str, response_id: i32) -> Widget;

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_action_area(&self) -> Widget;

    fn get_content_area(&self) -> Box;

    #[cfg(feature = "v3_12")]
    fn get_header_bar(&self) -> Option<Widget>;

    fn get_response_for_widget<T: IsA<Widget>>(&self, widget: &T) -> i32;

    fn get_widget_for_response(&self, response_id: i32) -> Option<Widget>;

    fn response(&self, response_id: i32);

    fn run(&self) -> i32;

    //fn set_alternative_button_order(&self, first_response_id: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn set_alternative_button_order_from_array(&self, n_params: i32, new_order: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 });

    fn set_default_response(&self, response_id: i32);

    fn set_response_sensitive(&self, response_id: i32, setting: bool);

    #[cfg(feature = "v3_12")]
    fn get_property_use_header_bar(&self) -> i32;

    #[cfg(feature = "v3_12")]
    fn set_property_use_header_bar(&self, use_header_bar: i32);

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_response<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Dialog> + IsA<Object>> DialogExt for O {
    fn add_action_widget<T: IsA<Widget>>(&self, child: &T, response_id: i32) {
        unsafe {
            ffi::gtk_dialog_add_action_widget(self.to_glib_none().0, child.to_glib_none().0, response_id);
        }
    }

    fn add_button(&self, button_text: &str, response_id: i32) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_dialog_add_button(self.to_glib_none().0, button_text.to_glib_none().0, response_id))
        }
    }

    //fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_dialog_add_buttons() }
    //}

    fn get_action_area(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_action_area(self.to_glib_none().0))
        }
    }

    fn get_content_area(&self) -> Box {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_content_area(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_header_bar(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_header_bar(self.to_glib_none().0))
        }
    }

    fn get_response_for_widget<T: IsA<Widget>>(&self, widget: &T) -> i32 {
        unsafe {
            ffi::gtk_dialog_get_response_for_widget(self.to_glib_none().0, widget.to_glib_none().0)
        }
    }

    fn get_widget_for_response(&self, response_id: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_dialog_get_widget_for_response(self.to_glib_none().0, response_id))
        }
    }

    fn response(&self, response_id: i32) {
        unsafe {
            ffi::gtk_dialog_response(self.to_glib_none().0, response_id);
        }
    }

    fn run(&self) -> i32 {
        unsafe {
            ffi::gtk_dialog_run(self.to_glib_none().0)
        }
    }

    //fn set_alternative_button_order(&self, first_response_id: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_dialog_set_alternative_button_order() }
    //}

    //fn set_alternative_button_order_from_array(&self, n_params: i32, new_order: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }) {
    //    unsafe { TODO: call ffi::gtk_dialog_set_alternative_button_order_from_array() }
    //}

    fn set_default_response(&self, response_id: i32) {
        unsafe {
            ffi::gtk_dialog_set_default_response(self.to_glib_none().0, response_id);
        }
    }

    fn set_response_sensitive(&self, response_id: i32, setting: bool) {
        unsafe {
            ffi::gtk_dialog_set_response_sensitive(self.to_glib_none().0, response_id, setting.to_glib());
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_property_use_header_bar(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-header-bar".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(feature = "v3_12")]
    fn set_property_use_header_bar(&self, use_header_bar: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "use-header-bar".to_glib_none().0, Value::from(&use_header_bar).to_glib_none().0);
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "close",
                transmute(close_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_response<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "response",
                transmute(response_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn close_trampoline<T>(this: *mut ffi::GtkDialog, f: glib_ffi::gpointer)
where T: IsA<Dialog> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&Dialog::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn response_trampoline<T>(this: *mut ffi::GtkDialog, response_id: libc::c_int, f: glib_ffi::gpointer)
where T: IsA<Dialog> {
    callback_guard!();
    let f: &Box_<Fn(&T, i32) + 'static> = transmute(f);
    f(&Dialog::from_glib_none(this).downcast_unchecked(), response_id)
}