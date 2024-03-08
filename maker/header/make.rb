require 'active_support'
require 'active_support/core_ext'
require 'erb'
require 'camelizable'
require 'yaml'

yml = YAML.load_file("#{__dir__}/header.yaml").deep_symbolize_keys
pp yml



erb = ERB.new(File.read("#{__dir__}/header.erb"))
File.write("#{__dir__}/../../src/headers.rs", erb.result(binding))