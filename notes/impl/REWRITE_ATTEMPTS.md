# Attempts at a source-level rewrite

## `as_ref` ❌

```rust
if let dog = animal.as_ref::<Dog>() {
    <Dog as Animal>::speak(&dog);
}
```

errors: 

- error[E0308]: mismatched types: expected `&Dog`, found `&&dyn Animal`

## `AsRef` ❌

```rust
if let dog = AsRef::<Dog>::as_ref(animal) {
    <Dog as Animal>::speak(&dog);
}
```

errors:

- `AsRef` trait not impl for `Dog`

## `Any::downcast_ref` ❌

```rust
if let Some(dog) = animal.downcast_ref::<Dog>() {
    <Dog as Animal>::speak(&dog);
}
```

errors:

- error[E0599]: no method named `downcast_ref` found for struct `Box<dyn Animal>` in the current scope

- even when `trait Animal : Any {...}`
    - apparently supertraits don't extend to trait objects or something (at
      least not yet)
      - [stackoverflow](https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object)
      - [stackoverflow](https://stackoverflow.com/questions/26126683/how-to-match-trait-implementors)

## `type_id()` ❌

```rust
if animal.type_id() == 1 {
    <Dog as Animal>::speak(&dog);
}
```

errors:

- error[E0308]: mismatched types: expected `&Dog`, found `&dyn Animal`

## `Any` + `as_any()` ✅

are we introducing another vtable call here?
- looked at assembly, doesn't seem to be: two paths, each == inlined speak code

```rust
trait Animal: Any {
    ...
    fn as_any(&self) -> &dyn Any;
}

impl Animal for Dog {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

if let Some(dog) = animal.as_any().downcast_ref::<Dog>() {
    <Dog as Animal>::speak(&dog);
}
```

## `type_id` + `transmute` ✅

```rust
    let animal = get_animal();
    let ti = animal.type_id();
    
    let rawptr = Box::into_raw(animal) as *const ();
    if ti == 1 {
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(rawptr);
            <Cat as Animal>::speak(cat);
        }
    } ...
```
assembly code inlines speak calls (no vtable)

