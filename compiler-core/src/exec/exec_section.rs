use celerctypes::ExecSection;

use crate::CompSection;

use super::MapSectionBuilder;

impl CompSection {
    /// Execute the section.
    ///
    /// Map features will be added to the builder
    pub fn exec(
        self, 
        section_number: usize,
        map_builder: &mut MapSectionBuilder
    ) -> ExecSection {
        ExecSection {
            name: self.name,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod ut {
    use super::*;

    #[test]
    fn test_name() {
        let test_section = CompSection {
            name: "test".to_string(),
            ..Default::default()
        };
        let exec_section = test_section.exec(1, &mut MapSectionBuilder::default());

        assert_eq!(exec_section.name, "test");
    }
}
