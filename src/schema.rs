pub enum PrimitiveType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    U8,
    U16,
    U32,
    U64,
    U128,
    String /* Can be constrained to e.g. only these enumerated values or this regex, length */,
    Binary, /* Can be constrained on length */
    Uuid,
    Date,
    DateTime,
    Geometry /* Can be constrained to e.g. only LineString */,
}

pub enum Type {
    Primitive(PrimitiveType)
}
