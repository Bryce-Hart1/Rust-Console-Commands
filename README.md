# Rust Console Commands

Two simple command-line utilities built in Rust, built ontop of the original commands provided


---

## Commands

### `mean`
Calculates the mean (average) of a list of whole numbers.

**Usage:**
```bash
mean <num1> <num2> ...
```

**Example:**
```bash
mean 10 20 30 40 50
# Mean: 30
```

**Notes:**
- Requires at least one number
- Accepts whole numbers (i32) only
- Decimals will be truncated (e.g. mean of 1 and 2 = 1)

---

### `clamp`
Clamps a number between a minimum and maximum value.

**Usage:**
```bash
clamp <number> <min> <max>
```

**Examples:**
```bash
clamp 150 0 100
# 150 is above range, clamped to max: 100

clamp -5 0 100
# -5 is below range, clamped to min: 0

clamp 42 0 100
# 42 is already within range [0, 100]
```

**Notes:**
- Accepts decimal numbers (f64)
- Min cannot be greater than max

---

## Building

build individually with Cargo:

```bash
cd mean
cargo build --release

cd clamp
cargo build --release
```
