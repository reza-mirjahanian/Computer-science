

# Best Practices for Security in Front-End Development

## 1. **Content Security Policy (CSP)**

- **What is CSP?**: Content Security Policy is a browser security feature that helps to detect and mitigate certain types of attacks, including Cross Site Scripting (XSS) and data injection attacks.
- **Best Practices:**
  - **Define a strong policy** by specifying allowed sources of content.
  - **Set `script-src` and `style-src` directives** to avoid inline scripts and styles.

Example of a CSP header:
```html
Content-Security-Policy: default-src 'self'; script-src 'self' https://apis.example.com; style-src 'self' 'unsafe-inline';
```

- **Why?**: It helps to prevent unauthorized scripts from running and ensures that only trusted sources can serve content.

---

## 2. **Cross-Origin Resource Sharing (CORS)**

- **What is CORS?**: CORS is a security feature that restricts how resources on a web page can be requested from another domain.
- **Best Practices:**
  - **Limit the allowed origins**: Avoid using `*` (wildcard) for allowing all domains.
  - **Use specific HTTP methods**: Define which HTTP methods are permitted (e.g., `GET`, `POST`, `PUT`).
  - **Enable CORS only for APIs** that need cross-origin access.

Example of CORS configuration in server (Node.js with Express):
```javascript
const cors = require('cors');
app.use(cors({
  origin: 'https://trusted-domain.com',
  methods: ['GET', 'POST'],
}));
```

---

## 3. **Avoid Inline JavaScript and CSS**

- **What’s the risk?**: Inline scripts and styles can lead to XSS vulnerabilities because malicious users may inject arbitrary scripts into your application.
- **Best Practices**:
  - Use **external scripts and styles**.
  - **Avoid `eval()`, `setTimeout()` with strings**, and other eval-like methods.
  - Use **CSP** with a `script-src` directive that blocks inline scripts.

Example:
```html
<!-- Avoid inline JavaScript -->
<script src="external.js"></script>
```

---

## 4. **Input Validation and Sanitization**

- **What’s the risk?**: Unsanitized inputs can be exploited by attackers to inject malicious code (e.g., SQL injection, XSS).
- **Best Practices**:
  - **Validate inputs on both client-side and server-side** (never trust client-side validation alone).
  - **Use whitelisting** instead of blacklisting for input validation.
  - **Sanitize HTML** using libraries like DOMPurify to prevent XSS.

Example of input validation:
```javascript
// Simple example for email validation
function validateEmail(email) {
  const regex = /^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6}$/;
  return regex.test(email);
}
```

Example of sanitizing input:
```javascript
// Using DOMPurify for sanitizing HTML
const cleanHTML = DOMPurify.sanitize('<div><script>alert("XSS")</script></div>');
console.log(cleanHTML); // <div></div>
```

---

## 5. **Cross-Site Scripting (XSS) Prevention**

- **What is XSS?**: XSS is a vulnerability that allows attackers to inject malicious scripts into web pages.
- **Best Practices**:
  - **Escape user input** when displaying it in HTML, JavaScript, or CSS contexts.
  - Use **CSP** to mitigate the risk of XSS attacks.
  - **Use frameworks** like React or Angular that automatically escape user input.
  - **Avoid `innerHTML`** for inserting untrusted content.

Example of escaping user input:
```javascript
// Example of escaping HTML content
function escapeHTML(str) {
  return str.replace(/[&<>"']/g, function(match) {
    return '&#' + match.charCodeAt(0) + ';';
  });
}
```

---

## 6. **Avoid Storing Sensitive Data in LocalStorage/SessionStorage**

- **What’s the risk?**: LocalStorage and SessionStorage are vulnerable to XSS attacks. Any malicious script can access the data stored in these mechanisms.
- **Best Practices**:
  - **Avoid storing sensitive information** such as JWT tokens, passwords, etc., in storage.
  - Use **Secure HTTP-only cookies** instead of LocalStorage for storing sensitive information.

Example:
```javascript
// Do NOT store sensitive data like tokens in localStorage
localStorage.setItem('jwt', 'sensitive-token');  // Avoid this
```

---

## 7. **Secure Authentication and Authorization**

- **What is Authentication?**: The process of verifying who a user is.
- **What is Authorization?**: The process of determining what an authenticated user is allowed to do.

- **Best Practices**:
  - **Use HTTPS** for all authentication requests.
  - Use **JWT** for secure stateless authentication. Ensure the JWT is signed and encrypted.
  - Implement **role-based access control (RBAC)** to restrict access to resources.
  - **Use multi-factor authentication (MFA)** wherever possible.

Example of JWT in a front-end:
```javascript
// Storing JWT in Secure HTTP-only Cookie (Preferred method)
document.cookie = 'jwt=your-jwt-token; Secure; HttpOnly; SameSite=Strict';
```

---

## 8. **Clickjacking Protection**

- **What is Clickjacking?**: Clickjacking tricks the user into clicking something different from what the user thinks they are clicking, by overlaying invisible frames over legitimate content.
- **Best Practices**:
  - Use the **`X-Frame-Options`** header or **`Content-Security-Policy: frame-ancestors`** directive to prevent your website from being embedded in a frame.
  
Example:
```html
<meta http-equiv="X-Frame-Options" content="DENY">
```

---

## 9. **HTTPS Everywhere**

- **What is HTTPS?**: HTTPS encrypts communication between the browser and the server, ensuring that data is transmitted securely.
- **Best Practices**:
  - Enforce **HTTPS** by using HTTP Strict Transport Security (HSTS).
  - Redirect all HTTP traffic to HTTPS.

Example of HSTS header:
```html
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
```

---

## 10. **Avoid Using `eval()` and Similar Functions**

- **What’s the risk?**: `eval()` and similar methods (`setTimeout()`, `setInterval()`) can execute arbitrary code, making your application vulnerable to code injection.
- **Best Practices**:
  - **Avoid using `eval()`** for dynamic code execution.
  - Use safer alternatives like `JSON.parse()` and `Function` constructors.

Example of avoiding `eval()`:
```javascript
// Instead of using eval():
const userInput = "2 + 2";
console.log(eval(userInput)); // Avoid this

// Use a safer approach
const result = Function('"use strict"; return (' + userInput + ')')();
console.log(result);
```

---

## 11. **Server-Side Security Practices**

While front-end security is crucial, **server-side** security must also be considered:
- **Never trust the client**: Always validate inputs and authenticate users on the server.
- **Use secure libraries and frameworks**: Ensure that you are using up-to-date, secure libraries.

---

## 12. **Third-Party Script Security**

- **What’s the risk?**: Including third-party libraries or scripts can introduce vulnerabilities if they are not secure.
- **Best Practices**:
  - **Subresource Integrity (SRI)**: Use SRI to ensure that a file has not been tampered with.
  - Regularly **audit third-party dependencies** for security vulnerabilities.

Example of SRI:
```html
<script src="https://cdn.example.com/library.js" integrity="sha384-abc123" crossorigin="anonymous"></script>
```

---

## 13. **Use Strong Session Management**

- **Best Practices**:
  - **Use secure cookies**: Always set the `Secure` and `HttpOnly` flags for cookies.
  - Set **Session Expiry** to limit the lifespan of sessions.
  - Implement **token-based authentication** to avoid session hijacking.

Example of setting a secure cookie:
```javascript
document.cookie = "session_id=abcd1234; Secure; HttpOnly; SameSite=Strict";
```

---

## 14. **Security Headers**

- **Important Security Headers**:
  - **Strict-Transport-Security (HSTS)**
  - **X-Frame-Options**
  - **X-Content-Type-Options**: Prevents browsers from interpreting files as a different MIME type.
  - **X-XSS-Protection**: Enables XSS filtering.

Example of setting security headers in a Node.js app:
```javascript
app.use(function(req, res, next) {
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('X-XSS-Protection', '1; mode=block');
  next();
});
```

---

## 15. **Regular Security Audits and Penetration Testing**

- Regularly audit your codebase for security vulnerabilities.
- Perform **penetration testing** to identify weak spots in your application.

