# Local AI Coder DevPod workspace (C++ / llama.cpp / NVIDIA CUDA)

This is a **development-container starter**, not an installer that changes Ubuntu's CUDA or NVIDIA driver setup.

## Design

- **Ubuntu host:** NVIDIA driver, Docker Engine, NVIDIA Container Toolkit, DevPod, and your editor.
- **Container:** CUDA 12.8 development user-space, C/C++ toolchain, CMake/Ninja, debugger, clangd, Python, Git, and llama.cpp source/build.
- **Workspace:** source files persist in the DevPod workspace; container rebuilds should not erase workspace files. Model weights are ignored by Git.

The container uses `nvidia/cuda:12.8.1-devel-ubuntu24.04`. It does not install or replace the host NVIDIA driver.

## First: inspect the host before installing anything

Run these read-only checks in Ubuntu:

```bash
lsb_release -ds
nvidia-smi
docker --version 2>/dev/null || true
docker info 2>/dev/null | sed -n '1,35p' || true
dpkg -l | grep -E '(^ii  (docker|containerd|nvidia-container-toolkit|libnvidia-container))' || true
df -h /
```

If `nvidia-smi` fails on the **host**, fix that before configuring containers. Do not install a CUDA toolkit or use NVIDIA's `.run` driver installer on the host for this project.

## Host setup (only install components that are missing)

### 1. Docker

Use Docker's official Ubuntu installation instructions: https://docs.docker.com/engine/install/ubuntu/

Ubuntu 22.04 can run Docker Engine. Before following any package-removal instructions, inspect the read-only checks above. **Do not blindly remove `docker.io`, `containerd`, or other packages** if you already use Docker or another container setup. Back up important containers/volumes and confirm the exact package plan before changing an existing installation.

For a clean system without an existing container engine, install Docker from the official repository using the current documentation. If Docker is already working, do not reinstall it just to follow this guide.

DevPod needs access to the Docker daemon. Adding your account to the `docker` group is convenient, but members of that group effectively have root-level control of the machine. Use it only if that trade-off is acceptable; otherwise keep using an appropriately secured Docker access method.

### 2. NVIDIA Container Toolkit

Follow NVIDIA's current Ubuntu instructions: https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/install-guide.html

Before configuring Docker, preserve any existing daemon config:

```bash
if [ -f /etc/docker/daemon.json ]; then
  sudo cp -a /etc/docker/daemon.json "/etc/docker/daemon.json.backup.$(date +%Y%m%d-%H%M%S)"
fi
```

After installing `nvidia-container-toolkit`, configure Docker and restart it using NVIDIA's documented commands:

```bash
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker
```

Restarting Docker can interrupt running containers. Review the daemon config before restarting if you already run workloads. This changes Docker's configuration, not the host CUDA toolkit or the NVIDIA kernel driver.

### 3. Test Docker's GPU access before DevPod

Use a CUDA container to test the bridge from Docker to the host driver:

```bash
sudo docker run --rm --gpus all nvidia/cuda:12.8.1-base-ubuntu22.04 nvidia-smi
```

Proceed only if it shows your NVIDIA GPU. If it fails, stop here and troubleshoot the NVIDIA Container Toolkit rather than changing your host CUDA installation.

### 4. Install DevPod CLI (x86-64 Linux)

The command below installs DevPod into your own `~/.local/bin` directory rather than `/usr/local/bin`:

```bash
mkdir -p "$HOME/.local/bin"
curl -fL https://github.com/loft-sh/devpod/releases/latest/download/devpod-linux-amd64 \
  -o "$HOME/.local/bin/devpod"
chmod 0755 "$HOME/.local/bin/devpod"
export PATH="$HOME/.local/bin:$PATH"
```

If that works, add `export PATH="$HOME/.local/bin:$PATH"` to `~/.bashrc` if it is not already there. Official install guide: https://devpod.sh/docs/getting-started/install

Register the Docker provider:

```bash
devpod provider add docker
```

## Start this workspace

Extract this folder somewhere in your home directory, open a terminal inside it, and run:

```bash
devpod up . --ide vscode
```

This uses VS Code on the host if it is installed. You can also open the folder with DevPod Desktop and select the Docker provider and your preferred IDE. The `.devcontainer/devcontainer.json` requests `--gpus=all` for the workspace.

Inside the workspace, confirm that the GPU is visible:

```bash
nvidia-smi
```

## Build llama.cpp with CUDA inside the container

From the workspace root, inside the container:

```bash
./scripts/build-llama-cpp.sh
./scripts/test-gpu.sh
```

Download a compatible **GGUF** model into `models/` (check the model's licence and hardware requirements). Then run a short test, replacing the path with the actual filename:

```bash
./build/llama.cpp/bin/llama-cli \
  -m models/your-coder-model.gguf \
  -p 'Write a C++ function that reverses a string.' \
  -n 128 -ngl 99
```

## Important safety / maintenance notes

- Never install `cuda-toolkit`, NVIDIA drivers, or NVIDIA `.run` installers on the host for this setup. CUDA **development libraries** belong in the container; the host only needs a compatible NVIDIA driver and NVIDIA Container Toolkit.
- The container and its image can be rebuilt independently of your workspace source. Keep important work in Git and back up any local changes.
- Do not commit model weights; the `.gitignore` excludes `models/`.
- A container is isolation, not a perfect security boundary. Do not run untrusted GitHub build scripts with privileged container flags or mount your whole home directory into a container.
- GPU visibility does not guarantee a model fits in VRAM. Choose model size and quantization based on the GPU's available VRAM.
