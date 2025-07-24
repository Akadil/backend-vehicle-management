use super::{
    dto::{SearchMaintenanceTypesQuery as Input, SearchMaintenanceTypesResponse as Output, MaintenanceTypeSearchResult},
    error::SearchMaintenanceTypesError as Error,
};
use domain::maintenance::repositories::maintenance_type_repository::MaintenanceTypeRepository;

pub struct SearchMaintenanceTypesUseCase<'a, MTR: MaintenanceTypeRepository + 'a> {
    maintenance_type_repository: &'a MTR,
}

impl<'a, MTR: MaintenanceTypeRepository + 'a> SearchMaintenanceTypesUseCase<'a, MTR> {
    pub fn new(maintenance_type_repository: &'a MTR) -> Self {
        SearchMaintenanceTypesUseCase {
            maintenance_type_repository,
        }
    }

    pub async fn execute(&self, query: Input) -> Result<Output, Error> {
        if query.search_term.trim().is_empty() {
            return Err(Error::EmptySearchTerm);
        }

        // Get all maintenance types and filter them in-memory
        // In a real implementation, you might want to add a search method to the repository
        let all_maintenance_types = self
            .maintenance_type_repository
            .get_all_view()
            .await?;

        let search_term_lower = query.search_term.to_lowercase();
        let mut filtered_results: Vec<MaintenanceTypeSearchResult> = all_maintenance_types
            .into_iter()
            .filter(|mt| {
                mt.name.to_lowercase().contains(&search_term_lower) ||
                mt.description.to_lowercase().contains(&search_term_lower)
            })
            .map(MaintenanceTypeSearchResult::from)
            .collect();

        // Apply limit if specified
        if let Some(limit) = query.limit {
            filtered_results.truncate(limit);
        }

        let total_found = filtered_results.len();

        Ok(Output {
            results: filtered_results,
            total_found,
            search_term: query.search_term,
        })
    }
}
