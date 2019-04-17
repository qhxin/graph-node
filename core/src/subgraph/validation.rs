use graph::prelude::*;

pub fn validate_manifest(manifest: SubgraphManifest) -> Result<SubgraphManifest, SubgraphRegistrarError> {
    // Validate that the manifest has a `source` address in each data source
    // which has call or block handlers
    let has_invalid_data_source = manifest
        .data_sources
        .iter()
        .any(|data_source| {
            let no_source_address = data_source
                .source
                .address
                .is_none();
            let has_call_handlers = !data_source
                .mapping
                .call_handlers
                .is_empty();
            let has_block_handlers = !data_source
                .mapping
                .block_handlers
                .is_empty();
            no_source_address && (has_call_handlers || has_block_handlers)
        });
    if has_invalid_data_source {
        return Err(SubgraphRegistrarError::ManifestValidationError(
            SubgraphManifestValidationError::SourceAddressRequired
        ))
    }

    // Validate that each block handler filter has an accepted type
    let has_invalid_block_filter = manifest
        .data_sources
        .iter()
        .any(|data_source| {
            if data_source.mapping.block_handlers.is_empty() {
                return false
            }
            data_source
                .mapping
                .block_handlers
                .iter()
                .any(|block_handler| {
                    if block_handler.filter.is_none() {
                        return false
                    }
                    !block_handler.filter.clone().unwrap().is_kind_call()
                })
        });
    if has_invalid_block_filter {
        return Err(SubgraphRegistrarError::ManifestValidationError(
            SubgraphManifestValidationError::InvalidBlockHandlerFilter
        ))
    }

    let has_too_many_block_handlers = manifest
        .data_sources
        .iter()
        .any(|data_source| {
            if data_source.mapping.block_handlers.is_empty() {
                return false
            }
            let mut non_filtered_block_handler_count = 0;
            let mut call_filtered_block_handler_count = 0;
            data_source
                .mapping
                .block_handlers
                .iter()
                .for_each(|block_handler| {
                    if block_handler.filter.is_none() {
                        non_filtered_block_handler_count += 1
                    }
                    call_filtered_block_handler_count += 1
                });
            return non_filtered_block_handler_count > 1 || call_filtered_block_handler_count > 1
        });
    if has_too_many_block_handlers {
        return Err(SubgraphRegistrarError::ManifestValidationError(
            SubgraphManifestValidationError::DataSourceBlockHandlerLimitExceeded
        ))
    }
    
    Ok(manifest)
}
