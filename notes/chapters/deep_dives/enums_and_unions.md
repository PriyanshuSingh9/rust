### Rust Enums vs TypeScript Union Types

**Example: Payment methods**

#### TypeScript - Union

```ts
type Payment =
  | { type: "cash"; amount: number }
  | { type: "card"; number: string }
  | { type: "crypto"; address: string };
```

- `Payment` can be **cash, card, or crypto**.
- TypeScript uses the `type` field to determine which one it is.
- You can use `if`/`switch` to narrow the type.

#### Rust - Enum

```rust
enum Payment {
    Cash(u32),
    Card(String),
    Crypto(String),
}
```

- `Payment` is **one concrete type**.
- It has exactly **three defined variants**.
- Each variant can hold different data.
- `match` can force you to handle every possibility:

```rust
match payment {
    Payment::Cash(amount) => {},
    Payment::Card(number) => {},
    Payment::Crypto(address) => {},
}
```

### Why Rust enums are powerful

If you later add:

```rust
BankTransfer(String)
```

to `Payment`, Rust will find every `match` that doesn't handle `BankTransfer` and give a **compile-time error**.

```text
Payment
├── Cash(amount)
├── Card(number)
├── Crypto(address)
└── BankTransfer(account)  ← newly added
```

**Mental model:**

> TypeScript union: **"this value can be A, B, or C."**
> Rust enum: **"this concrete type has exactly these possible variants."**

**Key caveat:** TypeScript discriminated unions can achieve very similar behavior; Rust's major advantage is that exhaustive matching and variant handling are built directly into the language's type system. Rust makes algebraic data types and exhaustive pattern matching a core part of its type system, whereas TypeScript achieves similar behavior through unions, discriminated unions, and control-flow type narrowing.
