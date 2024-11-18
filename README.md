
# 🌌 PROYECTO3_SPACE: Simulación del Sistema Solar con Shaders

¡Bienvenido a **PROYECTO3_SPACE**! Este proyecto simula un sistema solar con planetas renderizados mediante shaders personalizados, utilizando Rust y gráficos 3D en una implementación desde cero. Aquí puedes explorar un universo virtual con texturas, iluminación y modelos 3D de planetas, todo optimizado para rendimiento.

## 🖥️ Características

- **Sistema solar en 3D**:
  - Planetas con órbitas ajustables.
  - Shaders personalizados para cada planeta (sol, tierra, marte, etc.).
  - Modelos de planetas con niveles de detalle (LOD).
- **Controles interactivos**:
  - Navega por el sistema solar con el teclado.
  - Zoom dinámico con la cámara para explorar planetas.

## 📜 Requisitos del sistema

- **Rust**: Instala la última versión desde [Rust Lang](https://www.rust-lang.org/).
- **Bibliotecas utilizadas**:
  - `minifb`: Para la creación de la ventana gráfica.
  - `nalgebra_glm`: Para cálculos matemáticos de gráficos 3D.
  - `rayon`: Para paralelización de tareas.
  - `fastnoise_lite`: Para generar ruido procedural.

## 🔧 Instalación

1. Clona este repositorio:

   ```bash
   git clone https://github.com/tu-usuario/PROYECTO3_SPACE.git
   cd PROYECTO3_SPACE
   ```

2. Compila el proyecto en modo release para optimizar el rendimiento:

   ```bash
   cargo build --release
   ```

3. Ejecuta el programa:

   ```bash
   cargo run --release
   ```

## 🕹️ Controles

- **Movimiento de la cámara**:
  - `W`: Avanzar.
  - `S`: Retroceder.
  - `A`: Mover a la izquierda.
  - `D`: Mover a la derecha.
  - `Espacio`: Subir.
  - `Shift`: Bajar.
- **Rotación de la cámara**:
  - Flecha izquierda: Rotar a la izquierda.
  - Flecha derecha: Rotar a la derecha.
  - Flecha arriba: Rotar hacia arriba.
  - Flecha abajo: Rotar hacia abajo.

## 🌍 Cómo funciona

El programa utiliza un pipeline de renderizado personalizado basado en triángulos y shaders. A continuación, se describen los pasos principales:

1. **Carga de modelos**:
   - Los planetas se cargan desde archivos `.obj` usando niveles de detalle (LOD) dependiendo de la distancia.

2. **Transformaciones y shaders**:
   - Cada vértice se transforma al espacio de la cámara y se aplica un shader para calcular color, intensidad de luz y texturas.

3. **Rasterización y Z-buffering**:
   - Los triángulos visibles se rasterizan en píxeles con un algoritmo de rasterización paralelo.


## 📦 Estructura del proyecto

```
PROYECTO3_SPACE/
├── src/
│   ├── main.rs           # Entrada principal del programa
│   ├── framebuffer.rs    # Implementación del framebuffer
│   ├── triangle.rs       # Rasterización de triángulos
│   ├── vertex.rs         # Manejo de vértices
│   ├── shaders.rs        # Shaders para cada planeta
│   ├── camera.rs         # Manejo de la cámara
│   ├── planet.rs         # Lógica de movimiento de planetas
│   ├── color.rs          # Utilidades de color
│   ├── fragment.rs       # Fragmentos rasterizados
│   ├── obj.rs            # Carga de modelos .obj
├── assets/               # Archivos de modelos y texturas
│   ├── models/
│   │   ├── sphere.obj  # Modelo 
├── Cargo.toml            # Dependencias y configuración del proyecto
├── README.md             # Documentación
```

## ⚙️ Dependencias

En el archivo `Cargo.toml`:

```toml
[dependencies]
minifb = "0.20"
nalgebra-glm = "0.14"
rayon = "1.5"
fastnoise-lite = "0.5"


