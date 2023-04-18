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

def make_type(it)
  res = case it[:type]
  when "integer" then
    "usize"
  when "enum" then
    "HashSet<#{it[:name].ucc}>"
  when "enum_single" then
    "#{it[:name].ucc}"
  when "date" then
    "DateTime<Utc>"
  when "bool" then
    "bool"
  else
    "String"
  end
  if it[:required]
    res
  else
    "Option<#{res}>"
  end
end

def make_field_type(it)
  case it[:type]
  when "integer" then
    "usize"
  when "enum" then
    "HashSet<#{it[:name].ucc}>"
  when "enum_single" then
    "#{it[:name].ucc}"
  when "date" then
    "DateTime<Utc>"
  when "bool" then
    "bool"
  else
    "&str"
  end
end

def make_new_type(it)
  case it[:type]
  when "integer" then
    "usize"
  when "bool" then
    "bool"
  when "enum_single" then
    "#{it[:name].ucc}"
  else
    "&str"
  end
end

def make_query_value(it)
  case it[:type]
  when "enum" then
    ".iter().join(\",\")"
  when "string" then
    ""
  when "date" then
    ".format(\"%Y-%m-%dT%H%M%SZ\").to_string()"
  else
    ".to_string()"
  end
end

def required_new(it)
  case it[:type]
  when "string" then
    "#{it[:name].make_field}: #{it[:name].make_field}.to_owned()"    
  else
    "#{it[:name].make_field}"
  end
end

def make_type_simple(src)
  case src[:type]
  when "integer" then
    "usize"
  else
    "String"
  end
end

def make_body_type(src)
  res = case src[:type]
  when "enum_single" then
    src[:name].ucc
  when "string" then
    "String"
  when "object" then
    src[:name].ucc
  when "array" then
    "Vec<#{src[:items][:type] == "object" ? src[:name].singularize.ucc : make_type_simple(src)}>"
  else
    "String"
  end
  if !src[:required]
    "Option<#{res}>"
  else
    res
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
  path_parameters = paths.map{|it| ".replace(\":#{it[:name].make_field}\", &self.#{it[:name].make_field})"}
  enum_flag = queries.filter{|it| it[:type] == "enum"}.present?
  date_flag = queries.filter{|it| it[:type] == "date" }.present?

  required = (queries + form).filter{|it| it[:required] } + paths
  others = queries.filter{|it| !it[:required] }
  required_queries = queries.filter{|it| it[:required] }
  others_queries = queries.filter{|it| !it[:required] }

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
  new_array = auth[:keys].map{|it| "#{it}: &str"} + required.map{|it| "#{it[:name].make_field}: #{make_new_type(it)}"}
  serde_flag = @expantion_flag || bodies.present?
  new_array << "body: Body" if bodies.present?

  erb = ERB.new(File.read("api.erb"))
  File.write("../src/api/#{name}.rs", erb.result(binding))
end

Dir.glob('api/*.yaml').each do |path|
  execute(path)
end

