
# Swift MT Parsing Library — Full Specification, Project Structure & Developer API

---

## ✅ Objective

Define full API and project structure for Swift MT parsing library in Rust.

- SwiftField definitions (per field)
- SwiftMessage definitions (per message)
- Auto-injected Header parsing (Block 1-5)
- Semantic JSON serialization model
- Developer-facing code structure & file organization

---

## ✅ Design Layers

| Layer | Responsibility |
|---|---|
| SwiftField | Define strongly typed reusable field types |
| SwiftMessage | Compose MT messages using SwiftFields |
| SwiftHeader | Parse Header Blocks 1, 2, 3, 5 |
| SwiftParser | Full message parser |
| SwiftSemanticModel | Normalize for JSON serialization |

---

## ✅ SwiftField Specification

```rust
#[derive(SwiftField)]
pub struct FieldXX {
    #[format("FORMAT")]
    pub field_name: Type,
}
```

### Field Options

```rust
#[derive(SwiftField)]
pub enum FieldXX {
    #[field_option("A")]
    OptionA(FieldXXA),
    
    #[field_option("F")]
    OptionF(FieldXXF),

    #[field_option("K")]
    OptionK(FieldXXK),
}
```

---

## ✅ SwiftMessage Specification

```rust
#[derive(SwiftMessage)]
#[swift_message(mt = "103")]
pub struct MT103 {
    #[field("20")]
    pub field_20: Field20,
    #[field("23B")]
    pub field_23b: Field23B,
    #[field("32A")]
    pub field_32a: Field32A,
    #[field("50")]
    pub field_50: Field50,
    #[field("59")]
    pub field_59: Field59,
    #[field("71A")]
    pub field_71a: Field71A,
}
```

---

## ✅ SwiftHeader Specification

- Automatically injected into all SwiftMessages.

```rust
pub struct SwiftMessage<T: SwiftBody> {
    pub basic_header: BasicHeader,
    pub application_header: ApplicationHeader,
    pub user_header: Option<UserHeader>,
    pub trailer: Option<Trailer>,
    pub blocks: RawBlocks,
    pub message_type: String,
    pub field_order: Vec<String>,
    pub fields: T,
}
```

---

## ✅ Developer-Facing Project Structure

```
src/
│
├── lib.rs
│
├── fields/
│   ├── mod.rs
│   ├── field20.rs
│   ├── field23b.rs
│   ├── field32a.rs
│   ├── field50.rs
│   ├── field59.rs
│   ├── field71a.rs
│
├── messages/
│   ├── mod.rs
│   ├── mt103.rs
│   ├── mt202.rs
│
├── headers/
│   └── mod.rs (block1, block2, block3, block5 structs)
│
└── macros/
    └── derive_macros.rs
```

---

## ✅ Sample Field Definition Files

### fields/field50.rs

```rust
#[derive(SwiftField)]
pub enum Field50 {
    #[field_option("A")]
    OptionA(Field50A),
    #[field_option("F")]
    OptionF(Field50F),
    #[field_option("K")]
    OptionK(Field50K),
}

#[derive(SwiftField)]
pub struct Field50A {
    #[format("BIC")]
    pub bic: BIC,
}

#[derive(SwiftField)]
pub struct Field50F {
    #[format("structured_party_identifier")]
    pub party_identifier: StructuredPartyIdentifier,
}

#[derive(SwiftField)]
pub struct Field50K {
    #[format("4*35x")]
    pub name_and_address: Vec<String>,
}
```

### fields/field20.rs

```rust
#[derive(SwiftField)]
pub struct Field20 {
    #[format("16x")]
    pub value: String,
}
```

---

## ✅ Sample Message Definition Files

### messages/mt103.rs

```rust
#[derive(SwiftMessage)]
#[swift_message(mt = "103")]
pub struct MT103 {
    #[field("20")]
    pub field_20: Field20,
    #[field("23B")]
    pub field_23b: Field23B,
    #[field("32A")]
    pub field_32a: Field32A,
    #[field("50")]
    pub field_50: Field50,
    #[field("59")]
    pub field_59: Field59,
    #[field("71A")]
    pub field_71a: Field71A,
}
```

---

## ✅ Developer Usage Flow

```rust
let raw_mt103 = "... full MT103 message text ...";

let parsed: SwiftMessage<MT103> = SwiftParser::parse(raw_mt103)?;

let json_output = serde_json::to_string_pretty(&parsed)?;
println!("{}", json_output);
```

---

## ✅ Target Output

- JSON structure includes headers, raw blocks, fields, and field order fully normalized.

---

## ✅ Summary

- ✅ Fully modular project structure
- ✅ Reusable strongly typed SwiftField library
- ✅ Minimal message definition per message file (clean, maintainable)
- ✅ Full derive macro support for parsing & validation
- ✅ Production-grade JSON serialization format

---
