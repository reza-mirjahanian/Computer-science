# **JavaScript Concepts **

---

## **Event Bubbling**
- **Definition**:  
  - When an event is triggered on an element, the browser **first runs handlers on the target element**, then moves up to its ancestors (e.g., from a button to a form, then to the root HTML element).
- **Key Points**:
  - Default behavior in JavaScript.
  - Traverses **from the target element up to the root**.

---

## **Event Capturing**
- **Definition**:  
  - The **reverse of bubbling**: events are handled starting from the **root element down to the target**.
- **Key Points**:
  - Involves **three phases**: capturing, target, and bubbling.
  - Rarely used; requires explicit enabling via an `{ capture: true }` option.
  - Historically introduced by **Microsoft**, while **Netscape favored bubbling**.

---

## **Debouncing**
- **Purpose**:  
  - Limits the frequency of function calls (e.g., preventing excessive API requests during user typing).
- **Use Case**:  
  - Search bars delaying backend requests until the user **stops typing for a set interval** (e.g., 500ms).
- **Mechanism**:  
  - Resets a timer on each input; executes the function only after the timer completes.

---

## **Event Loop & `setTimeout` Execution**
- **Example Code Analysis**:  
  ```javascript
  console.log('First'); // Synchronous → logs first
  setTimeout(() => console.log('Second'), 0); // Asynchronous → queued for next event loop tick
  console.log('Third'); // Synchronous → logs second
  ```
- **Execution Order**:  
  - **First** → **Third** → **Second** (due to `setTimeout` deferral).

---

## **Prototypal Inheritance**
- **Definition**:  
  - Objects inherit properties/methods via a **prototype chain** (checked if absent on the object itself).
- **Class vs. Prototype**:  
  - **Class inheritance**: Uses `extends` for hierarchical relationships.
  - **Prototypal inheritance**: Links objects via `prototype` property.
- **Best Practice**:  
  - Avoid modifying **built-in prototypes** (e.g., `String.prototype`) to prevent compatibility issues.

---

## **Async Functions & Promises**
- **Async Functions**:  
  - Return a **Promise**, resolving with the function’s return value.
  - Syntactic sugar over Promises (simplifies `then`/`catch` chains).
- **Example**:  
  ```javascript
  async function fetchData() { return await getData(); } // Wraps result in a Promise
  ```

---

## **Pure Functions**
- **Definition**:  
  - Functions with **no side effects**; output depends **solely on input**.
- **Benefits**:  
  - **Deterministic** (predictable output).
  - Easy to test and debug.
- **Example**:  
  ```javascript
  const add = (a, b) => a + b; // No external state modification.
  ```

---

## **Polyfilling**
- **Purpose**:  
  - Adds missing features to older browsers (e.g., `Array.prototype.includes` for IE11).
- **Drawbacks**:  
  - **Larger code bundles** → impacts performance.
  - Requires **browser-specific targeting** to optimize.

---

## **Closures**
- **Definition**:  
  - Functions that **retain access to their lexical environment** (variables from parent scope).
- **Use Case**:  
  - Encapsulating variables (e.g., private state in modules).
- **Example**:  
  ```javascript
  function createGreeter(name) {
    return () => console.log(`Hello, ${name}!`); // Closure retains access to `name`
  }
  const greet = createGreeter('Alice');
  greet(); // Logs "Hello, Alice!"
  ```
- **Drawback**:  
  - Can cause **memory leaks** if closures retain large/unused data.