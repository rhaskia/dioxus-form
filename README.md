# Dioxus Form
This crate is a serializer of sorts, and using serde it can serialize a given struct (primitives, vecs, etc are currently a bit broken when deserializing) into an HTML form.
Currently it's made specifically for dioxus, so it requires a Signal<T> to work.

# To Do/Issues
- Better vec functionality, currently vecs are not expandable and do not have removable elements
- 
