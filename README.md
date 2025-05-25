# dx-chat

A Chat application built with [Dioxus](https://dioxuslabs.com/) and [Tailwind CSS](https://tailwindcss.com).

## Prerequisites

* [Rust](https://www.rust-lang.org/) (stable toolchain)
* [Node.js](https://nodejs.org/) and npm

## Installation

Before running the application, install the required tools:

1. **Dioxus CLI**

   ```bash
   cargo install dioxus-cli
   ```
2. **Tailwind CSS and its CLI**
    - Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
    - Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation

   ```bash
   npm install tailwindcss @tailwindcss/cli
   ```

## Running the Application

1. **Build and watch Tailwind CSS styles**

   ```bash
   npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
   ```
2. **Start the development server**

   ```bash
   dx serve
   ```

Your application will be available at `http://localhost:8080` by default.

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

## License

Distributed under the MIT License. See `LICENSE` for details.
