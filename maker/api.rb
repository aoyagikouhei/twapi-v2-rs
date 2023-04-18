require 'active_support'
require 'active_support/core_ext'
require 'erb'
require 'camelizable'
require 'yaml'

class String
  include Camelizable
  def make_field
    self.gsub(/\./, "_")
  end
end

# only object
def make_body(body, results, array_name=nil)
  body[:properties].each do |it|
    if it[:type] == "object"
      make_body(it, results)
    elsif it[:type] == "array" && it[:items][:type] == "object"
        make_body(it[:items], results, it[:name].singularize)
    elsif it[:type] == "enum_single"
      results << make_expantions(it)
    end
  end
  name = array_name || body[:name] || "body"
  properties = body[:properties]
  erb = ERB.new(File.read("body.erb"))
  results << erb.result(binding)
end

def make_expantions(it)
  @expantion_flag = true
  class_name = it[:name].make_field.ucc
  src = it[:value]
  ary = src.split(", ")
  all_flag = it[:type] == "enum"
  use_flag = false
  erb = ERB.new(File.read("expantions.erb"))
  erb.result(binding)
end

def execute(path)
  @expantion_flag = false
  m = /api\/(.+)\.yaml/.match(path)
  name = m[1]

  yml = YAML.load_file(path).deep_symbolize_keys
  paths = (yml[:paths] || []).map{|it| it[:required] = "true"; it }
  queries = yml[:queries] || []
  form = yml[:form] || []
  enum_flag = queries.filter{|it| it[:type] == "enum"}.present?
  date_flag = queries.filter{|it| it[:type] == "date" }.present?

  self_required = (queries + form).filter{|it| it[:required] } + paths
  required_queries = queries.filter{|it| it[:required] }
  non_required_queries = queries.filter{|it| !it[:required] }

  auth = if yml[:auth] == "basic" 
    {
      method: ".basic_auth(self.api_key_code, Some(self.api_secret_code))",
      keys: ["api_key_code", "api_secret_code"],
    }
  else
    {
      method: ".bearer_auth(self.bearer_code)",
      keys: ["bearer_code"],
    }
  end

  expantions = (queries + form).filter{|it| it[:type] == "enum" || it[:type] == "enum_single" }.map do |it|
    make_expantions(it)
  end
  bodies = []
  make_body(yml[:body], bodies) if yml[:body].present?
  serde_flag = @expantion_flag || bodies.present?

  api_struct = ERB.new(File.read("api_struct.erb")).result(binding)
  api_new = ERB.new(File.read("api_new.erb")).result(binding).gsub(/^/, "    ")
  setter = ERB.new(File.read("setter.erb")).result(binding).gsub(/^/, "    ")
  parameters = ERB.new(File.read("parameters.erb")).result(binding).gsub(/^/, "        ")

  erb = ERB.new(File.read("api.erb"))
  File.write("../src/api/#{name}.rs", erb.result(binding))
end

Dir.glob('api/*.yaml').each do |path|
  execute(path)
end

