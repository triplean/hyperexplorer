use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct FileResult {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub score: f64,
}

#[derive(Default)]
pub struct FileSearcher {
    files: Vec<FileResult>,
    name_index: HashMap<String, Vec<usize>>,
    extension_index: HashMap<String, Vec<usize>>,
}

impl FileSearcher {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            name_index: HashMap::new(),
            extension_index: HashMap::new(),
        }
    }

    pub fn index(&mut self, path: &str, recursive: bool) -> Result<usize, Box<dyn std::error::Error>> {
        self.clear();

        if recursive {
            self.scan_recursive(Path::new(path))?;
        } else {
            self.scan_directory(Path::new(path))?;
        }

        self.build_index();
        Ok(self.files.len())
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<FileResult> {
        if query.is_empty() {
            return Vec::new();
        }

        let query_lower = query.to_lowercase();
        let mut scored_results: Vec<FileResult> = Vec::new();

        for (i, file) in self.files.iter().enumerate() {
            if let Some(score) = self.calculate_score(&file.name, &query_lower) {
                let mut result = file.clone();
                result.score = score;
                scored_results.push(result);
            }
        }

        scored_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        scored_results.truncate(limit);

        scored_results
    }


    // We need to implement this at some point
    pub fn search_by_extension(&self, ext: &str, limit: usize) -> Vec<FileResult> {
        let ext_clean = ext.trim_start_matches('.').to_lowercase();

        if let Some(indices) = self.extension_index.get(&ext_clean) {
            indices.iter()
                .take(limit)
                .map(|&i| {
                    let mut result = self.files[i].clone();
                    result.score = 100.0;
                    result
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        let total_files = self.files.len();
        let directories = self.files.iter().filter(|f| f.is_dir).count();
        let files = total_files - directories;
        (total_files, directories, files)
    }

    pub fn search_timed(&self, query: &str, limit: usize) -> (Vec<FileResult>, f64) {
        let start = Instant::now();
        let results = self.search(query, limit);
        let elapsed_ms = start.elapsed().as_nanos() as f64 / 1_000_000.0;
        (results, elapsed_ms)
    }

    fn clear(&mut self) {
        self.files.clear();
        self.name_index.clear();
        self.extension_index.clear();
    }

    fn scan_directory(&mut self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if let Ok(metadata) = entry.metadata() {
                let file_result = FileResult {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path(),
                    size: metadata.len(),
                    is_dir: metadata.is_dir(),
                    score: 0.0,
                };
                self.files.push(file_result);
            }
        }
        Ok(())
    }

    fn scan_recursive(&mut self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Ok(metadata) = entry.metadata() {
                let file_result = FileResult {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: path.clone(),
                    size: metadata.len(),
                    is_dir: metadata.is_dir(),
                    score: 0.0,
                };
                self.files.push(file_result.clone());

                if metadata.is_dir() && !self.should_skip_dir(&file_result.name) {
                    let _ = self.scan_recursive(&path);
                }
            }
        }
        Ok(())
    }

    fn should_skip_dir(&self, dir_name: &str) -> bool {
        matches!(dir_name,
            ".git" | "node_modules" | "target" | ".vscode" | ".idea" |
            "__pycache__" | ".pytest_cache" | "dist" | "build"
        ) || dir_name.starts_with('.')
    }

    fn build_index(&mut self) {
        for (i, file) in self.files.iter().enumerate() {
            let name_lower = file.name.to_lowercase();

            self.name_index
                .entry(name_lower.clone())
                .or_default()
                .push(i);

            if let Some(ext) = Path::new(&file.name).extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                self.extension_index
                    .entry(ext_str)
                    .or_default()
                    .push(i);
            }

            for part in name_lower.split(&['-', '_', '.', ' ']) {
                if part.len() > 1 {
                    self.name_index
                        .entry(part.to_string())
                        .or_default()
                        .push(i);
                }
            }
        }
    }

    fn calculate_score(&self, filename: &str, query: &str) -> Option<f64> {
        let name_lower = filename.to_lowercase();

        if name_lower == query {
            return Some(100.0);
        }

        if name_lower.starts_with(query) {
            return Some(90.0);
        }

        if name_lower.contains(query) {
            return Some(80.0);
        }

        let similarity = self.fuzzy_similarity(query, &name_lower);
        if similarity > 0.6 {
            Some(similarity * 70.0)
        } else {
            None
        }
    }


    fn fuzzy_similarity(&self, pattern: &str, text: &str) -> f64 {
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let text_chars: Vec<char> = text.chars().collect();

        let mut matches = 0;
        let mut pattern_idx = 0;

        for text_char in text_chars {
            if pattern_idx < pattern_chars.len() && text_char == pattern_chars[pattern_idx] {
                matches += 1;
                pattern_idx += 1;
            }
        }

        if pattern_chars.is_empty() {
            0.0
        } else {
            matches as f64 / pattern_chars.len() as f64
        }
    }
}