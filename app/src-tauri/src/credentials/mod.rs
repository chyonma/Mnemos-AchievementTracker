use keyring::Entry;

const SERVICE: &str = "mnemos";

pub fn save_credential(key: &str, value: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE, key).map_err(|e| e.to_string())?;
    entry.set_password(value).map_err(|e| e.to_string())
}

pub fn get_credential(key: &str) -> Option<String> {
    Entry::new(SERVICE, key).ok()?.get_password().ok()
}

pub fn has_credential(key: &str) -> bool {
    get_credential(key).is_some()
}