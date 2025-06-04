#![doc = include_str!("../README.md")]
#![no_std]

/// Extension methods for the unit type `()` that construct common wrapper
/// values without explicit boiler-plate.
///
/// Import [`unit_ext`](crate) to bring the trait into scope.
///
/// # Examples
///
/// ```
/// use unit_ext::*;
///
/// let ok:  Result<_, ()> = ().ret_ok(1);
/// let err: Result<u8, _> = ().ret_err("boom");
///
/// let some = ().ret_some("hi");
/// let none: Option<i32> = ().ret_none();
///
/// let vec: Vec<u8> = ().ret_default();
/// ```
pub trait UnitExt: Sized {
    /// Returns `value`.
    ///
    /// This is mainly useful when syntactic symmetry with the other
    /// `ret_*` helpers is desired.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_ext::*;
    /// assert_eq!(().ret(42), 42);
    /// assert_eq!({ 24; }.ret(42), 42); // Can cause a Clippy warning/error.
    /// assert_eq!(println!("24").ret(42), 42);
    /// ```
    #[must_use]
    #[inline]
    fn ret<T>(self, value: T) -> T {
        value
    }

    /// Returns `T::default()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_ext::*;
    /// let v: Vec<u8> = println!("Creating empty vec").ret_default();
    /// assert!(v.is_empty());
    /// ```
    #[must_use]
    #[inline]
    fn ret_default<T: Default>(self) -> T {
        T::default()
    }

    /// Returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_ext::*;
    /// let none = ().ret_none::<u8>();
    /// assert_eq!(none, None);
    /// ```
    #[must_use]
    #[inline]
    fn ret_none<T>(self) -> Option<T> {
        None
    }

    /// Wraps `value` in [`Some`] and returns it.
    ///
    /// `value` is passed through `Into`, matching the behaviour of
    /// `Option::from`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_ext::*;
    /// let opt = ().ret_some(5);
    /// assert_eq!(opt, Some(5));
    /// ```
    #[must_use]
    #[inline]
    fn ret_some<T>(self, value: T) -> Option<T> {
        value.into()
    }

    /// Returns `Some(T::default())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_ext::*;
    /// let s: Option<String> = ().ret_some_default();
    /// assert_eq!(s, Some(String::new()));
    /// ```
    #[must_use]
    #[inline]
    fn ret_some_default<T: Default>(self) -> Option<T> {
        self.ret_default::<T>().into()
    }

    /// Returns `Err(value)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_ext::*;
    /// let e: Result<(), &str> = ().ret_err("nope");
    /// assert!(e.is_err());
    /// ```
    #[must_use]
    #[inline]
    fn ret_err<T, E>(self, value: E) -> Result<T, E> {
        Err(value)
    }

    /// Returns `Err(E::default())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_ext::*;
    /// let e: Result<i32, &str> = ().ret_err_default();
    /// assert!(e.is_err());
    /// ```
    #[must_use]
    #[inline]
    fn ret_err_default<T, E: Default>(self) -> Result<T, E> {
        self.ret_err(E::default())
    }

    /// Returns `Ok(value)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_ext::*;
    /// let ok = ().ret_ok::<_, ()>("yes");
    /// assert_eq!(ok, Ok("yes"));
    /// ```
    #[must_use]
    #[inline]
    fn ret_ok<T, E>(self, value: T) -> Result<T, E> {
        Ok(value)
    }

    /// Returns `Ok(T::default())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_ext::*;
    /// let ok: Result<Vec<u8>, ()> = ().ret_ok_default();
    /// assert_eq!(ok, Ok(Vec::new()));
    /// ```
    #[must_use]
    #[inline]
    fn ret_ok_default<T: Default, E>(self) -> Result<T, E> {
        self.ret_ok(T::default())
    }
}

/// Extension methods for any value that explicitly discard the value
/// and yield unit `()`.
///
/// The helpers make intent obvious when chaining iterator or async
/// pipelines.
///
/// # Examples
///
/// ```
/// use unit_ext::*;
///
/// (0..3).for_each(|n| n.discard_ret());
/// let x: Option<usize> = Some(0).discard_self().ret_some(1);
/// ```
pub trait RetExt: Sized {
    /// Discards `self`, returning `()`.
    ///
    /// Equivalent to `let _ = self;`.
    #[inline]
    fn discard_self(self) {
        let _ = self;
    }

    /// Alias for [`discard_self`](RetExt::discard_self)
    #[inline]
    fn discard_ret(self) {
        self.discard_self();
    }
}

impl UnitExt for () {}
impl<T> RetExt for T {}
