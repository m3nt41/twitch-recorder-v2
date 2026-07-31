# 🎬 Twitch Recorder

Una aplicación de escritorio simple y ligera para grabar streams en vivo de Twitch directamente en tu PC.

##  Características

- **Grabación en alta calidad:** Utiliza `streamlink` y `ffmpeg` para capturar la mejor calidad disponible.
- **Sin ventanas molestas:** Todo el proceso ocurre en segundo plano sin abrir terminales.
- **Archivos únicos:** Cada grabación se guarda con una marca de tiempo para que nunca se sobrescriban.
- **Organización automática:** Los vídeos se guardan automáticamente en una carpeta `grabaciones`.
- **Ligero y rápido:** Construido con Tauri (Rust + React), consume muy pocos recursos.

## 🛠️ Tecnologías utilizadas

- **Frontend:** React, TypeScript, Vite
- **Backend:** Rust, Tauri
- **Herramientas externas:** Streamlink, FFmpeg

##  Instalación

### Opción 1: Usar el instalador (Windows)
1. Ve a la sección de [Releases](https://github.com/m3nt41/twitch-recorder/releases) de este repositorio.
2. Descarga el archivo `.exe` más reciente.
3. Ejecútalo y sigue los pasos del instalador.

### Opción 2: Compilar desde el código fuente
Si tienes Rust, Node.js y pnpm instalados:

1. Clonar el repositorio
2. cd twitch-recorder
3. pnpm install
4. pnpm tauri dev (para desarrollo)
5. pnpm tauri build (para producción)

## 🎮 Cómo usar

1. Abre la aplicación **Twitch Recorder**.
2. Escribe el nombre del canal de Twitch que quieres grabar (ej: `auronplay`, `ibai`).
3. Haz clic en el botón **🔴 Grabar**.
4. Cuando quieras terminar, haz clic en **️ Parar**.
5. ¡Listo! Tu vídeo estará guardado en la carpeta `grabaciones`.

## 📝 Requisitos previos

Para que la aplicación funcione, es necesario tener instalados en el sistema:
- [Streamlink](https://streamlink.github.io/)
- [FFmpeg](https://ffmpeg.org/)

## 📄 Licencia

Este proyecto está bajo la Licencia MIT.