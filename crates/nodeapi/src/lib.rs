use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum NodeHealth {
    Healthy,
    PartiallyHealthy,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ServiceStatus {
    Up,
    Down,
    Initializing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeService {
    id: String,
    name: String,
    description: String,
    status: ServiceStatus,
}

#[derive(Clone)]
struct NodeApi {
    node_name: String,
    node_version: String,
    health: Arc<Mutex<NodeHealth>>,
    services: Arc<Mutex<Vec<NodeService>>>,
}

impl NodeApi {
    fn new(node_name: &str, node_version: &str) -> Self {
        Self {
            node_name.to_string(),
            node_version.to_string(),
            health: Arc::new(Mutex::new(NodeHealth::Healthy)),
            services: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn update_health(&self, new_health: NodeHealth) {
        let mut health = self.health.lock().unwrap();
        *health = new_health;
    }

    fn register_service(
        &self,
        id: String,
        name: String,
        description: String,
        status: ServiceStatus,
    ) {
        let mut services = self.services.lock().unwrap();
        services.push(NodeService {
            id,
            name,
            description,
            status,
        });
    }

    fn update_service_status(&self, service_id: &str, status: ServiceStatus) -> Result<(), String> {
        let mut services = self.services.lock().unwrap();
        for service in services.iter_mut() {
            if service.id == service_id {
                service.status = status;
                return Ok(());
            }
        }
        Err(format!("Service with id {} not found", service_id))
    }

    fn deregister_service(&self, service_id: &str) -> Result<(), String> {
        let mut services = self.services.lock().unwrap();
        if let Some(index) = services.iter().position(|s| s.id == service_id) {
            services.remove(index);
            return Ok(());
        }
        Err(format!("Service with id {} not found", service_id))
    }
}

async fn node_info(api: web::Data<NodeApi>) -> impl Responder {
    let response = serde_json::json!({
        "node_name": api.node_name,
        "node_version": api.node_version,
        "spec_version": "v0.0.1",
    });
    HttpResponse::Ok().json(response)
}

async fn health_check(api: web::Data<NodeApi>) -> impl Responder {
    let health = api.health.lock().unwrap();
    match *health {
        NodeHealth::Healthy => HttpResponse::Ok().finish(),
        NodeHealth::PartiallyHealthy => HttpResponse::PartialContent().finish(),
        NodeHealth::Unhealthy => HttpResponse::ServiceUnavailable().finish(),
    }
}

async fn list_services(api: web::Data<NodeApi>) -> impl Responder {
    let services = api.services.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({ "services": *services }))
}

async fn service_health(api: web::Data<NodeApi>, path: web::Path<String>) -> impl Responder {
    let service_id = path.into_inner();
    let services = api.services.lock().unwrap();

    if let Some(service) = services.iter().find(|s| s.id == service_id) {
        match service.status {
            ServiceStatus::Up => HttpResponse::Ok().finish(),
            ServiceStatus::Down => HttpResponse::ServiceUnavailable().finish(),
            ServiceStatus::Initializing => HttpResponse::PartialContent().finish(),
        }
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/eigen/node", web::get().to(node_info))
            .route("/eigen/node/health", web::get().to(health_check))
            .route("/eigen/node/services", web::get().to(list_services))
            .route(
                "/eigen/node/services/{id}/health",
                web::get().to(service_health),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


#[cfg(test)]
mod tests {

    use super::*;


    #[tokio::test]
    async fn test_node_api() {

        let node_api = NodeApi::new("test_avs", "v0.0.1");
        







    }


}