# remoppt

📱 **remoppt** lets you use your mobile phone to remotely control your computer’s PowerPoint slides over a local network — without any extra software or pairing.

Ideal for presentations, lectures, and talks where you want to move freely while using your phone to flip slides.

---

## 📦 Installation

Clone the repository:

```bash
git clone https://github.com/wjwei-handsome/remoppt.git
cd remoppt
cargo run
```

or just:

```bash
cargo install --git https://github.com/wjwei-handsome/remoppt.git
```

---

## 🚀 Usage

### 💻 On Your Computer

Run the program:

```bash
remoppt
```

It will show a QR code and a URL. You can use either to connect your phone.

### 📱 On Your Phone

Scan the QR code or open the URL in your browser. You’ll see two large buttons:
	•	⬅️ Previous slide (← key)
	•	➡️ Next slide (→ key)

Just tap the buttons to control your slides.

---

⚠️ Security Notice

This tool does not implement authentication or HTTPS. It’s intended for trusted LAN environments only.
Do not expose this to the public internet.

---

📄 License

MIT License

---

Made with 🦀 and ❤️ by wenjiewei
