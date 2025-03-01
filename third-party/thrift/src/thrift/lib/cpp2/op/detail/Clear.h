/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <cmath>

#include <folly/CPortability.h>
#include <folly/Portability.h>
#include <folly/io/IOBuf.h>
#include <folly/memory/SanitizeLeak.h>
#include <thrift/lib/cpp2/Adapt.h>
#include <thrift/lib/cpp2/FieldRef.h>
#include <thrift/lib/cpp2/Thrift.h>
#include <thrift/lib/cpp2/op/Compare.h>
#include <thrift/lib/cpp2/op/Create.h>
#include <thrift/lib/cpp2/type/NativeType.h>
#include <thrift/lib/cpp2/type/Tag.h>
#include <thrift/lib/cpp2/type/ThriftType.h>

namespace apache {
namespace thrift {
namespace op {
namespace detail {

// Helper that creates a leaky singleton.
template <typename F>
FOLLY_EXPORT const auto& staticDefault(F&& make) {
  static const auto& kVal = *make().release();
  return kVal;
}

// Gets a (potentally const&) value representing the default.
//
// C++'s default for the underlying native type, is the default for all
// unstructured types.
template <typename Tag, typename = void>
struct GetDefault {
  static_assert(!type::is_a_v<Tag, type::structured_c>, "");
  constexpr auto operator()() const { return type::native_type<Tag>{}; }
};

// Gets a (potentally const&) value representing the **intrinsic** default.
//
// This is the same as the default for all non-structured types.
template <typename Tag, typename = void>
struct GetIntrinsicDefault : GetDefault<Tag> {
  static_assert(!type::is_a_v<Tag, type::structured_c>, "");
};

// Clear the given value, setting it to it's intrinsic default.
template <typename TagOrId, typename PTag = void, typename = void>
struct Clear {
  using Tag = TagOrId;
  constexpr void operator()(type::native_type<Tag>& val) const {
    clear(val, Tag{});
  }

 private:
  template <typename T>
  static void clear(T& val, type::structured_c) {
    thrift::clear(val);
  }
  template <typename T>
  static void clear(T& val, type::container_c) {
    val.clear();
  }
  template <typename T>
  constexpr static void clear(T& val, type::all_c) {
    val = GetIntrinsicDefault<Tag>{}();
  }
};
template <typename Id>
struct Clear<Id, type::if_id<Id>, type::if_not_thrift_type_tag<Id>> {
  template <typename T>
  constexpr void operator()(T& val) const {
    Clear<Id, type::infer_tag<T>>{}(val);
  }
};
template <>
struct Clear<void> {
  template <typename Id, typename T>
  constexpr void operator()(Id, T& val) const {
    Clear<Id, type::infer_tag<T>>{}(val);
  }
  template <typename T>
  constexpr void operator()(T& val) const {
    Clear<type::infer_tag<T>>{}(val);
  }
};

// Checks if the given value is empty.
template <typename Tag>
struct IsEmpty {
  constexpr bool operator()(const type::native_type<Tag>& value) const {
    return empty(value, Tag{});
  }

 private:
  template <typename T>
  static bool empty(const T& val, type::structured_c) {
    return thrift::empty(val);
  }
  template <typename T>
  static bool empty(const T& val, type::container_c) {
    return val.empty();
  }
  template <typename T>
  static bool empty(const T& val, type::string_c) {
    return val.empty();
  }
  template <typename T>
  constexpr static bool empty(const T& val, type::all_c) {
    return op::identical<Tag>(val, GetIntrinsicDefault<Tag>{}());
  }
  template <typename ATag>
  static bool empty(const std::unique_ptr<folly::IOBuf>& val, ATag tag) {
    return val == nullptr || empty(*val, tag);
  }
};
template <>
struct IsEmpty<void> {
  template <typename T>
  constexpr bool operator()(T& val) const {
    return IsEmpty<type::infer_tag<T>>{}(val);
  }
};

// Gets the intrinsic default via `thrift::clear`
//
// Useful for any type that supports custom defaults.
template <typename Tag>
struct ThriftClearDefault {
  const auto& operator()() const {
    return staticDefault([] {
      auto value = std::make_unique<type::native_type<Tag>>();
      apache::thrift::clear(*value);
      return value;
    });
  }
};

// Gets the intrinsic default via `op::create`
template <typename Tag>
struct CreateDefault {
  const auto& operator()() const {
    return staticDefault([] {
      return std::make_unique<type::native_type<Tag>>(op::create<Tag>());
    });
  }
};

// Structured types have statically addressable defaults ...
template <typename T>
struct GetDefault<type::struct_t<T>> : CreateDefault<type::struct_t<T>> {};
template <typename T>
struct GetDefault<type::exception_t<T>> : CreateDefault<type::exception_t<T>> {
};
template <typename T>
struct GetDefault<type::union_t<T>> : CreateDefault<type::union_t<T>> {};
// ... and need to be cleared to get the intrinsic default.
template <typename T>
struct GetIntrinsicDefault<type::struct_t<T>>
    : ThriftClearDefault<type::struct_t<T>> {};
template <typename T>
struct GetIntrinsicDefault<type::exception_t<T>>
    : ThriftClearDefault<type::exception_t<T>> {};
// TODO(afuller): Is thrift::clear actually needed for union?
template <typename T>
struct GetIntrinsicDefault<type::union_t<T>>
    : ThriftClearDefault<type::union_t<T>> {};

template <typename Adapter, typename UTag, typename Struct, int16_t id>
using adapted_field_tag =
    type::field<type::adapted<Adapter, UTag>, FieldContext<Struct, id>>;

// Cache the result of op::create for adapters.
template <typename Adapter, typename UTag>
struct GetIntrinsicDefault<type::adapted<Adapter, UTag>>
    : CreateDefault<type::adapted<Adapter, UTag>> {};
template <typename Adapter, typename UTag, typename Struct, int16_t id>
struct GetIntrinsicDefault<adapted_field_tag<Adapter, UTag, Struct, id>> {
  using Tag = adapted_field_tag<Adapter, UTag, Struct, id>;
  const auto& operator()() const {
    return staticDefault([] {
      // TODO(afuller): Remove or move this logic to the adapter.
      auto& obj = *new Struct{};
      folly::lsan_ignore_object(&obj);
      apache::thrift::clear(obj);
      return std::make_unique<type::native_type<Tag>>(op::create<Tag>(obj));
    });
  }
};

// Delegate to adapter.
template <typename Adapter, typename UTag>
struct Clear<type::adapted<Adapter, UTag>> {
  using Tag = type::adapted<Adapter, UTag>;
  constexpr void operator()(type::native_type<Tag>& value) const {
    adapt_detail::clear<Adapter>(value);
  }
};

// Delegate to op::identical.
template <typename Adapter, typename UTag>
struct IsEmpty<type::adapted<Adapter, UTag>> {
  using Tag = type::adapted<Adapter, UTag>;
  constexpr bool operator()(const type::native_type<Tag>& value) const {
    return op::identical<Tag>(value, GetIntrinsicDefault<Tag>{}());
  }
};
template <typename Adapter, typename UTag, typename Struct, int16_t id>
struct IsEmpty<adapted_field_tag<Adapter, UTag, Struct, id>> {
  using Tag = adapted_field_tag<Adapter, UTag, Struct, id>;
  constexpr bool operator()(const type::native_type<Tag>& value) const {
    return op::identical<Tag>(value, GetIntrinsicDefault<Tag>{}());
  }
};

// TODO(dokwon): Support field_ref types.
template <typename Tag, typename Context>
struct GetIntrinsicDefault<type::field<Tag, Context>>
    : GetIntrinsicDefault<Tag> {};
template <typename Tag, typename Context>
struct IsEmpty<type::field<Tag, Context>> : IsEmpty<Tag> {};

struct ClearOptionalField {
  template <typename T, typename Struct>
  void operator()(optional_boxed_field_ref<T> field, Struct&) const {
    field.reset();
  }
  template <typename T, typename Struct>
  void operator()(optional_field_ref<T> field, Struct&) const {
    field.reset();
  }
  template <typename T, typename Union>
  void operator()(union_field_ref<T> field, Union& u) const {
    if (field.has_value()) {
      thrift::clear(u);
      assert(!field.has_value());
    }
  }
  template <typename T, typename Struct>
  void operator()(std::shared_ptr<T>& field, Struct&) const {
    field = nullptr;
  }
  template <typename T, typename Struct>
  void operator()(std::unique_ptr<T>& field, Struct&) const {
    field = nullptr;
  }
};

template <typename>
struct ClearField {};

template <typename Tag, typename Context>
struct ClearField<type::field<Tag, Context>> : ClearOptionalField {
  using ClearOptionalField::operator();

  template <typename T, typename Struct>
  void operator()(required_field_ref<T> field, Struct&) const {
    Clear<Tag>{}(*field);
  }

  template <typename T, typename Struct>
  void operator()(terse_field_ref<T> field, Struct&) const {
    Clear<Tag>{}(*field);
  }

  template <typename T, typename Struct>
  void operator()(field_ref<T> field, Struct&) const {
    Clear<Tag>{}(*field);
  }
};

template <typename Adapter, typename UTag, typename Struct, int16_t id>
struct ClearField<adapted_field_tag<Adapter, UTag, Struct, id>>
    : ClearOptionalField {
  using Tag = adapted_field_tag<Adapter, UTag, Struct, id>;
  static_assert(type::is_concrete_v<Tag>, "");

  using ClearOptionalField::operator();
  template <typename T>
  void operator()(required_field_ref<T> field, Struct& s) const {
    ::apache::thrift::adapt_detail::clear<Adapter, id>(*field, s);
  }

  template <typename T>
  void operator()(terse_field_ref<T> field, Struct& s) const {
    ::apache::thrift::adapt_detail::clear<Adapter, id>(*field, s);
  }

  template <typename T>
  void operator()(field_ref<T> field, Struct& s) const {
    ::apache::thrift::adapt_detail::clear<Adapter, id>(*field, s);
  }
};

template <typename Id, typename Tag>
struct Clear<Id, Tag, type::if_thrift_type_tag<Tag>> {
  static_assert(type::is_id_v<Id>, "");
  using T = type::native_type<Tag>;
  constexpr void operator()(T& val) const {
    using FieldTag = op::get_field_tag<Id, T>;
    ClearField<FieldTag>{}(op::get<Id, Tag>(val), val);
  }
};

} // namespace detail
} // namespace op
} // namespace thrift
} // namespace apache
