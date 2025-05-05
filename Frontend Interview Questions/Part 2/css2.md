### Senior Frontend Interview Questions   

---

#### **Critical CSS**
- **Definition**:  
  Critical CSS refers to the CSS styles required to display the elements visible above the fold when a page first loads.  
  - "Above the fold" refers to the portion of a webpage visible without scrolling.  

- **How to Extract Critical CSS**:  
  - **Use Tools or Plugins**:  
    - Example: Use `Webpack Plugin` or similar tools.  
    - These tools render the application in the background using a headless browser.  
  - **Mechanism**:  
    - Render the app for specific screen dimensions.
    - Utilize browser mechanisms like **CSS coverage** to extract only the necessary styles.
  - **Application in Performance**:  
    - Defer all non-critical CSS.  
    - Prevent the browser from parsing and rendering unnecessary CSS during initial load.  

---

#### **Accessibility and ARIA Attributes**
- **What Are ARIA Attributes?**  
  Accessible Rich Internet Applications (**ARIA**) attributes help define semantic roles for otherwise non-semantic HTML elements (e.g., `div`, `span`).  

- **When to Use ARIA Attributes?**  
  - When **semantic HTML** cannot be used due to design constraints or custom widget needs.  
  - Situations where accessibility constraints arise.

- **Semantic HTML Tags**:  
  Tags like `<footer>`, `<header>`, or `<img>` are inherently **semantic** and are directly understood by accessibility tools.  

---

#### **Defer vs. Async Attributes**
- **Purpose**:  
  These attributes dictate how and when JavaScript scripts are loaded and executed.  

- **`defer`**:  
  - Downloads the script in parallel while parsing HTML.  
  - Executes the script **after the HTML is completely parsed** (at the `DOMContentLoaded` event).  

- **`async`**:  
  - Downloads and executes the script **as soon as it's ready**, irrespective of whether the HTML is fully parsed.  

- **ES6 Modules**:  
  - Scripts in ES6 modules are **deferred by default** in most browsers.  

- **Why Use These Attributes?**  
  - They prevent scripts from **blocking the critical rendering path**, improving initial page load performance.  

---

#### **Static vs. Dynamic Imports**
- **Static Imports (Default Imports)**:  
  - Syntax: `import module from 'module';`.  
  - **Characteristics**:  
    - Processed at **build time**.  
    - Allow for tree-shaking and TypeScript type inference.  

- **Dynamic Imports**:  
  - Syntax: `import('module').then(...);`.  
  - **Characteristics**:  
    - Fetches the module **at runtime** (more like a function).  
    - Useful for **lazy-loading** modules or components.  
    - Enables conditional imports (e.g., importing based on user actions or events).  

- **Example of Use**:  
  - Avoid loading a library (e.g., `Lodash`) during the initial load. Instead, dynamically load it when a user action requires it.  

---

#### **Cumulative Layout Shift (CLS)**
- **What Is CLS?**  
  - One of the **Core Web Vitals**.  
  - Measures the layout shift that occurs during page load.  
  - A bad CLS score results when elements (e.g., images, fonts) load late and cause the visible layout to shift unexpectedly.  

- **Fixing a Poor CLS Score**:  
  1. **Debug**:  
     - Use tools like Google Chrome's performance insights to identify problematic elements.  
  2. **Images**:  
     - Set explicit `width` and `height` attributes on images.  
  3. **Fonts**:  
     - Optimize font loading (e.g., by specifying fallback heights).  
  4. **CSS**:  
     - Implement **critical CSS** and defer non-critical styles.  

---

#### **Essential vs. Derived State**
- **Essential State**:  
  - Independent state that changes based on user interaction or external data (e.g., API calls).  

- **Derived State**:  
  - State that is computed or deduced from other states.  

- **Example**:  
  - On a checkout page:  
    - **Essential State**: Items in the cart.  
    - **Derived State**: Totals, taxes, item count (calculated from the cart items).  
  - **Why Important?**  
    - Derived state avoids redundancy and ensures state consistency.  

--- 
