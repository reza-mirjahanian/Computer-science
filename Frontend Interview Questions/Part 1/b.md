# Senior Frontend Engineer Interview Questions & Answers

## Fundamentals

1.  **Question:** Explain the difference between `null` and `undefined` in JavaScript.
    * **Answer:**
        * `undefined` typically means a variable has been declared but has not yet been assigned a value. It's also the default return value of functions that don't explicitly return anything and the value of function parameters that were not provided.
        * `null` is an assignment value. It can be assigned to a variable as a representation of "no value" or an intentional absence of any object value.
        * *Key Difference:* `undefined` is usually the system's way of saying "value not assigned," while `null` is the programmer's way of saying "intentionally no value." In comparisons, `null == undefined` is `true`, but `null === undefined` is `false`.

2.  **Question:** What is the concept of "closure" in JavaScript? Provide a simple example.
    * **Answer:** A closure is the combination of a function bundled together (enclosed) with references to its surrounding state (the lexical environment). In other words, a closure gives you access to an outer function's scope from an inner function, even after the outer function has finished executing.
    * *Example:*
        ```javascript
        function outerFunction() {
          let outerVariable = 'I am outside!';

          function innerFunction() {
            console.log(outerVariable); // Accesses outerVariable
          }

          return innerFunction;
        }

        const myClosure = outerFunction();
        myClosure(); // Logs: "I am outside!"
        ```
        Here, `innerFunction` "closes over" `outerVariable` from `outerFunction`'s scope. Even though `outerFunction` has returned, `myClosure` (which is `innerFunction`) still remembers and can access `outerVariable`.

3.  **Question:** What is the CSS Box Model? Explain its components.
    * **Answer:** The CSS Box Model is a fundamental concept describing how elements are rendered on a webpage. Each HTML element is treated as a rectangular box. The model consists of four concentric layers:
        1.  **Content:** The actual content of the element (text, images, etc.), defined by its `width` and `height`.
        2.  **Padding:** Transparent space around the content, inside the border. Controlled by `padding` properties (`padding-top`, `padding-right`, etc.).
        3.  **Border:** A line surrounding the padding and content. Controlled by `border` properties (`border-width`, `border-style`, `border-color`).
        4.  **Margin:** Transparent space outside the border, separating the element from other elements. Controlled by `margin` properties (`margin-top`, `margin-right`, etc.).
    * The `box-sizing` property determines how the `width` and `height` are calculated:
        * `content-box` (default): `width` and `height` apply only to the content area. Padding and border are added *outside* this dimension.
        * `border-box`: `width` and `height` include content, padding, *and* border. This often makes layout calculations more intuitive.

4.  **Question:** Explain the difference between `==` and `===` in JavaScript.
    * **Answer:**
        * `==` (Loose Equality): Compares two values for equality *after* performing type coercion if the types are different. For example, `5 == '5'` evaluates to `true` because the string `'5'` is coerced to the number `5` before comparison.
        * `===` (Strict Equality): Compares two values for equality *without* performing type coercion. If the types are different, it immediately returns `false`. For example, `5 === '5'` evaluates to `false` because they are of different types (number and string).
        * *Best Practice:* It's generally recommended to use `===` to avoid unexpected behavior due to implicit type coercion.

5.  **Question:** What are semantic HTML elements? Why are they important?
    * **Answer:** Semantic HTML elements are HTML tags that clearly describe their meaning and purpose to both the browser and the developer. They define the *structure* and *meaning* of the content, rather than just its presentation.
    * *Examples:* `<header>`, `<footer>`, `<nav>`, `<article>`, `<section>`, `<aside>`, `<main>`. These contrast with non-semantic elements like `<div>` and `<span>` which are purely for styling or grouping.
    * *Importance:*
        1.  **Accessibility (a11y):** Screen readers and assistive technologies use semantic elements to understand the page structure and navigate content effectively for users with disabilities.
        2.  **SEO:** Search engines use semantic markup to better understand the content and context of a webpage, potentially improving search rankings.
        3.  **Maintainability:** Code becomes more readable and easier to understand for developers, improving collaboration and maintenance.
        4.  **Clarity:** Clearly defines different parts of a web page.

6.  **Question:** What is the `this` keyword in JavaScript and how does its value get determined?
    * **Answer:** The `this` keyword is a special identifier in JavaScript that refers to the *context* in which a function is executed. Its value is determined dynamically at runtime based on *how* the function is called, not where it's defined (lexically). The main rules are:
        1.  **Global Context:** When used outside any function (or in a non-strict mode function called without a specific context), `this` refers to the global object (`window` in browsers, `global` in Node.js). In strict mode (`'use strict'`), `this` is `undefined` in this scenario within functions.
        2.  **Object Method:** When a function is called as a method of an object (`object.method()`), `this` refers to the object the method was called on (`object`).
        3.  **Constructor:** When a function is used as a constructor with the `new` keyword (`new Function()`), `this` refers to the newly created instance.
        4.  **Explicit Binding (`call`, `apply`, `bind`):** These methods allow you to explicitly set the value of `this` for a function call.
            * `func.call(thisArg, arg1, arg2, ...)`: Calls `func` with `this` set to `thisArg` and specified arguments.
            * `func.apply(thisArg, [argsArray])`: Similar to `call`, but arguments are passed as an array.
            * `func.bind(thisArg)`: Returns a *new* function where `this` is permanently bound to `thisArg`.
        5.  **Arrow Functions:** Arrow functions (`=>`) do *not* have their own `this` binding. They inherit `this` lexically from their surrounding (enclosing) scope at the time they are defined.

7.  **Question:** Explain CSS Specificity. How is it calculated?
    * **Answer:** CSS Specificity is the algorithm browsers use to determine which CSS rule applies if multiple rules target the same element and property. More specific rules override less specific ones.
    * *Calculation:* Specificity is typically calculated using a weighting system, often represented as four levels (though conceptually it's more complex):
        1.  **Inline Styles:** Styles applied directly to an element using the `style` attribute (e.g., `<div style="color: red;">`). Has the highest specificity (1,0,0,0).
        2.  **IDs:** Selectors using an ID (e.g., `#myId`). (0,1,0,0).
        3.  **Classes, Attributes, Pseudo-classes:** Selectors using classes (`.myClass`), attribute selectors (`[type="text"]`), or pseudo-classes (`:hover`, `:focus`). (0,0,1,0).
        4.  **Elements and Pseudo-elements:** Selectors using element types (`div`, `p`) or pseudo-elements (`::before`, `::after`). (0,0,0,1).
    * *Rules:*
        * Specificity values are compared level by level, from left to right (Inline > ID > Class/Attr/Pseudo-class > Element/Pseudo-element).
        * The universal selector (`*`) and combinators (`+`, `>`, `~`, ` `) have no specificity value (0,0,0,0).
        * `:not()` pseudo-class itself adds no specificity, but its argument does.
        * `!important` rule: Appending `!important` to a CSS declaration overrides *any* other declaration, regardless of specificity. However, using `!important` is generally discouraged as it breaks the natural cascade and makes debugging harder. Inline styles still have higher precedence than `!important` rules defined in user agent stylesheets or user stylesheets.

## Practical / Coding

8.  **Question:** How would you debounce a function call in JavaScript? Write a simple debounce function.
    * **Answer:** Debouncing is a technique to ensure that a function is not called too frequently. It delays the execution of a function until a certain amount of time has passed without it being called again. This is useful for events like window resizing, scrolling, or handling input field changes where you only want to react after the user has stopped performing the action.
    * *Implementation:*
        ```javascript
        function debounce(func, delay) {
          let timeoutId;

          // Return a new function that wraps the original function
          return function(...args) {
            // Clear the previous timeout if it exists
            clearTimeout(timeoutId);

            // Set a new timeout to execute the function after the delay
            timeoutId = setTimeout(() => {
              // Call the original function with the correct 'this' context and arguments
              func.apply(this, args);
            }, delay);
          };
        }

        // Example Usage:
        function handleInput(event) {
          console.log('Fetching suggestions for:', event.target.value);
          // Imagine an API call here
        }

        const debouncedHandleInput = debounce(handleInput, 300); // Wait 300ms after last keystroke

        const inputElement = document.getElementById('myInput');
        inputElement.addEventListener('input', debouncedHandleInput);
        ```
    * *Explanation:* The `debounce` function returns a new function. Each time this new function is called, it clears any existing timeout and sets a new one. The original `func` is only executed if the `delay` period passes without any further calls to the debounced function. `apply(this, args)` ensures the original function receives the correct context and arguments.

9.  **Question:** Implement a function that deeply flattens an array. Example: `flatten([1, [2, [3, 4], 5], 6])` should return `[1, 2, 3, 4, 5, 6]`.
    * **Answer:**
        ```javascript
        function deepFlatten(arr) {
          let flattened = [];

          arr.forEach(item => {
            if (Array.isArray(item)) {
              // If the item is an array, recursively flatten it and concatenate
              flattened = flattened.concat(deepFlatten(item));
            } else {
              // If the item is not an array, push it directly
              flattened.push(item);
            }
          });

          return flattened;
        }

        // Alternative using flat() with Infinity (modern browsers)
        function deepFlattenModern(arr) {
            return arr.flat(Infinity);
        }

        // Example Usage:
        const nestedArray = [1, [2, [3, 4], 5], 6];
        console.log(deepFlatten(nestedArray));      // Output: [1, 2, 3, 4, 5, 6]
        console.log(deepFlattenModern(nestedArray)); // Output: [1, 2, 3, 4, 5, 6]
        ```
    * *Explanation:* The recursive approach iterates through the array. If an element is itself an array, it calls `deepFlatten` on that sub-array and concatenates the result. Otherwise, it pushes the element to the `flattened` array. The modern `arr.flat(Infinity)` method provides a built-in, concise way to achieve the same result.

10. **Question:** Write CSS using Flexbox to create a layout with a fixed-width sidebar on the left and a main content area that takes up the remaining space.
    * **Answer:**
        *HTML:*
        ```html
        <div class="container">
          <aside class="sidebar">Sidebar Content</aside>
          <main class="main-content">Main Content Area</main>
        </div>
        ```
        *CSS:*
        ```css
        .container {
          display: flex;
          min-height: 100vh; /* Example height */
        }

        .sidebar {
          width: 250px; /* Fixed width */
          flex-shrink: 0; /* Prevent sidebar from shrinking */
          background-color: lightgray; /* Example styling */
          padding: 1rem;
        }

        .main-content {
          flex-grow: 1; /* Allow main content to grow and take remaining space */
          background-color: white; /* Example styling */
          padding: 1rem;
        }
        ```
    * *Explanation:*
        * `display: flex;` on the container establishes a flex formatting context.
        * `width: 250px;` sets the fixed width for the sidebar.
        * `flex-shrink: 0;` ensures the sidebar doesn't shrink if the container becomes too narrow.
        * `flex-grow: 1;` allows the main content area to expand and fill any available space along the main axis.

11. **Question:** Explain how Promises work in JavaScript. Rewrite a callback-based function to use Promises.
    * **Answer:** A `Promise` is an object representing the eventual completion (or failure) of an asynchronous operation and its resulting value. It provides a cleaner way to handle async operations compared to traditional callbacks, avoiding "callback hell." A Promise can be in one of three states:
        1.  **Pending:** Initial state, neither fulfilled nor rejected.
        2.  **Fulfilled:** The operation completed successfully, and the promise has a resulting value.
        3.  **Rejected:** The operation failed, and the promise has a reason for the failure (an error).
    * Promises use `.then()` for handling fulfillment, `.catch()` for handling rejection, and `.finally()` for code that should run regardless of success or failure.
    * *Callback Example:*
        ```javascript
        function fetchDataCallback(url, successCallback, errorCallback) {
          // Simulate network request
          setTimeout(() => {
            if (Math.random() > 0.2) { // Simulate success
              successCallback({ data: `Data from ${url}` });
            } else { // Simulate error
              errorCallback(new Error(`Failed to fetch ${url}`));
            }
          }, 1000);
        }
        ```
    * *Promise Rewrite:*
        ```javascript
        function fetchDataPromise(url) {
          return new Promise((resolve, reject) => {
            // Simulate network request
            setTimeout(() => {
              if (Math.random() > 0.2) { // Simulate success
                resolve({ data: `Data from ${url}` }); // Fulfill the promise
              } else { // Simulate error
                reject(new Error(`Failed to fetch ${url}`)); // Reject the promise
              }
            }, 1000);
          });
        }

        // Usage:
        fetchDataPromise('/api/data')
          .then(response => {
            console.log('Success:', response.data);
          })
          .catch(error => {
            console.error('Error:', error.message);
          })
          .finally(() => {
            console.log('Fetch attempt finished.');
          });
        ```

12. **Question:** Write a React component that fetches data from an API (`https://jsonplaceholder.typicode.com/posts/1`) on mount and displays the post title. Handle loading and error states.
    * **Answer (using Hooks):**
        ```jsx
        import React, { useState, useEffect } from 'react';

        function PostDisplay() {
          const [post, setPost] = useState(null);
          const [loading, setLoading] = useState(true);
          const [error, setError] = useState(null);

          useEffect(() => {
            // Define async function inside useEffect or use IIFE
            const fetchPost = async () => {
              setLoading(true);
              setError(null); // Reset error state on new fetch
              try {
                const response = await fetch('https://jsonplaceholder.typicode.com/posts/1');
                if (!response.ok) {
                  throw new Error(`HTTP error! status: ${response.status}`);
                }
                const data = await response.json();
                setPost(data);
              } catch (err) {
                setError(err.message);
              } finally {
                setLoading(false);
              }
            };

            fetchPost();

            // Cleanup function (optional, not strictly needed for one-time fetch)
            return () => {
              // Controller can be used here to abort fetch if component unmounts
            };
          }, []); // Empty dependency array ensures this runs only once on mount

          if (loading) {
            return <div>Loading post...</div>;
          }

          if (error) {
            return <div>Error fetching post: {error}</div>;
          }

          if (!post) {
             return <div>No post data found.</div>; // Should ideally not happen if loading/error are handled
          }

          return (
            <div>
              <h1>Post Title</h1>
              <p>{post.title}</p>
              {/* Optionally display other post data */}
              {/* <p>{post.body}</p> */}
            </div>
          );
        }

        export default PostDisplay;
        ```
    * *Explanation:*
        * Uses `useState` to manage `post` data, `loading` state, and `error` state.
        * Uses `useEffect` with an empty dependency array (`[]`) to run the data fetching logic only once when the component mounts.
        * An `async` function `WorkspacePost` is defined inside `useEffect` to handle the asynchronous `Workspace` call using `async/await`.
        * Includes basic error handling (`try...catch`) and checks `response.ok`.
        * Uses a `finally` block to ensure `setLoading(false)` is called whether the fetch succeeds or fails.
        * Conditionally renders different UI based on the `loading` and `error` states.

13. **Question:** What is event delegation in JavaScript? Provide a use case and a code example.
    * **Answer:** Event delegation is a technique where instead of attaching an event listener to every individual child element, you attach a single listener to a common ancestor (parent) element. This listener then analyzes bubbled events to find a match on the target element.
    * *Use Cases:*
        * **Performance:** Reduces the number of event listeners, especially useful for large lists or tables where adding listeners to each item would be inefficient.
        * **Dynamic Content:** Automatically handles events for elements added to the DOM *after* the initial listener was attached, as the listener is on the parent which already exists.
    * *Code Example:* Imagine a list where new items can be added, and you want to log the text of any clicked list item.
        *HTML:*
        ```html
        <ul id="myList">
          <li>Item 1</li>
          <li>Item 2</li>
          <li>Item 3</li>
        </ul>
        <button id="addItemBtn">Add Item</button>
        ```
        *JavaScript:*
        ```javascript
        const list = document.getElementById('myList');
        const addItemBtn = document.getElementById('addItemBtn');
        let itemCount = 3;

        // Attach one listener to the parent <ul>
        list.addEventListener('click', function(event) {
          // Check if the clicked element is an <li>
          if (event.target && event.target.nodeName === 'LI') {
            console.log('Clicked item:', event.target.textContent);
            // Perform action on the specific li clicked (event.target)
            event.target.style.color = 'red'; // Example action
          }
        });

        // Add new items dynamically
        addItemBtn.addEventListener('click', function() {
          itemCount++;
          const newItem = document.createElement('li');
          newItem.textContent = `Item ${itemCount}`;
          list.appendChild(newItem);
          // No need to add a new listener for the new item!
        });
        ```
    * *Explanation:* The click listener is on the `<ul>`. When an `<li>` inside it is clicked, the event bubbles up to the `<ul>`. The listener checks `event.target` (the actual element clicked) to see if it's an `LI`. If so, it performs the action. This works even for items added later by the button.

14. **Question:** Write a JavaScript function that uses `async/await` to fetch data from two different API endpoints concurrently and returns the combined result.
    * **Answer:**
        ```javascript
        async function fetchConcurrentData(url1, url2) {
          try {
            // Start both fetch requests concurrently
            const promise1 = fetch(url1).then(res => {
              if (!res.ok) throw new Error(`HTTP error! status: ${res.status} for ${url1}`);
              return res.json();
            });

            const promise2 = fetch(url2).then(res => {
               if (!res.ok) throw new Error(`HTTP error! status: ${res.status} for ${url2}`);
               return res.json();
            });

            // Wait for both promises to resolve using Promise.all
            const [result1, result2] = await Promise.all([promise1, promise2]);

            // Combine or process the results as needed
            return { data1: result1, data2: result2 };

          } catch (error) {
            console.error("Error fetching data concurrently:", error);
            // Handle or re-throw the error appropriately
            throw error; // Or return a specific error structure
          }
        }

        // Example Usage:
        const apiUrl1 = 'https://jsonplaceholder.typicode.com/posts/1';
        const apiUrl2 = 'https://jsonplaceholder.typicode.com/users/1';

        fetchConcurrentData(apiUrl1, apiUrl2)
          .then(combinedData => {
            console.log('Combined Data:', combinedData);
            // Access combinedData.data1 and combinedData.data2
          })
          .catch(error => {
            console.error('Failed to fetch concurrent data:', error);
          });
        ```
    * *Explanation:*
        * The `Workspace` calls for `url1` and `url2` are initiated immediately without `await`ing them individually. This starts the network requests concurrently.
        * `Promise.all()` takes an array of promises and returns a single promise that resolves when *all* promises in the array have resolved. The resolved value is an array containing the resolved values of the input promises, in the same order.
        * `await Promise.all(...)` pauses the `WorkspaceConcurrentData` function until both fetch operations are complete (either fulfilled or one rejects).
        * Error handling is included for both individual fetches (checking `res.ok`) and for `Promise.all` (which rejects if any input promise rejects).

15. **Question:** Implement a simple version of `Array.prototype.map` function.
    * **Answer:**
        ```javascript
        function simpleMap(arr, callbackFn) {
          // Basic error checking
          if (!Array.isArray(arr)) {
            throw new TypeError('arr must be an array');
          }
          if (typeof callbackFn !== 'function') {
            throw new TypeError('callbackFn must be a function');
          }

          const resultArray = [];
          for (let i = 0; i < arr.length; i++) {
            // Check if the index exists (for sparse arrays, though less common now)
            // if (i in arr) { // More correct handling for sparse arrays
               // Call the callback with value, index, and original array
               const mappedValue = callbackFn(arr[i], i, arr);
               resultArray.push(mappedValue);
            // } else {
            //    resultArray.push(undefined); // Or handle sparse arrays differently
            // }
          }
          return resultArray;
        }

        // Example Usage:
        const numbers = [1, 2, 3, 4];
        const doubled = simpleMap(numbers, (num) => num * 2);
        console.log(doubled); // Output: [2, 4, 6, 8]

        const words = ["hello", "world"];
        const lengths = simpleMap(words, (word, index) => `${index}: ${word.length}`);
        console.log(lengths); // Output: ["0: 5", "1: 5"]
        ```
    * *Explanation:* The `simpleMap` function iterates over the input array `arr`. For each element, it calls the provided `callbackFn`, passing the element's value, index, and the original array (matching the signature of the native `map`). The return value of the `callbackFn` is pushed into a new `resultArray`. Finally, the `resultArray` containing the transformed elements is returned. Basic type checking is included.

## System Design / Architecture

16. **Question:** You need to build a highly interactive data dashboard (e.g., stock ticker, real-time analytics). What architectural considerations and technologies would you evaluate?
    * **Answer:**
        * **Real-time Data Delivery:**
            * **WebSockets:** Best for true bi-directional, low-latency communication. The server can push updates instantly. Requires server-side support. Ideal for high-frequency updates like stock tickers.
            * **Server-Sent Events (SSE):** Simpler than WebSockets for server-to-client streaming. Uses standard HTTP. Good for one-way updates (e.g., news feeds, notifications).
            * **Long Polling/Polling:** Less efficient fallback. Long polling keeps a connection open until data is available; regular polling requests data at intervals. Higher latency and overhead.
        * **Frontend Framework/Library:**
            * **React/Vue/Angular/Svelte:** Choose based on team familiarity, ecosystem, performance needs. All are capable. Consider their change detection strategies and rendering performance. Virtual DOM (React) or fine-grained reactivity (Svelte, Vue) can handle frequent updates efficiently.
        * **State Management:**
            * Frequent updates require efficient state management.
            * **Redux/Zustand/Pinia/NgRx:** Centralized stores are good for managing complex, shared state but can have boilerplate. Consider middleware for handling WebSocket connections.
            * **Component-level state / Context API:** Suitable for localized state, but can cause performance issues with very high-frequency updates if not optimized (e.g., using `React.memo`, `useMemo`, `useCallback`).
            * **Specialized libraries (e.g., Jotai, Valtio):** Atomic state management can be very performant for granular updates.
        * **Data Handling & Rendering:**
            * **Efficient Rendering:** Avoid re-rendering the entire dashboard on every update. Update only the components affected by the new data. Use keys correctly in lists. Techniques like memoization (`React.memo`, `useMemo`) are crucial.
            * **Data Throttling/Debouncing:** Throttle UI updates if the data comes in *too* fast (e.g., update the chart every 100ms even if data arrives every 10ms) to prevent overwhelming the browser. Debounce user interactions that trigger data refetches.
            * **Virtualization:** For large tables or lists, use windowing/virtualization libraries (e.g., `react-window`, `react-virtualized`) to render only the visible items.
        * **Performance Optimization:**
            * **Code Splitting:** Load different parts of the dashboard on demand.
            * **Web Workers:** Offload heavy computations (e.g., data processing, complex calculations) from the main thread to prevent UI freezes.
            * **Efficient Data Structures:** Choose appropriate data structures for quick lookups and updates.
        * **Scalability & Reliability:**
            * **Backend Scalability:** The backend needs to handle potentially many persistent connections (especially with WebSockets).
            * **Connection Management:** Handle reconnect logic gracefully if the WebSocket/SSE connection drops. Implement backoff strategies.
        * **Charting Libraries:** Choose a performant library (e.g., D3.js, Chart.js, Highcharts, Plotly.js) that can handle frequent data updates efficiently, possibly with canvas rendering for large datasets.

17. **Question:** Describe the concept of Micro-Frontends. What are the pros and cons, and when might you choose this architecture?
    * **Answer:**
        * **Concept:** Micro-Frontends is an architectural approach where a web application is decomposed into smaller, independent "micro-apps" or features, each typically developed, tested, and deployed autonomously by different teams. These micro-apps are then composed together into a cohesive user experience. Think of it like microservices for the frontend.
        * **Composition Techniques:**
            * **Build-time integration:** Publishing components as packages (less common for true micro-frontends).
            * **Server-side composition:** Assembling pages on the server (e.g., using Server-Side Includes or templating).
            * **Client-side composition (most common):**
                * *Iframes:* Simple isolation but can be clunky (routing, styling, communication).
                * *JavaScript Integration:* Each micro-app mounts itself onto a specific DOM element within a container application (e.g., using Web Components or frameworks like single-spa, Module Federation).
        * **Pros:**
            * **Team Autonomy & Scalability:** Independent teams can work on different parts of the application using their preferred stacks (within limits) and release schedules. Scales development effectively.
            * **Technology Diversity:** Allows gradual adoption of new technologies or frameworks for parts of the application without rewriting everything.
            * **Incremental Upgrades:** Easier to refactor or rewrite parts of a large legacy application incrementally.
            * **Smaller, Focused Codebases:** Each micro-frontend is simpler and easier to understand than a large monolith.
            * **Independent Deployments:** Reduces deployment risks; a failure in one micro-frontend is less likely to bring down the entire application.
        * **Cons:**
            * **Operational Complexity:** Managing multiple builds, deployments, and potentially different tech stacks increases complexity. Requires robust CI/CD.
            * **Payload Size:** Can lead to larger overall bundle sizes if dependencies (like React) are duplicated across micro-frontends (Module Federation helps mitigate this).
            * **Inter-app Communication:** Sharing state or coordinating actions between micro-frontends can be challenging (requires patterns like custom events, a shared event bus, or using browser storage).
            * **UI/UX Consistency:** Maintaining a consistent look, feel, and user experience across different micro-frontends requires strong design systems and shared component libraries.
            * **Performance:** Coordinating the loading and initialization of multiple applications can impact initial load time if not managed carefully.
        * **When to Choose:**
            * Large, complex applications developed by multiple independent teams.
            * Organizations wanting to incrementally modernize a legacy frontend monolith.
            * Situations where different parts of the application have vastly different technology needs or release cadences.
            * When organizational structure favors vertical slicing of features across teams.

18. **Question:** How would you design a shared component library for use across multiple frontend applications (potentially using different frameworks)?
    * **Answer:**
        * **Core Technology Choice:**
            * **Web Components:** The most framework-agnostic approach. Standardized browser technology. Can be used directly in HTML or integrated into React/Vue/Angular etc. Provides encapsulation (Shadow DOM) and custom element definitions. Requires polyfills for older browsers. Tooling (like Stencil.js or Lit) can simplify development.
            * **Framework-Specific Components + Adapters:** Build core components in one framework (e.g., React) and create wrappers/adapters for other frameworks (e.g., using React components within Angular via libraries). This can lead to higher maintenance overhead and potential inconsistencies.
            * **CSS-Only Framework / Utility Classes:** Focus on providing styling (like Bootstrap, Tailwind CSS) and potentially minimal JS for behaviour. Applications then build their own component logic using these styles. Less about shared *logic*, more about shared *appearance*.
        * **Design Principles:**
            * **Framework Agnostic (if possible):** Web Components are ideal here.
            * **Clear API & Props:** Define well-documented, consistent props for configuration and customization.
            * **Accessibility (a11y):** Build accessibility in from the start (ARIA attributes, keyboard navigation, focus management).
            * **Theming & Customization:** Provide mechanisms for applications to customize appearance (e.g., CSS Custom Properties for theming, props for variants, composition slots).
            * **Minimal Dependencies:** Keep external dependencies low to avoid conflicts and bloat in consuming applications.
        * **Development & Tooling:**
            * **Monorepo:** Use tools like Lerna, Nx, or Turborepo to manage the component library package(s) alongside potentially example consumer apps within a single repository.
            * **Build Process:** Configure a build process (e.g., using Rollup, Vite, or framework-specific CLIs) to output distributable formats (ESM, CJS, UMD) and potentially type definitions (`.d.ts`).
            * **Storybook / Documentation:** Use tools like Storybook to develop, test, and document components in isolation. Auto-generate documentation from code comments (e.g., JSDoc).
            * **Testing:** Implement unit tests (e.g., Jest, Testing Library) and visual regression tests (e.g., Chromatic, Percy) for components.
        * **Distribution & Versioning:**
            * **Package Manager:** Publish the library as versioned packages to a registry (npm, GitHub Packages, private registry).
            * **Semantic Versioning (SemVer):** Strictly follow SemVer to communicate breaking changes, new features, and patches.
        * **Governance & Contribution:**
            * Establish clear contribution guidelines, code review processes, and ownership.
            * Define a process for proposing and approving new components or changes.

19. **Question:** Imagine you are designing the state management for a complex e-commerce application (cart, user session, product filters, checkout process). What approaches would you consider and what are the trade-offs?
    * **Answer:**
        * **Approaches & Considerations:**
            1.  **Global State Management Library (Redux, Zustand, Pinia, NgRx):**
                * *Pros:* Centralized state, predictable state updates (especially with Redux's pure reducers), powerful middleware (logging, async actions), dev tools for debugging, good for sharing state across distant components (cart, user session).
                * *Cons:* Can involve boilerplate (especially Redux), learning curve, potentially overkill for simple state, performance can be impacted if not optimized (e.g., overly large state slices, unnecessary re-renders). Zustand/Pinia offer simpler APIs than Redux/NgRx.
                * *Use For:* User session, shopping cart, global UI state (e.g., modal visibility), potentially fetched product data cache.
            2.  **Framework-Specific Context API (React Context, Vue Provide/Inject):**
                * *Pros:* Built into the framework, simpler than external libraries for moderately complex state sharing, good for passing data down the component tree without prop drilling.
                * *Cons:* Can lead to performance issues if the context value changes frequently and consumers aren't memoized, less suited for very complex global state or side-effect management compared to dedicated libraries, lacks built-in dev tools like Redux DevTools.
                * *Use For:* Theme settings, user preferences within a section, passing down data/functions to deeply nested components within a specific feature (e.g., filter settings within the product listing page).
            3.  **Component Local State (`useState` in React, `data` in Vue):**
                * *Pros:* Simplest approach, colocated with the component logic, performant for state that doesn't need to be shared.
                * *Cons:* Not suitable for sharing state between components unless passed down via props (prop drilling).
                * *Use For:* Form input values, toggle states (e.g., dropdown open/closed), UI state specific to a single component.
            4.  **Server State Management / Caching Libraries (React Query, SWR, Apollo Client for GraphQL):**
                * *Pros:* Designed specifically for managing asynchronous data (fetching, caching, synchronization, background updates), handles loading/error states automatically, reduces boilerplate for data fetching logic, improves UX with features like stale-while-revalidate.
                * *Cons:* Adds another dependency, primarily focused on server state, not general UI state.
                * *Use For:* Managing product data, categories, search results, potentially user profile data fetched from the backend. Integrates well with other state management solutions.
            5.  **URL / Router State:**
                * *Pros:* Makes application state bookmarkable and shareable, leverages browser history.
                * *Cons:* Limited to serializable data, can make URLs long and complex, might require parsing/serialization logic.
                * *Use For:* Product filters (size, color, category), search queries, pagination state, current product ID on a detail page.
        * **Hybrid Strategy (Recommended):**
            * No single solution fits all needs. A combination is usually best:
                * **Global Store (e.g., Zustand/Redux):** For truly global state like user authentication/session and shopping cart contents.
                * **Server State Library (e.g., React Query/SWR):** For fetching, caching, and synchronizing product data, categories, etc. This handles async complexity effectively.
                * **Context API:** For feature-level state shared within a specific domain (e.g., checkout process state shared between steps).
                * **Local State:** For UI state within individual components (e.g., form inputs).
                * **URL State:** For filter parameters, search terms, pagination.
        * **Trade-offs Summary:** Complexity vs. Simplicity, Performance, Scalability, Debuggability, Boilerplate. The key is choosing the right tool for the right job based on the scope and lifetime of the state.

20. **Question:** Discuss strategies for optimizing the initial load performance (perceived and actual) of a large Single Page Application (SPA).
    * **Answer:** Optimizing initial load performance is crucial for user retention and experience. Strategies target reducing the amount of code/assets downloaded, parsed, and executed initially.
        * **Actual Load Time Optimization:**
            1.  **Code Splitting:** Break the main JavaScript bundle into smaller chunks.
                * *Route-based splitting:* Load code only for the current route (most common). Frameworks often have built-in support (React Router + `React.lazy`, Vue Router dynamic imports, Angular lazy-loaded modules).
                * *Component-based splitting:* Load large components or libraries only when needed (e.g., a complex chart library for a specific dashboard).
            2.  **Tree Shaking:** Use build tools (Webpack, Rollup) configured correctly to eliminate unused code (dead code elimination) from the final bundles. Ensure libraries used are tree-shakeable.
            3.  **Minimize Bundle Size:**
                * **Minification:** Remove whitespace, shorten variable names in JS/CSS.
                * **Compression:** Use server compression (Gzip, Brotli) to reduce file size over the network. Brotli generally offers better compression than Gzip.
                * **Dependency Analysis:** Audit dependencies (`webpack-bundle-analyzer`). Replace large libraries with smaller alternatives if possible. Avoid including large libraries if only using a small part (consider importing specific modules).
            4.  **Efficient Asset Loading:**
                * **Image Optimization:** Use modern formats (WebP, AVIF), responsive images (`srcset`), and lazy loading for offscreen images.
                * **Font Optimization:** Subset fonts, use `font-display: swap;` or `optional`, host fonts locally or use efficient CDN delivery.
                * **Use a CDN:** Serve static assets (JS, CSS, images) from a Content Delivery Network for faster delivery closer to the user.
        * **Perceived Load Time Optimization:**
            1.  **Critical CSS (Inline):** Extract the CSS needed to style the initial viewport content ("above-the-fold") and inline it in the `<head>` of the HTML. Load the rest of the CSS asynchronously.
            2.  **Server-Side Rendering (SSR) or Static Site Generation (SSG):**
                * *SSR:* Render the initial HTML on the server. The user sees content faster, although interactivity (hydration) might take slightly longer. Good for dynamic content and SEO.
                * *SSG:* Pre-render pages at build time. Fastest possible initial load, great for static content (blogs, marketing sites). Can be combined with client-side fetching for dynamic parts.
            3.  **Loading Indicators & Skeletons:** Show loading spinners, progress bars, or placeholder UI elements (skeleton screens) while data or code chunks are loading. This provides feedback and makes the wait feel shorter.
            4.  **Prioritize Content:** Ensure the most important content loads and renders first. Use techniques like deferring non-critical scripts (`defer` attribute) or loading them asynchronously (`async` attribute).
            5.  **Prefetching/Preloading:** Use resource hints (`<link rel="preload">`, `<link rel="prefetch">`) to tell the browser to fetch resources needed for the current page or likely subsequent navigations sooner. Use judiciously to avoid wasting bandwidth.
        * **Measurement & Monitoring:**
            * Use tools like Lighthouse, WebPageTest, and browser DevTools (Network, Performance tabs) to measure Core Web Vitals (LCP, FID, CLS) and identify bottlenecks.
            * Implement Real User Monitoring (RUM) to track performance for actual users.

## Edge-Case Scenarios

21. **Question:** How would you handle API request failures in a robust way? Consider different types of failures.
    * **Answer:** Robust error handling is crucial for a good user experience.
        1.  **Identify Failure Types:**
            * **Network Errors:** The request couldn't reach the server (offline, DNS issues, CORS errors). `Workspace` typically rejects with a `TypeError`. Axios might throw a specific network error.
            * **Server Errors (5xx):** The server encountered an internal error (e.g., 500 Internal Server Error, 503 Service Unavailable). The request reached the server, but it failed.
            * **Client Errors (4xx):** The client made an invalid request (e.g., 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found).
            * **Request Timeouts:** The request took too long to complete.
            * **Response Parsing Errors:** The server responded, but the response body wasn't valid JSON (if expected).
        2.  **Strategies:**
            * **Use `try...catch` with `async/await` or `.catch()` with Promises:** This is fundamental for catching errors during the asynchronous operation.
            * **Check `response.ok` (for `Workspace`):** `Workspace` only rejects on network errors, not on HTTP error statuses (4xx, 5xx). You *must* check `response.ok` (which is true for statuses 200-299) and throw an error manually if it's false to handle HTTP errors in the `catch` block. Libraries like Axios often throw errors for 4xx/5xx statuses automatically.
            * **Retry Logic (with Backoff):** For transient errors (network issues, 503 Service Unavailable), implement an automatic retry mechanism. Use exponential backoff (wait increasingly longer intervals between retries) to avoid overwhelming the server. Limit the number of retries. Consider if the request is idempotent (safe to retry) - GET, PUT, DELETE usually are; POST might not be.
            * **User Feedback:**
                * **Specific Error Messages:** Show informative messages to the user. Avoid generic "Something went wrong." Map error codes (401, 403, 404) to user-friendly explanations.
                * **Graceful Degradation:** If a non-critical API fails, can the application still function partially? (e.g., show cached data, disable the failing feature).
                * **Loading/Error States:** Clearly indicate when data is loading and when an error has occurred within the relevant UI component. Provide a way to manually retry if appropriate.
            * **Logging/Monitoring:** Log detailed error information (request URL, status code, error message, maybe request body for non-sensitive data) to a monitoring service (Sentry, Datadog) to help developers diagnose issues.
            * **Global Error Handling:** Implement a global error handler (e.g., using Axios interceptors, a higher-order component, or context) to catch unhandled promise rejections or common errors (like 401 Unauthorized, triggering a logout).
            * **Timeouts:** Use `AbortController` with `Workspace` or the `timeout` config in Axios to cancel requests that take too long.
        * *Example Snippet (fetch with retry):*
            ```javascript
            async function fetchDataWithRetry(url, options, retries = 3, delay = 500) {
              for (let i = 0; i < retries; i++) {
                try {
                  const response = await fetch(url, options);
                  if (!response.ok) {
                    // Only retry on server/network errors, not client errors (optional)
                    if (response.status >= 500 || response.status === 408 /* Timeout */) {
                         throw new Error(`HTTP error! status: ${response.status}`); // Throw to trigger retry
                    } else {
                         // Handle non-retryable client errors (4xx)
                         const errorData = await response.text(); // Or response.json() if applicable
                         console.error(`Client error ${response.status}: ${errorData}`);
                         // Maybe throw a specific error type?
                         throw new Error(`Client error: ${response.status}`); // Or return a specific result
                    }
                  }
                  return await response.json(); // Success!
                } catch (error) {
                  console.warn(`Attempt ${i + 1} failed: ${error.message}. Retrying in ${delay}ms...`);
                  if (i === retries - 1) throw error; // Rethrow after last attempt
                  await new Promise(resolve => setTimeout(resolve, delay));
                  delay *= 2; // Exponential backoff
                }
              }
            }
            ```

22. **Question:** Your SPA needs to work correctly even when the user's browser is offline or has intermittent connectivity. What strategies can you implement using Service Workers?
    * **Answer:** Service Workers act as programmable network proxies, allowing interception and handling of network requests. They are key to building Progressive Web Apps (PWAs) with offline capabilities.
        1.  **Caching Strategies:**
            * **Cache First:** Check the cache first. If the resource is found, serve it directly. If not, fetch from the network, serve it, and add it to the cache for future use. Ideal for static assets (app shell: HTML, CSS, JS, fonts, logos).
            * **Network First:** Try fetching from the network first. If successful, serve the response and update the cache. If the network fails (offline), serve the resource from the cache as a fallback. Good for frequently updated resources where freshness is preferred but offline access is still needed (e.g., user data, articles).
            * **Stale-While-Revalidate:** Serve the resource directly from the cache (fast). Simultaneously, send a network request in the background to fetch an updated version and update the cache for the *next* time the resource is requested. Excellent balance of speed and freshness.
            * **Cache Only:** Only serve resources from the cache. Requires pre-caching. Suitable for resources guaranteed not to change between app versions.
            * **Network Only:** Bypass the cache entirely. For resources that must always be fresh and have no offline requirement (e.g., sensitive financial data).
        2.  **Implementation Steps:**
            * **Registration:** Register the Service Worker script in your main application JavaScript. Check for browser support.
            * **Installation (`install` event):** Pre-cache essential application shell assets (HTML, CSS, JS, images, fonts) using the Cache API. This happens once when the Service Worker is first installed or updated.
            * **Activation (`activate` event):** Clean up old caches from previous Service Worker versions. Ensure the new Service Worker takes control.
            * **Fetch Interception (`Workspace` event):** Intercept outgoing network requests. Implement the desired caching strategy (Cache First, Network First, etc.) using `event.respondWith()`. Decide whether to fetch from network, serve from cache, or construct a custom response.
        3.  **Handling Dynamic Data & API Requests:**
            * **Caching API Responses:** Apply appropriate strategies (Network First, Stale-While-Revalidate) to API calls. Cache GET requests.
            * **Background Sync API:** For non-critical updates made while offline (e.g., sending a message, liking a post), queue the action using the Background Sync API. The browser will attempt to synchronize when connectivity is restored, even if the user has closed the tab.
            * **IndexedDB:** For storing larger amounts of structured data offline (e.g., user-generated content, fetched data for offline viewing). Service Workers can interact with IndexedDB.
        4.  **User Experience:**
            * Clearly indicate offline status to the user.
            * Explain which features are available offline.
            * Provide feedback when actions are queued for background sync.
        5.  **Tooling:** Libraries like Workbox (from Google) significantly simplify Service Worker development by providing pre-built recipes for common caching strategies, background sync, and more.

23. **Question:** How do you prevent Cross-Site Scripting (XSS) attacks in a frontend application, especially one using a modern framework like React/Vue/Angular?
    * **Answer:** XSS occurs when malicious scripts are injected into trusted websites, usually via user input that isn't properly sanitized. These scripts execute in the victim's browser and can steal session tokens, scrape data, or perform actions on behalf of the user.
        1.  **Framework Built-in Protection:** Modern frameworks like React, Vue, and Angular automatically sanitize data binding by default.
            * **React:** JSX automatically escapes content rendered within curly braces `{}`. `<div>{userInput}</div>` will render the *text* content of `userInput`, not interpret it as HTML.
            * **Angular:** Uses strict contextual escaping by default in templates (interpolation `{{ }}`, property binding `[]`).
            * **Vue:** Similar to React/Angular, `{{ }}` (mustaches) and `v-bind` escape content.
        2.  **Avoid `dangerouslySetInnerHTML` / `v-html` / `[innerHTML]`:** These mechanisms bypass the framework's built-in sanitization and allow rendering raw HTML. Only use them if absolutely necessary (e.g., displaying content from a trusted WYSIWYG editor) and *always* sanitize the HTML string first using a robust library.
        3.  **Sanitize User Input:** Even with framework protection, sanitize input *before* storing it if it might be rendered later in a non-standard context or used in insecure ways.
            * **Use Sanitization Libraries:** Employ libraries like DOMPurify on the client-side or appropriate libraries on the server-side to clean HTML strings, removing potentially malicious tags and attributes while allowing safe ones. Configure the library based on allowed HTML elements/attributes.
        4.  **Content Security Policy (CSP):** Implement a strong CSP via HTTP headers (`Content-Security-Policy`). This tells the browser which sources of content (scripts, styles, images, etc.) are allowed to be loaded and executed. A well-configured CSP can significantly mitigate XSS by:
            * Restricting inline scripts (`script-src 'self' 'nonce-...'` or `'sha256-...'`).
            * Preventing `eval()` and related functions (`script-src 'self'`).
            * Limiting where scripts can be loaded from (`script-src 'self' trusted-cdn.com`).
            * Controlling form submissions (`form-action 'self'`).
        5.  **Set HTTPOnly Cookies:** Store sensitive session tokens in cookies marked with the `HttpOnly` flag. This prevents JavaScript (including malicious XSS scripts) from accessing the cookie.
        6.  **Validate and Sanitize URL Parameters:** If user input from URLs is reflected on the page, ensure it's properly encoded or sanitized.
        7.  **Be Cautious with Third-Party Libraries:** Ensure any third-party JavaScript libraries included are trusted and don't have known vulnerabilities. Keep them updated.
        8.  **JSON Encoding:** When embedding data into HTML (e.g., preloading state), ensure it's properly JSON-encoded and embedded within `<script>` tags in a way that prevents HTML injection escaping the JSON context.

24. **Question:** Consider a feature where users can upload images. What potential issues and edge cases related to the *frontend* handling of file uploads should you consider?
    * **Answer:** Frontend file upload handling involves more than just a file input.
        1.  **File Type and Size Restrictions:**
            * **Client-Side Validation:** Use the `<input type="file" accept="image/png, image/jpeg">` attribute to *suggest* allowed types to the browser's file picker. Use JavaScript to check the `file.type` and `file.size` properties *before* starting the upload. This provides quick feedback but is **not** secure (can be bypassed).
            * **Server-Side Validation:** *Crucial*. The server *must* re-validate file type (using MIME type detection, not just the extension) and size, as client-side checks are easily circumvented.
        2.  **User Experience:**
            * **Progress Indication:** For large files, show a visual progress bar (`XMLHttpRequest.upload.onprogress` or the progress events from `Workspace` upload streaming if supported/used).
            * **Feedback:** Clearly indicate success, failure (with reason), or progress.
            * **Cancellation:** Provide a way for the user to cancel an ongoing upload. Requires using `AbortController` with `Workspace` or `XMLHttpRequest.abort()`.
            * **Preview:** Show a thumbnail preview of the selected image(s) before uploading using `FileReader` API (`readAsDataURL`).
            * **Multiple File Uploads:** Handle selecting and uploading multiple files gracefully (UI for displaying multiple files, tracking progress for each or overall).
        3.  **Performance:**
            * **Chunking:** For very large files, consider splitting the file into smaller chunks on the client and uploading them sequentially or in parallel. Requires server-side logic to reassemble the chunks. Improves reliability over unstable connections.
            * **Web Workers:** Image resizing or pre-processing on the client-side (if needed) should be done in a Web Worker to avoid blocking the main thread.
        4.  **Error Handling:**
            * Handle network errors during upload.
            * Handle server-side errors (validation failures, processing errors, storage issues).
            * Handle browser API errors (e.g., `FileReader` errors).
        5.  **Security:**
            * While primarily a backend concern, the frontend should ensure it sends files securely (HTTPS) and doesn't expose sensitive information during the process. Avoid reflecting file metadata (like full paths) directly in the UI.
        6.  **Browser/Device Compatibility:**
            * Test on different browsers and devices (mobile file inputs can behave differently).
            * Ensure APIs used (`FileReader`, `Workspace API`, `AbortController`) are adequately supported or polyfilled if necessary.
        7.  **Accessibility:**
            * Ensure the file input and related controls (upload button, progress indicators, error messages) are accessible to keyboard users and screen readers (use proper labels, ARIA attributes like `aria-valuenow` for progress).

25. **Question:** What happens if a user interacts with your SPA (e.g., clicks a button triggering an update) just as a new version of the application code is deployed? How can you handle this gracefully?
    * **Answer:** This scenario can lead to errors if the newly loaded code chunks are incompatible with the already running application version or if expected API endpoints have changed.
        1.  **Problem:** The user's browser has loaded the initial HTML and JS (version A). They perform an action. If a deployment (version B) happened in the meantime, a lazy-loaded code chunk or a subsequent API call might fetch resources from version B, potentially causing:
            * Errors due to mismatched function signatures or component APIs between version A and B code chunks.
            * Errors if version B expects a different API response format than what version A's code knows how to handle.
            * "ChunkLoadError" or similar if the specific chunk requested by version A no longer exists in version B's manifest.
        2.  **Strategies:**
            * **Full Page Reload on Navigation:** The simplest approach. If using client-side routing, detect that a new version is available (e.g., via a Service Worker update prompt or checking a version endpoint) and force a full page reload on the *next* route navigation. This ensures the user gets the complete new version. Can be slightly disruptive.
            * **Service Worker Update Notification:**
                * Configure the Service Worker to detect when a new version is waiting to be activated (`waiting` state).
                * Notify the user (e.g., via a toast message: "A new version is available. Refresh?").
                * If the user accepts, use `skipWaiting()` in the SW and then force a page reload (`window.location.reload()`). This gives the user control.
            * **Error Boundary Detection:** Wrap lazy-loaded components or routes in error boundaries (e.g., React Error Boundaries). If a `ChunkLoadError` or similar occurs, the error boundary can catch it and prompt the user to refresh the page to get the latest version.
            * **Build Manifest Hashing:** Ensure build tools generate unique hashes for filenames (`main.[contenthash].js`, `chunk.[contenthash].js`). When the browser requests a chunk using an old hash after a new deployment, it might receive a 404. This error can be caught (e.g., in error boundaries or global handlers) to trigger a refresh prompt.
            * **API Versioning:** Use API versioning (e.g., `/api/v1/users`, `/api/v2/users`) so that older frontend versions continue to work with older, still-supported API versions even after a new backend deployment. This decouples frontend and backend deployments but adds complexity.
            * **Blue/Green Deployments (Infrastructure Level):** While not strictly frontend code, this deployment strategy ensures that version B is fully deployed and tested on new infrastructure before traffic is switched. This doesn't solve the specific problem of a user mid-session during the switch, but reduces the *window* where inconsistencies might occur.
        * **Recommended Combination:** Often involves a Service Worker for update detection/prompting combined with robust error handling (error boundaries, global fetch error handlers) to catch chunk load errors and suggest a refresh as a fallback.

## Tricky / Gotchas

26. **Question:** What is the output of the following code and why?
    ```javascript
    for (var i = 0; i < 3; i++) {
      setTimeout(() => {
        console.log(i);
      }, 10);
    }
    ```
    * **Answer:**
        * **Output:**
            ```
            3
            3
            3
            ```
        * **Why:**
            1.  **`var` Scoping:** Variables declared with `var` are function-scoped (or globally scoped if not in a function), not block-scoped. This means there is only *one* `i` variable shared across all iterations of the loop.
            2.  **Asynchronous `setTimeout`:** `setTimeout` schedules the provided callback function to run *after* a minimum delay (10ms here). It does *not* pause the loop.
            3.  **Loop Completion:** The `for` loop runs to completion very quickly. By the time the first `setTimeout` callback is ready to execute (after ~10ms), the loop has already finished, and the value of the shared `i` variable is `3`.
            4.  **Closure:** Each `setTimeout` callback forms a closure, capturing the *variable* `i`, not its value at the time the timeout was scheduled. When the callbacks eventually run, they all reference the same `i` variable, which now holds the value `3`.
        * **How to fix (get 0, 1, 2):**
            * **Use `let`:** Change `var` to `let`. `let` is block-scoped, so each loop iteration gets its *own* `i` variable, which is captured by the closure.
                ```javascript
                for (let i = 0; i < 3; i++) { // Use let
                  setTimeout(() => {
                    console.log(i); // Logs 0, 1, 2
                  }, 10);
                }
                ```
            * **Use an IIFE (Immediately Invoked Function Expression) with `var` (older way):** Create a new scope for each iteration.
                ```javascript
                for (var i = 0; i < 3; i++) {
                  (function(j) { // IIFE creates a new scope
                    setTimeout(() => {
                      console.log(j); // Logs 0, 1, 2
                    }, 10);
                  })(i); // Pass current value of i into the IIFE
                }
                ```

27. **Question:** Explain Event Bubbling and Capturing in the context of the DOM event model. How can you stop propagation?
    * **Answer:**
        * **Event Propagation:** When an event occurs on an HTML element, it doesn't just happen on that single element. It goes through two main phases of propagation through the DOM tree:
            1.  **Capturing Phase:** The event travels *down* the DOM tree from the `window` to the target element. Event listeners attached in the capturing phase are triggered first.
            2.  **Target Phase:** The event reaches the target element where it originated. Listeners attached directly to the target are triggered.
            3.  **Bubbling Phase:** The event travels back *up* the DOM tree from the target element towards the `window`. Event listeners attached in the bubbling phase (the default) are triggered.
        * **`addEventListener`:** The `addEventListener` method allows specifying the phase:
            * `element.addEventListener('click', handler, false);` (or omit the last argument): Listener attached in the **bubbling** phase (default).
            * `element.addEventListener('click', handler, true);`: Listener attached in the **capturing** phase.
        * **Stopping Propagation:** You can prevent the event from traveling further in either phase using methods on the `event` object:
            * `event.stopPropagation()`: Prevents the event from propagating further up (bubbling) or down (capturing) the DOM tree beyond the current element. Listeners on ancestor elements (or descendant elements in the capturing phase) will not be triggered.
            * `event.stopImmediatePropagation()`: Does everything `stopPropagation()` does, but *also* prevents any *other* listeners attached to the *same element* for the *same event type* from being executed.
        * **`event.preventDefault()`:** This is different. It prevents the browser's default action associated with the event (e.g., preventing form submission on button click, preventing a link from navigating). It does *not* stop the event's propagation through the DOM.

28. **Question:** What is `NaN` in JavaScript? Why does `NaN === NaN` evaluate to `false`? How can you reliably check if a value is `NaN`?
    * **Answer:**
        * **`NaN` (Not-a-Number):** It's a special numeric value that represents an undefined or unrepresentable result of an arithmetic operation, such as:
            * `0 / 0`
            * `Math.sqrt(-1)`
            * `parseInt('hello')`
            * Any arithmetic operation involving `NaN` itself (e.g., `NaN + 5`)
        * **`NaN === NaN` is `false`:** According to the IEEE 754 standard for floating-point arithmetic (which JavaScript follows), `NaN` is defined as unequal to everything, including itself. This is because `NaN` often represents the result of *different* invalid operations, so two `NaN` values aren't necessarily the "same" invalid result.
        * **Reliable Check:** You cannot use `==` or `===` to check for `NaN`. Use:
            1.  **`Number.isNaN(value)`:** This is the most reliable method introduced in ES6. It checks if the `value` is the specific value `NaN` *without* type coercion. `Number.isNaN(NaN)` is `true`. `Number.isNaN('hello')` is `false`.
            2.  **`isNaN(value)` (Global function):** This is an older, less reliable function. It first tries to *coerce* the `value` to a number. If the coercion results in `NaN` (or if the value was already `NaN`), it returns `true`. This leads to potentially confusing results: `isNaN('hello')` is `true` (because `'hello'` coerces to `NaN`), but `isNaN(NaN)` is also `true`. `Number.isNaN()` is preferred because it doesn't perform this coercion.
            3.  **Self-Comparison (`value !== value`):** Since `NaN` is the only JavaScript value not equal to itself, `value !== value` will be `true` if and only if `value` is `NaN`. This is a clever but perhaps less readable check.

29. **Question:** Consider this CSS. Which color will the paragraph text be and why?
    ```html
    <div id="container" class="box special">
      <p class="content">Hello World</p>
    </div>
    ```
    ```css
    #container p { color: blue; } /* Rule A */
    div.box p.content { color: red; } /* Rule B */
    .box .content { color: green; } /* Rule C */
    p { color: purple; } /* Rule D */
    ```
    * **Answer:**
        * **Color:** The paragraph text will be **red**.
        * **Why (Specificity Calculation):** We need to calculate the specificity of each rule targeting the `p` element:
            * **Rule A:** `#container p`
                * IDs: 1 (`#container`)
                * Classes/Attributes/Pseudo-classes: 0
                * Elements: 1 (`p`)
                * Specificity: (0, 1, 0, 1)
            * **Rule B:** `div.box p.content`
                * IDs: 0
                * Classes/Attributes/Pseudo-classes: 2 (`.box`, `.content`)
                * Elements: 2 (`div`, `p`)
                * Specificity: (0, 0, 2, 2)
            * **Rule C:** `.box .content`
                * IDs: 0
                * Classes/Attributes/Pseudo-classes: 2 (`.box`, `.content`)
                * Elements: 0
                * Specificity: (0, 0, 2, 0)
            * **Rule D:** `p`
                * IDs: 0
                * Classes/Attributes/Pseudo-classes: 0
                * Elements: 1 (`p`)
                * Specificity: (0, 0, 0, 1)
        * **Comparison:**
            1.  Compare ID counts: Rule A (1) is higher than B, C, D (0). So Rule A wins based on ID specificity. **Wait, I made a mistake in the initial analysis. Rule B has the highest specificity.** Let's recalculate carefully.

            * Rule A: `#container p` -> 1 ID, 1 Element -> (0, 1, 0, 1)
            * Rule B: `div.box p.content` -> 2 Elements, 2 Classes -> (0, 0, 2, 2)
            * Rule C: `.box .content` -> 2 Classes -> (0, 0, 2, 0)
            * Rule D: `p` -> 1 Element -> (0, 0, 0, 1)

            Comparing specificity vectors from left to right:
            * ID column: Rule A (1) is highest.
            * Therefore, Rule A (`#container p`) is the most specific selector.

        * **Corrected Answer:**
            * **Color:** The paragraph text will be **blue**.
            * **Reason:** Rule A (`#container p`) has a specificity of (0, 1, 0, 1) due to the ID selector (`#container`). Rule B (`div.box p.content`) has a specificity of (0, 0, 2, 2). Rule C (`.box .content`) has (0, 0, 2, 0). Rule D (`p`) has (0, 0, 0, 1). Since the ID column is the most significant, Rule A's specificity (1 in the ID column) outweighs the others (0 in the ID column), making it the winning rule.

30. **Question:** What is the difference between `localStorage`, `sessionStorage`, and Cookies? When would you use each?
    * **Answer:** These are all client-side storage mechanisms, but they differ significantly in scope, lifespan, and usage:
        * **`localStorage`:**
            * **Capacity:** Larger (typically 5-10MB per origin).
            * **Lifespan:** Persistent. Data remains stored even after the browser window/tab is closed and the browser is restarted. It persists until explicitly cleared by the user (clearing browser data) or the web application (`localStorage.removeItem()`, `localStorage.clear()`).
            * **Accessibility:** Accessible from any window or tab within the same origin (`protocol` + `domain` + `port`). Not sent to the server with HTTP requests automatically. Accessible only via client-side scripts.
            * **Use Cases:** Storing user preferences (e.g., theme settings, UI choices), caching application data for offline use, saving user progress in a web application that doesn't require server interaction for saving.
        * **`sessionStorage`:**
            * **Capacity:** Larger (typically 5-10MB per origin).
            * **Lifespan:** Session-based. Data persists only as long as the browser tab or window is open. Closing the tab/window clears the storage for that session. Opening the same URL in a new tab starts a *new* session with separate storage.
            * **Accessibility:** Accessible only within the specific browser tab/window it was created in (same origin). Not sent to the server automatically. Accessible only via client-side scripts.
            * **Use Cases:** Storing temporary session-specific data (e.g., user's current state within a single workflow like form data across multiple steps, temporary UI state for the current tab), storing sensitive data briefly that shouldn't persist after the tab closes.
        * **Cookies:**
            * **Capacity:** Much smaller (typically ~4KB per cookie, limited number per domain).
            * **Lifespan:** Configurable. Can be session-based (deleted when browser closes) or persistent (deleted at a specified expiry date/time set via `Expires` or `Max-Age` attribute).
            * **Accessibility:** Accessible from both client-side scripts (unless `HttpOnly` flag is set) and **sent automatically to the server with every HTTP request** to the same domain (can increase request overhead).
            * **Use Cases:** Primarily used for server-side session management (e.g., storing session IDs), tracking user activity across sessions (e.g., analytics, remembering login status), storing small pieces of user-specific data needed by the server. The `HttpOnly` flag is crucial for security (preventing XSS from stealing session cookies). `Secure` flag ensures cookies are only sent over HTTPS. `SameSite` attribute helps mitigate CSRF attacks.
        * **Key Differences Summary:**
            | Feature        | `localStorage`          | `sessionStorage`        | Cookies                     |
            | -------------- | ----------------------- | ----------------------- | --------------------------- |
            | Capacity       | 5-10MB                  | 5-10MB                  | ~4KB                        |
            | Lifespan       | Persistent (until clear)| Per Tab/Window Session  | Configurable (Session/Expiry) |
            | Accessibility  | Same Origin (any window)| Same Origin (same tab)  | Same Domain (Client/Server) |
            | Sent to Server | No                      | No                      | Yes (automatic)             |
            | Primary Use    | User Prefs, Offline Data| Temp Session State      | Session Mgmt, Tracking    |

## Deep / Advanced

31. **Question:** Explain the Critical Rendering Path (CRP). How can you optimize it?
    * **Answer:**
        * **Concept:** The Critical Rendering Path refers to the sequence of steps the browser takes to convert HTML, CSS, and JavaScript into pixels on the screen. Optimizing the CRP means minimizing the time it takes to complete these steps for the initial view, leading to faster perceived and actual rendering performance.
        * **Steps:**
            1.  **HTML Parsing -> DOM Tree:** The browser parses the HTML markup and builds the Document Object Model (DOM) tree, representing the structure of the page.
            2.  **CSS Parsing -> CSSOM Tree:** The browser parses CSS files (external, internal, inline styles) and builds the CSS Object Model (CSSOM) tree, representing the styles associated with each DOM node. CSS parsing can block rendering.
            3.  **JavaScript Execution:** JavaScript can block DOM construction if it's encountered during parsing (`<script>` tags without `async` or `defer`). JS can also query and modify both the DOM and CSSOM.
            4.  **Render Tree Construction:** The browser combines the DOM and CSSOM trees into a Render Tree. This tree only includes visible elements (e.g., elements with `display: none` are excluded) and contains nodes with their computed styles.
            5.  **Layout (Reflow):** The browser calculates the exact size and position of each node in the Render Tree on the page. It determines the geometry of the page (where boxes go). This is often a computationally expensive step. Layout is triggered by changes affecting geometry (width, height, position, adding/removing elements).
            6.  **Paint (Rasterization):** The browser fills in the pixels for each node based on the calculated layout and styles (colors, borders, shadows). This involves drawing elements layer by layer.
            7.  **Composition:** The browser composites the painted layers onto the screen in the correct order. Operations affecting only composition (like `transform`, `opacity`) are generally cheaper as they don't require re-layout or re-paint and can often be handled by the GPU.
        * **Optimization Strategies:**
            1.  **Minimize Critical Resources:** Reduce the number of CSS and JavaScript files that block initial rendering.
            2.  **Optimize Resource Size:** Minify CSS/JS, compress assets (Gzip/Brotli), optimize images.
            3.  **Optimize Resource Loading Order:**
                * Load CSS early (`<link rel="stylesheet">` in `<head>`).
                * Use `async` or `defer` attributes for non-critical JavaScript (`<script async>` or `<script defer>`). `defer` executes scripts in order after HTML parsing but before `DOMContentLoaded`, while `async` executes whenever downloaded, potentially out of order and blocking parsing briefly. Place scripts just before `</body>` if not using `defer`/`async`.
            4.  **Unblock Rendering with CSS:**
                * Inline critical CSS for above-the-fold content directly in the `<head>`.
                * Load non-critical CSS asynchronously (e.g., using `media="print"` onload trick or `<link rel="preload" as="style" onload="...">`).
            5.  **Reduce Render Tree Complexity:** Simplify the DOM structure. Avoid overly complex CSS selectors.
            6.  **Avoid Forced Synchronous Layouts:** Reading layout properties (like `offsetHeight` or `getComputedStyle()`) shortly after making DOM changes that invalidate layout can force the browser to perform layout synchronously, which is detrimental to performance. Batch DOM reads and writes if possible.
            7.  **Optimize Paint and Composition:**
                * Promote elements that animate frequently to their own compositor layer using CSS `will-change: transform;` or `transform: translateZ(0);` (use judiciously, can increase memory usage).
                * Prefer animating `transform` and `opacity` as they often only trigger composition, which is cheaper than layout or paint.
                * Reduce paint areas and complexity (e.g., avoid complex `box-shadow` or gradients if not needed). Use browser DevTools (Performance tab, Layers panel) to diagnose layout, paint, and composition bottlenecks.

32. **Question:** What are Web Workers? Explain their types and use cases for improving frontend performance.
    * **Answer:**
        * **Concept:** Web Workers provide a mechanism to run scripts in background threads, separate from the main execution thread (which handles UI updates and user interactions). This prevents long-running or computationally intensive tasks from blocking the main thread, thus keeping the UI responsive and avoiding freezes.
        * **Characteristics:**
            * Run in parallel to the main thread.
            * Do *not* have direct access to the DOM (cannot manipulate UI elements directly).
            * Do *not* have access to the `window` object or some other browser APIs tied to the UI thread.
            * Communicate with the main thread using message passing (`postMessage()` to send, `onmessage` event listener to receive). Data is copied (not shared) between threads, though `Transferable Objects` can be used for efficient transfer of large data like `ArrayBuffer`.
            * Have access to many web APIs, including `Workspace`, `XMLHttpRequest`, `IndexedDB`, `Cache API`, `Promise`, `setTimeout`/`setInterval`, and `console`.
        * **Types:**
            1.  **Dedicated Workers:** The most common type. Owned by a single script/page instance. Created using `new Worker('worker.js')`. Terminated when the creating page/script closes or explicitly via `worker.terminate()`.
            2.  **Shared Workers:** Can be accessed by multiple scripts (from different windows, iframes, or even other workers) running in the same origin. They provide a shared state mechanism across different Browse contexts. Created using `new SharedWorker('shared_worker.js')`. Communication involves `MessagePort` objects. Less commonly used due to complexity.
            3.  **Service Workers:** A specialized type of worker acting as a proxy server between the browser and the network/cache. Key for offline capabilities (PWAs), handling push notifications, and background sync. They have a different lifecycle and registration process. (Covered in more detail in the offline question).
        * **Use Cases for Improving Performance:**
            * **Heavy Computations:** Performing complex calculations, data processing, encryption/decryption, text analysis without freezing the UI.
            * **Background Data Fetching/Processing:** Fetching large amounts of data and processing/parsing it (e.g., large JSON files) before sending a refined result to the main thread.
            * **Image Manipulation:** Resizing, filtering, or analyzing images on the client-side.
            * **Real-time Data Processing:** Handling high-frequency updates from WebSockets or other sources, performing aggregation or analysis before updating the UI minimally.
            * **Prefetching:** Proactively fetching resources in the background based on predicted user actions.
        * **Communication Example (Dedicated Worker):**
            * *main.js:*
                ```javascript
                if (window.Worker) {
                  const myWorker = new Worker('worker.js');

                  myWorker.postMessage({ command: 'startCalculation', data: [1, 2, 3] });

                  myWorker.onmessage = function(e) {
                    console.log('Message received from worker:', e.data);
                    // Update UI with result e.data.result
                  }

                  myWorker.onerror = function(e) {
                     console.error('Error from worker:', e.message);
                  }
                }
                ```
            * *worker.js:*
                ```javascript
                self.onmessage = function(e) {
                  console.log('Message received in worker:', e.data);
                  const { command, data } = e.data;

                  if (command === 'startCalculation') {
                    // Simulate heavy work
                    const result = data.reduce((sum, val) => sum + val, 0) * 1000;
                    // Send result back to main thread
                    self.postMessage({ result: result });
                  }
                }
                ```

33. **Question:** Explain Server-Side Rendering (SSR), Static Site Generation (SSG), Incremental Static Regeneration (ISR), and Client-Side Rendering (CSR). Compare their pros and cons.
    * **Answer:** These are different strategies for rendering web pages, primarily differing in *where* and *when* the HTML is generated.
        1.  **Client-Side Rendering (CSR):**
            * *How it works:* The server sends a minimal HTML shell and a JavaScript bundle. The browser downloads the JS, executes it (often using a framework like React/Vue/Angular), fetches data if needed, and then renders the page content dynamically in the browser (client-side). This is the default for many SPAs.
            * *Pros:* Rich interactivity after initial load, typically simpler backend (serves static assets + API), good for web applications with complex state and frequent updates.
            * *Cons:* Slow initial load time (Time to First Meaningful Paint/Content can be high), requires JS execution for content rendering, potentially poor SEO if search engine crawlers don't execute JS well (though Google is much better now), blank page effect until JS loads/runs ("white screen of death").
        2.  **Server-Side Rendering (SSR):**
            * *How it works:* When a user requests a page, the server fetches data, renders the full HTML for that page dynamically, and sends it to the browser. The browser displays the HTML quickly. Client-side JS (often from the same framework) then takes over ("hydrates") to make the page interactive. Frameworks like Next.js (React) and Nuxt.js (Vue) facilitate SSR.
            * *Pros:* Fast Time to First Contentful Paint (FCP), excellent SEO (search engines get fully rendered HTML), good perceived performance. Works well for pages with dynamic, user-specific content.
            * *Cons:* Slower Time To First Byte (TTFB) compared to SSG as rendering happens on demand, requires a running Node.js (or similar) server, potentially higher server load, complexity in managing server/client state ("hydration mismatches"). Time To Interactive (TTI) might still be delayed until JS hydrates.
        3.  **Static Site Generation (SSG):**
            * *How it works:* All HTML pages are pre-rendered at *build time*. The server then simply serves these static HTML files. Often used with headless CMSs. Frameworks like Next.js, Gatsby, Nuxt.js, Astro support SSG.
            * *Pros:* Fastest possible FCP and TTFB (serving static files is very fast), excellent SEO, highly scalable (can be served from CDNs), minimal server requirements, secure (reduced attack surface).
            * *Cons:* Build times can increase significantly with many pages, content is potentially stale between builds (requires rebuild/redeploy to update), not suitable for highly dynamic or personalized content without client-side fetching layered on top.
        4.  **Incremental Static Regeneration (ISR) (Next.js specific, similar concepts elsewhere):**
            * *How it works:* A hybrid approach combining SSG benefits with dynamic updates. Pages are generated statically at build time (like SSG). However, you can specify a `revalidate` time. When a request comes in after this time has passed, the browser gets the stale static page (fast load), but Next.js triggers a regeneration of the page in the background. Subsequent requests get the newly generated static page.
            * *Pros:* Retains the speed benefits of SSG for most users, allows pages to be updated automatically without a full site rebuild, better scalability than pure SSR for pages that update periodically.
            * *Cons:* Content can still be slightly stale for some users immediately after the revalidation period, potentially higher infrastructure complexity than pure SSG (needs a running Next.js server/function for regeneration), might hit external APIs more often than pure SSG.
        * **Comparison Summary:**
            | Feature        | CSR                               | SSR                                | SSG                                 | ISR                                      |
            | -------------- | --------------------------------- | ---------------------------------- | ----------------------------------- | ---------------------------------------- |
            | **Rendering** | Client (Browser)                  | Server (On Request)                | Server (At Build Time)              | Server (Build Time + On Demand Update)   |
            | **Initial Load**| Slow FCP, Fast TTI (post-load)    | Fast FCP, Slower TTI               | Fastest FCP, Fast TTI               | Fastest FCP (stale), Fast TTI            |
            | **SEO** | Okay (depends on crawler)         | Excellent                          | Excellent                           | Excellent                                |
            | **Dynamic Data**| High (Client Fetch)               | High (Server Fetch)                | Low (Requires Client Fetch/Rebuild) | Medium (Regenerates Periodically)        |
            | **Server Cost** | Low (Static Host + API)           | High (Requires Compute)            | Lowest (Static Host/CDN)            | Medium (Static + Compute for Regen)      |
            | **Use Case** | Dashboards, Complex Apps          | Dynamic Content, E-commerce        | Blogs, Docs, Marketing Sites        | Sites needing speed + periodic updates |

34. **Question:** Discuss memory leaks in JavaScript SPAs. What are common causes and how can you detect and prevent them?
    * **Answer:** Memory leaks occur when allocated memory is no longer needed by the application but is not released by the garbage collector (GC), leading to increasing memory consumption over time, eventually slowing down or crashing the application. SPAs are susceptible because they run for long periods without full page refreshes.
        * **Common Causes:**
            1.  **Accidental Global Variables:** Declaring variables without `let`, `const`, or `var` (in non-strict mode) attaches them to the global (`window`) object. These globals are never garbage collected unless explicitly nulled or the page is closed. Strict mode (`'use strict'`) helps prevent this.
            2.  **Detached DOM Elements:** Storing references to DOM elements in JavaScript variables, but removing those elements from the DOM without clearing the references. The GC cannot collect the elements because the JS references still exist.
                * *Example:* Storing elements in an array/object cache, removing them from the page, but forgetting to remove them from the cache.
            3.  **Lingering Event Listeners:** Attaching event listeners to DOM elements or global objects (`window`, `document`) but failing to remove them when the element or component is destroyed/unmounted. The listener callback often keeps references to its scope (closures), preventing the element and potentially other objects from being collected.
                * *Frameworks:* In frameworks like React/Vue/Angular, it's crucial to clean up listeners added manually (e.g., directly using `window.addEventListener` in `useEffect`) in the cleanup function (return function from `useEffect`, `beforeDestroy`/`unmounted` in Vue, `ngOnDestroy` in Angular). Framework-managed event handlers (like `<button onClick={...}>`) are usually handled automatically.
            4.  **Closures Capturing Unneeded References:** Inner functions (closures) retain access to variables from their outer scope. If a closure created in one context (e.g., an event handler, `setTimeout` callback) is kept alive longer than necessary, it can prevent the outer scope's variables (including potentially large objects or DOM elements) from being garbage collected.
            5.  **Intervals and Timeouts:** Forgetting to clear intervals (`clearInterval`) or timeouts (`clearTimeout`) can keep their callbacks (and associated closures/scopes) alive indefinitely.
            6.  **Caching Objects without Limits:** Storing large amounts of data in client-side caches (arrays, objects) without a mechanism to limit the size or evict old entries.
            7.  **Web Workers:** Failing to terminate Web Workers (`worker.terminate()`) when they are no longer needed.
        * **Detection:**
            1.  **Browser DevTools (Memory Tab):** This is the primary tool.
                * *Heap Snapshots:* Capture the state of the JS heap at different points in time (e.g., before and after performing an action suspected of leaking). Comparing snapshots can reveal objects that were created but not collected. Look for detached DOM trees and objects with large retained sizes.
                * *Allocation Timeline/Instrumentation:* Record memory allocations over time while interacting with the application. Look for patterns of continuously increasing memory usage that doesn't drop back down after actions are complete. Identify the functions responsible for allocations.
            2.  **Performance Monitor (Chrome DevTools):** Track "JS heap size" and "DOM Nodes" over time. A steady increase indicates a potential leak.
            3.  **Code Review:** Look specifically for patterns known to cause leaks (missing listener cleanup, global variables, detached DOM references).
        * **Prevention:**
            1.  **Use Strict Mode:** `'use strict';` helps prevent accidental globals.
            2.  **Proper Cleanup:** Always remove event listeners, clear intervals/timeouts, and terminate workers when components unmount or are no longer needed. Use framework lifecycle methods or `useEffect` cleanup functions.
            3.  **Avoid Global Variables:** Scope variables appropriately using `let` and `const`.
            4.  **Manage DOM References:** Nullify references to DOM elements when they are removed or no longer needed. Use `WeakMap` or `WeakSet` for caching DOM elements if appropriate, as they don't prevent garbage collection if the element is the *only* thing referencing it.
            5.  **Be Mindful of Closures:** Understand what variables closures capture and ensure long-lived closures don't hold onto unnecessary large objects or DOM references.
            6.  **Limit Caches:** Implement size limits or eviction policies (e.g., Least Recently Used - LRU) for application caches.
            7.  **Regularly Profile:** Use DevTools to check for memory issues during development and testing.

35. **Question:** Explain the concept of "Hydration" in the context of SSR/SSG frameworks like Next.js or Nuxt.js. What are common hydration issues and how can they be mitigated?
    * **Answer:**
        * **Concept:** Hydration is the process by which client-side JavaScript (typically from a framework like React, Vue, etc.) attaches event listeners and re-initializes the application state on top of the existing HTML markup that was sent by the server (either via SSR or generated via SSG). The server provides the initial structure and content (the "dry" HTML), and the client-side JS adds the interactivity and makes it a fully functional SPA (making it "wet" or "hydrated").
        * **Purpose:** To provide a fast First Contentful Paint (FCP) using the server-rendered HTML while still enabling the rich interactivity and client-side navigation capabilities of an SPA without throwing away the server-rendered DOM. The framework attempts to reuse the existing DOM nodes generated by the server instead of re-creating them from scratch.
        * **Process (Simplified React Example):**
            1.  Server renders React components to an HTML string (e.g., using `renderToString`).
            2.  Server sends this HTML (and necessary data) to the browser.
            3.  Browser renders the static HTML.
            4.  Browser downloads the client-side JavaScript bundle.
            5.  Client-side React runs and uses `hydrateRoot` (instead of `createRoot`) on the root DOM node containing the server-rendered HTML.
            6.  React walks the virtual DOM tree and compares it to the existing HTML structure. It attempts to "adopt" the existing DOM nodes and attaches event listeners.
        * **Common Hydration Issues ("Hydration Mismatches"):** Errors occur if the DOM structure or content rendered by the client-side JavaScript during the initial hydration pass does *not exactly match* the HTML structure rendered by the server.
            * **Causes:**
                * **Conditional Rendering Based on `window` or Browser APIs:** Code like `typeof window !== 'undefined' ? <DesktopView /> : <MobileView />` might render differently on the server (where `window` is undefined) than on the client.
                * **Timestamps or Random Numbers:** Generating values like `new Date()`, `Math.random()` directly in the render logic will produce different results on the server vs. client.
                * **Browser Extensions Modifying DOM:** Extensions can sometimes interfere with the initial HTML before hydration occurs.
                * **Incorrect HTML Structure:** Malformed HTML sent from the server (e.g., a `<div>` inside a `<p>`). React might correct this on the client, causing a mismatch.
                * **CSS Affecting Markup:** Certain CSS (like display properties) could technically cause subtle differences, although less common as a direct cause of React hydration errors.
                * **Third-party Scripts:** External scripts modifying the DOM before hydration completes.
            * **Consequences:** React usually logs a warning in development. In production, it might attempt to recover by discarding the server-rendered markup and performing a full client-side render of the mismatched subtree, which negates performance benefits and can sometimes cause visual glitches or errors.
        * **Mitigation Strategies:**
            1.  **Ensure Identical Rendering Logic:** The first render pass on the client *must* produce the same virtual DOM structure as the server did.
            2.  **Delay Client-Specific Rendering:** For components that *must* render differently on the client (e.g., based on `window.innerWidth`), use state and the `useEffect` hook (or equivalent lifecycle method). Render a placeholder or the server-rendered version initially, and then update the state in `useEffect` (which only runs on the client *after* the initial render/hydration).
                ```jsx
                // Example (React)
                function ResponsiveComponent() {
                  const [isClient, setIsClient] = useState(false);

                  useEffect(() => {
                    setIsClient(true); // Runs only on client, after initial render
                  }, []);

                  // Render server-compatible content initially, or a loader
                  if (!isClient) {
                    return <ServerFriendlyFallback />; // Or null, or a loader
                  }

                  // Now it's safe to use window object
                  return window.innerWidth > 768 ? <DesktopView /> : <MobileView />;
                }
                ```
            3.  **Suppress Hydration Warnings (Use Sparingly):** For unavoidable minor differences (like timestamps that are purely informational), React provides the `suppressHydrationWarning={true}` prop on an element. This should be a last resort.
            4.  **Check HTML Validity:** Ensure the server generates valid HTML markup.
            5.  **Consistent Data:** Ensure the data used for rendering is the same on both server and client during the initial pass.
            6.  **Disable Interfering Extensions/Scripts:** Rule out external factors during debugging.

36. **Question:** How does the browser's Event Loop work? Explain the roles of the Call Stack, Callback Queue (Task Queue), and Microtask Queue.
    * **Answer:** The Event Loop is the mechanism that allows JavaScript, which is single-threaded (meaning it can only execute one piece of code at a time), to handle concurrency and perform asynchronous operations (like `setTimeout`, DOM events, `Workspace` requests) without getting blocked.
        * **Components:**
            1.  **Call Stack:** A LIFO (Last-In, First-Out) stack where JavaScript function calls are tracked. When a function is called, a new frame is pushed onto the stack. When a function returns, its frame is popped off. JavaScript executes code found on the top of the stack. If the stack is empty, there's nothing to do.
            2.  **Web APIs / Browser APIs:** These are functionalities provided by the browser environment (not part of the JS engine itself), such as `setTimeout`, `setInterval`, DOM APIs (event listeners), `Workspace`, etc. When an asynchronous operation is initiated (e.g., `setTimeout(callback, 1000)`), the call to the Web API function itself completes quickly and is popped from the Call Stack. The Web API then handles the operation (e.g., the timer) in the background.
            3.  **Callback Queue (or Task Queue):** A FIFO (First-In, First-Out) queue where callback functions associated with completed asynchronous Web API operations are placed, waiting to be executed. When a Web API finishes its task (e.g., the timer expires, an event occurs, data is fetched), it pushes the corresponding callback function onto this queue. Examples: `setTimeout`/`setInterval` callbacks, DOM event handlers.
            4.  **Microtask Queue:** Another FIFO queue, but with higher priority than the Callback Queue. Callbacks associated with Promises (`.then()`, `.catch()`, `.finally()`) and other microtasks like `queueMicrotask()`, `MutationObserver` callbacks are placed here.
        * **Event Loop Process:** The Event Loop continuously monitors the Call Stack and the queues. Its job is roughly:
            1.  **Check Call Stack:** If the Call Stack is empty (meaning the current synchronous code has finished executing), proceed to the next step. If not empty, keep processing the stack.
            2.  **Process Microtasks:** Check the Microtask Queue. If it's *not* empty, take the oldest microtask from the queue, push it onto the Call Stack, and execute it. Repeat this step until the Microtask Queue is *completely empty*. Microtasks can enqueue other microtasks, which will also run before moving on.
            3.  **Process Macrotasks (Tasks):** If the Call Stack and Microtask Queue are both empty, check the Callback Queue (Task Queue). If it's *not* empty, take the oldest task (macrotask) from the queue, push its callback function onto the Call Stack, and execute it.
            4.  **Loop:** Go back to Step 1.
        * **Key Implications:**
            * **Non-Blocking:** Asynchronous operations don't block the main thread because their completion callbacks are queued and processed only when the Call Stack is clear.
            * **Microtask Priority:** Microtasks (like Promise resolutions) always run *before* the next macrotask (like `setTimeout` callbacks), even if the `setTimeout` delay was 0ms. This ensures Promise state updates happen predictably before other async events.
                ```javascript
                console.log('Start');

                setTimeout(() => {
                  console.log('Timeout Callback (Macrotask)');
                }, 0);

                Promise.resolve().then(() => {
                  console.log('Promise Resolved (Microtask)');
                });

                console.log('End');

                // Output:
                // Start
                // End
                // Promise Resolved (Microtask)
                // Timeout Callback (Macrotask)
                ```
            * **Starvation:** If code continuously adds microtasks, it could potentially starve the macrotask queue, delaying things like rendering updates or `setTimeout` callbacks (though browsers have mitigations). Similarly, long-running synchronous code on the Call Stack blocks the entire Event Loop.