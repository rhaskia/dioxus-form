use crate::Error;
use dioxus::prelude::*;
use serde::ser::{
    Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

pub fn create_form<T>(value: Signal<T>) -> Result<String, Error>
where
    T: Serialize,
{
    let mut serializer = FormBuilder {
        output: String::new(),
        nesting: vec![],
        list: vec![],
    };
    value.read().serialize(&mut serializer)?;
    Ok(serializer.output)
}

fn readable(snake_case: &str) -> String {
    let mut readable = String::new();
    let mut capitalize_next = true;

    for c in snake_case.chars() {
        if c == '_' {
            capitalize_next = true;
            readable.push(' ');
        } else if capitalize_next {
            readable.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            readable.push(c);
        }
    }

    readable
}

pub struct FormBuilder {
    output: String,
    nesting: Vec<String>,
    list: Vec<(usize, usize)>,
}

impl<'a> Serializer for &'a mut FormBuilder {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output += "<input name=\"";
        self.output += &self.nesting.join(".");
        self.output += ".b\"";
        if v { self.output += " checked"; }
        self.output += &format!(" type=\"checkbox\"/><br/>");
        self.output += "<input type=hidden value=\"off\" name=\"";
        self.output += &self.nesting.join(".");
        self.output += ".b\"/>";
        Ok(())
    }

    // TODO: limits for different types
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let name = format!("{}.n", &self.nesting.join("."));
        self.output += "<input name=\"";
        self.output += &name;
        self.output += "\" id=\"";
        self.output += &name;
        self.output += &format!("\" value = {v} type=\"number\" /><br/>");
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let name = format!("{}.n", &self.nesting.join("."));
        self.output += "<input name=\"";
        self.output += &name;
        self.output += "\" id=\"";
        self.output += &name;
        self.output += &format!("\" min=0 value = {v} type=\"number\" /><br/>");
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output += "<input name=\"";
        self.output += &self.nesting.join(".");
        self.output += ".n";
        self.output += &format!("\" value = {v} type=\"number\" /><br/>");
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.output += "<input max_length=1 name=\"";
        self.output += &self.nesting.join(".");
        self.output += ".s";
        self.output += &format!("\" value = {v:?}/><br/>");
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output += "<input name=\"";
        self.output += &self.nesting.join(".");
        self.output += ".s";
        self.output += &format!("\" value = {v:?}/><br/>");
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> { 
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            SerializeSeq::serialize_element(&mut seq, byte)?;
        }
        SerializeSeq::end(self)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        // TODO button that creates
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        // TODO: remove button
        value.serialize(&mut *self)?;
        self.output += "<button onclick=\"this.parentNode.removeChild(this.previousElementSibling);\">Remove</button>";
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Ok(()) }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        // self.output += &format!("<label>{name}</label>");
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.unwrap();
        let idx = self.nesting.len() - 1;
        self.list.push((len, len));
        self.nesting[idx] += "[0]";
        self.output += "<div class = \"formlist\">";
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.output += &format!("<fieldset name=\"{name}\" >");
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.output += &format!("<fieldset name=\"{name}\" >");
        Ok(self) // should be fine?
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.output += "<fieldset name=\"map\" >";
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.output += &format!("<fieldset name={name:?} >");
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.output += &format!("<fieldset name={name:?} >");
        Ok(self)
    }
}

impl<'a> SerializeStruct for &'a mut FormBuilder {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.output += &format!(
            "<label class=\"inputname\" for={key:?}>{} </label>",
            readable(key)
        );
        self.nesting.push(key.to_string());
        value.serialize(&mut **self)?;
        self.nesting.pop();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "</fieldset>";
        self.nesting.pop();
        Ok(())
    }
}

impl<'a> SerializeTupleStruct for &'a mut FormBuilder {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { todo!() }
}

impl<'a> SerializeStructVariant for &'a mut FormBuilder {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.output += &format!(
            "<label class=\"inputname\" for={key:?}>{} </label>",
            readable(key)
        );
        self.nesting.push(key.to_string());
        value.serialize(&mut **self)?;
        self.nesting.pop();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { 
        self.output += "</fieldset>";
        self.nesting.pop();
        Ok(())
    }
}

impl<'a> SerializeTupleVariant for &'a mut FormBuilder {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.list.last_mut().unwrap().0 -= 1;
        let list_idx = self.list.last().unwrap().1 - self.list.last().unwrap().0; 
        while self.nesting.last_mut().unwrap().pop() != Some('[') {}
        self.nesting.last_mut().unwrap().push_str(&format!("[{}]", list_idx - 1));
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "</div>";
        Ok(())
    }
}

impl<'a> SerializeTuple for &'a mut FormBuilder {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.list.last_mut().unwrap().0 -= 1;
        let list_idx = self.list.last().unwrap().1 - self.list.last().unwrap().0; 
        while self.nesting.last_mut().unwrap().pop() != Some('[') {}
        self.nesting.last_mut().unwrap().push_str(&format!("[{}]", list_idx - 1));
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "</div>";
        Ok(())
    }
}

impl<'a> SerializeSeq for &'a mut FormBuilder {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.list.last_mut().unwrap().0 -= 1;
        let list_idx = self.list.last().unwrap().1 - self.list.last().unwrap().0; 
        while self.nesting.last_mut().unwrap().pop() != Some('[') {}
        self.nesting.last_mut().unwrap().push_str(&format!("[{}]", list_idx - 1));
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let list_idx = self.list.last().unwrap().1; 
        while self.nesting.last_mut().unwrap().pop() != Some('[') {}
        self.nesting.last_mut().unwrap().push_str(&format!("[{}]", list_idx - 1));

        // self.output += "<button class=\"addbutton\" onclick=\"\">Add</button>";
        // self.output += "</div>";
        self.list.pop();
        Ok(())
    }
}

impl<'a> SerializeMap for &'a mut FormBuilder {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.nesting.push("key".to_string());
        key.serialize(&mut **self)?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)?;
        self.nesting.push("value".to_string());
        //self.nesting.pop();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "</fieldset>";
        self.nesting.pop();
        Ok(())
    }
}
