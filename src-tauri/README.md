# EduPage Grade Average Calculator

This script calculates your grade average on EduPage, factoring in a new grade. Since EduPage doesn't have this feature, this tool scrapes the site to do it.

![EduCalc screenshot](https://firebasestorage.googleapis.com/v0/b/personalsite-f2369.appspot.com/o/img%2Feducalc3.png?alt=media)

## Usage

### Option 1: From Source

1. Clone the repository and create a `.env` file in the root directory with your EduPage credentials:

   ```env
   USERNAME=your_username
   PASSWORD=your_password
   ```

2. Run the script:

   - For the GUI:

     ```bash
     cd src-tauri
     cargo tauri dev
     ```

   - For the TUI:
     ```bash
     cd src-tauri
     cargo tauri dev -- --term
     ```

### Option 2: From a Binary

1. Run the binary:

   - For the GUI:

     ```bash
     ./edupage-average-calculator
     ```

   - For the TUI:
     ```bash
     ./edupage-average-calculator --term
     ```

## How It Works

The script scrapes EduPage and calculates your grade average based on your current grades and the new one.

## Known Issues

- EduPage can be unreliable, and scraping in the TUI version might fail at times.
