use super::*;
use crate::error::*;

/// Trait used for incoming parameter conversion.
pub trait FromRobj<'a>: Sized {
    // Convert an incoming Robj from R into a value or an error.
    fn from_robj(_robj: &'a Robj) -> Result<Self> {
        Err("Unable to obtain value from R object.".into())
    }
}

macro_rules! impl_prim_from_robj {
    ($t: ty) => {
        impl<'a> FromRobj<'a> for $t {
            fn from_robj(robj: &'a Robj) -> Result<Self> {
                if let Some(v) = robj.as_integer_slice() {
                    match v.len() {
                        0 => Err(Error::ScalarLen(0)),
                        1 => {
                            if !v[0].is_na() {
                                Ok(v[0] as Self)
                            } else {
                                Err(Error::ScalarNA)
                            }
                        }
                        len => Err(Error::ScalarLen(len)),
                    }
                } else if let Some(v) = robj.as_real_slice() {
                    match v.len() {
                        0 => Err(Error::ScalarLen(0)),
                        1 => {
                            if !v[0].is_na() {
                                Ok(v[0] as Self)
                            } else {
                                Err(Error::ScalarNA)
                            }
                        }
                        len => Err(Error::ScalarLen(len)),
                    }
                } else {
                    Err(Error::NotAVectorType)
                }
            }
        }
    };
}

impl_prim_from_robj!(u8);
impl_prim_from_robj!(u16);
impl_prim_from_robj!(u32);
impl_prim_from_robj!(u64);
impl_prim_from_robj!(i8);
impl_prim_from_robj!(i16);
impl_prim_from_robj!(i32);
impl_prim_from_robj!(i64);
impl_prim_from_robj!(f32);
impl_prim_from_robj!(f64);

impl<'a> FromRobj<'a> for Rbool {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(v) = robj.as_logical_slice() {
            match v.len() {
                0 => Err(Error::ScalarLen(0)),
                1 => Ok(v[0]),
                len => Err(Error::ScalarLen(len)),
            }
        } else {
            Err(Error::TypeMismatch {
                expected: &"logical",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

impl<'a> FromRobj<'a> for bool {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(v) = robj.as_logical_slice() {
            match v.len() {
                0 => Err(Error::ScalarLen(0)),
                1 => {
                    if !v[0].is_na() {
                        Ok(v[0].into())
                    } else {
                        Err(Error::ScalarNA)
                    }
                }
                len => Err(Error::ScalarLen(len)),
            }
        } else {
            Err(Error::TypeMismatch {
                expected: &"logical",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

impl<'a> FromRobj<'a> for &'a str {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if robj.is_na() {
            Err(Error::ScalarNA)
        } else if let Some(s) = robj.as_str() {
            Ok(s)
        } else {
            Err(Error::TypeMismatch {
                expected: &"character",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

impl<'a> FromRobj<'a> for String {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if robj.is_na() {
            Err(Error::ScalarNA)
        } else if let Some(s) = robj.as_str() {
            Ok(s.to_string())
        } else {
            Err(Error::TypeMismatch {
                expected: &"character",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

impl<'a> FromRobj<'a> for Vec<Rbool> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(v) = robj.as_logical_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::TypeMismatch {
                expected: &"logical",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

impl<'a> FromRobj<'a> for Vec<bool> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(v) = robj.as_logical_slice() {
            if v.iter().any(|item| item.is_na()) {
                Err(Error::VectorNA)
            } else {
                Ok(v.iter().map(|item| item.into()).collect::<Vec<bool>>())
            }
        } else {
            Err(Error::TypeMismatch {
                expected: &"logical",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

impl<'a> FromRobj<'a> for Vec<i32> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(v) = robj.as_integer_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::TypeMismatch {
                expected: &"integer",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

impl<'a> FromRobj<'a> for Vec<f64> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(v) = robj.as_real_slice() {
            Ok(Vec::from(v))
        } else {
            Err(Error::TypeMismatch {
                expected: &"double",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

impl<'a> FromRobj<'a> for Vec<String> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if robj.is_na() {
            Err(Error::VectorNA)
        } else if let Some(v) = robj.as_string_vector() {
            let str_vec = v.to_vec();
            // check for NA's in the string vector
            if let Some(_str) = str_vec.iter().find(|&s| *s == na_str()) {
                Err(Error::VectorNA)
            } else {
                Ok(str_vec)
            }
        } else {
            Err(Error::TypeMismatch {
                expected: &"character",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

macro_rules! impl_iter_from_robj {
    ($t: ty, $iter_fn: ident, $expected: expr) => {
        impl<'a> FromRobj<'a> for $t {
            fn from_robj(robj: &'a Robj) -> Result<Self> {
                if let Some(v) = robj.$iter_fn() {
                    Ok(v)
                } else {
                    Err(Error::TypeMismatch {
                        expected: &$expected,
                        actual: robj.type_name().unwrap_or(&"unknown"),
                    })
                }
            }
        }
    };
}

impl_iter_from_robj!(StrIter, as_str_iter, "character");
impl_iter_from_robj!(ListIter, as_list_iter, "list");
impl_iter_from_robj!(IntegerIter<'a>, as_integer_iter, "integer");
impl_iter_from_robj!(RealIter<'a>, as_real_iter, "double");
impl_iter_from_robj!(LogicalIter<'a>, as_logical_iter, "logical");

/// Pass-through Robj conversion.
impl<'a> FromRobj<'a> for Robj {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        Ok(unsafe { new_borrowed(robj.get()) })
    }
}

impl<'a> FromRobj<'a> for HashMap<String, Robj> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(iter) = robj.as_named_list_iter() {
            Ok(iter
                .map(|(k, v)| (k.to_string(), v.to_owned()))
                .collect::<HashMap<String, Robj>>())
        } else {
            Err(Error::TypeMismatch {
                expected: &"list",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

impl<'a> FromRobj<'a> for HashMap<&str, Robj> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(iter) = robj.as_named_list_iter() {
            Ok(iter.map(|(k, v)| (k, v)).collect::<HashMap<&str, Robj>>())
        } else {
            Err(Error::TypeMismatch {
                expected: &"list",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

// NA-sensitive integer input handling
impl<'a> FromRobj<'a> for Option<i32> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(slice) = robj.as_integer_slice() {
            match slice.len() {
                0 => Err(Error::ScalarLen(0)),
                1 if slice[0].is_na() => Ok(None),
                1 => Ok(Some(slice[0])),
                len => Err(Error::ScalarLen(len)),
            }
        } else {
            Err(Error::TypeMismatch {
                expected: &"integer",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

// NA-sensitive logical input handling
impl<'a> FromRobj<'a> for Option<bool> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(v) = robj.as_logical_slice() {
            match v.len() {
                0 => Err(Error::ScalarLen(0)),
                1 => Ok(v[0].into()),
                len => Err(Error::ScalarLen(len)),
            }
        } else {
            Err(Error::TypeMismatch {
                expected: &"logical",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

// NA-sensitive real input handling
impl<'a> FromRobj<'a> for Option<f64> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(slice) = robj.as_real_slice() {
            match slice.len() {
                0 => Err(Error::ScalarLen(0)),
                1 if slice[0].is_na() => Ok(None),
                1 => Ok(Some(slice[0])),
                len => Err(Error::ScalarLen(len)),
            }
        } else {
            Err(Error::TypeMismatch {
                expected: &"double",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

// NA-sensitive string input handling
impl<'a> FromRobj<'a> for Option<&'a str> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(slice) = robj.as_str_vector() {
            match slice.len() {
                0 => Err(Error::ScalarLen(0)),
                1 if slice[0].is_na() => Ok(None),
                1 => Ok(Some(slice[0])),
                len => Err(Error::ScalarLen(len)),
            }
        } else {
            Err(Error::TypeMismatch {
                expected: &"character",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

// NA-sensitive string input handling
impl<'a> FromRobj<'a> for Option<String> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        <Option<&'a str>>::from_robj(robj).map(|ok| ok.map(<str>::to_string))
    }
}

impl<'a, T> FromRobj<'a> for &'a [T]
where
    Robj: AsTypedSlice<'a, T>,
{
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(slice) = robj.as_typed_slice() {
            Ok(slice)
        } else {
            Err(Error::TypeMismatch {
                expected: &"<...>",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

impl<'a> FromRobj<'a> for &'a [bool] {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(v) = robj.as_logical_slice() {
            if v.iter().any(|item| item.is_na()) {
                Err(Error::VectorNA)
            } else {
                // sizeof::<Rbool> == sizeof::<i32> == sizeof::<bool>
                Ok(unsafe { std::mem::transmute(v) })
            }
        } else {
            Err(Error::TypeMismatch {
                expected: &"logical",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

// Symbol input parameters.
impl<'a> FromRobj<'a> for Symbol<'a> {
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(x) = robj.as_symbol() {
            Ok(x)
        } else {
            Err(Error::TypeMismatch {
                expected: &"symbol",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        }
    }
}

// Matrix input parameters.
impl<'a, T> FromRobj<'a> for RArray<&'a [T], [usize; 2]>
where
    Robj: AsTypedSlice<'a, T>,
{
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(x) = robj.as_matrix() {
            Ok(x)
        } else if !robj.as_typed_slice().is_some() {
            Err(Error::TypeMismatch {
                expected: &"<...>",
                actual: robj.type_name().unwrap_or(&"unknown"),
            })
        } else {
            let dims = robj.dim().map(|v| v.len());
            Err(Error::MatrixDim {
                expected: 2,
                actual: dims,
            })
        }
    }
}

// Matrix input parameters.
impl<'a, T> FromRobj<'a> for RMatrix3D<&'a [T]>
where
    Robj: AsTypedSlice<'a, T>,
{
    fn from_robj(robj: &'a Robj) -> Result<Self> {
        if let Some(x) = robj.as_matrix3d() {
            Ok(x)
        } else {
            let dims = robj.dim().map(|v| v.len());
            Err(Error::MatrixDim {
                expected: 3,
                actual: dims,
            })
        }
    }
}
