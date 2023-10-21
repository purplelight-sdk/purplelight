/// Starts the Agera application, executing an initial
/// action. This performs bootstrapping of the application.
/// 
/// Defining a main Rust function instead of using `agera::application::start`
/// for executing initial actions may lead to panics.
/// 
/// # Syntax
/// 
/// ```ignore
/// agera::application::start!(async {
///     // Main action
/// });
/// ```
pub macro start {
    ($start_action:expr) => {
        if agera::target::is_native_target! {
            use ::agera::target::tokio as __agera_target_tokio__;

            #[cfg(not(target_arch = "wasm32"))]
            #[__agera_target_tokio__::main(crate = "__agera_target_tokio__")]
            async fn main() {
                let local_task_set = ::agera::target::tokio::task::LocalSet::new();
                local_task_set.run_until(async {
                    unsafe {
                        ::agera::application::BOOTSTRAPPED = true;
                    }
                    $start_action.await;
                }).await;
            }
        }

        if agera::target::is_browser_target! {
            fn main() {
                ::agera::common::future::exec(async {
                    unsafe {
                        ::agera::application::BOOTSTRAPPED = true;
                    }
                    $start_action.await;
                });
            }
        }
    },
}

/// Internal property.
#[doc(hidden)]
pub static mut BOOTSTRAPPED: bool = false;

pub(crate) macro assert_bootstrapped {
    () => {
        assert!(unsafe { crate::application::BOOTSTRAPPED }, "The application must be initialized through 'agera::application::start!(action)'.");
    },
}