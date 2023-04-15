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
  else
    "&str"
  end
end

def make_new_type(it)
  case it[:type]
  when "integer" then
    usize
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
  when "integer" then
    "#{it[:name].make_field}"
  else
    "#{it[:name].make_field}: #{it[:name].make_field}.to_owned()"
  end
end

def execute(path)
  m = /api\/(.+)\.yaml/.match(path)
  name = m[1]

  yml = YAML.load_file(path).deep_symbolize_keys
  paths = yml[:params].filter{|it| it[:place] == "path" }.map{|it| ".replace(\":#{it[:name].make_field}\", &self.#{it[:name].make_field})"}

  fields = yml[:params].filter{|it| /.fields$/ =~ it[:name] }.map{|it| "#{it[:name].gsub(/\./, "_")}"}
  required = yml[:params].filter{|it| it[:required] }
  others = yml[:params].filter{|it| !it[:required] }
  required_queries = yml[:params].filter{|it| it[:place] == "query" && it[:required] }
  others_queries = yml[:params].filter{|it| it[:place] == "query" && !it[:required] }
  expantions = yml[:params].filter{|it| (it[:type] == "enum" || it[:type] == "enum_single") && !(/.fields$/ =~ it[:name]) }.map do |it|
    class_name = it[:name].make_field.ucc
    src = it[:value]
    ary = src.split(", ")
    all_flag = it[:type] == "enum"
    erb = ERB.new(File.read("expantions.erb"))
    erb.result(binding)
  end

  erb = ERB.new(File.read("api.erb"))
  File.write("../src/api/#{name}.rs", erb.result(binding))
end

Dir.glob('api/*.yaml').each do |path|
  execute(path)
end
