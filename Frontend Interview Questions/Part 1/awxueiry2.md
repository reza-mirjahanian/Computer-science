# **Best Practices for Security in Front-End Development**



---

## **1. Protecting Against XSS (Cross-Site Scripting)**

### **Tips to Prevent XSS**
- **Sanitize User Input**: Clean any input received from users before rendering it in the DOM.
- **Escape Output**: For example, escape characters like `<`, `>`, and `&` before rendering them.
- **Set Content Security Policy (CSP)**: Configure CSP headers to block inline scripts or restrict script sources.
- **Avoid `innerHTML` or `document.write`**: Use safer APIs like `textContent` or explicit DOM manipulation.
- **Framework-Specific Tools**: Use templating libraries like React, Angular, and Vue to avoid manual DOM manipulations.

### **Code Examples**

#### **Unsafe Example** (Vulnerable to XSS):
```javascript
const userInput = "<script>alert('Hacked!')</script>";
document.getElementById("output").innerHTML = userInput; // Dangerous
```

#### **Safe Example** (Avoid XSS):
```javascript
const userInput = "<script>alert('Hacked!')</script>";
const escapedInput = userInput.replace(/</g, "&lt;").replace(/>/g, "&gt;");
document.getElementById("output").textContent = escapedInput; // Safe
```

#### **Using CSP Headers**
```html
<meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'">
```

### **Key Comparison**
| **Unsafe**                           | **Safe**                            |
|--------------------------------------|-------------------------------------|
| Using `innerHTML`                    | Use `textContent` or sanitized output |
| Allowing inline `<script>` tags      | Enforcing CSP policies              |

---

## **2. Preventing CSRF (Cross-Site Request Forgery)**

### **Tips for CSRF Prevention**
- **Use CSRF Tokens**: Always send CSRF tokens along with requests (especially POST, PUT, DELETE).
- **Validate Referer Headers**: Check HTTP headers to ensure requests originate from trusted domains.
- **Same-Site Cookies**: Use `SameSite` attribute in cookies to limit cross-origin requests.

### **Code Example**

#### **Setting CSRF Tokens**
```javascript
// Example with Axios
axios.defaults.headers.common['X-CSRF-TOKEN'] = 'your-csrf-token';

// Verify CSRF Tokens on server
fetch('/api/endpoint', {
  method: 'POST',
  headers: {
    'X-CSRF-TOKEN': csrfToken, // Custom header
  }
});
```

#### **Using SameSite Cookies**
```javascript
document.cookie = "session=abCdef12345; SameSite=Strict";
```

---

## **3. Secure Authentication Handling**

### **Tips for Secure Authentication**
- **Secure Storage**: Store sensitive tokens in `Secure` and `HttpOnly` cookies (NOT localStorage/sessionStorage).
- **Use HTTPS**: Remove plain HTTP completely for asset delivery.
- **JWT Best Practices**:
  - **Short Expiry**: Set short expiration for tokens.
  - **Encode and Sign**: Always sign JWT tokens with strong algorithms (e.g., HS256, RS512).
- **Two-Factor Authentication (2FA)**: Add an extra layer like OTP or hardware-based security keys.

### **Code Example**

#### **Storing Tokens Securely**
```javascript
// Set HttpOnly cookie from server-side
res.cookie('authToken', token, {
  httpOnly: true,
  secure: true,
  sameSite: 'Strict',
});
```

#### **JWT Token Example**
```javascript
// Generate token securely
const jwt = require('jsonwebtoken');
const token = jwt.sign({ userId: '123' }, process.env.SECRET_KEY, { algorithm: 'HS256', expiresIn: '1h' });
```

---

## **4. Avoiding Vulnerable Dependency Injection**

### **Tips**
- **Analyze Dependencies**: Regularly scan installed packages for security vulnerabilities.
- **Lock Versions**: Use tools like `package-lock.json` or `yarn.lock` to avoid unexpected updates.
- **Monitor Libraries**: Replace or patch libraries actively flagged for vulnerabilities.

### **Commands for Dependency Scanning**
```bash
# Scan for vulnerabilities
npm audit
yarn audit
```

---

## **5. Mitigating Clickjacking**

### **Tips**
- **Use X-Frame-Options Header**: Restrict embedding in `<iframe>` or `<object>` tags.
- **Content Security Policy**: Disable `frame-src` entirely unless needed.
- **Avoid Inline Frames**: Ensure critical pages are not embeddable.

### **Code Example**
```html
<meta http-equiv="X-Frame-Options" content="DENY">
```

---

## **6. Secure File Uploads**

### **Tips for File Upload Security**
- **Validate File Format**: Restrict file extensions. Example: `image/png`, not `.exe`.
- **Max Size Enforcement**: Set limits on maximum file size.
- **Sanitize File Names**: Avoid special characters and relative paths.
- **Avoid Direct Upload Serving**: Store files securely on the back end.
- **Set CORS Configuration**: Properly configure CORS for uploads.

### **Code Example (Edge Case Handling)**
```javascript
const validateFile = (file) => {
  const allowedExtensions = ['jpg', 'png', 'pdf'];
  const maxSize = 5 * 1024 * 1024; // 5MB
  const fileExtension = file.name.split('.').pop().toLowerCase();

  if (!allowedExtensions.includes(fileExtension)) {
    throw new Error('Invalid file type!');
  }
  if (file.size > maxSize) {
    throw new Error('File size too large!');
  }
};

document.getElementById('uploadBtn').addEventListener('change', (e) => {
  try {
    const file = e.target.files[0];
    validateFile(file);
    console.log('File is safe to upload');
  } catch (error) {
    alert(error.message);
  }
});
```

---

## **7. Secure API Calls**

### **Tips**
- **Validate Input**: Always sanitize input going to your API.
- **Use Rate Limiting**: Enforce rate limits to prevent API misuse.
- **Encrypt Data in Transit**: Ensure secure connections with HTTPS/TLS.
- **Disable CORS Where Not Needed**: Only whitelist domains you control.
- **Avoid API Keys in Front-End Code**: Store keys in the backend and use some form of proxy.

---

## **8. Preventing Sensitive Data Exposure**

### **Tips**
- **Avoid Hardcoding Secrets**: Never hardcode sensitive credentials in front-end code or `.env` files.
- **Environment Variables**: Use public keys or IDs only. Keep private keys in backend services.
- **Obfuscation**: Minify and bundle your front-end code to reduce readability.

---

## **9. Secure Error Handling**

### **Tips**
- **Minimize Error Details**: Ensure the front-end doesn't expose stack traces or sensitive debug data.
- **Generic Error Messages**: Replace specific error messages with generalized ones (e.g., `Invalid credentials` rather than `Incorrect password`).

#### **Code Example**
```javascript
try {
  // Some operation
} catch (error) {
  console.log('Something went wrong!'); // Avoid showing technical messages
}
```

---

## **10. Handle CORS Appropriately**

### **Tips**
- **Whitelist Trusted Domains**: Restrict CORS access to trusted domains and use short-lived preflight cache headers.
- **Block Wildcards (`*`)**: Ensure specific domains are given access instead of using a blind `*`.

### **Code Example**
```javascript
fetch('https://secure-api.com', {
  method: 'GET',
  mode: 'cors',
  credentials: 'include', // Ensure cookies are sent securely
});
```

---

### **Edge Cases**
1. **Attack via SVG Files**
   - SVG can execute JavaScript code when improperly sanitized.
   - **Solution:** Strip `<script>` and `<link>` tags from uploaded SVG files.

2. **Data Leakage via Error Logs in Debug Mode**
   - In production, disable verbose stack traces or expose API keys.
   - Use a `silent` error logger or on-the-fly obfuscation.

---

## **Summary of Front-End Security Principles**

| **Security Measure**          | **Tools/Practices**                          | **Purpose**                     |  
|-------------------------------|---------------------------------------------|---------------------------------|  
| **Avoid XSS**                 | Escape inputs, CSP headers                 | Prevent cross-scripting         |  
| **Limit CSRF**                | CSRF tokens, SameSite cookies              | Prevent unauthorized requests   |  
| **Secure APIs**               | CORS, token validation                     | Data protection                 |  
| **Use HTTPS**                 | Enforce TLS encryption                     | Secure data in transit          |  
| **Clickjacking Defense**      | X-Frame-Options, CSP                       | Disable iframe exploitation     |  