# Go slices as shared headers (`GO2RUST_GOSLICE=1`)

A Go slice is two things: a backing array that slices share, and a header
(offset, length, capacity) that each variable owns. The default lowering has
only one of them. Each slice is a `Vec<T>` behind a cell, so:

- `append` pushes onto the shared `Vec`, and every alias sees the new length;
- `xs = nil` in a callee empties the caller's slice;
- `t := s[1:3]` copies the elements, so a write through `t` is lost;
- `row := grid[0]` copies the row (DEV-71).

DEV-36 tracks this. `GO2RUST_GOSLICE=1` turns on the replacement. Without the
variable, the generated code is unchanged.

## The runtime type

`GoSlice<T>` (`go/goslice.go`) is a header over an
`Arc<UnsafeCell<Vec<T>>>` backing:

- `Clone` copies the header. Clones share the elements, as a Go slice
  assignment does.
- `Deref`/`DerefMut` to `[T]` give the header's window. Indexing, `len`,
  `iter`, `sort`, `join` and slice patterns keep working.
- `reslice3(lo, hi, max)` is `s[lo:hi:max]`. The result shares the backing.
- `push` is `s = append(s, v)`. While there is capacity it writes in place,
  past the end of the other headers' windows. Otherwise it copies the window
  into a new backing, growing the capacity the way Go's runtime does for
  small slices.
- The backing `Vec` is reserved up to the largest capacity any header
  holds. Pushing onto it therefore never moves elements that another header
  is reading.

The generated code shadows `Vec<T>` with `pub type Vec<T> = GoSlice<T>` and
`vec!` with a macro that builds a `GoSlice`. The types and literals that the
translator writes as `Vec<T>` and `vec![..]` are therefore Go slices. A
macro reached through an import, or defined in an `include!`d file, is
ambiguous with the prelude's `vec!`. So every generated file that is not
`include!`d defines the macro itself, at the top. Scratch vectors that are
not Go slices name `std::vec::Vec`: the defer stack, array-literal
construction, and the moved-out values of a range over `[]interface{}`.

Every binding inside the runtime starts with `__`. The crate root imports
every package-level variable of the Go package, and Rust does not let a
local or a parameter take the name of one.

## What the translator does differently

- `s[lo:hi]` on a slice emits `reslice3`. The default emits a copy.
- `append(s, v)` builds a new header from `s` and leaves `s` alone. The
  default pushes into `s`'s cell.
- `make([]T, n, c)` fills the backing with `c` zero values, each made on its
  own, and views `n` of them. Reslicing up to the capacity then reads zeros.
- `xs = v` for a local slice variable stores `v`'s header into `xs`'s own
  cell. The cell is never replaced, so a closure that captured `xs` sees the
  assignment. The header is read through `GoSliceHeader`, which accepts a
  cell, a header or an `Option` of one.
- A slice stored into a struct field or a map value gets a cell of its own.
  So does a `:=` variable whose value may be a cell that something else owns,
  such as a call result or a field read. Later assignments to the source
  variable then do not reach it.
- A slice parameter that the callee assigns to, or takes the address of, is
  copied into a cell of its own on entry.
- `[]interface{}` elements have no `Clone`. Their appends go through
  `GoSliceAppendBoxed`, which appends at the end of the shared backing when
  the slice is full instead of copying it.

## Known gaps

- Appending to a full `[]interface{}` that another header shares writes into
  the shared backing instead of copying it. Two appends to the same full
  slice can then see each other's element.
- A slice that the generated code builds with `Vec::with_capacity`, rather
  than with `make`, stores no zero values. Reslicing it past its length up to
  its capacity panics instead of reading zeros.
- A pointer to a slice element follows the variable's cell rather than the
  backing array. After an append that reallocates, it sees the new array,
  where Go's pointer stays on the old one.
