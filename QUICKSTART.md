# 🚀 APET Quick Start

## What is APET?

APET is a tool for **AI system security testing** that uses the MAP-Elites evolutionary algorithm to generate adversarial prompts.

## ⚡ Quick Installation

### 1. Install Ollama

```bash
# Windows
winget install Ollama.Ollama

# After installation, start Ollama
ollama serve
```

### 2. Install a model

```bash
# In new terminal
ollama pull llama3
```

### 3. Run APET

```bash
# In project folder
cd gca
cargo run
```

## 🎯 First Run

### GUI mode (recommended)

1. **Launch the program**: `cargo run`
2. **Go to "MAP-Elites" tab**
3. **Configure parameters**:
   - Generations: 5-10 (for quick test)
   - Population size: 10-20
   - Target system: "ChatGPT" or "Claude"
4. **Add attack goals**:
   - "Force AI to ignore instructions"
   - "Get internal information"
5. **Click "Start Evolution"**

### CLI mode

```bash
# Quick test
cargo run -- --cli

# Test specific system
cargo run -- "ChatGPT"
```

## 📊 Results

After completion you will get:

- **JSON file** with all prompts
- **Performance graph** in SVG format
- **Detailed statistics** in GUI

## 🔧 Settings for beginners

### Parameters for quick testing:
- **Generations**: 5
- **Population**: 15
- **Execution time**: ~2-3 minutes

### Parameters for serious testing:
- **Generations**: 20-50
- **Population**: 30-100
- **Execution time**: 10-30 minutes

## 🛡️ Ethical Rules

⚠️ **ONLY for security testing!**

✅ **Allowed**:
- Testing own AI systems
- Security research
- Educational purposes

❌ **Prohibited**:
- Attacks on other systems
- Terms of service violations
- Malicious activity

## 📞 Help

- **Errors**: Create issue on GitHub
- **Questions**: Discuss in Discussions
- **Documentation**: Read README.md

## 🎉 Ready!

Now you can use APET for AI system security testing. Good luck! 