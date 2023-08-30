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
