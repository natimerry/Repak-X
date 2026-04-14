//! VFX Updater - Tauri Commands
//! 
//! Exposes VFX pipeline operations to the frontend via Tauri IPC.

use tauri::{AppHandle, Manager, Window};
use std::path::{Path, PathBuf};

use super::models::{VfxSettings, VfxPipelineResult, AssetClassInfo, VfxTempDirectories};
use super::progress::TauriVfxProgressSink;
use super::file_ops::{
    get_uasset_tool_path, read_json_file, write_json_file, list_json_files,
    create_step_directory, cleanup_vfx_temp_directories, get_vfx_temp_base,
};
use super::pipeline::{
    create_pipeline_directories, cleanup_pipeline,
    step_extract_mod_assets, step_convert_mod_to_json,
    step_extract_vanilla_assets,
    step_convert_json_to_uassets, step_pack_to_iostore,
    detect_asset_class as pipeline_detect_asset_class,
};
use super::uasset_tool::{ensure_vfx_uat_session, close_vfx_uat_session};
use super::logging::{init_vfx_log, close_vfx_log, vfx_info, vfx_error};

/// App data directory matching the rest of Repak-X: %APPDATA%/Repak-X
fn vfx_app_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Repak-X")
}

// ============================================================================
// Session Management
// ============================================================================

#[tauri::command]
pub async fn vfx_start_session(app: AppHandle) -> Result<(), String> {
    // Initialize logging
    let app_data_dir = vfx_app_dir();
    let _ = std::fs::create_dir_all(&app_data_dir);
    let _ = init_vfx_log(&app_data_dir);
    
    let tool_path = get_uasset_tool_path(&app)?;
    ensure_vfx_uat_session(&tool_path).await?;
    vfx_info("VFX session started");
    Ok(())
}

#[tauri::command]
pub async fn vfx_stop_session() -> Result<(), String> {
    vfx_info("Stopping VFX session");
    close_vfx_uat_session().await;
    close_vfx_log();
    Ok(())
}

// ============================================================================
// Pipeline Steps
// ============================================================================

#[tauri::command]
pub async fn vfx_extract_mod_assets(
    window: Window,
    app: AppHandle,
    game_paks: String,
    mod_path: String,
    output_dir: String,
) -> Result<Vec<String>, String> {
    let tool_path = get_uasset_tool_path(&app)?;
    let progress = TauriVfxProgressSink::new(&window);
    
    step_extract_mod_assets(&tool_path, &game_paks, &mod_path, &output_dir, &progress).await
}

#[tauri::command]
pub async fn vfx_convert_uassets_to_json(
    window: Window,
    app: AppHandle,
    usmap_path: String,
    input_dir: String,
    output_dir: String,
) -> Result<Vec<String>, String> {
    let tool_path = get_uasset_tool_path(&app)?;
    let progress = TauriVfxProgressSink::new(&window);
    
    step_convert_mod_to_json(&tool_path, &usmap_path, &input_dir, &output_dir, &progress).await
}

#[tauri::command]
pub async fn vfx_extract_vanilla_assets(
    window: Window,
    app: AppHandle,
    game_paks: String,
    output_dir: String,
    filter_patterns: Vec<String>,
) -> Result<Vec<String>, String> {
    let tool_path = get_uasset_tool_path(&app)?;
    let progress = TauriVfxProgressSink::new(&window);
    
    step_extract_vanilla_assets(&tool_path, &game_paks, &output_dir, &filter_patterns, &progress).await
}

#[tauri::command]
pub async fn vfx_convert_json_to_uassets(
    window: Window,
    app: AppHandle,
    usmap_path: String,
    input_dir: String,
    output_dir: String,
) -> Result<Vec<String>, String> {
    let tool_path = get_uasset_tool_path(&app)?;
    let progress = TauriVfxProgressSink::new(&window);
    
    step_convert_json_to_uassets(&tool_path, &usmap_path, &input_dir, &output_dir, &progress).await
}

#[tauri::command]
pub async fn vfx_pack_to_iostore(
    window: Window,
    app: AppHandle,
    usmap_path: String,
    input_dir: String,
    output_base: String,
) -> Result<String, String> {
    let tool_path = get_uasset_tool_path(&app)?;
    let progress = TauriVfxProgressSink::new(&window);
    
    step_pack_to_iostore(&tool_path, &usmap_path, &input_dir, &output_base, &progress).await
}

// ============================================================================
// Asset Class Detection
// ============================================================================

#[tauri::command]
pub async fn vfx_detect_asset_class(
    app: AppHandle,
    usmap_path: String,
    file_path: String,
) -> Result<AssetClassInfo, String> {
    let tool_path = get_uasset_tool_path(&app)?;
    pipeline_detect_asset_class(&tool_path, &usmap_path, &file_path).await
}

// ============================================================================
// Temp Directory Management
// ============================================================================

#[tauri::command]
pub fn vfx_get_temp_dir() -> Result<String, String> {
    let base = get_vfx_temp_base()?;
    Ok(base.to_string_lossy().to_string())
}

#[tauri::command]
pub fn vfx_create_step_directory(step_name: String) -> Result<String, String> {
    let dir = create_step_directory(&step_name)?;
    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command]
pub fn vfx_create_pipeline_directories() -> Result<VfxTempDirectories, String> {
    create_pipeline_directories()
}

#[tauri::command]
pub fn vfx_cleanup_temp_directories() -> Result<(), String> {
    cleanup_vfx_temp_directories()
}

#[tauri::command]
pub fn vfx_cleanup_pipeline() -> Result<(), String> {
    cleanup_pipeline()
}

// ============================================================================
// UAssetTool Path
// ============================================================================

#[tauri::command]
pub fn vfx_get_uasset_tool_path() -> Result<String, String> {
    // Use the same logic as install_mod for finding UAssetTool
    use crate::install_mod::install_mod_logic::iotoc::find_uasset_tool;
    
    let tool_path = find_uasset_tool()
        .map_err(|e| format!("[VFX] {}", e))?;
    
    vfx_info(&format!("UAssetTool path: {}", tool_path.display()));
    Ok(tool_path.to_string_lossy().to_string())
}

// ============================================================================
// Asset Class Scanning
// ============================================================================

#[tauri::command]
pub async fn vfx_get_asset_classes(
    uat_path: String,
    usmap_path: String,
    uasset_paths: Vec<String>,
    window: Window,
) -> Result<std::collections::HashMap<String, String>, String> {
    use super::uasset_tool::batch_detect_asset_types;
    
    let progress = TauriVfxProgressSink::new(&window);
    batch_detect_asset_types(
        std::path::Path::new(&uat_path),
        &usmap_path,
        &uasset_paths,
        &progress,
    ).await
}

#[tauri::command]
pub fn vfx_is_updatable_class(class_name: String) -> bool {
    super::uasset_tool::is_updatable_class(&class_name)
}

#[tauri::command]
pub async fn vfx_copy_uasset_files(
    source_paths: Vec<String>,
    source_base_dir: String,
    dest_base_dir: String,
) -> Result<Vec<String>, String> {
    use std::path::{Path, PathBuf};
    
    let mut copied = Vec::new();
    
    // Normalize base dir for comparison
    let base_normalized: PathBuf = PathBuf::from(&source_base_dir).components().collect();
    
    for source_path in &source_paths {
        let source = Path::new(source_path);
        let source_normalized: PathBuf = source.components().collect();
        
        // Calculate relative path from source base (with normalized paths)
        let rel_path = source_normalized.strip_prefix(&base_normalized)
            .map_err(|e| {
                vfx_error(&format!("strip_prefix failed: source={}, base={}", source_path, source_base_dir));
                format!("[VFX] Failed to get relative path: {}", e)
            })?;
        
        let dest_path = Path::new(&dest_base_dir).join(rel_path);
        
        // Create parent directory
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("[VFX] Failed to create dir: {}", e))?;
        }
        
        // Copy the .uasset file
        std::fs::copy(source, &dest_path)
            .map_err(|e| format!("[VFX] Failed to copy {}: {}", source_path, e))?;
        
        // Copy companion files (.uexp, .ubulk, .uptnl) if they exist
        let stem = source.with_extension("");
        for ext in &["uexp", "ubulk", "uptnl"] {
            let companion = stem.with_extension(ext);
            if companion.exists() {
                let dest_companion = Path::new(&dest_base_dir)
                    .join(rel_path.with_extension(ext));
                std::fs::copy(&companion, &dest_companion).ok();
            }
        }
        
        copied.push(dest_path.to_string_lossy().to_string());
    }
    
    vfx_info(&format!("Copied {} non-updatable assets to output", copied.len()));
    Ok(copied)
}

// ============================================================================
// JSON File Operations
// ============================================================================

#[tauri::command]
pub async fn vfx_read_json_file(path: String) -> Result<String, String> {
    read_json_file(&path)
}

#[tauri::command]
pub async fn vfx_write_json_file(path: String, content: String) -> Result<(), String> {
    write_json_file(&path, &content)
}

#[tauri::command]
pub async fn vfx_list_json_files(dir: String) -> Result<Vec<String>, String> {
    list_json_files(&dir)
}

// ============================================================================
// Settings (VFX-specific)
// ============================================================================

#[tauri::command]
pub fn vfx_get_settings(_app: AppHandle) -> Result<VfxSettings, String> {
    let settings_path = vfx_app_dir().join("vfx_settings.json");
    
    if settings_path.exists() {
        let content = std::fs::read_to_string(&settings_path)
            .map_err(|e| format!("[VFX] Failed to read settings: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("[VFX] Failed to parse settings: {}", e))
    } else {
        Ok(VfxSettings::default())
    }
}

#[tauri::command]
pub fn vfx_save_settings(_app: AppHandle, settings: VfxSettings) -> Result<(), String> {
    let app_dir = vfx_app_dir();
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| format!("[VFX] Failed to create app data dir: {}", e))?;
    
    let settings_path = app_dir.join("vfx_settings.json");
    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("[VFX] Failed to serialize settings: {}", e))?;
    
    std::fs::write(&settings_path, content)
        .map_err(|e| format!("[VFX] Failed to write settings: {}", e))?;
    
    vfx_info(&format!("Settings saved to {:?}", settings_path));
    Ok(())
}
