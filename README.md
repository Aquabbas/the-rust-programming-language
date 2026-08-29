# The Rust Programming Language

---

[Where I'm at in the book](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

---

[First `LLM` Session - `OpenCode`](`~/src/github.com/Aquabbas/the-rust-programming-language`)

[First `LLM` Session - `claude.ai`](https://claude.ai/chat/1a35d3d2-f37d-4fc0-bb81-4324a49fc6c5)

---

## The Stack vs. The Heap

---

- **`Stack` (Faster):** Stores values in the order it gets them, and removes the values in the opposite order -> `Last In, First Out` (LIFO)
  - **Must Have:** `Known/Fixed size`

Ex: `Pushing onto the Stack`, `Popping off the Stack`

- **`Heap` (Slower):** Less organized -> `Memory Allocator` finds an empty spot in the `heap` and returns a `pointer` -> The `pointer` is stored in the `Stack`

Ex: `Allocating on the heap`, `Allocating`

"Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation." -> `The Rust Programming Language Book`, `Chapter 04.01`

---

### Ownership Rules

---

1. Each value in Rust has an `Owner`
2. There can _only be one_ `Owner` at a time
3. When the `Owner` is out of scope, the value will be dropped

---

I'm here in the Book now (Chapter `04.01`):

![[/home/abbashayder/Pictures/Screenshots/Screenshot_2026-03-25_21-58-02.png]]

---
