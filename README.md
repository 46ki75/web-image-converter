# web-image-converter

Convert images directly in the browser using WebAssembly — no server or Node.js required.

## Features

- 🖼️ Convert images to **PNG**, **JPEG**, **WebP**, or **BMP**
- 🧠 Automatically detects input format
- ⚙️ Powered by Rust & WebAssembly
- 💡 Runs entirely in the browser (frontend only)

## Installation

```bash
npm install web-image-converter
```

## Usage

```ts
import init, { jpeg, png, webp, bmp } from "web-image-converter";

const toPng = async (image: Uint8Array): Promise<Uint8Array> => {
  await init();
  return png().convert(image);
};

const toJpeg = async (image: Uint8Array): Promise<Uint8Array> => {
  await init();
  return jpeg().convert(image);
};

const toWebp = async (image: Uint8Array): Promise<Uint8Array> => {
  await init();
  return webp().convert(image);
};

const toBmp = async (image: Uint8Array): Promise<Uint8Array> => {
  await init();
  return bmp().convert(image);
};
```

### Input

- `image`: `Uint8Array` containing the image data.
- Format (JPEG, PNG, WebP, BMP) is **auto-detected**.

### Output

- `Uint8Array` containing the converted image.

> [!TIP]
> To avoid blocking the main UI thread during image processing, it is **strongly recommended** to use this library inside a **Web Worker**.

## Vite Development Server Configuration

When using Vite, WebAssembly modules may encounter MIME type issues during development. To resolve this, exclude `web-image-converter` from dependency optimization:

```ts
// vite.config.ts
import { defineConfig } from "vite";

export default defineConfig({
  // ...
  optimizeDeps: {
    exclude: ["web-image-converter"],
  },
  // ...
});
```

## License

This project is licensed under the **MIT License**.

> [!NOTE]
> Since this library uses **Rust crates compiled to WebAssembly with static linking**, third-party licenses of all linked dependencies are automatically bundled into the NPM package.
> You can find the list of these licenses in the included `LICENSE` or equivalent file.
>
> This ensures compliance with all licenses, including transitive dependencies of the WebAssembly module.
