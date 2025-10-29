

#[derive(Debug)]
pub struct Key {
    inner: String
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Self { inner: value }
    }
}

impl From<syn::Ident> for Key {
    fn from(value: syn::Ident) -> Self {
        Self {
            inner: value.to_string()
        }
    }
}


#[derive(Debug)]
pub struct Variable {
    inner: String
}

#[derive(Debug)]
pub enum Value {
    Char(i32),
    Number(f32)

}