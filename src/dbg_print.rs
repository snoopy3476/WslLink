/// Print debug msg, only when compiled in debug mode
///
/// # Examples
///
/// ```
/// // print only label
/// __wsllink_dbg!("label");
///
/// // print label and vars (with dbg! macro)
/// __wsllink_dbg!("label", var1, var2, ...);
///
/// // print only vars
/// //   almost same with dbg!() macro,
/// //   except that this macro only prints on debug mode
/// __wsllink_dbg!("", var1, var2, ...);
/// ```
///
#[macro_export]
macro_rules! __wsllink_dbg {

    // for label-only
    //   crate::__wsllink_dbg!("label");
    ($label:literal) => {

            // for debug mode
            #[cfg(debug_assertions)]
            {
                // print header
                crate::__wsllink_dbg_header!($label);
            }

            // for release mode
            #[cfg(not(debug_assertions))]
            {
                // do nothing on release mode, just removing unused warnings
                ($label)
            }

    };

    // for label and vars
    //   crate::__wsllink_dbg!("label", var1, var2, ...);
    //   crate::__wsllink_dbg!("", var1, var2, ...);
    ($label:literal, $($args:expr), +) => {
        {

            // for debug mode
            #[cfg(debug_assertions)]
            {
                // print header
                crate::__wsllink_dbg_header!($label);
                // print body
                crate::__wsllink_dbg_body!($($args),+);
            }

            // for release mode
            #[cfg(not(debug_assertions))]
            {
                // do nothing on release mode, just removing unused warnings
                ($label,$($args), +)
            }

        }
    };
}

/// Prints debug msg header only, if str is not empty (for the macro [`__wsllink_dbg`])
#[macro_export]
macro_rules! __wsllink_dbg_header {
    ($label:literal) => {
        #[cfg(debug_assertions)]
        {
            if !$label.is_empty() {
                use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
                let mut stderr = StandardStream::stderr(ColorChoice::Always);

                // set color
                stderr
                    .set_color(
                        ColorSpec::new()
                            .set_fg(Some(Color::Black))
                            .set_bg(Some(Color::Yellow)),
                    )
                    .ok();
                // print header
                eprint!(" [WSLLINK_DBG] {} ", $label);
                // reset color
                stderr.reset().ok();
                eprintln!();
            }
        }
    };
}

/// Prints debug msg body (args) only (for the macro [`__wsllink_dbg`])
#[macro_export]
macro_rules! __wsllink_dbg_body {
    ($($args:expr), +) => {
        #[cfg(debug_assertions)]
        {
            use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
            let mut stderr = StandardStream::stderr(ColorChoice::Always);

            // set color
            stderr.set_color(
                ColorSpec::new()
                    .set_fg(Some(Color::Yellow))
            ).ok();
            // print vars
            dbg!($($args),+);
            // reset color
            stderr.reset().ok();
            //eprintln!();
        }
    };
}