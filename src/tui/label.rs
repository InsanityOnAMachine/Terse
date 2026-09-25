pub struct Label ();

impl Label {
    pub fn new(key: impl AsRef<str>, action: impl AsRef<str>) -> String {
        String::from(" ") + key.as_ref() + ": " + action.as_ref() + " "
    }
    pub fn join(labels: Vec<String>) -> String {
        labels.join(" - ")
    }
}
