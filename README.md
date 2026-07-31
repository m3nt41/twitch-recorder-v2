# 🎬 Twitch Recorder

Una aplicación de escritorio ligera y moderna para grabar streams de Twitch en Windows.

## ✨ Características

- **Cero dependencias**: No necesitas instalar Python, Streamlink ni FFmpeg. Todo viene empaquetado.
- **Instalación sencilla**: Solo descarga el instalador `.exe` y listo.
- **Liberación inmediata**: Al detener la grabación, el archivo `.mp4` se libera al instante.
- **Rendimiento óptimo**: Hecha con Tauri v2, React y Rust.

## 🚀 Instalación

1. Ve a la sección de **[Releases](https://github.com/m3nt41/twitch-recorder-v2/releases)** de este repositorio.
2. Descarga el archivo `twitch-recorder_0.1.0_x64-setup.exe`.
3. Ejecuta el instalador y sigue los pasos.

## 💻 Uso

1. Abre la aplicación.
2. Escribe el nombre del canal de Twitch que está **en vivo**.
3. Haz clic en **"Grabar"**.
4. Cuando quieras terminar, haz clic en **"Detener"**.
5. El video se guardará en la carpeta `grabaciones` junto al ejecutable.

## 🛠️ Desarrollo

```bash
git clone https://github.com/m3nt41/twitch-recorder-v2.git
cd twitch-recorder-v2
pnpm install
pnpm tauri dev
pnpm tauri build

📄 Licencia
MIT
