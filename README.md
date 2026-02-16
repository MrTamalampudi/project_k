<p align="center"> <img width=300 src="./media/logo.svg"/></p> 

# Project K: Web Automation DSL Compiler

**Project K** is a domain-specific language (DSL) and compiler designed to simplify web automation tasks. Built in **Rust**, it allows users to write concise, readable automation scripts that are compiled and executed to control web browsers, leveraging the power of WebDriver under the hood.

> **Note:** This project uses [manodae LALR parser](https://github.com/MrTamalampudi/manodae) for syntax parsing and [webdriver-manager](https://github.com/MrTamalampudi/webdriver-manager) for automated driver management.

## 🚀 Features

* **Custom DSL:** Write automation logic in a simplified, human-readable language instead of verbose code.
* **Fast & Safe:** Built with Rust for performance and memory safety.
* **Automated Driver Management:** Automatically handles browser driver setup (Chromedriver).
* **Compiler Architecture:** Uses a Logos lexer and [LALR parser](https://github.com/MrTamalampudi/manodae) to process scripts.
* **Cross-Platform:** Works on Windows, Linux, and macOS.

## 🛠️ Installation

Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/MrTamalampudi/project_k.git
    cd project_k
    ```

2.  **Run directly via Cargo:**
    ```bash
    cargo run example.ll
    ```

## 📖 Usage

### Getting started
Create a file (e.g., `example.ll`) with your automation logic.
```
#testcase

urll = "https://github"+".com/"
navigate urll
wait 3
if 1 == 2 {
    navigate "https://google.com"
} else if urll == "https://google.com/" {
    navigate "https://docs.rs"
    wait 3
    navigate "https://github.com"
} else if 2 == 2 {
    navigate "https://youtube.com"
} else {
    navigate "https://x.com"
}
wait 1
navigate "https://facebook.com"
wait 1*2
```
