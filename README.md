## **Rust TCP Client**

This is a simple Rust-based TCP client that connects to a specified host and port, sends an HTTP GET request, and prints the response from the server. 

### **🛠 Features**
- Resolves the domain name to an IPv4 address.
- Establishes a TCP connection with the target server.
- Sends a predefined HTTP request.
- Reads and prints the server’s response.

---

## **🚀 Usage**
### **1️⃣ Build & Run**
```sh
cargo build --release
./target/release/tcp_client <host> <port>
```

### **2️⃣ Example**
```sh
./tcp_client www.google.com 80
```
**Output:**
```
host: www.google.com, port: 80
Resolved addresses: 142.251.33.174:80
Connected to server
HTTP/1.1 200 OK
...
```

---

## **📌 How It Works**
1. **Parses command-line arguments**: The program takes a `<host>` and `<port>` as input.
2. **Validates port input**: Ensures the port is a valid `u16` number.
3. **Resolves the domain**: Converts the domain name into an IPv4 address.
4. **Establishes a TCP connection**: Connects to the resolved IP and port.
5. **Sends an HTTP request**: Sends a hardcoded `"GET /"` request.
6. **Reads the server response**: Reads and prints the response.

---

## **⚠️ To-Do: Dynamic HTTP Request Handling**
### **🔧 Current Limitation**
Currently, the program **always sends the same predefined request**:
```rust
let request = "GET / HTTP/1.1\r\nHost: www.google.com\r\nConnection: close\r\n\r\n";
```
This means:
- The **requested URL path (`/`) is fixed**.
- The **HTTP method (GET) is not configurable**.
- The **Host header is hardcoded**.

### **✅ Next Steps**
Modify the code to allow **custom request input** from the command line:
```sh
./tcp_client <host> <port> "<HTTP Request>"
```
For example:
```sh
./tcp_client www.google.com 80 "GET /search?q=rust HTTP/1.1\r\nHost: www.google.com\r\nConnection: close\r\n\r\n"
```
This will:
- Accept user-defined HTTP requests.
- Allow sending **POST**, **PUT**, **DELETE**, etc.
- Enable **dynamic query parameters**.

---

## **📜 License**
This project is open-source and available for educational purposes.
