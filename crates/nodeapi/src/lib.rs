#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod error;

use ntex::web::{self, App, HttpResponse, HttpServer, Responder};

use error::NodeApiError;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeHealth {
    Healthy,
    PartiallyHealthy,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Up,
    Down,
    Initializing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeService {
    id: String,
    name: String,
    description: String,
    status: ServiceStatus,
}

#[derive(Clone, Deserialize)]
pub struct NodeApi {
    node_name: String,
    node_version: String,
    health: NodeHealth,
    services: Vec<NodeService>,
}

#[allow(unused)]
impl NodeApi {
    /// Creates a new instance of [`NodeApi`].
    ///
    /// # Arguments
    ///
    /// * `node_name` - A string slice that holds the name of the node.
    /// * `node_version` - A string slice that holds the version of the node.
    ///
    /// # Returns
    ///
    /// A new instance of [`NodeApi`] with the specified node name and version,
    /// and default health status set to `Healthy` and an empty list of services.
    pub fn new(node_name: &str, node_version: &str) -> Self {
        Self {
            node_name: node_name.to_string(),
            node_version: node_version.to_string(),
            health: NodeHealth::Healthy,
            services: Vec::new(),
        }
    }

    ///
    /// Updates the health status of the node.
    ///
    /// # Arguments
    ///
    /// * `new_health` - The new health status to be set for the node.
    pub fn update_health(&mut self, new_health: NodeHealth) {
        self.health = new_health;
    }

    /// Registers a new service with the node.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice that holds the unique identifier of the service.
    /// * `name` - A string slice that holds the name of the service.
    /// * `description` - A string slice that holds the description of the service.
    /// * `status` - The status of the service, represented by the `ServiceStatus` enum.
    pub fn register_service(
        &mut self,
        id: &str,
        name: &str,
        description: &str,
        status: ServiceStatus,
    ) {
        let mut services = &mut self.services;
        services.push(NodeService {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            status,
        });
    }

    /// Updates the status of a registered service.
    ///
    /// # Arguments
    ///
    /// * `service_id` - A string slice that holds the unique identifier of the service.
    /// * `status` - The new status to be set for the service, represented by the `ServiceStatus` enum.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(())` if the service status was updated successfully,
    /// or an `Err` with a message if the service with the specified id was not found.
    pub fn update_service_status(
        &mut self,
        service_id: &str,
        status: ServiceStatus,
    ) -> Result<(), NodeApiError> {
        let mut services = &mut self.services;
        for service in services.iter_mut() {
            if service.id == service_id {
                service.status = status;
                return Ok(());
            }
        }

        Err(NodeApiError::ServiceIdNotFound(service_id.to_string()))
    }

    /// Deregisters a service from the node.
    ///
    /// # Arguments
    ///
    /// * `service_id` - A string slice that holds the unique identifier of the service.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(())` if the service was deregistered successfully,
    /// or an `Err` with a message if the service with the specified id was not found.
    pub fn deregister_service(&mut self, service_id: &str) -> Result<(), NodeApiError> {
        let mut services = &mut self.services;
        if let Some(index) = services.iter().position(|s| s.id == service_id) {
            services.remove(index);
            return Ok(());
        }
        Err(NodeApiError::ServiceIdNotFound(service_id.to_string()))
    }
}

#[allow(unused)]
pub async fn node_info(api: web::types::State<NodeApi>) -> impl Responder {
    let response = serde_json::json!({
        "node_name": api.node_name,
        "node_version": api.node_version,
        "spec_version": "v0.0.1",
    });
    HttpResponse::Ok().json(&response)
}

#[allow(unused)]
pub async fn health_check(api: web::types::State<NodeApi>) -> impl Responder {
    let health = &api.health;

    match health {
        NodeHealth::Healthy => HttpResponse::Ok().finish(),
        NodeHealth::PartiallyHealthy => HttpResponse::PartialContent().finish(),
        NodeHealth::Unhealthy => HttpResponse::ServiceUnavailable().finish(),
    }
}

#[allow(unused)]
pub async fn list_services(api: web::types::State<NodeApi>) -> impl Responder {
    let services = &api.services;
    HttpResponse::Ok().json(&serde_json::json!({ "services": *services }))
}

#[allow(unused)]
pub async fn service_health(
    api: web::types::State<NodeApi>,
    path: web::types::Path<String>,
) -> impl Responder {
    let service_id = path.into_inner();
    let services = &api.services;

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

/// Function to create the Actix HTTP server
/// This function sets up the server and routes.
/// External users can call this function to create and run the server.
pub fn create_server(api: NodeApi, ip_port_addr: String) -> std::io::Result<ntex::server::Server> {
    let server = HttpServer::new(move || {
        App::new()
            .state(api.clone()) // Use the provided NodeApi instance
            .route("/eigen/node", web::get().to(node_info))
            .route("/eigen/node/health", web::get().to(health_check))
            .route("/eigen/node/services", web::get().to(list_services))
            .route(
                "/eigen/node/services/{id}/health",
                web::get().to(service_health),
            )
    })
    .bind(ip_port_addr.clone())?
    .run();
    info!("node api server running at port :{}", ip_port_addr);
    Ok(server)
}

#[cfg(test)]
mod tests {

    use super::*;
    use ntex::{http, web::test};
    use reqwest::Client;
    #[tokio::test]
    async fn test_node_handler() {
        let mut node_api = NodeApi::new("test_avs", "v0.0.1");
        node_api.register_service(
            "test_service_id",
            "test_service_name",
            "test_service_description",
            ServiceStatus::Initializing,
        );

        let app = App::new()
            .state(node_api.clone())
            .route("/eigen/node", web::get().to(node_info));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/eigen/node").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body);

        // The expected JSON response
        let expected_body =
            "{\"node_name\":\"test_avs\",\"node_version\":\"v0.0.1\",\"spec_version\":\"v0.0.1\"}";

        assert_eq!(body_str, expected_body);
    }

    #[tokio::test]
    async fn test_list_services_handler() {
        let tests = vec![
            (
                NodeApi::new("test_avs", "v0.0.1"),
                http::StatusCode::OK,
                "{\"services\":[]}",
            ),
            (
                {
                    let mut node_api = NodeApi::new("test_avs", "v0.0.1");
                    node_api.register_service(
                        "testServiceId",
                        "testServiceName",
                        "testServiceDescription",
                        ServiceStatus::Up,
                    );
                    node_api
                },
                http::StatusCode::OK,
                "{\"services\":[{\"id\":\"testServiceId\",\"name\":\"testServiceName\",\"description\":\"testServiceDescription\",\"status\":\"Up\"}]}",
            ),
            (
                {
                    let mut node_api = NodeApi::new("test_avs", "v0.0.1");
                    node_api.register_service(
                        "testServiceId",
                        "testServiceName",
                        "testServiceDescription",
                        ServiceStatus::Up,
                    );
                    node_api.register_service(
                        "testServiceId2",
                        "testServiceName2",
                        "testServiceDescription2",
                        ServiceStatus::Down,
                    );
                    node_api
                },
                http::StatusCode::OK,
                "{\"services\":[{\"id\":\"testServiceId\",\"name\":\"testServiceName\",\"description\":\"testServiceDescription\",\"status\":\"Up\"},{\"id\":\"testServiceId2\",\"name\":\"testServiceName2\",\"description\":\"testServiceDescription2\",\"status\":\"Down\"}]}",
            ),
        ];

        for (node_api, expected_status, expected_body) in tests {
            let app = test::init_service(
                App::new()
                    .state(node_api.clone())
                    .route("/eigen/node/services", web::get().to(list_services)),
            )
            .await;

            let req = test::TestRequest::get()
                .uri("/eigen/node/services")
                .to_request();
            let resp = app.call(req).await.unwrap();

            // Assert status code
            assert_eq!(resp.status(), expected_status);

            let body = test::read_body(resp).await;
            let body_str = String::from_utf8_lossy(&body);

            let actual_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
            let expected_json: serde_json::Value = serde_json::from_str(expected_body).unwrap();

            assert_eq!(actual_json, expected_json);
        }
    }

    #[tokio::test]
    async fn test_service_health_handler() {
        let mut node_api = NodeApi::new("test_avs", "v0.0.1");

        // Register a service to the NodeApi
        node_api.register_service(
            "testServiceId",
            "testServiceName",
            "testServiceDescription",
            ServiceStatus::Up,
        );

        // Initialize the app with the NodeApi
        let app = test::init_service(App::new().state(node_api.clone()).route(
            "/eigen/node/services/{id}/health",
            web::get().to(service_health),
        ))
        .await;

        // Test Case 1: Service exists (should return 200 OK)
        let req = test::TestRequest::get()
            .uri("/eigen/node/services/testServiceId/health")
            .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK); // Expect 200 OK for existing service

        // Test Case 2: Service does not exist (should return 404 Not Found)
        let req = test::TestRequest::get()
            .uri("/eigen/node/services/nonexistentService/health")
            .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
        let body = test::read_body(resp).await;
        assert_eq!(body.len(), 0);
    }

    #[ntex::test]
    async fn test_create_server() -> std::io::Result<()> {
        // Create a NodeApi instance and register a service
        let mut node_api = NodeApi::new("test_node", "v1.0.0");
        node_api.register_service(
            "test_service",
            "Test Service",
            "Test service description",
            ServiceStatus::Up,
        );

        // Set up a server running on a test address (e.g., 127.0.0.1:8081)
        let ip_port_addr = "127.0.0.1:8081".to_string();
        let server = create_server(node_api.clone(), ip_port_addr.clone())?;

        // Start the server in a background task
        ntex::rt::spawn(server);

        // Give the server some time to start up
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        // Test the /eigen/node route
        // Use Reqwest to send HTTP requests to the running server
        let client = Client::new();

        // Test the /eigen/node route
        let resp = client
            .get(format!("http://{}/eigen/node", ip_port_addr))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::OK);

        // Test the /eigen/node/health route
        let resp = client
            .get(format!("http://{}/eigen/node/health", ip_port_addr))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::OK);

        // // Test the /eigen/node/services route
        let resp = client
            .get(format!("http://{}/eigen/node/services", ip_port_addr))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::OK);

        // Test the /eigen/node/services/{id}/health route
        let resp = client
            .get(format!(
                "http://{}/eigen/node/services/test_service/health",
                ip_port_addr
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::OK);

        Ok(())
    }
}
