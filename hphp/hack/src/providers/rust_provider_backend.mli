(*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)

type t

val make : ParserOptions.t -> t

(** Initialize with a given [Rust_provider_backend.t] value (constructed on the
    Rust side) instead of the default backend constructed by [make]. *)
val set : t -> unit

val push_local_changes : t -> unit

val pop_local_changes : t -> unit

module Decl : sig
  val direct_decl_parse_and_cache :
    t -> Relative_path.t -> string -> Direct_decl_parser.parsed_file_with_hashes

  (** Directly add decls to the underlying store without processing them (no
      removing php_stdlib decls, deduping, or removing naming conflict losers) *)
  val add_shallow_decls : t -> (string * Shallow_decl_defs.decl) list -> unit

  val get_fun : t -> string -> Shallow_decl_defs.fun_decl option

  val get_shallow_class : t -> string -> Shallow_decl_defs.class_decl option

  val get_typedef : t -> string -> Shallow_decl_defs.typedef_decl option

  val get_gconst : t -> string -> Shallow_decl_defs.const_decl option

  val get_module : t -> string -> Shallow_decl_defs.module_decl option

  val get_folded_class : t -> string -> Decl_defs.decl_class_type option

  val declare_folded_class : t -> string -> unit

  val get_old_shallow_classes_batch :
    t -> string list -> Shallow_decl_defs.class_decl option SMap.t

  val get_old_defs :
    t ->
    FileInfo.names ->
    Shallow_decl_defs.fun_decl option SMap.t
    * Shallow_decl_defs.typedef_decl option SMap.t
    * Shallow_decl_defs.const_decl option SMap.t
    * Shallow_decl_defs.module_decl option SMap.t

  val oldify_defs : t -> FileInfo.names * Decl_class_elements.t SMap.t -> unit

  val remove_defs : t -> FileInfo.names * Decl_class_elements.t SMap.t -> unit

  val remove_old_defs :
    t -> FileInfo.names * Decl_class_elements.t SMap.t -> unit
end

module File : sig
  type file_type =
    | Disk of string
    | Ide of string

  val get : t -> Relative_path.t -> file_type option

  val get_contents : t -> Relative_path.t -> string

  val provide_file_for_tests : t -> Relative_path.t -> string -> unit

  val provide_file_for_ide : t -> Relative_path.t -> string -> unit

  val provide_file_hint : t -> Relative_path.t -> file_type -> unit

  val remove_batch : t -> Relative_path.Set.t -> unit
end

module Naming : sig
  module type ReverseNamingTable = sig
    type pos

    val add : t -> string -> pos -> unit

    val get_pos : t -> string -> pos option

    val remove_batch : t -> string list -> unit
  end

  module Types : sig
    include
      ReverseNamingTable
        with type pos = FileInfo.pos * Naming_types.kind_of_type

    val get_canon_name : t -> string -> string option
  end

  module Funs : sig
    include ReverseNamingTable with type pos = FileInfo.pos

    val get_canon_name : t -> string -> string option
  end

  module Consts : sig
    include ReverseNamingTable with type pos = FileInfo.pos
  end

  module Modules : sig
    include ReverseNamingTable with type pos = FileInfo.pos
  end

  val get_db_path : t -> Naming_sqlite.db_path option

  val set_db_path : t -> Naming_sqlite.db_path -> unit

  (** This function searches all three namespaces (types, funs, consts) to
      find which one contains each Dep.t. The earlier functions in this module
      only search one specified namespace. Note: this function doesn't
      use the sharedmem cache of names - doesn't benefit from it, doesn't
      write into it. *)
  val get_filenames_by_hash : t -> Typing_deps.DepSet.t -> Relative_path.Set.t
end
