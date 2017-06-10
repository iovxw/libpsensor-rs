use std::time::Duration;

mod sys;

#[derive(Debug)]
pub struct Psensor {
    inner: *mut sys::psensor,
    owned: bool,
}

impl Psensor {
    unsafe fn from_raw(raw: *mut sys::psensor) -> Psensor {
        Psensor {
            inner: raw,
            owned: false,
        }
    }

    pub fn get_current_measure(&self) -> Measure {
        unsafe {
            let raw = sys::psensor_get_current_measure(self.inner);
            Measure::from_raw(raw)
        }
    }

    pub fn get_current_value(&self) -> f64 {
        unsafe { sys::psensor_get_current_value(self.inner) }
    }
}

impl Drop for Psensor {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                sys::psensor_free(self.inner);
            }
        }
    }
}

#[derive(Debug)]
pub struct PsensorList {
    inner: *mut *mut sys::psensor,
    vec: Vec<Psensor>,
}

impl PsensorList {
    unsafe fn update_vec(&mut self) {
        let v: &[*mut sys::psensor] = std::slice::from_raw_parts_mut(self.inner, self.len());
        self.vec.clear();
        for raw in v {
            self.vec.push(Psensor::from_raw(*raw));
        }
    }

    pub fn new() -> PsensorList {
        PsensorList {
            inner: &mut std::ptr::null_mut(),
            vec: Vec::new(),
        }
    }

    pub fn push(&mut self, mut value: Psensor) {
        unsafe {
            value.owned = false;
            sys::psensor_list_append(&mut self.inner, value.inner);
            self.update_vec();
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { sys::psensor_list_size(self.inner) as usize }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Psensor> {
        self.vec.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Psensor> {
        self.vec.iter_mut()
    }
}

impl Drop for PsensorList {
    fn drop(&mut self) {
        unsafe {
            sys::psensor_list_free(self.inner);
        }
    }
}

macro_rules! impl_index {
    ( $output:ty, $idx:ty ) => {
        impl std::ops::Index<$idx> for PsensorList {
            type Output = $output;

            #[inline]
            fn index(&self, i: $idx) -> &Self::Output {
                self.vec.index(i)
            }
        }
    };
}

impl_index!(Psensor, usize);
impl_index!([Psensor], std::ops::Range<usize>);
impl_index!([Psensor], std::ops::RangeTo<usize>);
impl_index!([Psensor], std::ops::RangeFrom<usize>);
impl_index!([Psensor], std::ops::RangeFull);

impl<'a> IntoIterator for &'a PsensorList {
    type Item = &'a Psensor;
    type IntoIter = std::slice::Iter<'a, Psensor>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut PsensorList {
    type Item = &'a mut Psensor;
    type IntoIter = std::slice::IterMut<'a, Psensor>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl Default for PsensorList {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Measure {
    pub value: f64,
    pub time: Duration,
}

impl Measure {
    unsafe fn from_raw(raw: *mut sys::measure) -> Measure {
        Measure {
            value: (*raw).value,
            time: Duration::new((*raw).time.tv_sec as u64, (*raw).time.tv_usec as u32 * 1000),
        }
    }
}
