pub struct Rwstruct {
    pub age:i32,
    pub name:String
}
impl Rwstruct {
    pub fn getfile(self) -> (i32,String) {
        (self.age,self.name)
    }
}