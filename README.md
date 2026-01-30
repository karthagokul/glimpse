# Glimpse
### System Flight Recorder & Local Intelligence Co-Pilot for Linux

**Tagline:** *Linux is transparent. Let’s make it readable.*  
**Status:** Design Phase (v0.1)  
**License:** Apache-2.0 (recommended)  
**Platforms:** Linux (Desktop & Server)

---

## What is Glimpse?

**Glimpse** is a system observability utility that turns fragmented Linux system activity into a **human-readable, causal timeline**.

It answers the questions Linux users ask every day:

- *What changed on my system yesterday?*
- *Why did this service suddenly fail after an update?*
- *Why is my laptop fan spinning or battery draining?*
- *What caused this error — and what should I do next?*

Glimpse does **not** introduce new logs.  
It creates a **Narrative Layer** over existing system signals.

Think of it as:

> **Git for your OS history — with explanations.**

---

## Why Glimpse Exists

Linux systems log everything, yet understanding *what happened* still requires:
- Correlating `journalctl`, `/var/log/*`, package histories, and config files
- Interpreting cryptic error messages
- Guessing causality across unrelated tools

This creates a **Black Box Effect**:
- Updates feel risky
- Failures feel random
- Recovery feels intimidating

**Glimpse removes that fear by making system history visible, readable, and explainable.**

---

## What Glimpse Is (and Is Not)

### Glimpse **is**:
- A **System Flight Recorder**
- A **Causal Analysis Engine**
- A **Recovery Assistant** (suggests, never acts)
- **Local-first** and **privacy-respecting**

### Glimpse **is not**:
- A backup or snapshot tool (e.g., Timeshift)
- A system cleaner or optimizer
- A remote monitoring agent
- An AI autopilot that modifies your system

> **Glimpse never changes system state automatically.**  
> It explains and suggests — you stay in control.

---

## Core Architecture

Glimpse follows a **Listen → Think → Display** model.

### 1. Flight Recorder (Collector Daemon)
A lightweight background service that records meaningful system events.

- **Language:** Rust or Go
- **Design:** Event-driven (no polling)
- **Mechanisms:** `dbus`, `inotify`, `netlink`, system hooks

**Event Sources**
- **Packages:** APT, Snap, Flatpak, PIP, Cargo
- **System:** Boot, shutdown, sleep, wake, reboots
- **Services:** systemd start / stop / crash
- **Hardware:** USB, display, network interfaces (udev)
- **Files:** Config changes in `/etc/` and selected `~/.config/`

---

### 2. Cortex (Analysis & Intelligence Layer)
Correlates events and explains *why* things happened.

- **Language:** Python (bridged)
- **Causality:** Timestamp correlation, process ancestry
- **Privacy:** 100% local

**Optional Intelligence (Later MVP)**
- On-demand **local Small Language Model (SLM)**
- Runs offline via `llama.cpp` / `Ollama`
- Quantized, CPU-only, user-initiated

---

### 3. Canvas (User Interface)
A calm, modern system history viewer.

- **Tech:** GTK4 + Libadwaita
- **Design:** Vertical timeline (Git-style)
- **Focus:** Signal over noise

---

## Feature Set

### Feature 1: Unified System Timeline
A single chronological view of meaningful system events.

Examples:
- **[Package]** Installed `nvidia-driver-535`
- **[Config]** `/etc/default/grub` modified (automatic)
- **[Service]** `gdm3` failed to start
- **[Hardware]** USB webcam disconnected
- **[Network]** Wi-Fi interface lost DHCP lease

**Value:** Cause-and-effect becomes obvious in seconds.

---

### Feature 2: Session Grouping (Boot-to-Shutdown)
Events are grouped into **sessions**, forming readable stories.

---

### Feature 3: Human-Readable Narration
Raw system events are translated into plain language while preserving technical detail.

---

### Feature 4: Config Change Tracking (Visual Diffs)
When key configuration files change, Glimpse records a snapshot.

---

### Feature 5: Causal Analysis — “Why is this happening?”
Explains *intent*, not just symptoms.

---

### Feature 6: Human Error Translator (Local-First AI)
Optional, on-demand explanation of cryptic errors.

---

### Feature 7: Recovery Assistant (Suggest, Don’t Act)
Generates safe next steps without automatic changes.

---

### Feature 8: Package Inspector (Post-MVP)
Pre-install audit for packages.

---

## Event Severity & Noise Control

All events are classified into relevance tiers:
- Critical
- Informational
- Background

---

## Confidence & Transparency

Every explanation includes a confidence label:
- Confirmed
- Likely
- Uncertain

---

## Security Model

- Privileged collector daemon
- Unprivileged UI
- Strict IPC boundary
- No telemetry

---

## Data Retention & Control

- Local SQLite storage
- Configurable retention
- Category-based pruning

---

## Non-Goals

Glimpse will **not**:
- Act as a backup tool
- Automatically fix systems
- Upload data to the cloud

---

## Roadmap

- Phase 1: Skeleton (MVP 0.1)
- Phase 2: Narration
- Phase 3: Causality
- Phase 4: Intelligence

---

## Final Philosophy

> **Glimpse doesn’t fix your computer.  
> It gives you the understanding to fix it yourself — without fear.**
