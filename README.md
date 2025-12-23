
# Image Pixel Weaver

Image Pixel Weaver is a Rust command-line tool that combines two images by **alternating their pixels**. The result is a single “woven” image where pixels from both inputs are interleaved in a checkerboard-like pattern.

---

## Features

- 📷 Supports common image formats via the `image` crate
- 📐 Automatically resizes images to the same dimensions
- 🧵 Alternates pixels from each image (RGBA-aware)
- 💾 Saves output in the same format as the input images
- ⚠️ Robust error handling with custom error types

---

## How It Works

1. Loads two images from disk
2. Ensures both images have the same file format
3. Resizes the larger image to match the smaller one
4. Converts both images to RGBA pixel buffers
5. Alternates pixels between the two images
6. Writes the combined image to disk

Each pixel (4 bytes: RGBA) is taken alternately from image 1 and image 2.

---

## Installation

Make sure you have Rust installed:

```bash
rustup install stable
````

Clone the repository and build:

```bash
cargo build --release
```

---

## Usage

```bash
cargo run --release -- \
  --img-1 path/to/image1.png \
  --img-2 path/to/image2.png \
  --output output.png
```

> Both input images **must be the same format** (e.g., PNG + PNG).

---

## Example

**Input:**

* `image1.png`
* `image2.png`

**Output:**

* `output.png` (alternating pixels from each image)

Visually, the result appears as a woven or checkerboard blend of both images.

---

## Project Structure

```text
.
├── src
│   ├── main.rs        # Application logic
│   └── args.rs        # Command-line argument parsing
├── Cargo.toml
└── README.md
```

---

## Error Handling

The application uses a custom `ImageDataErr` enum to handle:

* Image format mismatches
* File read failures
* Decode errors
* Buffer size issues
* Save failures

Errors are propagated cleanly using `Result`.

---

## Performance Notes

* Images are converted to RGBA for consistent pixel handling
* Uses Triangle filtering for resizing
* Current pixel interleaving uses vector splicing (can be optimized if needed)

---

## Dependencies

* [`image`](https://crates.io/crates/image) – image decoding, resizing, and encoding

---

## Future Improvements

* Support different blending modes
* Optimize pixel merging logic
* Add parallel processing
* Allow row-wise or column-wise interleaving
* Optional alpha blending

---

## License

MIT License

