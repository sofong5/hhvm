<?hh

// This error handler swallows typehint errors, which is
// disallowed in RepoAuthoritative mode. Thus, this test
// is set to be norepo.
function err($code, $msg) {
  echo "Handled {$code}: $msg", "\n";
  return true;
}

class CExplicit implements StringishObject{
  public function __toString()[] {
    return __CLASS__;
  }
}

class CImplicit {
  public function __toString()[] {
    return __CLASS__;
  }
}

trait TStringishObject {
  public function __toString()[] { return __TRAIT__; }
}

interface IStringishObject {
  public function __toString();
}

class CThruTrait {
  use TStringishObject;
}

function f1(<<__Soft>> ?StringishObject $x): void {
  $s = StringishObject::class;
  echo ($x is StringishObject) ? "true" : "false", ", ";
  echo (is_a($x, $s)) ? "true" : "false", ", ";
  var_dump($x);
  echo "\n";
}

function f2(<<__Soft>> StringishObject $x): void {
  $s = StringishObject::class;
  echo ($x is StringishObject) ? "true" : "false", ", ";
  echo (is_a($x, $s)) ? "true" : "false", ", ";
  var_dump($x);
  echo "\n";
}

function test_functionality() {
  echo '********** static string **********', "\n";
  f1("a boring string");
  f2("a boring string");

  echo '********** dynamic string **********', "\n";
  $x = "hello ";
  if (time() > 0) {
    $x .= "world";
  }
  f1($x);
  f2($x);

  echo '********** explicit implements **********', "\n";
  $explicit = new CExplicit();
  f1($explicit);
  f2($explicit);

  echo '********** implicit implements **********', "\n";
  $implicit = new CImplicit();
  f1($implicit);
  f2($implicit);

  echo '********** via trait implements **********', "\n";
  $via_trait = new CThruTrait();
  f1($via_trait);
  f2($via_trait);

  echo '********** null **********', "\n";
  f1(null);
  f2(null);

  echo '********** array **********', "\n";
  f1(varray[1, 2, 3]);
  f2(varray[1, 2, 3]);

  echo '********** number **********', "\n";
  f1(10);
  f2(10);
  f1(-4.2);
  f2(-4.2);
}

function test_reflection() {
  echo "\n",
    '--------------------', ' ', __FUNCTION__, ' ', '--------------------',
    "\n\n";
  var_dump(interface_exists(StringishObject::class));

  var_dump(is_a(CExplicit::class, StringishObject::class));
  var_dump(is_subclass_of(CExplicit::class, StringishObject::class));

  var_dump(is_a(CImplicit::class, StringishObject::class));
  var_dump(is_subclass_of(CImplicit::class, StringishObject::class));

  $rc = new ReflectionClass(CExplicit::class);
  var_dump($rc->getInterfaceNames());
  var_dump($rc->implementsInterface(StringishObject::class));
  var_dump($rc->isSubclassOf(StringishObject::class));
  var_dump($rc->implementsInterface(Stringish::class));
  var_dump($rc->isSubclassOf(Stringish::class));

  $rc = new ReflectionClass(CImplicit::class);
  var_dump($rc->getInterfaceNames());
  var_dump($rc->implementsInterface(StringishObject::class));
  var_dump($rc->isSubclassOf(StringishObject::class));
  var_dump($rc->implementsInterface(Stringish::class));
  var_dump($rc->isSubclassOf(Stringish::class));

  $rc = new ReflectionClass(CThruTrait::class);
  var_dump($rc->getInterfaceNames());
  var_dump($rc->implementsInterface(StringishObject::class));
  var_dump($rc->isSubclassOf(StringishObject::class));
  var_dump($rc->implementsInterface(Stringish::class));
  var_dump($rc->isSubclassOf(Stringish::class));

  $rc = new ReflectionClass(TStringishObject::class);
  var_dump($rc->getInterfaceNames());
  var_dump($rc->implementsInterface(StringishObject::class));
  var_dump($rc->isSubclassOf(StringishObject::class));
  var_dump($rc->implementsInterface(Stringish::class));
  var_dump($rc->isSubclassOf(Stringish::class));

  $rc = new ReflectionClass(IStringishObject::class);
  var_dump($rc->getInterfaceNames());
  var_dump($rc->implementsInterface(StringishObject::class));
  var_dump($rc->isSubclassOf(StringishObject::class));
  var_dump($rc->implementsInterface(Stringish::class));
  var_dump($rc->isSubclassOf(Stringish::class));

}

<<__EntryPoint>>
function main_stringish() {
set_error_handler(err<>);

test_functionality();
test_reflection();
}
