### Senior TypeScript Interview Questions Breakdown  

---

#### **What Is a Generic Function in TypeScript?**
- **Definition**:  
  A **generic function** in TypeScript allows creating type-flexible functions without relying on the `any` type.  
  - It uses **type arguments** to enable different data types to work with a single function.  

- **Example**:  
  ```typescript
  function myNewFunction<T>(param: T): T {
      return param;
  }

  // Usage:
  const result1 = myNewFunction<number>(42);   // Valid: T = number
  const result2 = myNewFunction<string>("hi"); // Valid: T = string
  const result3 = myNewFunction<number>("hi"); // Error: "hi" is not of type number
  ```
  - The function `myNewFunction` accepts a **type argument T**, which ensures its usage works for the provided type.  

- **Purpose**:  
  - To provide flexibility with precise type safety.  
  - Useful when working with different data types but still wanting to maintain type checks.  

---

#### **What Is `as const` in TypeScript?**
- **Definition**:  
  The **`as const`** keyword seals an object or array, making all of its properties **immutable** and their values treated as literal types.  

- **Why Use It?**  
  - Ensures that properties of an object cannot be reassigned or modified, even at nested levels.  
  - Great for defining configuration objects that should remain unchanged.  

- **Example**:  
  ```typescript
  const config = {
      host: "localhost",
      port: 3000
  } as const;

  config.host = "127.0.0.1"; // Error: Cannot assign to 'host' because it is a read-only property
  ```

- **Comparison with `Object.freeze`:**
  - **`as const`**:  
    - Works at **build time**, preventing type violations during development.
    - Affects **nested properties** as well.  
  - **`Object.freeze`**:  
    - Works at **runtime**, ensuring no modifications during execution.  
    - Freezes only the **first level** of an object (nested properties remain mutable).  
    ```typescript
    const frozenConfig = Object.freeze({
        host: "localhost",
        nested: { path: "/api" }
    });

    frozenConfig.nested.path = "/new-api"; // Allowed! Nested property is not frozen
    ```

---

#### **What Does the `private` Modifier Do in TypeScript?**
- **Definition**:  
  The `private` keyword makes a class member (variable or method) **accessible only within the declaring class**.  

- **Example**:
  ```typescript
  class Service {
      private config: string;

      constructor(config: string) {
          this.config = config;
      }
  }

  const service = new Service("myConfig");
  console.log(service.config); // Error: 'config' is private and only accessible within class 'Service'
  ```
  
- **Purpose**:  
  - Encapsulates data and prevents unauthorized access.  
  - Adheres to the **SOLID principle** of exposing only what is necessary.  

---

#### **What Is a Decorator in TypeScript?**
- **Definition**:  
  A **decorator** is a special function in TypeScript that can modify or enhance the behavior of classes, methods, properties, or parameters.

- **Use Cases**:  
  - Commonly used to wrap logic, reuse code, or extend functionality without directly modifying the class or function.  
  - Popular in frameworks like **Angular** or **NestJS** for marking classes/services as injectable.

- **Example**:  
  ```typescript
  function Injectable(constructor: Function) {
      console.log(`${constructor.name} is now injectable`);
  }

  @Injectable
  class Service {}
  ```

- **Purpose**:
  - Fosters **composition over inheritance**.  
  - Avoids deep class hierarchies, improving code maintainability.  

---

#### **Type vs. Interface in TypeScript**
- **Definitions**:
  - **Interface**: Defines the shape or structure of an object, designed to be extensible.  
  - **Type**: Provides a way to define custom types, unions, intersections, and more.  

- **Differences**:  
  - **Interface Merging**:  
    - Interfaces combine automatically at build time.  
    - Great for libraries needing extensibility (e.g., `Express.Request`).  
    - Example:  
      ```typescript
      interface User {
          name: string;
      }

      interface User {
          age: number;
      }

      const user: User = { name: "John", age: 25 }; // Works: Merged at build time
      ```
  - **Types Are Unique**:  
    - Types cannot merge.  
    - Example:
      ```typescript
      type User = { name: string; };
      type User = { age: number; }; // Error: Duplicate identifier 'User'
      ```

- **When to Use Each**:  
  - Use **interfaces** for things you anticipate extending (e.g., library configurations, props, etc.).  
  - Use **types** for domain-specific entities (e.g., `Product`, `Order`, etc.).  

---

#### **What Is a Type Guard in TypeScript?**
- **Definition**:  
  A **type guard** is a function or expression that helps identify specific types within a **union type** during runtime.  

- **Use Case**:  
  - When working with union types and needing to distinguish between subtypes for type-safe handling.  

- **Example**:
  ```typescript
  type Fruit = { kind: "pineapple" | "apple" | "strawberry" };

  function isPineapple(fruit: Fruit): fruit is { kind: "pineapple" } {
      return fruit.kind === "pineapple";
  }

  const fruit: Fruit = { kind: "apple" };

  if (isPineapple(fruit)) {
      console.log("It's a pineapple");
  } else {
      console.log("It's not a pineapple");
  }
  ```

---

#### **Structural Typing vs. Nominal Typing**
- **Structural Typing**:  
  - TypeScript employs structural typing.  
  - Two objects are considered the same if they have the same shape or structure, regardless of their declared type.  

  - **Example**:  
    ```typescript
    class A { name: string = ""; }
    class B { name: string = ""; }

    const obj: A = new B(); // Allowed: A and B have the same structure
    ```

- **Nominal Typing**:  
  - Found in languages like Java or C#.  
  - Two objects are equal only if they share the exact same declared type or class.  

  - **Example in Nominal Typing (conceptual)**:  
    - `A` and `B` would not match unless they were of the same declared class.

- **Advantages of Structural Typing in TypeScript**:
  - Provides more flexibility.  
  - Encourages an interoperable code style without strict class-based constraints.  

---

