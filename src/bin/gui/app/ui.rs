// UIモジュールの公開窓口。各UI部品はAppActionを発行し、
// 設定変更はController、描画要求はComputeManager/ExportManagerに委譲する。
pub mod export;
pub mod panels;
pub mod settings;
pub mod formula;