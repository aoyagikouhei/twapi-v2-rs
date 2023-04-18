require 'active_support'
require 'active_support/core_ext'
require 'erb'
require 'camelizable'
require 'yaml'

class String
  include Camelizable
end

def execute(path)
  pp path
  m = /fields\/(.+)\.yaml/.match(path)
  pp m
  name = m[1]
  yml = YAML.load_file(path).deep_symbolize_keys

  class_name = "#{name.ucc}Fields"
  src = yml[:value]
  ary = src.split(", ")
  all_flag = true
  use_flag = true
  erb = ERB.new(File.read("expantions.erb"))
  File.write("../src/fields/#{name}_fields.rs", erb.result(binding))
end

Dir.glob('fields/*.yaml').each do |path|
  execute(path)
end
