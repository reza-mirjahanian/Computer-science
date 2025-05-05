# React Hooks Rules  
**Rule 1**: Always call hooks **at the top level** of React components.  
- Avoid using hooks inside **loops**, **conditions**, or nested functions.  
- Ensures consistent hook execution order.  

**Rule 2**: Only call hooks in **React components** or **custom hooks**.  
- Never use hooks in regular JavaScript functions.  

---

# React.lazy & Suspense  
**Mechanism**:  
- **React.lazy**: Enables dynamic imports for code splitting (e.g., `const Component = React.lazy(() => import('./Component'))`).  
  - Webpack splits code into separate bundles.  
- **Suspense**: Displays a fallback UI (e.g., loading spinner) while the lazy component loads.  

**Pros**:  
- Reduces initial bundle size.  
- Improves load times for non-critical components (e.g., route-based splitting).  

**Cons**:  
- Overuse can harm performance (excessive HTTP requests).  
- **Incompatible with server-side rendering (SSR)**.  

---

# Key Attribute in Lists  
**Purpose**: Helps React track elements during re-renders for **efficient updates**.  
- **Why**:  
  - Prevents unnecessary re-renders of unchanged elements.  
  - Avoids bugs when list order/contents change (e.g., deletions).  
- **Best Practice**: Use **unique, stable identifiers** (not array indices).  

---

# useMemo Hook  
**Purpose**: Memoizes expensive computations to optimize re-renders.  
- **Syntax**: `const memoizedValue = useMemo(() => computeValue(), [dependencies])`  
- **Use Cases**:  
  - Heavy calculations (e.g., filtering large arrays).  
  - Avoiding redundant re-renders in child components.  
- **Note**: Overuse can lead to memory overhead.  

---

# React Context  
**Purpose**: Manages global state without prop drilling.  
- **Performance Impact**:  
  - **Re-renders**: All consumers re-render when context value changes.  
  - **Optimization**: Split unrelated state into separate contexts.  

**Ideal State for Context**:  
- Global data (e.g., authentication status, theme, language).  
- Avoid: Local/component-specific state or complex state transitions (use **Redux**/**state machines** instead).  

---

# Class vs. Functional Components  
**Class-Exclusive Features**:  
- **Lifecycle Methods** (e.g., `componentDidCatch` for error boundaries).  
- **Legacy APIs**: `getSnapshotBeforeUpdate`, `getDerivedStateFromError`.  

**Functional Components**:  
- Use **hooks** (e.g., `useEffect`, `useState`) to replicate lifecycle behavior.  
- **Limitation**: Cannot create error boundaries without class components.