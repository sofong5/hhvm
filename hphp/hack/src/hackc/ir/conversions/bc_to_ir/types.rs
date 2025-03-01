// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use ffi::Maybe;
use ffi::Str;
use hhbc::TypeInfo;
use ir::BaseType;
use ir::ClassId;
use ir::StringInterner;
use ir::UserType;
use maplit::hashmap;
use once_cell::sync::OnceCell;

pub(crate) fn convert_type<'a>(ty: &TypeInfo<'a>, strings: &StringInterner) -> UserType {
    let user_type = ty.user_type.into_option();
    let name = ty.type_constraint.name.into_option();
    let constraint_ty = if let Some(name) = name {
        cvt_constraint_type(name, strings)
    } else {
        user_type
            .as_ref()
            .and_then(|user_type| {
                use std::collections::HashMap;
                static UNCONSTRAINED_BY_NAME: OnceCell<HashMap<Str<'static>, BaseType>> =
                    OnceCell::new();
                let unconstrained_by_name = UNCONSTRAINED_BY_NAME.get_or_init(|| {
                    hashmap! {
                        ir::types::BUILTIN_NAME_VOID => BaseType::Void,
                        ir::types::BUILTIN_NAME_SOFT_VOID => BaseType::Void,
                    }
                });

                unconstrained_by_name.get(user_type).cloned()
            })
            .unwrap_or(BaseType::Mixed)
    };

    UserType {
        user_type: user_type.map(|u| strings.intern_bytes(u.as_ref())),
        enforced: ir::EnforceableType {
            ty: constraint_ty,
            modifiers: ty.type_constraint.flags,
        },
    }
}

pub(crate) fn convert_maybe_type<'a>(
    ty: Maybe<&TypeInfo<'a>>,
    strings: &StringInterner,
) -> ir::UserType {
    match ty {
        Maybe::Just(ty) => convert_type(ty, strings),
        Maybe::Nothing => ir::UserType::empty(),
    }
}

fn cvt_constraint_type<'a>(name: Str<'a>, strings: &StringInterner) -> BaseType {
    use std::collections::HashMap;
    static CONSTRAINT_BY_NAME: OnceCell<HashMap<Str<'static>, BaseType>> = OnceCell::new();
    let constraint_by_name = CONSTRAINT_BY_NAME.get_or_init(|| {
        hashmap! {
            ir::types::BUILTIN_NAME_ANY_ARRAY => BaseType::AnyArray,
            ir::types::BUILTIN_NAME_ARRAYKEY => BaseType::Arraykey,
            ir::types::BUILTIN_NAME_BOOL => BaseType::Bool,
            ir::types::BUILTIN_NAME_CLASSNAME => BaseType::Classname,
            ir::types::BUILTIN_NAME_DARRAY => BaseType::Darray,
            ir::types::BUILTIN_NAME_DICT => BaseType::Dict,
            ir::types::BUILTIN_NAME_FLOAT => BaseType::Float,
            ir::types::BUILTIN_NAME_INT => BaseType::Int,
            ir::types::BUILTIN_NAME_KEYSET => BaseType::Keyset,
            ir::types::BUILTIN_NAME_NONNULL => BaseType::Nonnull,
            ir::types::BUILTIN_NAME_NORETURN => BaseType::Noreturn,
            ir::types::BUILTIN_NAME_NOTHING => BaseType::Nothing,
            ir::types::BUILTIN_NAME_NULL => BaseType::Null,
            ir::types::BUILTIN_NAME_NUM => BaseType::Num,
            ir::types::BUILTIN_NAME_RESOURCE => BaseType::Resource,
            ir::types::BUILTIN_NAME_STRING => BaseType::String,
            ir::types::BUILTIN_NAME_THIS => BaseType::This,
            ir::types::BUILTIN_NAME_TYPENAME => BaseType::Typename,
            ir::types::BUILTIN_NAME_VARRAY => BaseType::Varray,
            ir::types::BUILTIN_NAME_VARRAY_OR_DARRAY => BaseType::VarrayOrDarray,
            ir::types::BUILTIN_NAME_VEC => BaseType::Vec,
            ir::types::BUILTIN_NAME_VEC_OR_DICT => BaseType::VecOrDict,
        }
    });

    constraint_by_name.get(&name).cloned().unwrap_or_else(|| {
        let name = ClassId::new(strings.intern_bytes(name.as_ref()));
        BaseType::Class(name)
    })
}
