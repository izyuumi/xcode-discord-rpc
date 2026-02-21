/// Represents the different file languages supported
#[derive(Debug)]
pub enum FileLanguage {
    /// Swift
    Swift,
    /// C++
    Cpp,
    /// C
    C,
    /// Ruby
    Ruby,
    /// Java
    Java,
    /// Json
    Json,
    /// Metal
    Metal,
    /// Objective-C
    ObjectiveC,
    /// Python
    Python,
    /// TypeScript
    TypeScript,
    /// JavaScript
    JavaScript,
    /// Markdown
    Markdown,
    /// YAML
    Yaml,
    /// Shell script
    Shell,
    /// TOML
    Toml,
    /// Storyboard / XIB (Interface Builder)
    InterfaceBuilder,
    /// Property list
    Plist,
    /// Makefile
    Makefile,
    /// Unknown or unsupported
    Unknown,
}

impl FileLanguage {
    /// Returns the asset key and text for the `FileLanguage` as (text, image)
    pub fn get_asset_keys(&self) -> (&'static str, &'static str) {
        match self {
            FileLanguage::Swift => ("Swift", "swift"),
            FileLanguage::Cpp => ("C++", "cpp"),
            FileLanguage::C => ("C", "c"),
            FileLanguage::Ruby => ("Ruby", "ruby"),
            FileLanguage::Java => ("Java", "java"),
            FileLanguage::Json => ("Json", "jSON"),
            FileLanguage::Metal => ("Metal", "metal"),
            FileLanguage::ObjectiveC => ("Objective-C", "objc"),
            FileLanguage::Python => ("Python", "python"),
            FileLanguage::TypeScript => ("TypeScript", "typescript"),
            FileLanguage::JavaScript => ("JavaScript", "javascript"),
            FileLanguage::Markdown => ("Markdown", "markdown"),
            FileLanguage::Yaml => ("YAML", "yaml"),
            FileLanguage::Shell => ("Shell", "shell"),
            FileLanguage::Toml => ("TOML", "toml"),
            FileLanguage::InterfaceBuilder => ("Interface Builder", "xcode"),
            FileLanguage::Plist => ("Property List", "xcode"),
            FileLanguage::Makefile => ("Makefile", "xcode"),
            FileLanguage::Unknown => ("Xcode", "xcode"),
        }
    }

    /// Returns the text for the `FileLanguage`
    pub fn get_text_asset_key(&self) -> &'static str {
        self.get_asset_keys().0
    }

    /// Returns the asset key for the `FileLanguage`
    pub fn get_image_asset_key(&self) -> &'static str {
        self.get_asset_keys().1
    }
}

/// Trait for converting types to `FileLanguage`
pub trait ToFileLanguage {
    /// Converts the implementing type to a `FileLanguage`
    fn to_file_language(&self) -> FileLanguage;
}

/// Implementation of `ToFileLanguage` for `str`
impl ToFileLanguage for str {
    /// Converts `str` to the corresponding `FileLanguage`
    fn to_file_language(&self) -> FileLanguage {
        match self {
            "swift" => FileLanguage::Swift,
            "cpp" | "cp" | "cxx" | "cc" | "hpp" | "hxx" => FileLanguage::Cpp,
            "c" => FileLanguage::C,
            "m" | "mm" | "h" => FileLanguage::ObjectiveC,
            "ruby" | "rb" => FileLanguage::Ruby,
            "java" => FileLanguage::Java,
            "json" => FileLanguage::Json,
            "metal" => FileLanguage::Metal,
            "py" | "python" => FileLanguage::Python,
            "ts" | "tsx" => FileLanguage::TypeScript,
            "js" | "jsx" | "mjs" | "cjs" => FileLanguage::JavaScript,
            "md" | "markdown" => FileLanguage::Markdown,
            "yml" | "yaml" => FileLanguage::Yaml,
            "sh" | "bash" | "zsh" | "fish" => FileLanguage::Shell,
            "toml" => FileLanguage::Toml,
            "storyboard" | "xib" => FileLanguage::InterfaceBuilder,
            "plist" => FileLanguage::Plist,
            "Makefile" => FileLanguage::Makefile,
            _ => FileLanguage::Unknown,
        }
    }
}

/// Trait for getting file extensions
pub trait FileExtention {
    fn get_file_extension(&self) -> String;
}

/// Implementation of `FileExtention` for `str`
impl FileExtention for str {
    /// Returns the file extension of the string
    fn get_file_extension(&self) -> String {
        self.split('.').next_back().unwrap_or("").trim().to_string()
    }
}
