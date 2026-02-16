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
## Expressions

Expressions are the building blocks used to represent values, perform calculations, and evaluate conditions in your test scripts.

## Literal Expressions

#### Number
Represents numeric values (integers or decimals).

```title="Syntax"
Number
```

```title="Example"
#testcase

count = 10
price = 99.99
negative = -5
```

#### String
Represents text values enclosed in double quotes.

```title="Syntax"
"String"
```

```title="Example"
#testcase

name = "John Doe"
url = "https://google.com"
message = "Hello, World!"
```

#### Boolean
Represents true or false values.

```title="Syntax"
true | false
```

```title="Example"
#testcase

isEnabled = true
isHidden = false
```

#### Identifier (Variable)
References a previously declared variable by its name.

```title="Syntax"
Ident
```

```title="Example"
#testcase

baseUrl = "https://example.com"
navigate baseUrl

title = get title
assert title == "Example"
```

## Unary Expressions

#### Negation
Negates a boolean expression using the `!` operator.

```title="Syntax"
!Expression
```

```title="Example"
#testcase

isVisible = is element "//div[@id='modal']" displayed
assert !isVisible
```

#### Grouped Expression
Groups expressions using parentheses to control evaluation order.

```title="Syntax"
(Expression)
```

```title="Example"
#testcase

result = (5 + 3) * 2
assert result == 16

isValid = (count > 0) and (count < 100)
```

## Binary Expressions

### Comparison Expressions

#### Equality
Checks if two expressions are equal.

```title="Syntax"
Expression == Expression
```

```title="Example"
#testcase

title = get title
assert title == "Google"
```

#### Not Equal
Checks if two expressions are not equal.

```title="Syntax"
Expression != Expression
```

```title="Example"
#testcase

status = get text from element "//span[@id='status']"
assert status != "Error"
```

#### Greater Than
Checks if the left expression is greater than the right.

```title="Syntax"
Expression > Expression
```

```title="Example"
#testcase

count = 10
assert count > 5
```

#### Less Than
Checks if the left expression is less than the right.

```title="Syntax"
Expression < Expression
```

```title="Example"
#testcase

price = 50
assert price < 100
```

#### Greater Than or Equal
Checks if the left expression is greater than or equal to the right.

```title="Syntax"
Expression >= Expression
```

```title="Example"
#testcase

age = 18
assert age >= 18
```

#### Less Than or Equal
Checks if the left expression is less than or equal to the right.

```title="Syntax"
Expression <= Expression
```

```title="Example"
#testcase

quantity = 5
assert quantity <= 10
```

### Arithmetic Expressions

#### Addition
Adds two expressions together.

```title="Syntax"
Expression + Expression
```

```title="Example"
#testcase

total = 10 + 5
assert total == 15

greeting = "Hello, " + "World!"
```

#### Subtraction
Subtracts the right expression from the left.

```title="Syntax"
Expression - Expression
```

```title="Example"
#testcase

difference = 20 - 8
assert difference == 12
```

#### Multiplication
Multiplies two expressions.

```title="Syntax"
Expression * Expression
```

```title="Example"
#testcase

product = 6 * 7
assert product == 42
```

#### Division
Divides the left expression by the right.

```title="Syntax"
Expression / Expression
```

```title="Example"
#testcase

quotient = 20 / 4
assert quotient == 5
```

#### Modulus
Returns the remainder of dividing the left expression by the right.

```title="Syntax"
Expression % Expression
```

```title="Example"
#testcase

remainder = 17 % 5
assert remainder == 2
```

### Logical Expressions

#### And
Returns true if both expressions are true.

```title="Syntax"
Expression and Expression
```

```title="Example"
#testcase

isEnabled = is element "//button[@id='submit']" enabled
isDisplayed = is element "//button[@id='submit']" displayed
assert isEnabled and isDisplayed
```

#### Or
Returns true if at least one expression is true.

```title="Syntax"
Expression or Expression
```

```title="Example"
#testcase

hasError = is element "//div[@class='error']" displayed
hasWarning = is element "//div[@class='warning']" displayed
showMessage = hasError or hasWarning
```

## Array Expressions

#### Empty Array
Creates an empty array.

```title="Syntax"
[]
```

```title="Example"
#testcase

emptyList = []
```

#### Array with Elements
Creates an array with one or more elements separated by commas.

```title="Syntax"
[Expression, Expression, ...]
```

```title="Example"
#testcase

numbers = [1, 2, 3, 4, 5]
names = ["Alice", "Bob", "Charlie"]
mixed = [1, "two", true]
```

## Getter Expressions

Getters can be used as expressions to retrieve values from the browser or elements.

#### Get Title
Returns the title of the current page.

```title="Syntax"
get title
```

```title="Example"
#testcase

navigate "https://google.com"
pageTitle = get title
assert pageTitle == "Google"
```

#### Get Current URL
Returns the URL of the current page.

```title="Syntax"
get current url
```

```title="Example"
#testcase

navigate "https://google.com"
currentUrl = get current url
```

#### Get Attribute
Returns the value of a specified attribute from an element.

```title="Syntax"
get attribute Expression from element Expression
```

```title="Example"
#testcase

href = get attribute "href" from element "//a[@id='link']"
className = get attribute "class" from element "//div[@id='container']"
```

#### Is Displayed
Checks whether an element is displayed on the page.

```title="Syntax"
is element Expression displayed
```

```title="Example"
#testcase

isVisible = is element "//div[@id='modal']" displayed
assert isVisible == true
```

#### Is Enabled
Checks whether an element is enabled.

```title="Syntax"
is element Expression enabled
```

```title="Example"
#testcase

canClick = is element "//button[@id='submit']" enabled
```

#### Is Selected
Checks whether an element (such as a checkbox or radio button) is selected.

```title="Syntax"
is element Expression selected
```

```title="Example"
#testcase

isChecked = is element "//input[@type='checkbox']" selected
```

#### Get Text
Returns the visible text content of an element.

```title="Syntax"
get text from element Expression
```

```title="Example"
#testcase

buttonLabel = get text from element "//button[@id='submit']"
assert buttonLabel == "Submit"
```

#### Get CSS Value
Returns the value of a specified CSS property from an element.

```title="Syntax"
get css value Expression from element Expression
```

```title="Example"
#testcase

bgColor = get css value "background-color" from element "//div[@id='header']"
fontSize = get css value "font-size" from element "//p[@class='text']"
```

#### Get Tag Name
Returns the tag name of an element.

```title="Syntax"
get tag name from element Expression
```

```title="Example"
#testcase

tag = get tag name from element "//*[@id='container']"
assert tag == "div"
```

## Teststeps

## Web Driver

#### Navigate
Load a new web page in the current browser window.

```title="Syntax"
navigate Expression
```

```title="Example"
#testcase

navigate "https://google.com"
```

#### Close
Close the current window, quitting the browser if it's the last window currently open.

```title="Example"
#testcase

navigate "https://google.com"
close
```

## Navigation

#### Back
Move back a single "item" in the browser's history.

```title="Example" linenums="1"
#testcase

navigate "https://google.com"
back
```

#### Forward
Move a single "item" forward in the browser's history.

```title="Example" linenums="1"
#testcase

navigate "https://google.com"
forward
```

#### Refresh
Refresh the current page

```title="Example"
#testcase

navigate "https://google.com"
refresh
```

## Element Actions

#### Click
Clicks the element identified by the given expression.

```title="Syntax"
click Expression
```

```title="Example"
#testcase

navigate "https://google.com"
click "//button[@id='submit']"
```

#### Sendkeys
Simulates typing into an element, which may set its value.

```title="Syntax"
enter Expression in element Expression
```

```title="Example"
#testcase

navigate "https://google.com"
enter "hello world" in element "//input[@name='search']"
```

## Custom Actions

#### Variable Declaration
Assigns a value to a variable for later use.

```title="Syntax"
Ident = Expression
```

```title="Example"
#testcase

myVar = "hello"
count = 10
url = "https://google.com"
navigate url
```

#### Assert
Validates that an expression evaluates to true.

Here the expression must return a boolean value otherwise you'll get an error

```title="Syntax"
assert Expression
```

```title="Example"
#testcase

navigate "https://google.com"
title = get title
assert title == "Google"
```

## Alert Actions

#### Accept Alert
Accepts the currently displayed alert dialog.

```title="Syntax"
accept alert
```

```title="Example"
#testcase

navigate "https://example.com"
click "//button[@id='show-alert']"
accept alert
```

#### Dismiss Alert
Dismisses the currently displayed alert dialog.

```title="Syntax"
dismiss alert
```

```title="Example"
#testcase

navigate "https://example.com"
click "//button[@id='show-alert']"
dismiss alert
```

#### Send Keys to Alert
Sends text input to an alert prompt dialog.

```title="Syntax"
enter Expression in alert
```

```title="Example"
#testcase

navigate "https://example.com"
click "//button[@id='show-prompt']"
enter "my input" in alert
accept alert
```

## Getters

#### Get Title
Returns the title of the current page.

```title="Syntax"
get title
```

```title="Example"
#testcase

navigate "https://google.com"
pageTitle = get title
assert pageTitle == "Google"
```

#### Get Current URL
Returns the URL of the current page.

```title="Syntax"
get current url
```

```title="Example"
#testcase

navigate "https://google.com"
currentUrl = get current url
assert currentUrl == "https://www.google.com/"
```

#### Get Attribute
Returns the value of a specified attribute from an element.

```title="Syntax"
get attribute Expression from element Expression
```

```title="Example"
#testcase

navigate "https://example.com"
hrefValue = get attribute "href" from element "//a[@id='link']"
```

#### Is Displayed
Checks whether an element is displayed on the page.

```title="Syntax"
is element Expression displayed
```

```title="Example"
#testcase

navigate "https://example.com"
isVisible = is element "//div[@id='content']" displayed
assert isVisible == true
```

#### Is Enabled
Checks whether an element is enabled.

```title="Syntax"
is element Expression enabled
```

```title="Example"
#testcase

navigate "https://example.com"
isEnabled = is element "//button[@id='submit']" enabled
assert isEnabled == true
```

#### Is Selected
Checks whether an element (such as a checkbox or radio button) is selected.

```title="Syntax"
is element Expression selected
```

```title="Example"
#testcase

navigate "https://example.com"
isChecked = is element "//input[@type='checkbox']" selected
assert isChecked == false
```

#### Get Text
Returns the visible text content of an element.

```title="Syntax"
get text from element Expression
```

```title="Example"
#testcase

navigate "https://example.com"
buttonText = get text from element "//button[@id='submit']"
assert buttonText == "Submit"
```

#### Get CSS Value
Returns the value of a specified CSS property from an element.

```title="Syntax"
get css value Expression from element Expression
```

```title="Example"
#testcase

navigate "https://example.com"
bgColor = get css value "background-color" from element "//div[@id='header']"
```

#### Get Tag Name
Returns the tag name of an element.

```title="Syntax"
get tag name from element Expression
```

```title="Example"
#testcase

navigate "https://example.com"
tagName = get tag name from element "//div[@id='content']"
assert tagName == "div"
```
