/// Logical is a wrapper for i32 in the context of an R's tristate boolean.
/// It can be TRUE, FALSE or NA_LOGICAL.
#[derive(PartialEq, Eq)]
pub struct Logical(pub i32);

impl Logical {
    pub const TRUE: Logical = Logical(1);
    pub const FALSE: Logical = Logical(0);
    pub const NA: Logical = Logical(std::i32::MIN);

    /// Test if TRUE
    pub fn is_true(&self) -> bool {
        self.0 == (Logical::TRUE).0
    }

    /// Test if FALSE
    pub fn is_false(&self) -> bool {
        self.0 == (Logical::FALSE).0
    }
}

impl Clone for Logical {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Logical {}

impl From<i32> for Logical {
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl From<Option<i32>> for Logical {
    // 0 -> FALSE
    // 1 -> TRUE
    // All other values are treated as 'invalid', i.e. `NA`
    fn from(v: Option<i32>) -> Self {
        match v {
            Some(0) => Logical::FALSE,
            Some(1) => Logical::TRUE,
            _ => Logical::NA,
        }
    }
}

impl From<bool> for Logical {
    fn from(v: bool) -> Self {
        Self(v as i32)
    }
}

impl From<Option<bool>> for Logical {
    // Some(bool) -> bool
    // None -> NA
    fn from(v: Option<bool>) -> Self {
        match v {
            Some(false) => Logical::FALSE,
            Some(true) => Logical::TRUE,
            _ => Logical::NA,
        }
    }
}

impl From<Logical> for bool {
    // `true` only if R logical is `TRUE`
    // NAs are `false`
    fn from(v: Logical) -> Self {
        v == Logical::TRUE
    }
}

impl From<Logical> for Option<bool> {
    // TRUE/FALSE -> true/false
    // NA -> None
    fn from(v: Logical) -> Self {
        match v {
            Logical::TRUE => Some(true),
            Logical::FALSE => Some(false),
            _ => None,
        }
    }
}

impl From<&Logical> for bool {
    // `true` only if R logical is `TRUE`
    // NAs are `false`
    fn from(v: &Logical) -> Self {
        v.0 == (Logical::TRUE).0
    }
}

impl From<&Logical> for Option<bool> {
    // TRUE/FALSE -> true/false
    // NA -> None
    fn from(v: &Logical) -> Self {
        if v.0 == (Logical::TRUE).0 {
            Some(true)
        } else if v.0 == (Logical::FALSE).0 {
            Some(false)
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Logical(0) => write!(f, "FALSE"),
            Logical(1) => write!(f, "TRUE"),
            Logical(std::i32::MIN) => write!(f, "NA_LOGICAL"),
            _ => write!(f, "Logical({})", self.0),
        }
    }
}
