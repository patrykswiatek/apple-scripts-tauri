# Tauri + SvelteKit Application - Managing AppleScript

## Description

The application is used for saving AppleScript scripts in the application memory and later executing them using created buttons. There are two main menu options in the application: adding buttons and removing buttons.

## Features

### Adding Buttons

1. **Adding a New Button:**
    - Users can add a new button by providing a button title and pasting an AppleScript.
    - After saving changes, a new button is created and appears on the button list below the menu.

2. **Running Scripts:**
    - Clicking on the created button executes the previously saved AppleScript.

### Removing Buttons

1. **Removing Selected Buttons:**
    - Users can select buttons they no longer need.
    - After selecting buttons for removal and confirming the selection, the chosen buttons are removed from the button list.

## Installation and Running

### Requirements

- Node.js
- npm package manager
- Rust Compiler (for Tauri handling)

### Step 1: Installation

Install dependencies with:
```bash
npm install
```

### Step 2: Running

Run the development server using:
```bash
npm run tauri dev
```

### Step 3: Build

Build the app:
```bash
npm run tauri build
```
