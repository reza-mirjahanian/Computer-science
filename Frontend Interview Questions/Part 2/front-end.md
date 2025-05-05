

---

### **1. Difference Between Cookies, Local Storage, and Session Storage**  
- **Cookies**:  
  - **Storage Limit**: ~4 KB.  
  - **Persistence**: Expires based on set date.  
  - **Use Case**: Authentication (e.g., tokens), sharing state between server/client. Sent with every HTTP request.  
  - **Security**: HTTPS-protected.  

- **Local Storage**:  
  - **Storage Limit**: ~5-10 MB (varies by browser).  
  - **Persistence**: Persists indefinitely until cleared.  
  - **Use Case**: Storing app state (e.g., user preferences like dark mode).  

- **Session Storage**:  
  - **Storage Limit**: ~5-10 MB.  
  - **Persistence**: Deleted when the tab/browser is closed.  
  - **Use Case**: Temporary data (e.g., form inputs during checkout to survive page refreshes).  

---

### **2. Front-End Performance Optimizations (React App Example)**  
- **Module Bundler (Webpack/Vite)**:  
  - **Polyfills**: Add backward-compatible code for unsupported browser features.  
  - **Compression**: Use GZIP/Brotli to reduce bundle size (~70% reduction).  
  - **Minification/Uglify**: Remove whitespace, shorten variable names.  
  - **Source Maps**: Map minified code to original source for debugging.  
  - **Code Splitting**: Load only necessary JS for initial render; lazy-load remaining.  
  - **Tree Shaking**: Remove unused code (requires ES6 modules).  

- **Additional Optimizations**:  
  - CDN for assets (images/fonts).  
  - Image optimization (dimensions, format, lazy loading).  

---

### **3. Optimizing Large Images (eCommerce Example)**  
- **Resize Images**: Serve appropriately sized images (e.g., 800px width instead of 3000px).  
- **Compression**: Use tools to reduce file size (e.g., WebP format, metadata removal).  
- **CDN**: Host images on CDN with caching/auto-resizing (e.g., Cloudflare, AWS CloudFront).  
- **Lazy Loading**: Load images on scroll; set `width`/`height` to avoid layout shifts.  
- **Responsive Images**: Use `srcset` to serve different sizes based on viewport.  

---

### **4. Managing Code Quality in Large-Scale Apps**  
- **Tools & Practices**:  
  - **Linters**: Enforce code style (ESLint, Prettier).  
  - **Testing**: Unit tests (Jest), E2E tests (Cypress).  
  - **Dependency Scans**: Check for vulnerabilities (npm audit, Snyk).  
  - **Accessibility**: Linting for a11y (axe-core).  
  - **Monitoring**: Lighthouse for Core Web Vitals; Sentry for error tracking.  

---

### **5. Preventing XSS Attacks**  
- **What is XSS?**: Attackers inject malicious scripts into your app (e.g., via user inputs like comments).  
- **Prevention**:  
  - **Sanitize Inputs**: Remove/escape unsafe HTML/JS from user inputs.  
  - **Avoid Direct Rendering**: Never use `dangerouslySetInnerHTML` (React) or equivalent.  
  - **Content Security Policy (CSP)**: Restrict sources of executable scripts.  

---

### **6. How CDNs Work**  
- **Function**: Distribute assets globally via edge servers (closer to users for lower latency).  
- **Advantages**:  
  - Faster load times.  
  - Reduced server load.  
  - Built-in caching/image optimization.  
- **Providers**: AWS CloudFront, Cloudflare, Azure CDN.  

---

### **7. Micro Frontends**  
- **What?**: Split a monolithic app into smaller, independently deployable apps (e.g., header, checkout).  
- **When to Use**:  
  - Large teams (~30+ engineers) needing parallel workflows.  
  - Deployment bottlenecks (e.g., teams blocking each other).  
- **Trade-offs**:  
  - Increased complexity (shared state, cross-app communication).  
  - Advanced tooling required (module federation, CI/CD pipelines).  

---

**Key Takeaways for Interviews**:  
- Focus on **performance optimization** (bundling, lazy loading, CDNs).  
- Practice **debugging tools** (Chrome DevTools, Lighthouse).  
- Understand **security** (XSS, CSP) and **scalability** (micro frontends).