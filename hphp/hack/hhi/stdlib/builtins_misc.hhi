<?hh /* -*- php -*- */
/**
 * Copyright (c) 2014, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 */

namespace HH {
function server_warmup_status(): string;
function execution_context()[read_globals]: string;

<<__SupportDynamicType>>
function array_mark_legacy<T>(T $in, bool $recursive = false)[]: T;
<<__SupportDynamicType>>
function array_unmark_legacy<T>(T $in, bool $recursive = false)[]: T;
<<__SupportDynamicType>>
function array_mark_legacy_recursive<T>(T $in)[]: T;
<<__SupportDynamicType>>
function array_unmark_legacy_recursive<T>(T $in)[]: T;
function is_array_marked_legacy(mixed $in)[]: bool;
}
namespace {
const float INF = 0.0;
const float NAN = 0.0;
<<__PHPStdLib>>
function connection_aborted();
<<__PHPStdLib>>
function connection_status();
<<__PHPStdLib>>
function connection_timeout();
<<__PHPStdLib>>
function constant(string $name);
<<__PHPStdLib>>
function defined(string $name, bool $autoload = true);
<<__PHPStdLib>>
function get_browser($user_agent = null, $return_array = false);
<<__PHPStdLib>>
function ignore_user_abort(bool $setting = false);
<<__PHPStdLib>>
function pack(string $format, ...$args)[];
<<__PHPStdLib>>
function sleep(int $seconds);
<<__PHPStdLib>>
function usleep(int $micro_seconds);
<<__PHPStdLib>>
function time_nanosleep(int $seconds, int $nanoseconds);
<<__PHPStdLib>>
function time_sleep_until(float $timestamp);
<<__PHPStdLib>>
function uniqid(string $prefix = "", bool $more_entropy = false);
<<__PHPStdLib>>
function unpack(string $format, string $data)[];
<<__PHPStdLib>>
function sys_getloadavg();
<<__PHPStdLib>>
function hphp_process_abort($magic);
<<__PHPStdLib>>
function hphp_to_string($v)[];
}
