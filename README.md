#EQUIPO
CASTRO LUNA CESAR ARMANDO
ZACARIAS HERNANDEZ ANGEL DAVID
ZAMORA MARTINEZ KIMBERLY PAOLA DEL ROCIO 


# 💻 Sistema de Gestión de Empleados (CRUD) - API REST en Rust

¡Bienvenido al repositorio del **Sistema de Gestión de Empleados**! Este proyecto es una solución integral para la administración de personal en tiempo real. Cuenta con una arquitectura robusta que combina un backend de alto rendimiento desarrollado en **Rust**, persistencia de datos en **MySQL (XAMPP)**, y un entorno de contenedores optimizado con **Docker** para entornos de despliegue simulados.

Adicionalmente, se incluye un **Panel CRUD Administrativo** automatizado en PowerShell que permite interactuar con el sistema de manera completamente gráfica e inmediata.

---

## 🚀 Características Principales

- **Backend de Alto Rendimiento:** API REST construida con Rust, garantizando seguridad de memoria y concurrencia eficiente.
- **Validación Estricta de Datos:** Control riguroso de formatos (fechas en formato estándar `YYYY-MM-DD`, llaves primarias únicas y control de géneros).
- **Persistencia Relacional:** Conexión directa a la base de datos relacional de empleados en MySQL.
- **Ecosistema Aislado (Docker):** Inclusión de contenedores independientes configurados en puertos seguros (ej. `9090`) para simular entornos productivos sin colisionar con servicios locales (Apache/XAMPP).
- **Interfaz Gráfica Interactiva:** Panel administrativo completo para operaciones *Create, Read, Update y Delete* (CRUD) en tiempo real.

---

## 🛠️ Arquitectura del Sistema

El proyecto se divide en tres capas fundamentales que trabajan en armonía:

1. **El Servidor (Backend):** Desarrollado en Rust, encargado de procesar las peticiones HTTP (`GET`, `POST`, `PUT`, `DELETE`) y validar los datos antes de impactar la base de datos.
2. **La Base de Datos (Persistencia):** Administrada a través de XAMPP (phpMyAdmin) utilizando el motor MySQL en su puerto estándar `3306`.
3. **El Contenedor (Infraestructura):** Un contenedor Docker configurado en modo aislado para la simulación del despliegue en red.

---

## 📦 Requisitos Previos

Para clonar y ejecutar este proyecto localmente, necesitarás tener instalado:

- [Rust y Cargo](https://www.rust-lang.org/) (Edición 2021 o superior)
- [XAMPP](https://www.apachefriends.org/) (Con los servicios de Apache y MySQL habilitados)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/) (Con soporte para contenedores de Windows/Linux)
- Windows PowerShell (Para el panel de gestión gráfica)

---

## 🔧 Instrucciones de Configuración y Uso

Sigue estos pasos en orden para desplegar el entorno completo en tu máquina local:

### 1. Preparar la Base de Datos
1. Abre el panel de **XAMPP Control Panel**.
2. Inicia los servicios de **Apache** y **MySQL** (asegúrate de que los indicadores estén en verde brillante).
3. Accede a `http://localhost/phpmyadmin` en tu navegador y asegúrate de tener importada la base de datos llamada `employees`.

### 2. Levantar la Infraestructura en Docker
Para desplegar el contenedor de infraestructura simulada en un puerto seguro que no interfiera con tu servidor local, ejecuta el siguiente comando en tu terminal de PowerShell:
```powershell
docker run -d -p 9090:80 --name servidor-api-rust alpine sleep infinity
