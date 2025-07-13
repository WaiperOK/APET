# APET - Adversarial Prompt Engineering Toolkit

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

A Rust-based toolkit for adversarial prompt engineering research using MAP-Elites evolutionary algorithm. APET helps researchers test AI safety by generating and evaluating adversarial prompts through systematic exploration of attack strategies.

## 🚀 Features

- **MAP-Elites Algorithm**: Evolutionary algorithm for generating diverse adversarial prompts
- **Dual Interface**: Both CLI and GUI modes for different use cases
- **Ollama Integration**: Real-time AI model interaction for prompt generation
- **Behavioral Classification**: Automatically categorizes prompts by technique and complexity
- **Statistical Analysis**: Comprehensive fitness tracking and diversity metrics
- **Multi-language Support**: English and Russian interface localization
- **Export Functionality**: JSON export of results for further analysis
- **Visualization**: Grid-based visualization of MAP-Elites archive

## 📋 Prerequisites

- **Rust**: 1.70+ (install from [rustup.rs](https://rustup.rs/))
- **Ollama**: Running locally on port 11434
- **AI Model**: Any model compatible with Ollama (e.g., `llama3.2:latest`)

## 🛠️ Installation

1. **Clone the repository**:
```bash
git clone https://github.com/WaiperOK/APET.git
cd APET/gca
```

2. **Install dependencies**:
```bash
cargo build --release
```

3. **Set up Ollama**:
```bash
# Install Ollama (if not already installed)
curl -fsSL https://ollama.com/install.sh | sh

# Start Ollama service
ollama serve

# Pull a model (in another terminal)
ollama pull llama3.2:latest
```

## 🎯 Usage

### CLI Mode
```bash
cargo run cli
```

The CLI mode provides:
- Real-time MAP-Elites evolution
- Progress tracking with statistics
- Automatic result export to JSON
- DOT graph generation for visualization

### GUI Mode
```bash
cargo run
```

The GUI mode offers:
- User-friendly interface with tabs (Dashboard, Generator, Results, Settings)
- Real-time progress visualization
- Interactive grid display
- Statistical graphs and metrics
- Language switching (English/Russian)
- Font customization

## 📊 MAP-Elites Algorithm

APET uses MAP-Elites to explore the space of adversarial prompts across two dimensions:

### Behavioral Dimensions
- **Technique** (5 categories): Roleplay, System, Bypass, Admin, General
- **Complexity** (4 levels): Simple, Moderate, Complex, Advanced

### Fitness Evaluation
Prompts are evaluated based on:
- Keyword analysis for adversarial indicators
- Complexity scoring
- Effectiveness potential
- Diversity metrics

## 🔧 Configuration

### Settings (GUI Mode)
- **Generations**: Number of evolution cycles (default: 10)
- **Population Size**: Individuals per generation (default: 20)
- **Mutation Rate**: Probability of mutation (default: 0.7)
- **Grid Size**: MAP-Elites archive dimensions (default: 5x4)

### Model Configuration
- **Ollama URL**: Default `http://localhost:11434`
- **Model Selection**: Choose from available Ollama models
- **Target System**: Define the system to test against

## 📈 Output Files

### JSON Export
- **CLI**: `apet_mapelites_results.json`
- **GUI**: `apet_gui_real_results.json`

Contains:
- Complete MAP-Elites grid with all elites
- Generation statistics (fitness, coverage, diversity)
- Best performing prompts
- Evolution parameters and metadata

### Visualization
- **DOT Graph**: `attack.png.dot` (CLI mode)
- **Grid Display**: Real-time visualization in GUI

## 🧪 Research Applications

APET is designed for:
- **AI Safety Research**: Testing model robustness against adversarial inputs
- **Red Team Exercises**: Systematic exploration of attack vectors
- **Prompt Engineering**: Understanding effective prompt structures
- **Security Assessment**: Evaluating AI system vulnerabilities

## 🔒 Ethical Considerations

This tool is intended for:
- ✅ Academic research
- ✅ Authorized security testing
- ✅ AI safety improvement
- ✅ Educational purposes

**Not intended for**:
- ❌ Malicious attacks
- ❌ Unauthorized system access
- ❌ Harmful content generation
- ❌ Illegal activities

## 📁 Project Structure

```
APET/
├── gca/
│   ├── src/
│   │   ├── main.rs         # CLI implementation
│   │   ├── gui.rs          # GUI implementation
│   │   ├── ai/
│   │   │   └── mod.rs      # Ollama integration
│   │   ├── adversarial.rs  # Adversarial prompt logic
│   │   ├── map_elites.rs   # MAP-Elites algorithm
│   │   ├── graph/          # Visualization utilities
│   │   ├── math/           # Mathematical functions
│   │   └── scanner/        # Analysis tools
│   ├── Cargo.toml          # Dependencies
│   └── README.md           # This file
```

## 🔄 Development Workflow

1. **Make Changes**: Edit source code in `src/`
2. **Test**: Run `cargo test` for unit tests
3. **Build**: Use `cargo build --release` for optimized builds
4. **Run**: Execute with `cargo run` (GUI) or `cargo run cli`

## 📝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📋 Dependencies

Key dependencies include:
- `reqwest`: HTTP client for Ollama API
- `tokio`: Async runtime
- `serde`: JSON serialization
- `eframe/egui`: GUI framework
- `rand`: Random number generation
- `anyhow`: Error handling

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Support

For issues, questions, or contributions:
- 📧 Open an issue on GitHub
- 💬 Join discussions in the repository
- 🐛 Report bugs with detailed reproduction steps

## 🎓 Citation

If you use APET in your research, please cite:

```bibtex
@software{apet2024,
  title={APET: Adversarial Prompt Engineering Toolkit},
  author={WaiperOK},
  year={2024},
  url={https://github.com/WaiperOK/APET}
}
```

---

**⚠️ Disclaimer**: This tool is for research and educational purposes only. Users are responsible for ensuring ethical and legal use of the software. 