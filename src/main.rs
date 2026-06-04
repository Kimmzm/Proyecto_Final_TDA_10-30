use actix_web::{get, post, put, delete, web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool, Row};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::NaiveDate;

struct AppState {
    db: Pool<MySql>,
}

// Estructura para formatear la respuesta unificada requerida
#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    message: String,
    data: Option<T>,
}

// Estructura para serializar los datos de empleados devueltos en el JSON
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Employee {
    emp_no: i32,
    birth_date: String,
    first_name: String,
    last_name: String,
    gender: String,
    hire_date: String,
    salary: Option<i32>,      
    dept_no: Option<String>,   
}

// Estructura de payload para registro y actualización con validaciones
#[derive(Deserialize)]
struct EmployeePayload {
    emp_no: i32,
    birth_date: String,
    first_name: String,
    last_name: String,
    gender: String,
    hire_date: String,
    salary: Option<i32>,      
}

#[derive(Deserialize)]
struct TransferPayload {
    new_dept_no: String,
}

// =========================================================================
// ENDPOINTS DE LA API REST (RESPUESTAS ESTÁNDAR EN JSON Y VALIDACIONES)
// =========================================================================

// READ: Obtener lista de empleados
#[get("/api/employees")]
async fn get_employees(data: web::Data<AppState>) -> impl Responder {
    let query = "
        SELECT e.emp_no, e.birth_date, e.first_name, e.last_name, e.gender, e.hire_date, s.salary, d.dept_no 
        FROM employees e
        INNER JOIN salaries s ON e.emp_no = s.emp_no
        INNER JOIN dept_emp d ON e.emp_no = d.emp_no
        WHERE s.to_date = '9999-01-01' AND d.to_date = '9999-01-01'
        LIMIT 50";

    match sqlx::query(query).fetch_all(&data.db).await {
        Ok(rows) => {
            let mut employees = Vec::new();
            for row in rows {
                let b_date: NaiveDate = row.get(1);
                let h_date: NaiveDate = row.get(5);
                employees.push(Employee {
                    emp_no: row.get(0),
                    birth_date: b_date.to_string(),
                    first_name: row.get(2),
                    last_name: row.get(3),
                    gender: row.get(4),
                    hire_date: h_date.to_string(),
                    salary: Some(row.get(6)),
                    dept_no: Some(row.get(7)),
                });
            }
            HttpResponse::Ok().json(ApiResponse {
                status: "ok".to_string(),
                message: "Listado de empleados obtenido exitosamente".to_string(),
                data: Some(employees),
            })
        },
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: format!("Error interno al consultar la base de datos: {}", e),
            data: None,
        }),
    }
}

// CREATE: Registrar un nuevo empleado con validaciones severas
#[post("/api/employees")]
async fn create_employee(data: web::Data<AppState>, emp: web::Json<EmployeePayload>) -> impl Responder {
    // Agregamos log visible para ver los registros en consola cuando tus compañeros envíen datos
    println!(" Petición POST recibida: Intentando registrar a {} {}", emp.first_name, emp.last_name);

    // 1. Validaciones básicas de campos vacíos
    if emp.first_name.trim().is_empty() || emp.last_name.trim().is_empty() || emp.gender.trim().is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Validación fallida: Los campos first_name, last_name y gender son requeridos y no pueden estar vacíos".to_string(),
            data: None,
        });
    }

    // 2. Validación de Género estricto
    if emp.gender != "M" && emp.gender != "F" {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Validación fallida: El campo gender debe ser exactamente 'M' o 'F'".to_string(),
            data: None,
        });
    }

    // 3. Validación de Fechas correctas
    let b_date = match NaiveDate::parse_from_str(&emp.birth_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Validación fallida: Formato de birth_date inválido. Debe ser YYYY-MM-DD".to_string(),
            data: None,
        }),
    };

    let h_date = match NaiveDate::parse_from_str(&emp.hire_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Validación fallida: Formato de hire_date inválido. Debe ser YYYY-MM-DD".to_string(),
            data: None,
        }),
    };

    // 4. Validación Opcional de Salario si se proporciona
    if let Some(sal) = emp.salary {
        if sal <= 0 {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Validación fallida: El salario debe ser un número entero mayor a 0".to_string(),
                data: None,
            });
        }
    }

    // Ejecución del Insert
    let result = sqlx::query("INSERT INTO employees (emp_no, birth_date, first_name, last_name, gender, hire_date) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(emp.emp_no)
        .bind(b_date)
        .bind(&emp.first_name)
        .bind(&emp.last_name)
        .bind(&emp.gender)
        .bind(h_date)
        .execute(&data.db)
        .await;

    match result {
        Ok(_) => {
            println!(" Empleado {} {} guardado con éxito.", emp.first_name, emp.last_name);
            HttpResponse::Created().json(ApiResponse::<()> {
                status: "ok".to_string(),
                message: "Empleado registrado con éxito en la base de datos".to_string(),
                data: None,
            })
        },
        Err(e) => {
            println!(" Fallo al guardar empleado: {}", e);
            HttpResponse::BadRequest().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Error de base de datos (Posible ID duplicado): {}", e),
                data: None,
            })
        },
    }
}

// UPDATE: Actualizar datos de un empleado existente (Petición PUT limpia)
#[put("/api/employees/{id}")]
async fn update_employee(data: web::Data<AppState>, id: web::Path<i32>, emp: web::Json<EmployeePayload>) -> impl Responder {
    if emp.first_name.trim().is_empty() || emp.last_name.trim().is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Validación fallida: Nombre y apellido no pueden actualizarse a campos vacíos".to_string(),
            data: None,
        });
    }

    let b_date = NaiveDate::parse_from_str(&emp.birth_date, "%Y-%m-%d").unwrap_or_default();
    let h_date = NaiveDate::parse_from_str(&emp.hire_date, "%Y-%m-%d").unwrap_or_default();

    let result = sqlx::query("UPDATE employees SET birth_date = ?, first_name = ?, last_name = ?, gender = ?, hire_date = ? WHERE emp_no = ?")
        .bind(b_date)
        .bind(&emp.first_name)
        .bind(&emp.last_name)
        .bind(&emp.gender)
        .bind(h_date)
        .bind(*id)
        .execute(&data.db)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().json(ApiResponse::<()> {
            status: "ok".to_string(),
            message: "Datos del empleado actualizados correctamente".to_string(),
            data: None,
        }),
        Ok(_) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "El empleado con el ID especificado no fue localizado".to_string(),
            data: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: format!("Error interno del servidor: {}", e),
            data: None,
        }),
    }
}

// DELETE: Eliminar un registro de empleado
#[delete("/api/employees/{id}")]
async fn delete_employee(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query("DELETE FROM employees WHERE emp_no = ?").bind(*id).execute(&data.db).await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().json(ApiResponse::<()> {
            status: "ok".to_string(),
            message: "Empleado eliminado con éxito del sistema".to_string(),
            data: None,
        }),
        Ok(_) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Error: El ID de empleado especificado no existe".to_string(),
            data: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: format!("Error crítico al procesar la baja: {}", e),
            data: None,
        }),
    }
}

// CASO DE USO 1: Cálculo de Salario Neto Financiero
#[get("/api/employees/{id}/net-salary")]
async fn calculate_net_salary(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let query = "SELECT first_name, last_name, salary FROM employees e INNER JOIN salaries s ON e.emp_no = s.emp_no WHERE e.emp_no = ? AND s.to_date = '9999-01-01'";
    
    match sqlx::query(query).bind(*id).fetch_optional(&data.db).await {
        Ok(Some(r)) => {
            let name: String = r.get(0);
            let last: String = r.get(1);
            let gross_salary: i32 = r.get(2);
            let tax = (gross_salary as f64) * 0.16;
            let net_salary = (gross_salary as f64) - tax;

            let info_msg = format!(
                "Caso Finanzas: {} {} percibe bruto de ${}.00. Deducción Impuesto (16%): ${:.2}. Neto: ${:.2}",
                name, last, gross_salary, tax, net_salary
            );

            HttpResponse::Ok().json(ApiResponse::<String> {
                status: "ok".to_string(),
                message: "Cálculo de nómina ejecutado exitosamente".to_string(),
                data: Some(info_msg),
            })
        },
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Empleado no encontrado en el sistema de nómina activa".to_string(),
            data: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: format!("Error al calcular nómina: {}", e),
            data: None,
        }),
    }
}

// CASO DE USO 2: Transferencia Organizacional de Área
#[put("/api/employees/{id}/transfer")]
async fn transfer_department(data: web::Data<AppState>, id: web::Path<i32>, payload: web::Json<TransferPayload>) -> impl Responder {
    if payload.new_dept_no.trim().is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Validación fallida: El código del nuevo departamento es obligatorio".to_string(),
            data: None,
        });
    }

    let result = sqlx::query("UPDATE dept_emp SET dept_no = ? WHERE emp_no = ? AND to_date = '9999-01-01'")
        .bind(&payload.new_dept_no)
        .bind(*id)
        .execute(&data.db)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().json(ApiResponse::<()> {
            status: "ok".to_string(),
            message: "Caso de Uso Recursos Humanos: El empleado ha sido reasignado de departamento exitosamente".to_string(),
            data: None,
        }),
        Ok(_) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "No se detectó una asignación estructural activa para este empleado".to_string(),
            data: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: format!("Error interno al transferir: {}", e),
            data: None,
        }),
    }
}

// =========================================================================
// FUNCIÓN PRINCIPAL DE ARRANQUE DEL BACKEND (ESCUCHA EN TODA LA RED LOCAL)
// =========================================================================
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Conexión directa a la IP local de tu base de datos XAMPP en Windows
    let database_url = "mysql://root:@127.0.0.1:3306/employees";

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Error crítico al enlazar con phpMyAdmin");

    println!(" API REST en Rust corriendo ");
    println!(" pueden conectarse usando la direccion: http://192.168.201.107:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(get_employees)
            .service(create_employee)
            .service(update_employee)
            .service(delete_employee)
            .service(calculate_net_salary)
            .service(transfer_department)
    })
    .bind(("0.0.0.0", 8080))? // Vincula a todas las interfaces de red de tu PC
    .run()
    .await
}