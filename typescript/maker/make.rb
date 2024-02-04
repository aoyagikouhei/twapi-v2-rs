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
  erb = ERB.new(File.read("#{__dir__}/responses.erb"))
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

  File.write("#{__dir__}/../responses/#{name}.ts", res.gsub(/USE_DATE/, @date_flag ? "\nuse chrono::prelude::*;" : ""))
end

Dir.glob("#{__dir__}/../../maker/responses/*.yaml").each do |path|
  execute_responses(path)
end

