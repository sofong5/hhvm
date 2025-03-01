<?hh
<<__EntryPoint>>
function entrypoint_autoload_006(): void {

  HH\autoload_set_paths(
  	dict[
  		'class' => dict[
  			'autoload_interface' => 'autoload_interface.p5c',
  			'autoload_implements' => 'autoload_implements.p5c',
  		],
  	],
  	__DIR__.'/',
  );

  var_dump(interface_exists('autoload_interface', false));
  var_dump(class_exists('autoload_implements', false));

  $o = new autoload_implements;
  var_dump($o);
  var_dump($o is autoload_interface);
  unset($o);

  var_dump(interface_exists('autoload_interface', false));
  var_dump(class_exists('autoload_implements', false));

  echo "===DONE===\n";
}
