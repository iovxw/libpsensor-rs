extern crate futures;
extern crate tokio_core;

extern crate libpsensor_sys as sys;

use std::ffi::CStr;
use std::collections::HashMap;
use std::time::Duration;

use futures::{stream, Stream, Poll};
use tokio_core::reactor::{Interval, Handle};

pub fn new(dur: Duration, handle: &Handle) -> (HashMap<String, Psensor>, PsensorStream) {
    let mut pointer: *mut *mut sys::psensor = std::ptr::null_mut();
    unsafe {
        sys::psensor_amd_list_append(&mut pointer, 1);
        sys::psensor_nvidia_list_append(&mut pointer, 1);
        if sys::psensor_udisks2_is_supported() {
            sys::psensor_udisks2_list_append(&mut pointer, 1);
        } else if sys::psensor_atasmart_is_supported() {
            sys::psensor_atasmart_list_append(&mut pointer, 1);
        } else {
            sys::psensor_hddtemp_list_append(&mut pointer, 1);
        }
        sys::psensor_lmsensor_list_append(&mut pointer, 1);
    }
    let len = unsafe { sys::psensor_list_size(pointer) as usize };
    let tmp: &[*mut sys::psensor] = unsafe { std::slice::from_raw_parts_mut(pointer, len) };
    let mut map = HashMap::with_capacity(len);
    for psensor in tmp {
        let p = unsafe { Psensor::from_raw(*psensor) };
        map.insert(p.id.clone(), p);
    }

    (map, PsensorStream::new(pointer, dur, handle))
}

#[derive(Debug)]
pub struct Psensor {
    name: String,
    id: String,
    chip: String,
    sensor: PsensorType,
    max: f64,
    min: f64,
}

impl Psensor {
    unsafe fn from_raw(raw: *mut sys::psensor) -> Psensor {
        Psensor {
            name: CStr::from_ptr((*raw).name).to_string_lossy().into_owned(),
            id: CStr::from_ptr((*raw).id).to_string_lossy().into_owned(),
            chip: CStr::from_ptr((*raw).chip).to_string_lossy().into_owned(),
            sensor: PsensorType::from_raw((*raw).type_),
            max: (*raw).max,
            min: (*raw).min,
        }
    }
}

#[derive(Debug)]
pub enum PsensorType {
    Hdd,
    Cpu,
    Gpu,
    Fan,
    Other(bool), // is temperature?
}

impl PsensorType {
    fn from_raw(raw: std::os::raw::c_uint) -> PsensorType {
        use PsensorType::*;
        if raw & sys::psensor_type_SENSOR_TYPE_NVCTRL != 0 {
            if raw & sys::psensor_type_SENSOR_TYPE_TEMP != 0 {
                return Gpu;
            } else if raw & sys::psensor_type_SENSOR_TYPE_GRAPHICS != 0 {
                return Other(false); // Graphics usage
            } else if raw & sys::psensor_type_SENSOR_TYPE_VIDEO != 0 {
                return Other(false); // Video usage
            } else if raw & sys::psensor_type_SENSOR_TYPE_MEMORY != 0 {
                return Other(false); // Memory usage
            } else if raw & sys::psensor_type_SENSOR_TYPE_PCIE != 0 {
                return Other(false); // PCIe usage
            }
            return Other(false); // NVIDIA GPU
        }

        if raw & sys::psensor_type_SENSOR_TYPE_ATIADL != 0 {
            if raw & sys::psensor_type_SENSOR_TYPE_TEMP != 0 {
                return Gpu;
            } else if raw & sys::psensor_type_SENSOR_TYPE_RPM != 0 {
                return Fan;
            }
            return Other(false); // AMD GPU Usage
        }
        if raw & sys::psensor_type_SENSOR_TYPE_HDD_TEMP == sys::psensor_type_SENSOR_TYPE_HDD_TEMP {
            return Hdd;
        }
        if raw & sys::psensor_type_SENSOR_TYPE_CPU_USAGE ==
           sys::psensor_type_SENSOR_TYPE_CPU_USAGE {
            return Other(false); // CPU Usage
        }
        if raw & sys::psensor_type_SENSOR_TYPE_RPM != 0 {
            return Fan;
        }
        if raw & sys::psensor_type_SENSOR_TYPE_CPU != 0 {
            return Cpu;
        }
        if raw & sys::psensor_type_SENSOR_TYPE_TEMP != 0 {
            return Other(true); // Temperature
        }
        if raw & sys::psensor_type_SENSOR_TYPE_REMOTE != 0 {
            return Other(false); // Remote
        }
        if raw & sys::psensor_type_SENSOR_TYPE_MEMORY != 0 {
            return Other(false); // Memory
        }
        Other(false)
    }
}

pub struct PsensorStream {
    pointer: *mut *mut sys::psensor,
    stream: Box<Stream<Item = (String, f64), Error = std::io::Error>>,
}

impl PsensorStream {
    fn new(pointer: *mut *mut sys::psensor, dur: Duration, handle: &Handle) -> PsensorStream {
        let stream = Interval::new(dur, handle)
            .unwrap()
            .map(move |_| {
                PsensorStream::update(pointer);
                let len = unsafe { sys::psensor_list_size(pointer) as usize };
                let sensors: &[*mut sys::psensor] =
                    unsafe { std::slice::from_raw_parts_mut(pointer, len) };
                let mut r = Vec::with_capacity(len);
                for &sensor in sensors {
                    let id = unsafe { CStr::from_ptr((*sensor).id).to_string_lossy().into_owned() };
                    let value = unsafe { sys::psensor_get_current_value(sensor) };
                    r.push(Ok((id, value)));
                }
                stream::iter(r.into_iter())
            })
            .flatten();
        PsensorStream {
            pointer: pointer,
            stream: Box::new(stream),
        }
    }

    fn update(pointer: *mut *mut sys::psensor) {
        unsafe {
            sys::psensor_amd_list_update(pointer);
            sys::psensor_nvidia_list_update(pointer);
            if sys::psensor_udisks2_is_supported() {
                sys::psensor_udisks2_list_update(pointer);
            } else if sys::psensor_atasmart_is_supported() {
                sys::psensor_atasmart_list_update(pointer);
            } else {
                sys::psensor_hddtemp_list_update(pointer);
            }
            sys::psensor_lmsensor_list_update(pointer);
        }
    }
}

impl Stream for PsensorStream {
    type Item = (String, f64);
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.stream.poll()
    }
}

impl Drop for PsensorStream {
    fn drop(&mut self) {
        unsafe {
            sys::psensor_list_free(self.pointer);
        }
    }
}
