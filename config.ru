#!/usr/bin/env rackup

$LOAD_PATH.unshift(File.expand_path('../lib', __FILE__))
require 'rust_rack'

run RustRack
