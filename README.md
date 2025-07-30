# Feather Calendar

A lightweight, **pinnable desktop calendar** that won't get in your way during meetings.
It's a simple and fast Windows application that you can quickly open when you need it and close when you're done.

<img width="1063" height="399" alt="image" src="https://github.com/user-attachments/assets/9a38c400-2d55-4e01-8acb-f71018dda288" />

## What is Feather Calendar?

"When should we schedule the next meeting?" This is a common question during web conferences.
The default OS calendar disappears when you click on another window, and full-featured schedulers like Outlook can be overkill when all you want to do is **just check a date**.

Feather Calendar was created to solve these minor frustrations.
By completely **removing scheduling features** and focusing solely on **providing a quick way to view dates**, it offers a level of lightness and convenience that other calendars don't.

## How is it different from other calendars?

To clarify its position, here's a feature comparison with major calendar applications.

| Feature | Feather Calendar | Windows Default | Google Calendar (Web) | Outlook (Desktop) |
|:---|:---:|:---:|:---:|:---:|
| **Installation** | **◎ (Not Required)** | ◎ (OS Integrated) | ◎ (Not Required) | ✕ (Required) |
| **Startup Speed / Lightweight** | **◎** | ○ | △ (Browser dependent) | ✕ |
| **Always on Top (Pinnable)** | **◎** | ✕ | ✕ | ✕ |
| **Stays Visible (Doesn't disappear)** | **◎** | ✕ | ○ (Tab/Window) | ○ (Window) |
| **Multi-Month View** | **◎ (3 Months)** | ✕ (1 Month) | ○ | ○ |
| **Scheduling / Sync** | **✕ (Not Supported)** | ○ (Limited) | ◎ | ◎ |

As this table shows, Feather Calendar is a specialized tool for "quick date reference," not "event management."

## Features

- 🚀 **No Installation Required**: Just unzip the file and run the `.exe`. It doesn't clutter your registry.
- 📌 **Always on Top (Pinnable)**: Keep the calendar visible above all other windows with a single click.
- 📅 **Intuitive 3-Month View**: See the previous, current, and next months at a glance for easy scheduling across months.
- 🎨 **Date Highlighting**: Click on any date to highlight it, using it as a simple marker.
- 🌗 **Theme-Aware**: Automatically switches between light and dark modes to match your OS settings.
- 💨 **Lightweight & Fast**: Built with Rust and egui for low memory usage and snappy performance on any PC.
- 📐 **Responsive Design**: The calendar layout automatically adjusts to fit the window size.

## Use Cases

- Scheduling meetings during web conferences or online classes.
- Keeping a calendar permanently visible on a secondary monitor.
- For developers who need to quickly check dates and days of the week.
- For anyone frustrated with the "disappearing" default OS calendar.
- When you need a quick, offline calendar, even if you normally use a web-based one like Google Calendar.

## How to Use

### Getting Started
1.  **Download**: Get the latest zip file from the **[Releases page](https://github.com/FlatBone/feather-calendar/releases)**.
2.  **Extract**: Unzip the downloaded file to your desired location.
3.  **Launch**: Double-click `Feather-Calendar.exe` to start the application.

### Usage
#### Calendar View
The calendar displays the previous, current, and next months based on today's date.

#### Highlighting Dates
You can highlight a date by clicking on it. Click the date again to remove the highlight.

#### Changing the Displayed Month
Click the arrow buttons next to "Today" to change the displayed month. Click the "Today" button to return to the default view.

#### Pinning (Always on Top)
You'll find the "Pin" button to the right of the month navigation controls. Click it to keep the calendar always on top of other windows. Click the button again to unpin it.

## System Requirements

- **OS**: Windows 10, Windows 11

## For Developers

### How to Build

To build this application from the source code, follow these steps.

1.  **Set up the Rust environment.**

    The easiest way is to use [rustup](https://rustup.rs/).

    ```bash
    # Installs rustup and the default toolchain
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    **For Windows (MSVC) users:**
    The `x86_64-pc-windows-msvc` toolchain requires the Microsoft C++ Build Tools. Please install them via the [Visual Studio Installer](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
    In the installer, select the "C++ build tools" workload.

    After installation, it is recommended to run the build commands in the **Developer Command Prompt for VS** to ensure all necessary environment variables are set correctly.

2.  **Clone this repository.**

    ```sh
    git clone https://github.com/FlatBone/feather-calendar.git
    cd feather-calendar
    ```

3.  **Build the application.**

    ```bash
    cargo build --release
    ```

      Once the build is complete, the executable will be located at `target/release/feather_calendar.exe`.

### Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/)
- **GUI Framework**: [egui](https://github.com/emilk/egui) (Windowing by [eframe](https://github.com/emilk/egui/tree/master/crates/eframe))
- **Date/Time Handling**: [chrono](https://crates.io/crates/chrono)

## License

This project is licensed under the [MIT License](LICENSE).
