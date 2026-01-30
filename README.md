# Glimpse 👁️

> **Linux is transparent. Let’s make it readable.**

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform](https://img.shields.io/badge/platform-Linux-green)](https://www.linux.org/)
[![Status](https://img.shields.io/badge/status-Design%20Phase%20v0.1-orange)]()

## What is Glimpse?

Glimpse is a system observability utility that transforms fragmented Linux system activity into a human-readable, causal timeline.

It addresses the common frustration of "flying blind" on Linux. When a system breaks, answers are often hidden across disconnected logs like `journalctl`, `dpkg`, and cryptic config files. Glimpse does not create new logs; instead, it acts as a **Narrative Layer** over existing signals, turning raw data into a clear story.

Think of it as a flight recorder for your operating system that helps you understand exactly what changed, why a service failed, or what triggered a system resource spike.

## Why Glimpse Exists

Linux systems log nearly everything, but understanding that data requires correlating timestamped text files, decoding error messages, and guessing causality. This creates a "Black Box Effect" where updates feel risky and failures appear random.

Glimpse removes that fear by making system history visible and explainable. It is designed to be a System Flight Recorder and Causal Analysis Engine that suggests safe recovery steps without acting as an automatic pilot. It respects your privacy by running entirely locally.



## Core Architecture

The application follows a simple **Listen → Think → Display** model designed to remain lightweight on your resources.

The **Flight Recorder** (Backend) is written in Rust. It runs as a background daemon consuming approximately 5MB of RAM. It listens to system events via `dbus`, `inotify`, and `netlink` without polling, ensuring minimal CPU usage.

The **Canvas** (Frontend) is built with Python, GTK4, and Libadwaita. It serves as the viewer and intelligence layer, correlating timestamps to find causality—for example, linking a system crash to a USB device removal milliseconds prior.

## Key Features

**Unified System Timeline**
View a single chronological feed of meaningful events. Instead of raw log dumps, you see clear actions: a package installation, a configuration file modification, or a service failure. Cause-and-effect becomes obvious when viewed in sequence.

**Causal Analysis**
Stop guessing why your system is behaving strangely. Glimpse traces the ancestry of processes and events. If your CPU usage spikes, Glimpse can identify that a specific background update process is the root cause, rather than just showing you the symptom.

**Visual Config Diffs**
When a package update silently modifies a configuration file in `/etc/`, Glimpse captures a snapshot. You can view a side-by-side comparison of exactly what changed, allowing you to revert unintended edits easily.

**Human Error Translator**
Future versions will include an on-demand, local-only Small Language Model (SLM). This feature will translate cryptic error codes into plain English explanations and offer safe, verified recovery suggestions.

## Getting Started

*Note: Glimpse is currently in the Design Phase (v0.1). These instructions are for developers contributing to the project.*

**Prerequisites**
You will need Rust (latest stable), Python 3.10+, and GTK4 development headers installed on your system.

**Build Instructions**

1.  Clone the repository:
    `git clone https://github.com/yourusername/glimpse.git`

2.  Build the Rust backend (Daemon):
    Navigate to the `backend` directory and run `cargo build --release`. You can test the collector manually by running `cargo run`.

3.  Run the Python frontend (UI):
    Navigate to the `frontend` directory, install dependencies with `pip install -r requirements.txt`, and launch the application using `python3 main.py`.

## Roadmap

The project is evolving in four distinct phases. **Phase 1 (Skeleton)** focuses on setting up the Rust daemon, SQLite schema, and a basic GTK4 timeline. **Phase 2 (Narration)** will introduce system service and hardware monitoring. **Phase 3 (Causality)** adds process tree analysis, and **Phase 4 (Intelligence)** will integrate the local LLM for error translation.

## Contributing

We welcome contributions to help make Linux more accessible. Please read our `CONTRIBUTING.md` file for details on our code of conduct and the process for submitting pull requests.

## License

Glimpse is free software distributed under the **GNU GPLv3 License**. See the `LICENSE` file for more information.

> **Final Philosophy:** Glimpse doesn’t fix your computer. It gives you the understanding to fix it yourself—without fear.
