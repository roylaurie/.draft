pub trait Builder<'original>: Sized {
    type Type;

    fn new() -> Self;
    fn editor(original: &'original mut Self::Type) -> Self { todo!() }
    fn build(self) -> Self::Type; 

    fn edit(self, composite_fields_changed: Option<Vec<Field>>) -> Result<Vec<Field>, ()> {
        todo!()
    }

    fn set(&mut self, field: &str, raw_value: String) -> Result<(), ()> {
        todo!()
    }

    fn validate(self) -> Self {
        self
    }
}

pub enum FieldValueType {
    String,
    Integer,
    Float,
    Boolean,
    StringArray
}

pub struct Field {
    pub name: &'static str,
    pub value_type: FieldValueType
}

