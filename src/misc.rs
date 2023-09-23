/// Ternary operator emulation.
/// 
/// # Examples
/// 
/// ```rust
/// let val = either!(is_beautiful()
///     => beautiful_process()
///      ; awesome_process()
/// )
/// ```
macro_rules! either {
    ( $condition:expr => $true:expr ; $false:expr ) => {
        if $condition {
            $true
        }
        else {
            $false
        }
    }
}

pub(crate) use either;


/// Multiline string builder from arbitrary literals.
/// 
/// Inserts line feed between literals passed to the macro.
/// No line feed is inserted after a literal, if it is the last
/// one in the list. 
/// 
/// Returns an empty string, when no arguments are provided.
/// 
/// # Examples
/// 
/// ```rust
/// let s1 = multiline!("test 1");
/// let s2 = multiline!("test 1", "test 2");
/// 
/// // Will print:
/// // test 1
/// println!("{}", s1);
/// 
/// // Will print:
/// // test 1
/// // test 2
/// println!("{}", s2);
/// ```
macro_rules! multiline {
    () => {
        ""
    };
    ( $fe:expr, $($e:expr),+ $(,)? ) => {
        concat!(
            $fe, $("\n", $e,)+
        )
    };
}

pub(crate) use multiline;
