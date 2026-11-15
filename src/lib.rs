/// Skyvia integration pipeline and mapping model for SigmaOS cloud ETL.

#[derive(Debug, Clone)]
pub struct FieldMapping {
    pub source_field: String,
    pub target_field: String,
    pub transform: Option<String>,
}

impl FieldMapping {
    pub fn direct(src: &str, tgt: &str) -> Self {
        Self {
            source_field: src.into(),
            target_field: tgt.into(),
            transform: None,
        }
    }

    pub fn with_transform(mut self, t: &str) -> Self {
        self.transform = Some(t.into());
        self
    }

    pub fn has_transform(&self) -> bool {
        self.transform.is_some()
    }

    pub fn describe(&self) -> String {
        match &self.transform {
            Some(t) => format!("{} -> {} [{}]", self.source_field, self.target_field, t),
            None => format!("{} -> {}", self.source_field, self.target_field),
        }
    }
}

#[derive(Debug, Default)]
pub struct IntegrationPackage {
    pub name: String,
    pub mappings: Vec<FieldMapping>,
}

impl IntegrationPackage {
    pub fn add_mapping(&mut self, m: FieldMapping) {
        self.mappings.push(m);
    }
    pub fn mapping_count(&self) -> usize {
        self.mappings.len()
    }
    pub fn transformed_count(&self) -> usize {
        self.mappings.iter().filter(|m| m.has_transform()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_mapping_no_transform_aaa() {
        let m = FieldMapping::direct("first_name", "FirstName");
        assert!(!m.has_transform());
    }

    #[test]
    fn test_mapping_with_transform_aaa() {
        let m = FieldMapping::direct("dob", "BirthDate").with_transform("FORMAT_DATE");
        assert!(m.has_transform());
    }

    #[test]
    fn test_describe_with_transform_aaa() {
        let m = FieldMapping::direct("email", "Email").with_transform("LOWER");
        assert!(m.describe().contains("LOWER"));
    }

    #[test]
    fn test_package_counts_aaa() {
        let mut pkg = IntegrationPackage {
            name: "CRM sync".into(),
            mappings: Vec::new(),
        };
        pkg.add_mapping(FieldMapping::direct("a", "A"));
        pkg.add_mapping(FieldMapping::direct("b", "B").with_transform("TRIM"));
        assert_eq!(pkg.mapping_count(), 2);
        assert_eq!(pkg.transformed_count(), 1);
    }
}
