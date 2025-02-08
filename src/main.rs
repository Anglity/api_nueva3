use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use std::collections::HashMap; // Importar HashMap


const FIREBASE_URL: &str = "https://appparalelo-13c98-default-rtdb.firebaseio.com/"; // Reemplaza con tu URL Firebase
const API_VIEJA_URL: &str = "http://167.71.164.51:8000/api/pedidos"; // API vieja

// Estructura del pedido
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pedido {
    id: String,
    nombre_cliente: String,
    contacto: String,
    producto: String,
    cantidad: u32,
    fecha_entrega: String,
    direccion: Option<String>,
    #[serde(default = "estado_por_defecto")] // Mantiene estado predeterminado si no existe
    estado: String,
}

// Estado predeterminado para pedidos nuevos
fn estado_por_defecto() -> String {
    "En proceso".to_string()
}

// 🔹 **1️⃣ Función para sincronizar pedidos desde la API vieja a Firebase**
// 🔹 **Función para sincronizar pedidos**
async fn sincronizar_pedidos(client: &Arc<Client>) -> Result<(), reqwest::Error> {
    println!("⏳ Sincronizando pedidos desde la API vieja...");

    // **Paso 1: Obtener pedidos de la API vieja**
    let response = client.get(API_VIEJA_URL).send().await?;
    
    // 🔹 Convertir la respuesta en un mapa en lugar de una lista
    let pedidos_viejos: HashMap<String, Pedido> = response.json().await?;

    // **Paso 2: Convertir HashMap en un Vec<Pedido>**
    let pedidos_lista: Vec<Pedido> = pedidos_viejos.into_values().collect();

    // **Paso 3: Guardar cada pedido en Firebase**
    for mut pedido in pedidos_lista {
        let firebase_url = format!("{}/pedidos/{}.json", FIREBASE_URL, pedido.id);

        // **Verificar si el pedido ya existe en Firebase**
        let existing_response = client.get(&firebase_url).send().await;
        if let Ok(existing_res) = existing_response {
            if let Ok(existing_pedido) = existing_res.json::<Pedido>().await {
                // 🚀 **Mantener el estado actual del pedido**
                pedido.estado = existing_pedido.estado.clone();
            }
        }

        // **Guardar o actualizar el pedido en Firebase**
        client.put(&firebase_url).json(&pedido).send().await?;
    }

    println!("✅ Sincronización completada.");
    Ok(())
}

// 🔹 **2️⃣ Endpoint para sincronizar pedidos manualmente**
async fn sincronizar_pedidos_manual(
    client: web::Data<Arc<Client>>,
) -> impl Responder {
    match sincronizar_pedidos(client.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("📡 Sincronización manual completada"),
        Err(e) => {
            println!("❌ Error en la sincronización manual: {:?}", e);
            HttpResponse::InternalServerError().body("Error al sincronizar manualmente")
        }
    }
}

// 🔹 **3️⃣ Obtener todos los pedidos desde Firebase**
async fn obtener_pedidos(
    client: web::Data<Arc<Client>>,
) -> impl Responder {
    let url = format!("{}/pedidos.json", FIREBASE_URL);
    match client.get(&url).send().await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(e) => {
                println!("❌ Error al parsear pedidos de Firebase: {:?}", e);
                HttpResponse::InternalServerError().body("Error al parsear pedidos")
            }
        },
        Err(e) => {
            println!("❌ Error al obtener pedidos desde Firebase: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener pedidos")
        }
    }
}

// 🔹 **4️⃣ Actualizar el estado de un pedido en Firebase**
async fn actualizar_estado(
    client: web::Data<Arc<Client>>,
    path: web::Path<String>,
    nuevo_estado: web::Json<String>,
) -> impl Responder {
    let pedido_id = path.into_inner();
    let url = format!("{}/pedidos/{}/estado.json", FIREBASE_URL, pedido_id);

    match client.put(&url).json(&nuevo_estado.into_inner()).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!("🔄 Estado actualizado en Firebase para ID: {}", pedido_id);
                HttpResponse::Ok().body("Estado actualizado en Firebase")
            } else {
                HttpResponse::InternalServerError().body("❌ Error al actualizar estado")
            }
        }
        Err(e) => {
            println!("❌ Error al actualizar estado en Firebase: {:?}", e);
            HttpResponse::InternalServerError().body("Error al actualizar estado")
        }
    }
}

// 🔹 **5️⃣ Sincronizar estados en tiempo real desde Firebase**
async fn sincronizar_estados_en_tiempo_real(client: Arc<Client>) {
    loop {
        let url = format!("{}/pedidos.json", FIREBASE_URL);

        match client.get(&url).send().await {
            Ok(response) => {
                let response_text = response.text().await.unwrap_or_else(|_| "Error al leer respuesta".to_string());

                if response_text == "null" {
                    println!("⚠️ No hay pedidos almacenados en Firebase.");
                } else {
                    println!("✅ Pedidos sincronizados en tiempo real: {}", response_text);
                }
            }
            Err(e) => println!("❌ Error al sincronizar estados: {:?}", e),
        }
        sleep(Duration::from_secs(3)).await; // Verificar cada 3 segundos
    }
}

// 🔹 **6️⃣ Configuración del servidor**
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(Client::new());

    // **Inicia sincronización en tiempo real**
    let client_clone = client.clone();
    tokio::spawn(async move {
        sincronizar_estados_en_tiempo_real(client_clone).await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/sincronizar", web::get().to(sincronizar_pedidos_manual))  // Sincronizar manualmente
            .route("/pedidos", web::get().to(obtener_pedidos))                 // Obtener pedidos desde Firebase
            .route("/pedidos/{id}/estado", web::patch().to(actualizar_estado)) // Actualizar estado de un pedido
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
