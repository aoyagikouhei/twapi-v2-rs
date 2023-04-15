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

yml = YAML.load_file(File.join('api', 'get_2_tweets_id_liking_users.yaml')).deep_symbolize_keys
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
  erb = ERB.new(File.read("expantions.erb"))
  erb.result(binding)
end

erb = ERB.new(File.read("api.erb"))
puts erb.result(binding)