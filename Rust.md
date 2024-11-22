```markdown
# Comandos básicos para trabajar con Rust en Visual Studio Code

### 1. **Instalación y configuración inicial**
- **Instalar Rust**  
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Verificar instalación**  
  ```bash
  rustc --version
  cargo --version
  ```

- **Extensión para VS Code**  
  Instalar la extensión **Rust Analyzer** desde la tienda de extensiones de VS Code.

---

### 2. **Crear y gestionar proyectos**
- **Crear un nuevo proyecto**  
  ```bash
  cargo new nombre_del_proyecto
  ```
- **Moverse al directorio del proyecto**  
  ```bash
  cd nombre_del_proyecto
  ```
- **Inicializar un proyecto existente**  
  ```bash
  cargo init
  ```

---

### 3. **Compilar y ejecutar**
- **Compilar el proyecto**  
  ```bash
  cargo build
  ```
- **Compilar en modo release (óptimo)**  
  ```bash
  cargo build --release
  ```
- **Ejecutar el proyecto**  
  ```bash
  cargo run
  ```
- **Verificar si el código compila**  
  ```bash
  cargo check
  ```

---

### 4. **Gestionar dependencias**
- **Agregar una dependencia al archivo `Cargo.toml`**  
  ```bash
  cargo add nombre_paquete
  ```
- **Actualizar dependencias**  
  ```bash
  cargo update
  ```

---

### 5. **Probar el código**
- **Ejecutar pruebas**  
  ```bash
  cargo test
  ```

---

### 6. **Documentación**
- **Generar documentación**  
  ```bash
  cargo doc --open
  ```

---

### 7. **Formato y estilo del código**
- **Formatear código automáticamente**  
  ```bash
  cargo fmt
  ```
- **Revisar problemas con el código**  
  ```bash
  cargo clippy
  ```

---

### 8. **Otras utilidades**
- **Limpiar archivos generados por compilaciones anteriores**  
  ```bash
  cargo clean
  ```
- **Listar dependencias del proyecto**  
  ```bash
  cargo tree
  ```

---

### 9. **Atajos en VS Code**
- **Formatear código:**  
  `Shift + Alt + F` (o configurar `rustfmt` como formateador predeterminado en la configuración de VS Code).  
- **Ejecutar terminal integrada:**  
  `Ctrl + \`  

```