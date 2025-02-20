use glib::object::IsA;
use glib::translate::*;

use crate::prelude::*;
use crate::MessageDialog;

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
use std::boxed::Box as Box_;
#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
use std::pin::Pin;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::MessageDialog>> Sealed for T {}
}

pub trait MessageDialogExtManual: sealed::Sealed + IsA<MessageDialog> + 'static {
    #[doc(alias = "adw_message_dialog_get_response_label")]
    #[doc(alias = "get_response_label")]
    fn response_label(&self, response: &str) -> glib::GString {
        assert!(self.as_ref().has_response(response));

        unsafe {
            from_glib_none(ffi::adw_message_dialog_get_response_label(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_message_dialog_add_responses")]
    fn add_responses(&self, ids_and_labels: &[(&str, &str)]) {
        ids_and_labels.iter().for_each(|(id, label)| {
            self.add_response(id, label);
        });
    }

    #[doc(alias = "adw_message_dialog_choose")]
    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    fn choose<P: FnOnce(glib::GString) + 'static>(
        self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn choose_trampoline<P: FnOnce(glib::GString) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let result = from_glib_none(ffi::adw_message_dialog_choose_finish(
                _source_object as *mut _,
                res,
            ));
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = choose_trampoline::<P>;
        unsafe {
            ffi::adw_message_dialog_choose(
                self.upcast().into_glib_ptr(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "adw_message_dialog_choose")]
    fn choose_future(self) -> Pin<Box_<dyn std::future::Future<Output = glib::GString> + 'static>> {
        Box_::pin(gio::GioFuture::new(
            &self,
            move |obj: &Self, cancellable, send| {
                obj.clone().choose(Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }
}

impl<O: IsA<MessageDialog>> MessageDialogExtManual for O {}
