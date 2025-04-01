# Gem Project

The **Gem Project** is a desktop application built using [Tauri](https://tauri.app/) and [Svelte](https://svelte.dev/). It allows users to create, manage, and view receipts seamlessly. The app integrates with [Supabase](https://supabase.com/) for backend services and uses [Sass](https://sass-lang.com/) for styling, ensuring a modern and responsive user interface.

## Features

- **Receipt Creation**: Easily create detailed receipts with customizable fields.
- **Receipt Management**: View, edit, and delete receipts with a user-friendly interface.
- **Supabase Integration**: Store and retrieve receipt data securely in the cloud.
- **Sass Styling**: Enjoy a sleek and responsive design powered by Sass.
- **Cross-Platform**: Runs on Windows, macOS, and Linux thanks to Tauri.

## Tech Stack

- **Frontend**: Svelte
- **Backend**: Supabase
- **Styling**: Sass
- **Desktop Framework**: Tauri

## Getting Started

1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/gem_project.git
    cd gem_project
    ```

2. Install dependencies:
    ```bash
    npm install
    ```

3. Configure Supabase:
    - Create a Supabase project.
    - Add your Supabase API keys to the constants located in db.rs file.

4. Run the development server:
    ```bash
    pnpm run dev
    ```

5. Build the desktop app:
    ```bash
    pnpm run tauri build
    ```
