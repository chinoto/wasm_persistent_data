use std::collections::HashMap;
use std::ptr::NonNull;

pub enum Guardian {
    Data(u32),
    Map(HashMap<u32, Guardian>),
}

#[no_mangle]
pub fn guardian_new() -> *mut Guardian {
    Box::into_raw(Box::new(Guardian::Map(HashMap::new())))
}

#[no_mangle]
pub fn guardian_free(guardian: *mut Guardian) -> bool {
    if !guardian.is_null() {
        unsafe { Box::from_raw(guardian) };
        true
    } else {
        false
    }
}

#[no_mangle]
pub fn guardian_which(guardian: *mut Guardian) -> u8 {
    if let Some(mut x) = NonNull::new(guardian) {
        match unsafe { x.as_mut() } {
            Guardian::Data(_) => 1,
            Guardian::Map(_) => 2,
        }
    } else {
        0
    }
}

#[no_mangle]
pub fn guardian_get_data(guardian: *mut Guardian) -> u32 {
    (|| {
        if let Guardian::Data(map) = unsafe { NonNull::new(guardian)?.as_ref() } {
            Some(*map)
        } else {
            None
        }
    })()
    .unwrap_or(0)
}

#[no_mangle]
pub fn guardian_set_data(guardian: *mut Guardian, data: u32) -> bool {
    (|| {
        *unsafe { NonNull::new(guardian)?.as_mut() } = Guardian::Data(data);
        Some(true)
    })()
    .unwrap_or_default()
}

#[no_mangle]
pub fn guardian_get_map_elem(guardian: *mut Guardian, key: u32) -> *mut Guardian {
    (|| {
        if let Guardian::Map(map) = unsafe { NonNull::new(guardian)?.as_mut() } {
            Some(map.entry(key).or_insert(Guardian::Data(0)) as *mut Guardian)
        } else {
            None
        }
    })()
    .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub fn guardian_detach_map_elem(guardian: *mut Guardian, key: u32) -> *mut Guardian {
    (|| {
        if let Guardian::Map(map) = unsafe { NonNull::new(guardian)?.as_mut() } {
            Some(Box::into_raw(Box::new(map.remove(&key)?)))
        } else {
            None
        }
    })()
    .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub fn guardian_delete_map_elem(guardian: *mut Guardian, key: u32) -> bool {
    (|| {
        if let Guardian::Map(map) = unsafe { NonNull::new(guardian)?.as_mut() } {
            map.remove(&key).map(|_| true)
        } else {
            None
        }
    })()
    .unwrap_or_default()
}

#[no_mangle]
pub fn guardian_replace(guardian: *mut Guardian, map: *mut Guardian) -> bool {
    if map.is_null() {
        return false;
    }
    (|| {
        unsafe { *NonNull::new(guardian)?.as_mut() = *Box::from_raw(map) };
        Some(true)
    })()
    .unwrap_or_default()
}

#[test]
fn test_guardian() {
    let guardian_root = guardian_new();
    assert_eq!(guardian_which(guardian_root), 2);

    let guardian_a = guardian_get_map_elem(guardian_root, 1);
    assert_eq!(guardian_which(guardian_a), 1);
    assert!(guardian_set_data(guardian_a, 2));
    assert_eq!(guardian_get_data(guardian_a), 2);

    let guardian_b = guardian_get_map_elem(guardian_root, 3);
    assert!(guardian_replace(guardian_b, guardian_new()));

    let guardian_ba = guardian_get_map_elem(guardian_b, 4);
    assert!(guardian_set_data(guardian_ba, 5));

    // Make sure that we can get to guardian_ba from root again
    assert_eq!(
        guardian_get_data(guardian_get_map_elem(
            guardian_get_map_elem(guardian_root, 3),
            4
        )),
        5
    );
    assert!(guardian_free(guardian_root));
}
