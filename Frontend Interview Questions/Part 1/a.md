

### Senior Frontend Engineer Interview Questions

---

## **Fundamentals (Conceptual Definitions and Simple Examples)**

1. **What is the Document Object Model (DOM)?**
   - **Answer**: The DOM is a programming interface for web documents. It represents the page so that programs can manipulate the structure, style, and content of web pages. It's a tree-like structure where each node represents an element or attribute on the page. JavaScript interacts with the DOM to update the content dynamically.

2. **Explain the difference between `null` and `undefined` in JavaScript.**
   - **Answer**: 
     - `null` is an assignment value, meaning it represents the intentional absence of any object value. It is often used to indicate that a variable has been explicitly set to have no value.
     - `undefined` is a type itself and is the default value of variables that have been declared but not initialized. It also indicates the absence of value but is automatically assigned by JavaScript.

3. **What are closures in JavaScript?**
   - **Answer**: A closure is a function that remembers its lexical scope even when the function is executed outside that scope. This is possible because the inner function maintains access to variables in its outer function.

4. **What is event delegation in JavaScript?**
   - **Answer**: Event delegation is a technique of handling events at a higher level in the DOM rather than attaching event listeners to individual elements. This is useful for dynamically added elements, improving performance by reducing the number of event listeners.

5. **Describe the CSS Box Model.**
   - **Answer**: The CSS Box Model describes how the elements on a page are structured, which consists of:
     - **Content**: The actual content of the element (e.g., text, images).
     - **Padding**: The space between the content and the border.
     - **Border**: The edge surrounding the padding (if any).
     - **Margin**: The space outside the border, creating separation between elements.

---

## **Practical / Coding (Write Code Snippets, Explain Outputs)**

6. **Write a JavaScript function to deep clone an object.**
   ```javascript
   function deepClone(obj) {
       return JSON.parse(JSON.stringify(obj));
   }

   const obj = { a: 1, b: { c: 2 } };
   const clonedObj = deepClone(obj);
   console.log(clonedObj);  // { a: 1, b: { c: 2 } }
   ```
   - **Explanation**: This method uses `JSON.parse()` to convert the object into a JSON string and then back into an object. It's a quick way to clone an object but doesn't handle functions, `undefined`, or circular references.

7. **Write a function to debounce an event handler in JavaScript.**
   ```javascript
   function debounce(func, delay) {
       let timeout;
       return function (...args) {
           clearTimeout(timeout);
           timeout = setTimeout(() => func(...args), delay);
       };
   }

   const log = () => console.log('Debounced!');
   const debouncedLog = debounce(log, 2000);

   window.addEventListener('resize', debouncedLog);
   ```
   - **Explanation**: Debouncing ensures that the `log` function is only called once within the specified delay (2 seconds in this case), even if the resize event is triggered multiple times.

8. **What is the output of the following code?**
   ```javascript
   const a = [1, 2, 3];
   const b = a;
   b[0] = 5;
   console.log(a);
   ```
   - **Answer**: The output will be `[5, 2, 3]`. This is because arrays in JavaScript are reference types, so `b` is a reference to the same array as `a`, modifying `b` also modifies `a`.

9. **How would you write a function to check if a string is a palindrome?**
   ```javascript
   function isPalindrome(str) {
       const cleaned = str.replace(/[^a-zA-Z0-9]/g, '').toLowerCase();
       return cleaned === cleaned.split('').reverse().join('');
   }

   console.log(isPalindrome("A man, a plan, a canal, Panama"));  // true
   ```
   - **Explanation**: This function first removes non-alphanumeric characters and converts the string to lowercase to make the check case-insensitive. Then it checks if the string equals its reverse.

10. **What is a `Promise` in JavaScript, and how would you use it?**
    - **Answer**: A `Promise` is an object representing the eventual completion or failure of an asynchronous operation. It's used to handle asynchronous operations such as API calls or timeouts.
      ```javascript
      function fetchData() {
          return new Promise((resolve, reject) => {
              setTimeout(() => {
                  resolve("Data fetched!");
              }, 2000);
          });
      }

      fetchData().then((data) => console.log(data));
      ```

---

## **System Design / Architecture (Diagrams, Trade-offs)**

11. **Design a scalable web application architecture.**
    - **Answer**: A scalable architecture for a web application could look like the following:
      - **Frontend**: React.js or Angular for the user interface.
      - **Backend**: Node.js or Python (Flask/Django) handling REST APIs.
      - **Database**: NoSQL (MongoDB) for unstructured data or SQL (PostgreSQL) for relational data.
      - **Caching**: Redis to cache frequently accessed data.
      - **Load Balancer**: Nginx or HAProxy to distribute traffic to multiple backend servers.
      - **CDN**: Cloudflare or AWS CloudFront to deliver static assets quickly.
      - **Microservices**: Decompose the system into multiple services that scale independently.

12. **Explain the concept of "Single Page Application" (SPA) and how it affects frontend architecture.**
    - **Answer**: A Single Page Application (SPA) loads a single HTML page and dynamically updates the content as the user interacts with the app. This reduces the need for full page reloads and provides a smoother user experience. In the frontend architecture, SPAs typically use a JavaScript framework like React or Vue.js, and interact with backend APIs to fetch and update data without reloading the page.

13. **How would you handle authentication in a web application?**
    - **Answer**: Authentication can be handled using JSON Web Tokens (JWT). 
      - When a user logs in, a server generates a JWT containing the user's information.
      - The token is then sent to the client (frontend), where it is stored in local storage or cookies.
      - For every subsequent request, the client sends the token as a header for authentication.
      - The server verifies the token before responding to the request.

14. **Design a file upload system for a web application.**
    - **Answer**: The file upload system can be designed using:
      - **Frontend**: Use an HTML `<input type="file">` element to allow users to select files. Use JavaScript (e.g., `fetch` or `XMLHttpRequest`) to send the files as multipart form-data to the backend.
      - **Backend**: Use a Node.js server with `multer` or a similar library to handle file uploads. Files can be stored in a cloud service like AWS S3 or locally on a file system, with metadata stored in a database.
      - **Security**: Validate the file types and sizes to prevent uploading malicious files.

15. **What is the trade-off between server-side rendering (SSR) and client-side rendering (CSR)?**
    - **Answer**: 
      - **SSR**: The server sends a fully rendered page to the client, which improves SEO and performance for the first load.
      - **CSR**: The browser renders the page dynamically after receiving the HTML, allowing for better interactivity and faster subsequent page loads. However, it can hurt SEO and the first load may be slower.

---

## **Edge-Case Scenarios (Failure Modes, Rare Inputs)**

16. **What happens if a `setTimeout` is called with a delay of 0 milliseconds?**
    - **Answer**: Even though the delay is 0 milliseconds, the callback function will be executed after the current execution context finishes. This means it will be executed as soon as the call stack is clear.

17. **What would happen if the user refreshes the page while an AJAX request is in progress?**
    - **Answer**: The AJAX request will be aborted when the page is refreshed because the JavaScript environment is destroyed and reloaded. If the page needs to keep the request alive during refresh, techniques like session persistence or background tasks are necessary.

18. **How do you handle large file uploads in a frontend application?**
    - **Answer**: For large file uploads, consider:
      - **Chunked uploads**: Break the file into smaller chunks and upload them sequentially.
      - **Progress bar**: Use the `XMLHttpRequest` or `fetch` API with progress events to show the upload progress to the user.
      - **Compression**: Compress the file on the client side before uploading to reduce the size.

19. **What would you do if your SPA is rendering data too slowly due to large initial payloads?**
    - **Answer**: Consider:
      - **Code splitting**: Load only the required JavaScript for the initial page load.
      - **Lazy loading**: Load additional data as the user scrolls or interacts with the app.
      - **Caching**: Cache API responses to reduce loading times for subsequent requests.

20. **How would you handle different time zones in a web application?**
    - **Answer**: You should always store dates and times in UTC format on the server. When displaying the time on the frontend, convert it to the user's local time zone using JavaScript's `Date` object or a library like `moment-timezone` or `date-fns-tz`.

---

## **Tricky / Gotchas (Common Interview Puzzles, Counter-Intuitive Behaviors)**

21. **What is the result of `[] == ![]` in JavaScript?**
    - **Answer**: This evaluates to `true`. 
      - `[]` is falsy, so `![]` becomes `false`.
      - When comparing `[] == false`, JavaScript performs type coercion and treats `[]` as an empty string, which is loosely equal to `false`.

22. **Why is the following code problematic?**
    ```javascript
    let x = 0;
    if (x) {
        console.log("This will never log.");
    }
    ```
    - **Answer**: This condition will never be true because `x` is `0`, which is a falsy value in JavaScript. In JavaScript, `0`, `null`, `undefined`, `NaN`, `false`, and `""` are falsy values.

23. **Explain the difference between `let`, `const`, and `var` in JavaScript.**
    - **Answer**: 
      - `var`: Function-scoped, allows variable hoisting, can be re-assigned.
      - `let`: Block-scoped, does not allow hoisting like `var`, can be re-assigned.
      - `const`: Block-scoped, must be assigned an initial value, and cannot be re-assigned (but the object it references can be modified).

24. **What happens when you use `this` inside a function that is executed asynchronously in JavaScript?**
    - **Answer**: The value of `this` may be different than expected. For instance, in an asynchronous function, `this` might refer to the global object (`window` in browsers) instead of the object that the function is a method of. This can be mitigated using `.bind()`, `.call()`, or `.apply()`.

25. **What does `==` do in JavaScript?**
    - **Answer**: The `==` operator in JavaScript performs type coercion before comparing values. For example, `0 == '0'` is `true`, because `'0'` is converted to a number before comparison.

---

## **Deep / Advanced (Performance Tuning, Theoretical Limits, Research-Level Topics)**

26. **How would you optimize a React application with performance issues?**
    - **Answer**: 
      - **Component Memoization**: Use `React.memo` to prevent unnecessary re-renders.
      - **Lazy Loading**: Use `React.lazy()` for code splitting and loading components only when needed.
      - **Avoid Anonymous Functions**: Avoid creating new functions inside the render method to prevent unnecessary re-renders.
      - **Use PureComponent or shouldComponentUpdate**: Optimize class components by extending `PureComponent` or implementing `shouldComponentUpdate` to avoid unnecessary renders.
      - **Virtualization**: Use libraries like `react-window` or `react-virtualized` for rendering large lists efficiently by rendering only what is visible.

27. **What is the importance of Web Workers and when should they be used?**
    - **Answer**: Web Workers are JavaScript threads that run in the background, separate from the main execution thread, allowing for concurrent processing. They are useful for CPU-intensive tasks (like image processing) to avoid blocking the main thread and affecting the user interface performance.

28. **How would you handle large-scale state management in a frontend application?**
    - **Answer**: Use tools like Redux or Context API for managing global state. For complex applications, use a more scalable solution like **Redux Toolkit** or **Recoil**, which provides better performance optimizations and ease of use. Local component states should be used sparingly for better performance.

29. **What is the importance of tree-shaking in modern JavaScript applications?**
    - **Answer**: Tree-shaking is the process of removing unused code from the final bundle. This improves load times by reducing the size of JavaScript files. Tools like Webpack and Rollup perform tree-shaking by analyzing the module dependency graph and eliminating dead code.

30. **How would you prevent a memory leak in a React application?**
    - **Answer**: 
      - Ensure that asynchronous tasks (like `setTimeout`, `fetch`, or subscriptions) are canceled when the component unmounts.
      - Use the `useEffect` hook with a cleanup function to clear timers, cancel network requests, or unsubscribe from subscriptions.
      - Avoid storing large objects in state unless necessary.

