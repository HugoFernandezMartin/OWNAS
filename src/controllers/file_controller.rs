use crate::{
    core::responses::{ResponseType, ServerResponse},
    file_manager::{create_file, delete_file, list_files, read_file, write_in_file},
};

pub async fn list_files_controller(workspace_path: &str) -> ServerResponse {
    return match list_files(workspace_path).await {
        Ok(files) => ServerResponse::Success(ResponseType::Files(files)),
        Err(e) => {
            tracing::error!(error = %e, "Unable to list files from directory");
            ServerResponse::Error(e.to_string())
        }
    };
}

pub async fn create_file_controller(workspace_path: &str, file_name: &str) -> ServerResponse {
    return match create_file(workspace_path, &file_name).await {
        Ok(_) => {
            ServerResponse::Success(ResponseType::Info("File created succesfully".to_string()))
        }
        Err(e) => {
            tracing::error!(error = %e, "Unable to create file");
            ServerResponse::Error(e)
        }
    };
}

pub async fn delete_file_controller(workspace_path: &str, file_name: &str) -> ServerResponse {
    return match delete_file(workspace_path, &file_name).await {
        Ok(()) => {
            ServerResponse::Success(ResponseType::Info("File deleted succesfully".to_string()))
        }
        Err(e) => {
            tracing::error!(error = %e, "Unable to delete file");
            ServerResponse::Error(e)
        }
    };
}

pub async fn write_in_file_controller(
    workspace_path: &str,
    file_name: &str,
    text: &str,
) -> ServerResponse {
    return match write_in_file(workspace_path, file_name, text).await {
        Ok(()) => {
            ServerResponse::Success(ResponseType::Info("Write in file succesfully".to_string()))
        }
        Err(e) => {
            tracing::error!(error = %e, "Unable to write in file");
            ServerResponse::Error(e.to_string())
        }
    };
}

pub async fn read_file_controller(workspace_path: &str, file_name: &str) -> ServerResponse {
    return match read_file(workspace_path, &file_name).await {
        Ok(file_data) => ServerResponse::Success(ResponseType::Info(file_data)),
        Err(e) => {
            tracing::error!(error = %e, "Unable to read file");
            ServerResponse::Error(e.to_string())
        }
    };
}
