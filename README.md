# PyWeb Protocol 170 – Rust Package Build

Dieses Repository enthält native Python-Erweiterungen in Rust, verpackt via [maturin](https://github.com/PyO3/maturin).  
Dieser Guide zeigt, wie man unter **Windows mit WSL (Ubuntu)** ein **Linux-kompatibles `.whl`** erstellst.

---

## 🔧 Voraussetzungen (via Linux WSL)

- Windows mit WSL2 (Ubuntu)
- GitHub Personal Access Token (für privaten Clone)
- Python 3.12
- Rust (`cargo`)
- `maturin`

---

## 🧱 Setup-Schritte (einmalig)

# Installatiomn
## 1. WSL Setup (nur einmal nötig)
wsl --install Ubuntu

## === Im Ubuntu-Terminal ===
sudo apt update
sudo apt install -y curl build-essential python3.12-venv git

## 2. Rust installieren
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
cargo --version  # check

## 3. Reboot ist nicht zwingend nötig, wenn source gemacht
### sudo reboot

## 4. Projekt klonen
git clone https://<GHP-TOKEN>@github.com/Vive-bit/PyWeb.git
cd PyWeb
git switch packages/rs/proto170

## 5. Virtuelle Umgebung vorbereiten
python3.12 -m venv .venv
source .venv/bin/activate
pip install --upgrade pip
pip install maturin

## 6. Paket bauen
maturin build --release --manylinux 2014

## 7. Datei ins dist-Verzeichnis
# mkdir -p dist
cp target/wheels/*.whl dist/

## 9. Commit für das Wheel
git add dist/*.whl .gitignore
git commit -m "release: linux wheel build"
git push origin packages/rs/proto170
