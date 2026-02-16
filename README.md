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

## Data Types

Like any robust language, this language relies on a set of atomic data types to handle state,
logic, and web interactions. this language provides 5 fundamental data types.

- Number
- String
- Boolean
- Array
- WebElement

### Number

``` py title="Example"
count = 42          // Unsigned
temperature = -5    // Signed
price = 19.99       // Float
```
### String

**Syntax**: Strings must be enclosed in double quotes `"`.

**Constraint**: this language strings are single-line only.

**Escaping**: To include a newline within a string, use the standard `\n` escape character. To include a double quote, use `\"`.

```py title="Example"
greeting = "Hello, World!"
error_msg = "Invalid input.\nPlease try again." // Valid single-line literal with escape
```

### Boolean
The Boolean type represents logical entities and binary states.

**Values**: Can only be `true` or `false`.

**Usage**: primarily used in conditional testing `(if/else)` and loop termination logic.


```py title="Example"
isVisible = true
hasLoaded = false
```

### Arrays
Arrays are ordered collections of elements used to group data .
In this language, Arrays are designed primarily for iteration or sequential processing rather than random access.

**Constraint** :

1. No Index Access. You cannot access specific elements using an index (e.g., `arr[0]` is invalid in this language).
2. Arrays in this language are immutable, you can't add or delete from defined collection(future versions may resolve this constraint).

**Usage**: Arrays are typically used as inputs for loops (iterators) or manipulated as a whole set.

Examples:

```py title="Example"
// Declaration
numbers = [1, 2, 3, 4]
userList = ["Alice", "Bob", "Charlie"]

for num in numbers {
  click "Data "+num
}

// INVALID in this language:
// var user = userList[1] -> This will throw an error
```

### WebElement
The WebElement is a first-class citizen in this language,
designed specifically for web automation and interaction.
It represents a specific node in the DOM (Document Object Model) of a web page.

**Purpose**: To store references to buttons, input fields, divs, or any interactable HTML tag.

```py title="Example"
login = find element "Login"

//use login variable to perform element actions like click

click login
```

