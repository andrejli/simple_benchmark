pub struct Ryba {
    pub active: bool,
    pub druh: String,
    pub dlzka: i32,
    pub vaha: i32,
}

impl Ryba {
    /// Simple getter function
    pub fn get_active(&self) -> bool { return self.active} // getter
    pub fn set_active(&mut self, value: bool) { self.active = value } // setter
}