// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use crate::gen::global_options::GlobalOptions;
use crate::i_set;
use crate::s_map;
use crate::s_set;

const DEFAULT: GlobalOptions<'_> = GlobalOptions {
    tco_experimental_features: s_set::SSet::empty(),
    tco_migration_flags: s_set::SSet::empty(),
    tco_num_local_workers: None,
    tco_parallel_type_checking_threshold: 10,
    tco_max_typechecker_worker_memory_mb: None,
    tco_defer_class_declaration_threshold: None,
    tco_prefetch_deferred_files: false,
    tco_remote_type_check_threshold: 1_000_000,
    tco_remote_type_check: false,
    tco_remote_worker_key: None,
    tco_remote_check_id: None,
    tco_remote_max_batch_size: 8000,
    tco_remote_min_batch_size: 5000,
    tco_num_remote_workers: 0,
    so_remote_version_specifier: None,
    so_remote_worker_vfs_checkout_threshold: 0,
    so_naming_sqlite_path: None,
    po_auto_namespace_map: &[],
    po_codegen: false,
    po_deregister_php_stdlib: false,
    po_disallow_toplevel_requires: false,
    po_allow_unstable_features: false,
    tco_log_large_fanouts_threshold: None,
    tco_log_inference_constraints: false,
    tco_language_feature_logging: false,
    tco_timeout: 0,
    tco_disallow_invalid_arraykey: false,
    tco_disallow_byref_dynamic_calls: false,
    tco_disallow_byref_calls: true,
    allowed_fixme_codes_strict: i_set::ISet::empty(),
    log_levels: s_map::SMap::empty(),
    po_disable_lval_as_an_expression: false,
    tco_shallow_class_decl: false,
    tco_force_shallow_decl_fanout: false,
    tco_remote_old_decls_no_limit: false,
    tco_force_load_hot_shallow_decls: false,
    tco_fetch_remote_old_decls: false,
    tco_populate_member_heaps: true,
    tco_skip_hierarchy_checks: false,
    tco_skip_tast_checks: false,
    tco_like_type_hints: false,
    tco_union_intersection_type_hints: false,
    tco_coeffects: true,
    tco_coeffects_local: true,
    tco_strict_contexts: true,
    tco_like_casts: false,
    tco_simple_pessimize: 0.0,
    tco_complex_coercion: false,
    tco_check_xhp_attribute: false,
    tco_check_redundant_generics: false,
    tco_disallow_unresolved_type_variables: false,
    po_enable_class_level_where_clauses: false,
    po_disable_legacy_soft_typehints: true,
    po_allowed_decl_fixme_codes: i_set::ISet::empty(),
    po_allow_new_attribute_syntax: false,
    tco_global_inference: false,
    tco_gi_reinfer_types: &[],
    tco_ordered_solving: false,
    tco_const_static_props: false,
    po_disable_legacy_attribute_syntax: false,
    tco_const_attribute: false,
    po_const_default_func_args: false,
    po_const_default_lambda_args: false,
    po_disallow_silence: false,
    po_abstract_static_props: false,
    po_parser_errors_only: false,
    tco_check_attribute_locations: true,
    po_disallow_func_ptrs_in_constants: false,
    tco_error_php_lambdas: false,
    tco_disallow_discarded_nullable_awaitables: false,
    po_enable_xhp_class_modifier: false,
    po_disable_xhp_element_mangling: false,
    po_disable_xhp_children_declarations: false,
    glean_service: "",
    glean_hostname: "",
    glean_port: 0,
    glean_reponame: "",
    symbol_write_ownership: false,
    symbol_write_root_path: "",
    symbol_write_hhi_path: "",
    symbol_write_ignore_paths: &[],
    symbol_write_index_paths: &[],
    symbol_write_index_paths_file: None,
    symbol_write_include_hhi: true,
    symbol_write_index_paths_file_output: None,
    symbol_write_sym_hash_in: None,
    symbol_write_exclude_out: None,
    symbol_write_sym_hash_out: false,
    po_enable_enum_classes: true,
    po_disable_hh_ignore_error: 0,
    tco_is_systemlib: false,
    tco_higher_kinded_types: false,
    tco_method_call_inference: false,
    tco_report_pos_from_reason: false,
    tco_typecheck_sample_rate: 1.0,
    tco_enable_sound_dynamic: false,
    po_disallow_fun_and_cls_meth_pseudo_funcs: false,
    po_disallow_inst_meth: false,
    tco_use_direct_decl_parser: true,
    tco_ifc_enabled: &[],
    tco_global_access_check_files_enabled: &[],
    tco_global_access_check_functions_enabled: s_set::SSet::empty(),
    tco_global_access_check_on_write: true,
    tco_global_access_check_on_read: true,
    po_enable_enum_supertyping: true,
    po_interpret_soft_types_as_like_types: false,
    tco_enable_strict_string_concat_interp: false,
    tco_ignore_unsafe_cast: false,
    tco_no_parser_readonly_check: false,
    tco_enable_expression_trees: false,
    tco_enable_modules: false,
    tco_allowed_expression_tree_visitors: &[],
    tco_math_new_code: false,
    tco_typeconst_concrete_concrete_error: false,
    tco_enable_strict_const_semantics: 0,
    tco_strict_wellformedness: 0,
    tco_meth_caller_only_public_visibility: true,
    tco_require_extends_implements_ancestors: false,
    tco_strict_value_equality: false,
    tco_enforce_sealed_subclasses: false,
    tco_everything_sdt: false,
    tco_pessimise_builtins: false,
    tco_explicit_consistent_constructors: 0,
    tco_require_types_class_consts: 0,
    tco_type_printer_fuel: 100,
    tco_log_saved_state_age_and_distance: false,
    tco_specify_manifold_api_key: false,
    tco_saved_state_manifold_api_key: None,
    tco_profile_top_level_definitions: false,
    tco_allow_all_files_for_module_declarations: false,
    tco_allowed_files_for_module_declarations: &[],
    tco_use_manifold_cython_client: false,
    tco_record_fine_grained_dependencies: false,
    tco_loop_iteration_upper_bound: None,
    tco_expression_tree_virtualize_functions: false,
    tco_substitution_mutation: false,
    tco_use_type_alias_heap: false,
};

impl GlobalOptions<'static> {
    pub const DEFAULT: &'static Self = &DEFAULT;

    pub const fn default_ref() -> &'static Self {
        Self::DEFAULT
    }
}

impl Default for &GlobalOptions<'_> {
    fn default() -> Self {
        GlobalOptions::default_ref()
    }
}

impl Eq for GlobalOptions<'_> {}

impl std::hash::Hash for GlobalOptions<'_> {
    fn hash<H>(&self, _: &mut H) {
        unimplemented!()
    }
}

impl no_pos_hash::NoPosHash for GlobalOptions<'_> {
    fn hash<H>(&self, _: &mut H) {
        unimplemented!()
    }
}

impl Ord for GlobalOptions<'_> {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        unimplemented!()
    }
}
