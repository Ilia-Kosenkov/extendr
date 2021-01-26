/// Rbool is a wrapper for i32 in the context of an R's tristate boolean.
/// It can be TRUE, FALSE or NA_LOGICAL.
#[derive(PartialEq, Eq)]
pub struct Rbool(pub i32);

impl Rbool {
    pub const TRUE: Rbool = Rbool(1);
    pub const FALSE: Rbool = Rbool(0);
    pub const NA: Rbool = Rbool(std::i32::MIN);

    /// Test if TRUE
    pub fn is_true(&self) -> bool {
        self.0 == (Rbool::TRUE).0
    }

    /// Test if FALSE
    pub fn is_false(&self) -> bool {
        self.0 == (Rbool::FALSE).0
    }
}

impl Clone for Rbool {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Rbool {}

impl From<i32> for Rbool {
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl From<Option<i32>> for Rbool {
    // 0 -> FALSE
    // 1 -> TRUE
    // All other values are treated as 'invalid', i.e. `NA`
    fn from(v: Option<i32>) -> Self {
        match v {
            Some(0) => Rbool::FALSE,
            Some(1) => Rbool::TRUE,
            _ => Rbool::NA,
        }
    }
}

impl From<bool> for Rbool {
    fn from(v: bool) -> Self {
        Self(v as i32)
    }
}

impl From<Option<bool>> for Rbool {
    // Some(bool) -> bool
    // None -> NA
    fn from(v: Option<bool>) -> Self {
        match v {
            Some(false) => Rbool::FALSE,
            Some(true) => Rbool::TRUE,
            _ => Rbool::NA,
        }
    }
}

impl From<Rbool> for bool {
    // `true` only if R logical is `TRUE`
    // NAs are `false`
    fn from(v: Rbool) -> Self {
        v == Rbool::TRUE
    }
}

impl From<Rbool> for Option<bool> {
    // TRUE/FALSE -> true/false
    // NA -> None
    fn from(v: Rbool) -> Self {
        match v {
            Rbool::TRUE => Some(true),
            Rbool::FALSE => Some(false),
            _ => None,
        }
    }
}

impl From<&Rbool> for bool {
    // `true` only if R logical is `TRUE`
    // NAs are `false`
    fn from(v: &Rbool) -> Self {
        v.0 == (Rbool::TRUE).0
    }
}

impl From<&Rbool> for Option<bool> {
    // TRUE/FALSE -> true/false
    // NA -> None
    fn from(v: &Rbool) -> Self {
        if v.0 == (Rbool::TRUE).0 {
            Some(true)
        } else if v.0 == (Rbool::FALSE).0 {
            Some(false)
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Rbool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rbool(0) => write!(f, "FALSE"),
            Rbool(1) => write!(f, "TRUE"),
            Rbool(std::i32::MIN) => write!(f, "NA_LOGICAL"),
            _ => write!(f, "Rbool({})", self.0),
        }
    }
}
