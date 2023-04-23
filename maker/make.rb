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
  def make_name
    if self == "type"
      "r#type"
    else
      self
    end
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
  methods = []
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
  @date_flag = queries.filter{|it| it[:type] == "date" }.present?

  self_required = (queries + form).filter{|it| it[:required] } + paths
  required_queries = queries.filter{|it| it[:required] }
  non_required_queries = queries.filter{|it| !it[:required] }

  fields = queries.filter{|it| /\.fields$/ =~ it[:name]}

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

  expantions = (queries + form).filter{|it| (it[:type] == "enum" || it[:type] == "enum_single") && !(/\.fields$/ =~ it[:name]) }.map do |it|
    make_expantions(it)
  end
  bodies = []
  make_body(yml[:body], bodies) if yml[:body].present?

  response_list = []
  responses, refs, map = make_response("response", yml.dig(:response, :properties), false)
  response_list << responses if responses.present?
  if map.present?
    map.each_pair do |key, value|
      responses, _, _ = make_response(key.to_s, value.dig(:properties), false)
      response_list << responses
    end
  end

  serde_flag = @expantion_flag || bodies.present? || response_list.present?
  api_struct = ERB.new(File.read("api_struct.erb")).result(binding)
  api_new = ERB.new(File.read("api_new.erb")).result(binding).gsub(/^/, "    ")
  setter = ERB.new(File.read("setter.erb")).result(binding).gsub(/^/, "    ")
  parameters = ERB.new(File.read("parameters.erb")).result(binding).gsub(/^/, "        ")
  erb = ERB.new(File.read("api.erb"))
  File.write("../src/api/#{name}.rs", erb.result(binding))
end

def execute_expantions(path)
  m = /fields\/(.+)\.yaml/.match(path)
  name = m[1]
  yml = YAML.load_file(path).deep_symbolize_keys

  class_name = name.make_field.ucc + "Fields"
  src = yml[:value]
  ary = src.split(", ")
  all_flag = true
  use_flag = true
  methods = yml[:methods] || []
  erb = ERB.new(File.read("expantions.erb"))
  File.write("../src/fields/#{name}_fields.rs", erb.result(binding))
end

def calc_refs(name, properties, refs)
  properties.each_pair do |key, value|
    if value[:type] == "object"
      if value[:ref].present? && value[:ref] != name.to_s
        refs << value[:ref]
      elsif value[:properties].present?
        calc_refs(key, value[:properties], refs)
      end
    elsif value[:type] == "array" && value[:items][:type] == "object" && value[:items][:ref] != name.to_s
      refs << value[:items][:ref]
    end
  end
end

def make_response(name, properties, independence_flag)
  return ["", [], {}] if properties.blank?
  refs = []
  @enums = {}
  @inner_map = {}
  calc_refs(name, properties, refs)
  refs.uniq!
  class_name = name.make_field.ucc
  erb = ERB.new(File.read("responses.erb"))
  [erb.result(binding), refs, @inner_map]
end

def execute_responses(path)
  @date_flag = false
  m = /responses\/(.+)\.yaml/.match(path)
  name = m[1]
  yml = YAML.load_file(path).deep_symbolize_keys
  properties = yml[:response][:properties]
  independence_flag = true
  res, refs, map = make_response(name, properties, independence_flag)
  map.deep_dup.each_pair do |key, value|
    responses, _, _ = make_response(key.to_s, value.dig(:properties), false)
    res = res + "\n" + responses
  end

  File.write("../src/responses/#{name}.rs", res.gsub(/USE_DATE/, @date_flag ? "\nuse chrono::prelude::*;" : ""))
end

Dir.glob('api/*.yaml').each do |path|
  execute(path)
end

Dir.glob('fields/*.yaml').each do |path|
  execute_expantions(path)
end

Dir.glob('responses/*.yaml').each do |path|
  execute_responses(path)
end